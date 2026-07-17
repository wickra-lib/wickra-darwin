package wickra

import (
	"encoding/json"
	"fmt"
	"math"
	"strings"
	"testing"
)

const spec = `{"seed":1,"population":8,"generations":3,` +
	`"mutation_rate":0.2,"crossover_rate":0.6,"fitness":"sharpe",` +
	`"search_space":{"indicators":[{"name":"rsi","param_ranges":[{"min":2,"max":30,"step":1}]}],` +
	`"rules":"single_threshold","max_conditions":1},"elitism":1,"top":3}`

// evolveCmd builds an evolve command over a deterministic sine price path.
func evolveCmd() string {
	var b strings.Builder
	b.WriteString(`{"cmd":"evolve","data":{"AAA":[`)
	for i := 0; i < 250; i++ {
		close := 100.0 + 10.0*math.Sin(float64(i)*0.1) + 0.05*float64(i)
		open := 100.0 + 10.0*math.Sin(float64(i-1)*0.1) + 0.05*float64(i-1)
		high := math.Max(close, open) + 1.0
		low := math.Min(close, open) - 1.0
		if i > 0 {
			b.WriteByte(',')
		}
		fmt.Fprintf(&b, `{"time":%d,"open":%g,"high":%g,"low":%g,"close":%g,"volume":1000.0}`,
			1700000000+i*3600, open, high, low, close)
	}
	b.WriteString(`]}}`)
	return b.String()
}

func TestVersion(t *testing.T) {
	if Version() == "" {
		t.Fatal("empty version")
	}
}

func TestEvolveOutput(t *testing.T) {
	d, err := New(spec)
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
		Best    []json.RawMessage `json:"best"`
	}
	if err := json.Unmarshal([]byte(raw), &out); err != nil {
		t.Fatal(err)
	}
	if len(out.History) != 4 {
		t.Fatalf("expected 4 history entries, got %d: %s", len(out.History), raw)
	}
}

func TestInvalidSpecIsError(t *testing.T) {
	if _, err := New("{ not valid json"); err == nil {
		t.Fatal("expected an error for an invalid spec")
	}
}

func TestSetSpecThenEvolve(t *testing.T) {
	d, err := New("{}")
	if err != nil {
		t.Fatal(err)
	}
	defer d.Close()
	ok, err := d.Command(`{"cmd":"set_spec","spec":` + spec + `}`)
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(ok, `"ok":true`) {
		t.Fatalf("expected ok:true, got: %s", ok)
	}
	if _, err := d.Command(evolveCmd()); err != nil {
		t.Fatal(err)
	}
}
