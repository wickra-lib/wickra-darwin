// A runnable C++ example: evolve strategy specs over a small deterministic
// universe and print the search summary.
//
// This goes through `wickra_darwin.hpp`, the C++ hull shipped beside the C
// header, because that hull is what a C++ caller is meant to use: it owns and
// frees the handle, runs the two-call length protocol behind
// `wickra_darwin_command` for you, and turns a refusal into an exception rather
// than a negative integer that is easy to ignore. Calling the C functions
// directly from C++ works too -- `evolve.c` shows that -- but then the hull
// would be shipped without anything building it.
#include <cmath>
#include <cstdio>
#include <string>

#include "wickra_darwin.hpp"

static const char *SPEC =
    "{\"seed\":7,\"population\":10,\"generations\":4,"
    "\"mutation_rate\":0.2,\"crossover_rate\":0.6,\"fitness\":\"sharpe\","
    "\"search_space\":{\"indicators\":[{\"name\":\"rsi\",\"param_ranges\":[{\"min\":2,\"max\":30}]}],"
    "\"rules\":\"single_threshold\",\"max_conditions\":1},\"elitism\":1,\"top\":3}";

static std::string build_command() {
    std::string s = "{\"cmd\":\"evolve\",\"data\":{\"SYM\":[";
    char bar[256];
    for (int i = 0; i < 16; i++) {
        double close = 100.0 + 8.0 * std::sin(i / 4.0) + 0.1 * i;
        double open = 100.0 + 8.0 * std::sin((i - 1) / 4.0) + 0.1 * (i - 1);
        std::snprintf(bar, sizeof(bar),
                      "%s{\"time\":%ld,\"open\":%.3f,\"high\":%.3f,\"low\":%.3f,\"close\":%.3f,\"volume\":1000}",
                      i > 0 ? "," : "", 1700000000L + static_cast<long>(i) * 3600, open,
                      (close > open ? close : open) + 1.0, (close < open ? close : open) - 1.0, close);
        s += bar;
    }
    s += "]}}";
    return s;
}

int main() {
    try {
        wickra::Darwin darwin(SPEC);
        const std::string report = darwin.command(build_command());
        std::printf("wickra-darwin %s\n", wickra::Darwin::version().c_str());
        std::printf("report bytes: %d\n", static_cast<int>(report.size()));
    } catch (const wickra::DarwinError &err) {
        // Every failure arrives here: a spec the core rejects, a command it does
        // not know, a response that changed length between the two ABI calls.
        std::fprintf(stderr, "%s\n", err.what());
        return 1;
    }
    return 0;
}
