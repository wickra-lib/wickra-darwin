package wickra

// The batch search, driven through the JSON command boundary.
//
// There is no streaming half in DARWIN: a search is a batch computation over a
// whole dataset. What stands in its place is the property that makes the batch
// usable -- the same seed reproduces the same search -- and it has to hold through
// this binding, not only in Rust. A binding that let anything of its own into the
// loop (a map iteration order, a locale-formatted number in the payload) would
// produce a search that is reproducible only on the machine that ran it.
//
// The second test names `macd`, which takes three parameters. Until the search
// space resolved through the registry, `indicator_kind` was a four-name allowlist
// that declared every indicator as taking one -- so this spec was unreachable
// twice over, and no binding could have run it.

import (
	"encoding/json"
	"fmt"
	"math"
	"strings"
	"testing"
)

func batchSpec(indicators string) string {
	return fmt.Sprintf(`{"seed":7,"population":8,"generations":3,`+
		`"mutation_rate":0.2,"crossover_rate":0.6,"fitness":"sharpe",`+
		`"elitism":1,"top":3,"search_space":{"indicators":[%s],`+
		`"rules":"single_threshold","max_conditions":2}}`, indicators)
}

const oneParameter = `{"name":"rsi","param_ranges":[{"min":2,"max":30,"step":1}]}`

// `macd` takes three parameters. The four-name allowlist this search space
// replaced declared every indicator as taking one, so this spec could not be
// expressed at all.
const threeParameters = `{"name":"macd","param_ranges":[` +
	`{"min":8,"max":16,"step":2},{"min":20,"max":30,"step":2},` +
	`{"min":5,"max":12,"step":1}]}`

func batchCandles(n int) string {
	parts := make([]string, 0, n)
	for i := 0; i < n; i++ {
		f := float64(i)
		close := 100.0 + 10.0*math.Sin(f*0.1) + 0.05*f
		opn := 100.0 + 10.0*math.Sin((f-1)*0.1) + 0.05*(f-1)
		high, low := close, opn
		if opn > close {
			high, low = opn, close
		}
		parts = append(parts, fmt.Sprintf(
			`{"time":%d,"open":%g,"high":%g,"low":%g,"close":%g,"volume":1000}`,
			1700000000+i*3600, opn, high+1.0, low-1.0, close))
	}
	return "[" + strings.Join(parts, ",") + "]"
}

func batchData() string {
	candles := batchCandles(250)
	return `{"AAA":` + candles + `,"BBB":` + candles + `}`
}

func batchSearch(t *testing.T, spec string) string {
	t.Helper()
	d, err := New(spec)
	if err != nil {
		t.Fatalf("new darwin: %v", err)
	}
	defer d.Close()
	out, err := d.Command(`{"cmd":"evolve","data":` + batchData() + `}`)
	if err != nil {
		t.Fatalf("evolve: %v", err)
	}
	return out
}

func TestTheBatchSearchIsReproducibleFromItsSeed(t *testing.T) {
	spec := batchSpec(oneParameter)
	if batchSearch(t, spec) != batchSearch(t, spec) {
		t.Error("the same seed produced two different searches")
	}
}

func TestAThreeParameterIndicatorIsSearchable(t *testing.T) {
	out := batchSearch(t, batchSpec(threeParameters))
	var report struct {
		History []json.RawMessage `json:"history"`
		Best    []json.RawMessage `json:"best"`
	}
	if err := json.Unmarshal([]byte(out), &report); err != nil {
		t.Fatalf("parse report: %v", err)
	}
	if len(report.History) == 0 || len(report.Best) == 0 {
		t.Errorf("the search produced nothing: %s", out)
	}
}

func TestANameTheRegistryDoesNotKnowIsRefused(t *testing.T) {
	spec := batchSpec(`{"name":"notanindicator","param_ranges":[{"min":2,"max":30,"step":1}]}`)
	if _, err := New(spec); err == nil {
		t.Error("an unknown indicator name must be refused")
	}
}
