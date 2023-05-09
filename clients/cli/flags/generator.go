package flags

import (
	"fmt"

	g_proto "google.golang.org/protobuf/proto"

	"github.com/golang/protobuf/proto"
	"github.com/spf13/pflag"
	"google.golang.org/protobuf/runtime/protoiface"
	"google.golang.org/protobuf/types/descriptorpb"

	apiv1alpha "github.com/kaskada-ai/kaskada/gen/proto/go/kaskada/kaskada/v1alpha"
)

type generator struct {
	item protoiface.MessageV1
}

func NewGenerator(item protoiface.MessageV1) *generator {
	return &generator{item: item}
}

func (g *generator) Generate() *pflag.FlagSet {
	// Get the descriptor for the message type
	messageDescriptor := proto.MessageReflect(g.item).Descriptor()

	// Loop through the fields of the message type
	for i := 0; i < messageDescriptor.Fields().Len(); i++ {
		// Get the field descriptor for the current field
		fd := messageDescriptor.Fields().Get(i)
		opts := fd.Options().(*descriptorpb.FieldOptions)

		// Get the name and type of the field
		name := string(fd.Name())
		fieldType := fd.Kind()
		optional := fd.HasOptionalKeyword()
		sensitive := g_proto.GetExtension(opts, apiv1alpha.E_Sensitive).(bool)
		defaultString := g_proto.GetExtension(opts, apiv1alpha.E_DefaultStringValue).(string)

		// Print the name and type of the field
		fmt.Printf("Name: %s, Type: %s, Sensitive:%v, Default: %s,  Optional: %v\n", name, fieldType, sensitive, defaultString,  optional)

	}
	return nil
}
