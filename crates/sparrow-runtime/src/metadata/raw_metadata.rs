use std::io::{BufReader, Cursor};
use std::str::FromStr;
use std::sync::Arc;

use arrow::array::ArrowPrimitiveType;
use arrow::datatypes::{DataType, Field, Schema, SchemaRef, TimestampMillisecondType};
use error_stack::{IntoReport, IntoReportCompat, ResultExt};
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use tempfile::NamedTempFile;

use sparrow_api::kaskada::v1alpha::source_data::{self, Source};

use sparrow_api::kaskada::v1alpha::PulsarConfig;

use crate::metadata::file_from_path;
use crate::stores::object_store_url::ObjectStoreKey;
use crate::stores::{ObjectStoreRegistry, ObjectStoreUrl};
use crate::streams;

#[derive(derive_more::Display, Debug)]
pub enum Error {
    #[display(fmt = "object store error for path: {_0}")]
    ObjectStore(String),
    #[display(fmt = "no object store registry")]
    MissingObjectStoreRegistry,
    #[display(fmt = "local file error")]
    LocalFile,
    #[display(fmt = "download error")]
    Download,
    #[display(fmt = "reading schema error")]
    ReadSchema,
    #[display(fmt = "pulsar subscription error")]
    PulsarSubscription,
    #[display(fmt = "failed to get pulsar schema: {_0}")]
    PulsarSchema(String),
    #[display(fmt = "unsupport column detected: '{_0}")]
    UnsupportedColumn(String),
}

impl error_stack::Context for Error {}

#[non_exhaustive]
pub struct RawMetadata {
    /// The raw schema of the data source backing this metadata.
    pub raw_schema: SchemaRef,
    /// The schema of the data source as presented to the user.
    ///
    /// This is the result of applying schema conversions to the raw schema,
    /// such as removing time zones, dropping decimal columns, etc.
    pub table_schema: SchemaRef,
}

/// For Pulsar, we want to keep the original user_schema around for use
/// by the consumer.  This is because we want the RawMetadata.raw_schema
/// to include the publish time metadata, but if we include that when creating the
/// Pulsar consumer, Pulsar will correctly reject it as a schema mismatch.
pub struct PulsarMetadata {
    /// the schema as defined by the user on the topic, corresponding to the messages created
    /// with no additional metadata
    pub user_schema: SchemaRef,
    /// schema that includes metadata used by Sparrow
    pub sparrow_metadata: RawMetadata,
}

impl RawMetadata {
    pub async fn try_from(
        source: &Source,
        object_store_registry: &ObjectStoreRegistry,
    ) -> error_stack::Result<Self, Error> {
        match source {
            source_data::Source::ParquetPath(path) => {
                Self::try_from_parquet(path, object_store_registry).await
            }
            source_data::Source::CsvPath(path) => {
                Self::try_from_csv(path, object_store_registry).await
            }
            source_data::Source::CsvData(content) => {
                let string_reader = BufReader::new(Cursor::new(content));
                Self::try_from_csv_reader(string_reader)
            }
            source_data::Source::PulsarSubscription(ps) => {
                let config = ps.config.as_ref().ok_or(Error::PulsarSubscription)?;
                Ok(Self::try_from_pulsar(config).await?.sparrow_metadata)
            }
        }
    }

    /// Create `RawMetadata` from a raw schema.
    pub fn from_raw_schema(raw_schema: SchemaRef) -> error_stack::Result<Self, Error> {
        // Convert the raw schema to a table schema.
        let table_schema = convert_schema(raw_schema.as_ref())?;

        Ok(Self {
            raw_schema,
            table_schema,
        })
    }

    /// Create a `RawMetadata` from a parquet string path and object store registry
    async fn try_from_parquet(
        path: &str,
        object_store_registry: &ObjectStoreRegistry,
    ) -> error_stack::Result<Self, Error> {
        let object_store_url = ObjectStoreUrl::from_str(path)
            .change_context_lazy(|| Error::ObjectStore(path.to_owned()))?;
        let object_store_key = object_store_url
            .key()
            .change_context_lazy(|| Error::ObjectStore(path.to_owned()))?;
        match object_store_key {
            ObjectStoreKey::Local => {
                let path = object_store_url
                    .path()
                    .change_context_lazy(|| Error::ObjectStore(path.to_owned()))?
                    .to_string();
                // The local paths are formatted file:///absolute/path/to/file.file
                // The Object Store path strips the prefix file:/// but we need to add the
                // root slash back prior to opening the file.
                let path = format!("/{}", path);
                let path = std::path::Path::new(&path);
                Self::try_from_parquet_path(path)
            }
            _ => {
                let download_file = NamedTempFile::new().map_err(|_| Error::Download)?;
                object_store_url
                    .download(object_store_registry, download_file.path())
                    .await
                    .change_context_lazy(|| Error::Download)?;
                Self::try_from_parquet_path(download_file.path())
            }
        }
    }

