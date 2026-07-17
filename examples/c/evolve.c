/* A runnable C example: evolve strategy specs over a small deterministic
 * universe through the wickra-darwin C ABI and print the search summary. Every
 * language example builds the same universe and prints the same summary. */
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_darwin.h"

static const char *SPEC =
    "{\"seed\":7,\"population\":10,\"generations\":4,"
    "\"mutation_rate\":0.2,\"crossover_rate\":0.6,\"fitness\":\"sharpe\","
    "\"search_space\":{\"indicators\":[{\"name\":\"rsi\",\"param_ranges\":[{\"min\":2,\"max\":30}]}],"
    "\"rules\":\"single_threshold\",\"max_conditions\":1},\"elitism\":1,\"top\":3}";

/* Build the shared 16-bar universe as an evolve command JSON. */
static void build_command(char *out, size_t cap) {
    size_t n = 0;
    n += (size_t)snprintf(out + n, cap - n, "{\"cmd\":\"evolve\",\"data\":{\"SYM\":[");
    for (int i = 0; i < 16; i++) {
        double close = 100.0 + 8.0 * sin(i / 4.0) + 0.1 * i;
        double open = 100.0 + 8.0 * sin((i - 1) / 4.0) + 0.1 * (i - 1);
        double high = (close > open ? close : open) + 1.0;
        double low = (close < open ? close : open) - 1.0;
        n += (size_t)snprintf(out + n, cap - n, "%s{\"time\":%ld,\"open\":%.3f,\"high\":%.3f,\"low\":%.3f,\"close\":%.3f,\"volume\":1000}",
                              i > 0 ? "," : "", 1700000000L + (long)i * 3600, open, high, low, close);
    }
    snprintf(out + n, cap - n, "]}}");
}

int main(void) {
    WickraDarwin *darwin = wickra_darwin_new(SPEC);
    if (!darwin) {
        fprintf(stderr, "failed to build darwin\n");
        return 1;
    }
    char cmd[2048];
    build_command(cmd, sizeof(cmd));

    int len = wickra_darwin_command(darwin, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed: code %d\n", len);
        wickra_darwin_free(darwin);
        return 1;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        wickra_darwin_free(darwin);
        return 1;
    }
    wickra_darwin_command(darwin, cmd, buf, (size_t)len + 1);

    printf("wickra-darwin %s\n", wickra_darwin_version());
    printf("report bytes: %d\n", len);

    free(buf);
    wickra_darwin_free(darwin);
    return 0;
}
