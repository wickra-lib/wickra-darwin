// A runnable Go example: evolve strategy specs over a small deterministic
// universe and print the search summary.
//
//	go run examples/go/evolve.go
//
// Every language example builds the same universe and prints the same summary.
package main

import (
	"encoding/json"
	"fmt"
	"math"
	"strings"

	wickra "github.com/wickra-lib/wickra-darwin/bindings/go"
)

const spec = `{"seed":7,"population":10,"generations":4,` +
	`"mutation_rate":0.2,"crossover_rate":0.6,"fitness":"sharpe",` +
	`"search_space":{"indicators":[{"name":"rsi","param_ranges":[{"min":2,"max":30}]}],` +
	`"rules":"single_threshold","max_conditions":1},"elitism":1,"top":3}`

func evolveCommand() string {
	var b strings.Builder
	b.WriteString(`{"cmd":"evolve","data":{"SYM":[`)
	for i := 0; i < 16; i++ {
		close := 100.0 + 8.0*math.Sin(float64(i)/4.0) + 0.1*float64(i)
		open := 100.0 + 8.0*math.Sin(float64(i-1)/4.0) + 0.1*float64(i-1)
		if i > 0 {
			b.WriteByte(',')
		}
		fmt.Fprintf(&b, `{"time":%d,"open":%.3f,"high":%.3f,"low":%.3f,"close":%.3f,"volume":1000}`,
			1700000000+i*3600, open, math.Max(close, open)+1.0, math.Min(close, open)-1.0, close)
	}
	b.WriteString(`]}}`)
	return b.String()
}

func main() {
	darwin, err := wickra.New(spec)
	if err != nil {
		panic(err)
	}
	defer darwin.Close()
	raw, err := darwin.Command(evolveCommand())
	if err != nil {
		panic(err)
	}
	var report struct {
		History []json.RawMessage `json:"history"`
		Best    []json.RawMessage `json:"best"`
	}
	if err := json.Unmarshal([]byte(raw), &report); err != nil {
		panic(err)
	}
	fmt.Printf("wickra-darwin %s\n", wickra.Version())
	fmt.Printf("generations: %d\n", len(report.History))
	fmt.Printf("hall of fame: %d\n", len(report.Best))
}
