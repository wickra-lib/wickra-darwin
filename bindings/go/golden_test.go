package wickra

// The cross-language golden invariant seen from Go: the same seed yields
// byte-identical output across calls and across instances. The response bytes are
// what every other binding produces too, because the whole search lives once in
// the Rust core and this binding forwards its JSON verbatim.

import (
	"encoding/json"
	"testing"
)

func TestEvolveByteIdenticalAcrossInstances(t *testing.T) {
	cmd := evolveCmd()

	a, err := New(spec)
	if err != nil {
		t.Fatal(err)
	}
	defer a.Close()
	b, err := New(spec)
	if err != nil {
		t.Fatal(err)
	}
	defer b.Close()

	ra, err := a.Command(cmd)
	if err != nil {
		t.Fatal(err)
	}
	rb, err := b.Command(cmd)
	if err != nil {
		t.Fatal(err)
	}
	if ra != rb {
		t.Fatalf("expected byte-identical output, got:\n a: %s\n b: %s", ra, rb)
	}
}

func TestDifferentSeedStillValid(t *testing.T) {
	other := `{"seed":99,"population":8,"generations":3,` +
		`"mutation_rate":0.2,"crossover_rate":0.6,"fitness":"sharpe",` +
		`"search_space":{"indicators":[{"name":"rsi","param_ranges":[{"min":2,"max":30,"step":1}]}],` +
		`"rules":"single_threshold","max_conditions":1},"elitism":1,"top":3}`

	d, err := New(other)
	if err != nil {
		t.Fatal(err)
	}
	defer d.Close()
	raw, err := d.Command(evolveCmd())
	if err != nil {
		t.Fatal(err)
	}
	var out struct {
		History []json.RawMessage `json:"history"`
	}
	if err := json.Unmarshal([]byte(raw), &out); err != nil {
		t.Fatal(err)
	}
	if len(out.History) == 0 {
		t.Fatal("expected a non-empty history")
	}
}
