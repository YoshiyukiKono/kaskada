use std::fs::File;

use error_stack::{IntoReportCompat, ResultExt};
use fallible_iterator::FallibleIterator;
use futures::TryStreamExt;
use sparrow_api::kaskada::v1alpha::compile_request::ExpressionKind;
use sparrow_api::kaskada::v1alpha::execute_request::output_to::Destination;
use sparrow_api::kaskada::v1alpha::execute_request::OutputTo;
use sparrow_api::kaskada::v1alpha::object_store_destination::FileFormat;
use sparrow_api::kaskada::v1alpha::{
    compute_table, file_path, CompileRequest, ComputeTable, ExecuteRequest, FeatureSet,
    FenlDiagnostics, ObjectStoreDestination, PerEntityBehavior, TableConfig, TableMetadata,
};
use sparrow_compiler::InternalCompileOptions;
use sparrow_qfr::kaskada::sparrow::v1alpha::FlightRecordHeader;
use sparrow_runtime::s3::S3Helper;
use sparrow_runtime::PreparedMetadata;
use tempfile::NamedTempFile;
use uuid::Uuid;

use crate::structs::{ExampleExpression, ExampleTable, FunctionExample};

#[derive(derive_more::Display, Debug)]
pub enum Error {
    #[display(fmt = "failed to prepare input")]
    PrepareInput,
    #[display(fmt = "failed to compile query")]
    CompileQuery,
    #[display(fmt = "failed to compile query:\n{_0}")]
    QueryErrors(FenlDiagnostics),
    #[display(fmt = "failed to execute query")]
    ExecuteQuery,
}

#[derive(derive_more::Display, Debug)]
#[display(fmt = "for table '{_0}'")]
pub struct TableName(String);

#[derive(derive_more::Display, Debug)]
#[display(fmt = "for example '{_0}'")]
pub struct Example(String);

impl error_stack::Context for Error {}

/// Execute the example and return the result as a CSV string.
pub(super) async fn execute_example(
    example: &FunctionExample,
    s3_helper: S3Helper,
) -> error_stack::Result<String, Error> {
    // 1. Prepare the file
    let mut preparer = ExampleInputPreparer::new();
    let tables = preparer.prepare_inputs(&example.input_csv, &example.tables)?;

    let query = match &example.expression {
        ExampleExpression::Expression(simple) => {
            // We start with the `result` in the base record so it comes last.
            format!("{{ result: {simple} }} | extend(Input)")
        }
        ExampleExpression::FullExpression(full) => full.clone(),
    };
    let result = sparrow_compiler::compile_proto(
        CompileRequest {
            tables: tables.clone(),
            feature_set: Some(FeatureSet {
                formulas: vec![],
                query,
            }),
            slice_request: None,
            expression_kind: ExpressionKind::Complete as i32,
            experimental: false,
            per_entity_behavior: PerEntityBehavior::All as i32,
        },
        InternalCompileOptions::default(),
    )
    .await
    .change_context(Error::CompileQuery)?;

    error_stack::ensure!(
        result.plan.is_some(),
        Error::QueryErrors(result.fenl_diagnostics.unwrap_or_default())
    );

    let tempdir = tempfile::Builder::new()
        .prefix("example")
        .tempdir()
        .unwrap();

    let destination = ObjectStoreDestination {
        output_prefix_uri: format!("file:///{}", tempdir.path().display()),
        format: FileFormat::Csv.into(),
    };
    let output_to = OutputTo {
        destination: Some(Destination::ObjectStore(destination)),
    };

    let stream = sparrow_runtime::execute::execute(
        ExecuteRequest {
            plan: result.plan,
            tables,
            output_to: Some(output_to),
            limits: None,
            compute_snapshot_config: None,
            changed_since: None,
            final_result_time: None,
        },
        s3_helper,
        None,
        FlightRecordHeader::default(),
    )
    .await
    .change_context(Error::ExecuteQuery)?;

    let output_paths = stream
        .map_ok(|item| item.output_paths.unwrap_or_default().paths)
        .try_concat()
        .await
        .unwrap();

    assert_eq!(
        output_paths.len(),
        1,
        "Expected one output, but got {output_paths:?}"
    );

    assert!(
        output_paths[0].starts_with("file:///"),
        "expected local file prefix"
    );
    let output_path = output_paths[0].strip_prefix("file:///").unwrap();
    let output_path = std::path::Path::new(output_path);

    // Drop the first four (key columns).
    //
    // Note: This currently writes to CSV and then parses it.
    // There may be faster ways, but this lets us re-use the existing functionality
    // to write to CSV rather than creating special functionality just for examples.
    // We could also consider other options for removing the rows (regex, etc.)
    // but this works.
    let mut table = prettytable::Table::from_csv_file(output_path).unwrap();
    for row in table.row_iter_mut() {
        row.remove_cell(0);
        row.remove_cell(0);
        row.remove_cell(0);
        row.remove_cell(0);
    }

    let content =
        String::from_utf8(table.to_csv(Vec::new()).unwrap().into_inner().unwrap()).unwrap();

    tempdir.close().unwrap();

    Ok(content)
}

