// A runnable C++ example: evolve strategy specs over a small deterministic
// universe through the wickra-darwin C ABI and print the search summary.
#include <cmath>
#include <cstdio>
#include <string>
#include <vector>

#include "wickra_darwin.h"

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
    WickraDarwin *darwin = wickra_darwin_new(SPEC);
    if (!darwin) {
        std::fprintf(stderr, "failed to build darwin\n");
        return 1;
    }
    std::string cmd = build_command();

    int len = wickra_darwin_command(darwin, cmd.c_str(), nullptr, 0);
    if (len < 0) {
        wickra_darwin_free(darwin);
        return 1;
    }
    std::vector<char> buf(static_cast<size_t>(len) + 1);
    wickra_darwin_command(darwin, cmd.c_str(), buf.data(), buf.size());

    std::printf("wickra-darwin %s\n", wickra_darwin_version());
    std::printf("report bytes: %d\n", len);

    wickra_darwin_free(darwin);
    return 0;
}
