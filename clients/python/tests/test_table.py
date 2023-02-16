from pathlib import Path
from unittest.mock import patch

import pytest

import kaskada.client
import kaskada.kaskada.v1alpha.common_pb2 as common_pb
import kaskada.kaskada.v1alpha.table_service_pb2 as table_pb
import kaskada.table


@patch("kaskada.client.Client")
def test_table_load_parquet(mockClient):
    table_name = "test_table"
    local_file = "local.parquet"
    expected_request = table_pb.LoadDataRequest(
        table_name=table_name,
        file_input=common_pb.FileInput(
            file_type="FILE_TYPE_PARQUET", uri=f"file:///{Path(local_file).absolute()}"
        ),
    )

    kaskada.table.load(table_name, local_file, client=mockClient)
    mockClient.table_stub.LoadData.assert_called_with(
        expected_request, metadata=mockClient.get_metadata()
    )


@patch("kaskada.client.Client")
def test_table_load_csv(mockClient):
    table_name = "test_table"
    local_file = "local.csv"
    expected_request = table_pb.LoadDataRequest(
        table_name=table_name,
        file_input=common_pb.FileInput(
            file_type="FILE_TYPE_CSV", uri=f"file:///{Path(local_file).absolute()}"
        ),
    )

    kaskada.table.load(table_name, local_file, client=mockClient)
    mockClient.table_stub.LoadData.assert_called_with(
        expected_request, metadata=mockClient.get_metadata()
    )


@patch("kaskada.client.Client")
def test_table_load_invalid_type(mockClient):
    table_name = "test_table"
    local_file = "local.img"
    with pytest.raises(Exception):
        kaskada.table.load(table_name, local_file, client=mockClient)