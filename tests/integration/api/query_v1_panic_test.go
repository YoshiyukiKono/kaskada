package api_test

import (
	"context"
	"os"

	. "github.com/onsi/ginkgo/v2"
	. "github.com/onsi/gomega"
	. "github.com/onsi/gomega/gstruct"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/status"
	"google.golang.org/protobuf/types/known/wrapperspb"

	. "github.com/kaskada/kaskada-ai/tests/integration/api/matchers"
	v1alpha "github.com/kaskada/kaskada-ai/wren/gen/kaskada/kaskada/v1alpha"
)

var _ = Describe("Query V1 when Sparrow panics", func() {
	var ctx context.Context
	var cancel context.CancelFunc
	var conn *grpc.ClientConn
	var tableClient v1alpha.TableServiceClient
	var queryClient v1alpha.QueryServiceClient
	var tableName string

	BeforeEach(func() {
		if os.Getenv("LOCAL") == "true" {
			Skip("tests running locally, skipping sparrow panic test")
		}

		//get connection to wren
		ctx, cancel, conn = getContextCancelConnection(30)
		ctx = metadata.AppendToOutgoingContext(ctx, "client-id", *integrationClientID)

		// get a grpc client for the table & compute services
		tableClient = v1alpha.NewTableServiceClient(conn)
		queryClient = v1alpha.NewQueryServiceClient(conn)

		tableName = "query_v1_panic"

		// create table, load table data
		table := &v1alpha.Table{
			TableName:           tableName,
			TimeColumnName:      "purchase_time",
			EntityKeyColumnName: "customer_id",
			SubsortColumnName: &wrapperspb.StringValue{
				Value: "subsort_id",
			},
		}
		_, err := tableClient.CreateTable(ctx, &v1alpha.CreateTableRequest{Table: table})
		Expect(err).ShouldNot(HaveOccurredGrpc())
		loadTestFileIntoTable(ctx, conn, table, "purchases/purchases_part1.parquet")
	})

	AfterEach(func() {
		// delete table
		_, err := tableClient.DeleteTable(ctx, &v1alpha.DeleteTableRequest{TableName: tableName})
		Expect(err).ShouldNot(HaveOccurredGrpc())

		cancel()
		conn.Close()
	})

	It("should be reported in a timely manner", func() {
		createQueryRequest := &v1alpha.CreateQueryRequest{
			Query: &v1alpha.Query{
				Expression:     "__INTERNAL_COMPILE_PANIC__",
				ResponseAs:     &v1alpha.Query_AsFiles{AsFiles: &v1alpha.AsFiles{FileType: v1alpha.FileType_FILE_TYPE_PARQUET}},
				ResultBehavior: v1alpha.Query_RESULT_BEHAVIOR_ALL_RESULTS,
			},
		}

		stream, err := queryClient.CreateQuery(ctx, createQueryRequest)
		Expect(err).ShouldNot(HaveOccurredGrpc())
		Expect(stream).ShouldNot(BeNil())

		res, err := getMergedCreateQueryResponse(stream)
		Expect(err).Should(HaveOccurred())
		Expect(res).Should(BeNil())

		//inspect error response
		errStatus, ok := status.FromError(err)
		Expect(ok).Should(BeTrue())
		Expect(errStatus.Code()).Should(Equal(codes.Internal))
		Expect(errStatus.Message()).Should(ContainSubstring("internal error"))
	})

	It("should support queries after ", func() {
		// First, cause a panic.
		createQueryRequest := &v1alpha.CreateQueryRequest{
			Query: &v1alpha.Query{
				Expression:     "__INTERNAL_COMPILE_PANIC__",
				ResponseAs:     &v1alpha.Query_AsFiles{AsFiles: &v1alpha.AsFiles{FileType: v1alpha.FileType_FILE_TYPE_PARQUET}},
				ResultBehavior: v1alpha.Query_RESULT_BEHAVIOR_ALL_RESULTS,
			},
		}

		stream, err := queryClient.CreateQuery(ctx, createQueryRequest)
		Expect(err).ShouldNot(HaveOccurredGrpc())
		Expect(stream).ShouldNot(BeNil())

		res, err := getMergedCreateQueryResponse(stream)
		Expect(err).Should(HaveOccurred())
		Expect(res).Should(BeNil())

		// inspect error response
		errStatus, ok := status.FromError(err)
		Expect(ok).Should(BeTrue())
		Expect(errStatus.Code()).Should(Equal(codes.Internal))
		Expect(errStatus.Message()).Should(ContainSubstring("internal error"))

		// Then, run a query and verify we get the right results
		createQueryRequest = &v1alpha.CreateQueryRequest{
			Query: &v1alpha.Query{
				Expression: `
{
time: query_v1_panic.purchase_time,
entity: query_v1_panic.customer_id,
max_amount: query_v1_panic.amount | max(),
min_amount: query_v1_panic.amount | min(),
}`,
				ResponseAs:     &v1alpha.Query_AsFiles{AsFiles: &v1alpha.AsFiles{FileType: v1alpha.FileType_FILE_TYPE_PARQUET}},
				ResultBehavior: v1alpha.Query_RESULT_BEHAVIOR_ALL_RESULTS,
			},
		}

		stream, err = queryClient.CreateQuery(ctx, createQueryRequest)
		Expect(err).ShouldNot(HaveOccurredGrpc())
		Expect(stream).ShouldNot(BeNil())

		res, err = getMergedCreateQueryResponse(stream)
		Expect(err).ShouldNot(HaveOccurred())

		VerifyRequestDetails(res.RequestDetails)
		Expect(res.GetFileResults()).ShouldNot(BeNil())
		Expect(res.GetFileResults().Paths).Should(HaveLen(1))

		Expect(res.Analysis.Schema).Should(ContainElements(
			primitiveSchemaField("time", v1alpha.DataType_PRIMITIVE_TYPE_TIMESTAMP_NANOSECOND),
			primitiveSchemaField("entity", v1alpha.DataType_PRIMITIVE_TYPE_STRING),
			primitiveSchemaField("max_amount", v1alpha.DataType_PRIMITIVE_TYPE_I64),
			primitiveSchemaField("min_amount", v1alpha.DataType_PRIMITIVE_TYPE_I64),
		))

		resultsUrl := res.GetFileResults().Paths[0]
		firstResults := downloadParquet(resultsUrl)

		Expect(firstResults).Should(HaveLen(10))
		Expect(firstResults[9]).Should(MatchFields(IgnoreExtras, Fields{
			"Time":       PointTo(BeEquivalentTo(1578182400000000000)),
			"Entity":     PointTo(Equal("patrick")),
			"Max_amount": PointTo(BeEquivalentTo(5000)),
			"Min_amount": PointTo(BeEquivalentTo(3)),
		}))
	})
})