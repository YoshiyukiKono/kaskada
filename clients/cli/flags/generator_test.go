package flags

import (
	"testing"

	apiv1alpha "github.com/kaskada-ai/kaskada/gen/proto/go/kaskada/kaskada/v1alpha"
)

func TestGenerate(t *testing.T) {

	g := NewGenerator(&apiv1alpha.PulsarConfig{})

	g.Generate()
}