    /// Create a `RawMetadata` from a CSV string path and object store registry
    async fn try_from_csv(
        path: &str,
        object_store_registry: &ObjectStoreRegistry,
    ) -> error_stack::Result<Self, Error> {
        let object_store_url = ObjectStoreUrl::from_str(path)
            .change_context_lazy(|| Error::ObjectStore(path.to_owned()))?;
        let object_store_key = object_store_url
            .key()
            .change_context_lazy(|| Error::ObjectStore(path.to_owned()))?;
        match object_store_key {
            ObjectStoreKey::Local => {
                let path = object_store_url
                    .path()
                    .change_context_lazy(|| Error::ObjectStore(path.to_owned()))?
                    .to_string();
                let path = format!("/{}", path);
                let file = file_from_path(std::path::Path::new(&path))
                    .into_report()
                    .change_context_lazy(|| Error::LocalFile)?;
                Self::try_from_csv_reader(file)
            }
            _ => {
                let download_file = NamedTempFile::new()
                    .into_report()
                    .change_context_lazy(|| Error::Download)?;
                object_store_url
                    .download(object_store_registry, download_file.path())
                    .await
                    .change_context_lazy(|| Error::Download)?;
                let file = file_from_path(download_file.path())
                    .into_report()
                    .change_context_lazy(|| Error::Download)?;
                Self::try_from_csv_reader(file)
            }
        }
    }

    /// Create a `RawMetadata` from a Pulsar topic.
    pub(crate) async fn try_from_pulsar(
        config: &PulsarConfig,
    ) -> error_stack::Result<PulsarMetadata, Error> {
        // the user-defined schema in the topic
        let pulsar_schema = streams::pulsar::schema::get_pulsar_schema(
            config.admin_service_url.as_str(),
            config.tenant.as_str(),
            config.namespace.as_str(),
            config.topic_name.as_str(),
            config.auth_params.as_str(),
        )
        .await
        .change_context_lazy(|| Error::PulsarSchema("unable to get schema".to_owned()))?;

        // inject _publish_time field so that we have a consistent column to sort on
        // (this will always be our time_column in Pulsar sources)
        let publish_time = Field::new("_publish_time", TimestampMillisecondType::DATA_TYPE, false);
        let mut new_fields = pulsar_schema.fields.clone();
        new_fields.push(publish_time);
        tracing::debug!("pulsar schema fields: {:?}", new_fields);

        Ok(PulsarMetadata {
            user_schema: Arc::new(pulsar_schema),
            sparrow_metadata: Self::from_raw_schema(Arc::new(Schema::new(new_fields)))?,
        })
    }

    /// Create a `RawMetadata` fram a Parquet file path.
    fn try_from_parquet_path(path: &std::path::Path) -> error_stack::Result<Self, Error> {
        let file = file_from_path(path)
            .into_report()
            .change_context_lazy(|| Error::LocalFile)?;
        let parquet_reader = ParquetRecordBatchReaderBuilder::try_new(file)
            .into_report()
            .change_context_lazy(|| Error::ReadSchema)?;
        let raw_schema = parquet_reader.schema();
        Self::from_raw_schema(raw_schema.clone())
    }

    /// Create a `RawMetadata` from a reader of a CSV file or string.
    fn try_from_csv_reader<R>(reader: R) -> error_stack::Result<Self, Error>
    where
        R: std::io::Read + std::io::Seek,
    {
        use arrow::csv::ReaderBuilder;

        let raw_reader = ReaderBuilder::new()
            .has_header(true)
            // We only need the first row to find the minimum timestamp.
            .with_batch_size(1)
            // Use up to 1000 records to infer schemas.
            //
            // CSV is mostly used for small tests, so we expect to get enough
            // information about a given CSV file pretty quick. If this doesn't,
            // we can increase, or allow the user to specify the schema.
            .infer_schema(Some(1000))
            .build(reader)
            .into_report()
            .change_context_lazy(|| Error::ReadSchema)?;

        let raw_schema = raw_reader.schema();
        Self::from_raw_schema(raw_schema)
    }
}

/// Converts the schema to a table schema
fn convert_schema(schema: &Schema) -> error_stack::Result<SchemaRef, Error> {
    let fields = schema
        .fields()
        .iter()
        .map(convert_field)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Arc::new(Schema::new(fields)))
}