struct ExampleInputPreparer {
    /// Holds the prepared files so they aren't dropped this is.
    prepared_files: Vec<NamedTempFile>,
}

impl ExampleInputPreparer {
    fn new() -> Self {
        Self {
            prepared_files: vec![],
        }
    }

    // TODO: Use the DataFixture to accomplish this?
    fn prepare_inputs(
        &mut self,
        input_csv: &Option<String>,
        tables: &[ExampleTable],
    ) -> error_stack::Result<Vec<ComputeTable>, Error> {
        let mut prepared_tables = Vec::with_capacity(tables.len() + 1);
        if let Some(input_csv) = input_csv {
            prepared_tables.push(
                self.prepare_input(
                    TableConfig::new("Input", &Uuid::new_v4(), "time", None, "key", "grouping"),
                    input_csv,
                )
                .attach_printable_lazy(|| TableName("Input".to_owned()))?,
            );
        }

        for table in tables {
            prepared_tables.push(
                self.prepare_input(table.table_config.clone(), &table.input_csv)
                    .attach_printable_lazy(|| TableName(table.table_config.name.clone()))?,
            );
        }

        Ok(prepared_tables)
    }

    fn prepare_input(
        &mut self,
        config: TableConfig,
        input_csv: &str,
    ) -> error_stack::Result<ComputeTable, Error> {
        let prepared_batches: Vec<_> = sparrow_runtime::prepare::prepared_batches(
            &file_path::Path::CsvData(input_csv.to_owned()),
            &config,
            &None,
        )
        .change_context(Error::PrepareInput)?
        .collect()
        .change_context(Error::PrepareInput)?;

        let (prepared_batch, metadata) = &prepared_batches[0];

        let prepared_file = tempfile::Builder::new()
            .suffix(".parquet")
            .tempfile()
            .unwrap();
        let output_file = File::create(prepared_file.path()).unwrap();
        let mut output = parquet::arrow::arrow_writer::ArrowWriter::try_new(
            output_file,
            prepared_batch.schema(),
            None,
        )
        .unwrap();
        output.write(prepared_batch).unwrap();
        output.close().unwrap();

        let metadata_file = tempfile::Builder::new()
            .suffix(".parquet")
            .tempfile()
            .unwrap();
        let metadata_output_file = File::create(metadata_file.path()).unwrap();
        let mut metadata_output = parquet::arrow::arrow_writer::ArrowWriter::try_new(
            metadata_output_file,
            metadata.schema(),
            None,
        )
        .unwrap();
        metadata_output.write(metadata).unwrap();
        metadata_output.close().unwrap();

        let prepared_metadata = PreparedMetadata::try_from_local_parquet_path(
            prepared_file.path(),
            metadata_file.path(),
        )
        .into_report()
        .change_context(Error::PrepareInput)?;
        let table_metadata = TableMetadata {
            schema: Some(prepared_metadata.table_schema.as_ref().try_into().unwrap()),
            file_count: 1,
        };
        self.prepared_files.push(prepared_file);
        self.prepared_files.push(metadata_file);

        Ok(ComputeTable {
            config: Some(config),
            file_sets: vec![compute_table::FileSet {
                slice_plan: None,
                prepared_files: vec![prepared_metadata.try_into().unwrap()],
            }],
            metadata: Some(table_metadata),
        })
    }
}