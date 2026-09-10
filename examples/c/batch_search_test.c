/* The batch search, through the C ABI's two-call idiom.
 *
 * There is no streaming half in DARWIN: a search is a batch computation over a
 * whole dataset. What stands in its place is the property that makes the batch
 * usable — the same seed reproduces the same search — and it has to hold across
 * this boundary, where every call asks for the response length first and reads
 * it second.
 *
 * The second check names `Macd`, which takes three parameters. Until the search
 * space resolved through the registry, `indicator_kind` was a four-name
 * allowlist that declared every indicator as taking one, so this spec was
 * unreachable twice over and no C caller could have expressed it.
 */
#include <math.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_darwin.h"

#define BARS 250

static const char *ONE_PARAMETER =
    "{\"name\":\"rsi\",\"param_ranges\":[{\"min\":2,\"max\":30,\"step\":1}]}";

static const char *THREE_PARAMETERS =
    "{\"name\":\"macd\",\"param_ranges\":["
    "{\"min\":8,\"max\":16,\"step\":2},"
    "{\"min\":20,\"max\":30,\"step\":2},"
    "{\"min\":5,\"max\":12,\"step\":1}]}";

/* A spec over the given indicator list. Caller frees. */
static char *spec_with(const char *indicators) {
    size_t cap = strlen(indicators) + 256;
    char *out = (char *)malloc(cap);
    if (out) {
        snprintf(out, cap,
                 "{\"seed\":7,\"population\":8,\"generations\":3,"
                 "\"mutation_rate\":0.2,\"crossover_rate\":0.6,\"fitness\":\"sharpe\","
                 "\"elitism\":1,\"top\":3,\"search_space\":{\"indicators\":[%s],"
                 "\"rules\":\"single_threshold\",\"max_conditions\":2}}",
                 indicators);
    }
    return out;
}

/* Two symbols of the same deterministic path. Caller frees. */
static char *dataset(void) {
    size_t cap = BARS * 160 + 64;
    char *bars = (char *)malloc(cap);
    if (!bars) {
        return NULL;
    }
    size_t used = 0;
    used += (size_t)snprintf(bars + used, cap - used, "[");
    for (int i = 0; i < BARS; i++) {
        double f = (double)i;
        double close = 100.0 + 10.0 * sin(f * 0.1) + 0.05 * f;
        double opn = 100.0 + 10.0 * sin((f - 1.0) * 0.1) + 0.05 * (f - 1.0);
        double high = (close > opn ? close : opn) + 1.0;
        double low = (close < opn ? close : opn) - 1.0;
        used += (size_t)snprintf(
            bars + used, cap - used,
            "%s{\"time\":%lld,\"open\":%g,\"high\":%g,\"low\":%g,\"close\":%g,\"volume\":1000}",
            i ? "," : "", 1700000000LL + (long long)i * 3600, opn, high, low, close);
    }
    snprintf(bars + used, cap - used, "]");

    size_t dcap = strlen(bars) * 2 + 64;
    char *data = (char *)malloc(dcap);
    if (data) {
        snprintf(data, dcap, "{\"AAA\":%s,\"BBB\":%s}", bars, bars);
    }
    free(bars);
    return data;
}

/* Run one command through the documented two-call idiom. Caller frees. */
static char *run(WickraDarwin *handle, const char *cmd) {
    int32_t len = wickra_darwin_command(handle, cmd, NULL, 0);
    if (len < 0) {
        fprintf(stderr, "command failed: code %d\n", (int)len);
        return NULL;
    }
    char *buf = (char *)malloc((size_t)len + 1);
    if (!buf) {
        return NULL;
    }
    if (wickra_darwin_command(handle, cmd, buf, (size_t)len + 1) != len) {
        free(buf);
        return NULL;
    }
    return buf;
}

/* Run a whole search under `spec`. Caller frees. */
static char *search(const char *spec, const char *data) {
    WickraDarwin *handle = wickra_darwin_new(spec);
    if (!handle) {
        return NULL;
    }
    size_t cap = strlen(data) + 64;
    char *cmd = (char *)malloc(cap);
    if (!cmd) {
        wickra_darwin_free(handle);
        return NULL;
    }
    snprintf(cmd, cap, "{\"cmd\":\"evolve\",\"data\":%s}", data);
    char *out = run(handle, cmd);
    free(cmd);
    wickra_darwin_free(handle);
    return out;
}

int main(void) {
    char *data = dataset();
    char *one = spec_with(ONE_PARAMETER);
    char *three = spec_with(THREE_PARAMETERS);
    if (!data || !one || !three) {
        fprintf(stderr, "could not build the inputs\n");
        free(data);
        free(one);
        free(three);
        return 1;
    }

    int failures = 0;

    char *first = search(one, data);
    char *second = search(one, data);
    if (!first || !second) {
        fprintf(stderr, "the search did not run\n");
        failures++;
    } else if (strcmp(first, second) != 0) {
        fprintf(stderr, "the same seed produced two different searches\n");
        failures++;
    }
    free(first);
    free(second);

    char *multi = search(three, data);
    if (!multi) {
        fprintf(stderr, "a three-parameter indicator did not search\n");
        failures++;
    } else {
        if (strstr(multi, "\"history\"") == NULL || strstr(multi, "\"best\"") == NULL) {
            fprintf(stderr, "the three-parameter search produced nothing: %.160s\n", multi);
            failures++;
        }
        free(multi);
    }

    /* A name the registry does not know is refused at construction. */
    char *bad = spec_with(
        "{\"name\":\"notanindicator\",\"param_ranges\":[{\"min\":2,\"max\":30,\"step\":1}]}");
    if (bad) {
        WickraDarwin *handle = wickra_darwin_new(bad);
        if (handle) {
            fprintf(stderr, "an unknown indicator name must be refused\n");
            wickra_darwin_free(handle);
            failures++;
        }
        free(bad);
    }

    free(data);
    free(one);
    free(three);

    if (failures > 0) {
        fprintf(stderr, "%d check(s) failed\n", failures);
        return 1;
    }
    printf("the batch search is reproducible, and the registry is reachable\n");
    return 0;
}