/// Arrow doesn't support time zones very well; it assumes all have a time zone
/// of `None`, which will use system time. Sparrow only operates on
/// [arrow::datatypes::TimestampNanosecondType], and currently cannot pass
/// through a time zone. In order to load and operate on time-zoned data, this
/// is a hack that forcibly casts all timestamp types to a time zone of `None`.
/// This can cause incorrect errors and possible ordering problems when multiple
/// input files have different time zones.
///
/// Arrow also does not support Decimal types. As of now, we are currently
/// dropping the columns that are Decimal types since we do not support at query
/// time either.
fn convert_field(field: &Field) -> error_stack::Result<Field, Error> {
    match field.data_type() {
        DataType::Timestamp(time_unit, Some(tz)) => {
            // TODO: We discard this because the conversion from an Arrow
            // schema to the Schema protobuf currently fails on such timestamp columns.
            tracing::warn!(
                "Time zones are unsupported. Interpreting column '{}' with time zone '{}' as UTC",
                tz,
                field.name()
            );
            Ok(Field::new(
                field.name(),
                DataType::Timestamp(time_unit.clone(), None),
                field.is_nullable(),
            ))
        }
        DataType::Decimal128(_, _) | DataType::Decimal256(_, _) => {
            tracing::warn!("Decimal columns are unsupported: '{}'", field.name());
            error_stack::bail!(Error::UnsupportedColumn(format!(
                "Decimal columns are unsupported: {}",
                field.name()
            )))
        }
        _ => Ok(field.clone()),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use arrow::datatypes::{DataType, Field, Schema, TimeUnit};

    use crate::RawMetadata;

    #[test]
    fn test_raw_metadata() {
        let raw_schema = Arc::new(Schema::new(vec![
            Field::new(
                "time",
                DataType::Timestamp(TimeUnit::Nanosecond, None),
                false,
            ),
            Field::new("subsort", DataType::UInt64, false),
            Field::new("key", DataType::UInt64, false),
            Field::new("a", DataType::Int64, true),
            Field::new("b", DataType::Int64, true),
            Field::new("c", DataType::Int64, true),
        ]));

        let metadata = RawMetadata::from_raw_schema(raw_schema.clone()).unwrap();
        assert_eq!(metadata.raw_schema, raw_schema);
        assert_eq!(metadata.table_schema, raw_schema);
    }

    #[test]
    fn test_raw_metadata_conversion() {
        let raw_schema = Arc::new(Schema::new(vec![
            Field::new("time", DataType::Utf8, false),
            // Time zone should be removed.
            Field::new(
                "time_zone",
                DataType::Timestamp(TimeUnit::Nanosecond, Some("UTC".to_owned())),
                false,
            ),
            Field::new("subsort", DataType::UInt64, false),
            Field::new("key", DataType::UInt64, false),
            Field::new("a", DataType::Int64, true),
            Field::new("b", DataType::Int64, true),
            Field::new("c", DataType::Int64, true),
        ]));

        let converted_schema = Arc::new(Schema::new(vec![
            Field::new("time", DataType::Utf8, false),
            Field::new(
                "time_zone",
                DataType::Timestamp(TimeUnit::Nanosecond, None),
                false,
            ),
            Field::new("subsort", DataType::UInt64, false),
            Field::new("key", DataType::UInt64, false),
            Field::new("a", DataType::Int64, true),
            Field::new("b", DataType::Int64, true),
            Field::new("c", DataType::Int64, true),
        ]));

        let metadata = RawMetadata::from_raw_schema(raw_schema.clone()).unwrap();
        assert_eq!(metadata.raw_schema, raw_schema);
        assert_eq!(metadata.table_schema, converted_schema);
    }

    #[test]
    fn test_raw_metadata_timestamp_drop_timezones() {
        let raw_schema = Arc::new(Schema::new(vec![
            Field::new("time", DataType::Utf8, false),
            // Time zone should be removed.
            Field::new(
                "time_zone_micro",
                DataType::Timestamp(TimeUnit::Microsecond, Some("UTC".to_owned())),
                false,
            ),
            Field::new(
                "time_zone_nano",
                DataType::Timestamp(TimeUnit::Nanosecond, Some("UTC".to_owned())),
                false,
            ),
            Field::new(
                "time_zone_second",
                DataType::Timestamp(TimeUnit::Second, Some("UTC".to_owned())),
                false,
            ),
            Field::new(
                "time_zone_milli",
                DataType::Timestamp(TimeUnit::Millisecond, Some("UTC".to_owned())),
                false,
            ),
        ]));

        let converted_schema = Arc::new(Schema::new(vec![
            Field::new("time", DataType::Utf8, false),
            Field::new(
                "time_zone_micro",
                DataType::Timestamp(TimeUnit::Microsecond, None),
                false,
            ),
            Field::new(
                "time_zone_nano",
                DataType::Timestamp(TimeUnit::Nanosecond, None),
                false,
            ),
            Field::new(
                "time_zone_second",
                DataType::Timestamp(TimeUnit::Second, None),
                false,
            ),
            Field::new(
                "time_zone_milli",
                DataType::Timestamp(TimeUnit::Millisecond, None),
                false,
            ),
        ]));

        let metadata = RawMetadata::from_raw_schema(raw_schema.clone()).unwrap();
        assert_eq!(metadata.raw_schema, raw_schema);
        assert_eq!(metadata.table_schema, converted_schema);
    }

    #[test]
    fn test_raw_metadata_decimal_errors() {
        let raw_schema = Arc::new(Schema::new(vec![Field::new(
            "decimal_col",
            DataType::Decimal128(0, 0),
            false,
        )]));

        let metadata = RawMetadata::from_raw_schema(raw_schema.clone());
        match metadata {
            Ok(_) => panic!("should not have succeeded"),
            Err(e) => {
                assert_eq!(
                    e.as_error().to_string(),
                    "unsupport column detected: 'Decimal columns are unsupported: decimal_col"
                )
            }
        }
    }
}
