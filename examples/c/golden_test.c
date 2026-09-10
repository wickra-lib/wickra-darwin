/* Cross-language golden parity, from C.
 *
 * Run each committed golden/specs/*.json over the committed universe and assert
 * the report equals golden/expected/<spec>.json byte-for-byte. The ABI returns
 * the core's compact command output verbatim, so byte equality is the exact
 * cross-language parity check — the same one Python, Node, Go, C#, Java, R and
 * WASM make.
 *
 * C has no directory API that is portable between POSIX and Windows, so the spec
 * list is globbed by CMake at configure time and written into golden_specs.h.
 * That keeps the property the other bindings get from a runtime glob: a spec
 * added to the corpus is covered here without editing this file. A
 * hand-maintained list would silently skip it, which is the failure this whole
 * corpus exists to prevent.
 *
 * The universe is read from golden/data.json rather than the per-symbol CSVs:
 * the ABI speaks JSON, and a CSV parser here would be a second implementation of
 * the loader rather than a test of the search.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "wickra_darwin.h"

#include "golden_specs.h" /* GOLDEN_DIR, GOLDEN_SPECS, GOLDEN_SPEC_COUNT */

/* Read a whole file. Caller frees. */
static char *slurp(const char *path) {
    FILE *file = fopen(path, "rb");
    if (!file) {
        fprintf(stderr, "cannot open %s\n", path);
        return NULL;
    }
    if (fseek(file, 0, SEEK_END) != 0) {
        fclose(file);
        return NULL;
    }
    long size = ftell(file);
    if (size < 0 || fseek(file, 0, SEEK_SET) != 0) {
        fclose(file);
        return NULL;
    }
    char *buf = (char *)malloc((size_t)size + 1);
    if (!buf) {
        fclose(file);
        return NULL;
    }
    size_t got = fread(buf, 1, (size_t)size, file);
    fclose(file);
    buf[got] = '\0';
    return buf;
}

/* Trim ASCII whitespace in place; returns the start of the trimmed text. */
static char *trim(char *text) {
    while (*text == ' ' || *text == '\n' || *text == '\r' || *text == '\t') {
        text++;
    }
    size_t len = strlen(text);
    while (len > 0) {
        char last = text[len - 1];
        if (last != ' ' && last != '\n' && last != '\r' && last != '\t') {
            break;
        }
        text[--len] = '\0';
    }
    return text;
}

static char *join(const char *a, const char *b, const char *c) {
    size_t len = strlen(a) + strlen(b) + strlen(c) + 1;
    char *out = (char *)malloc(len);
    if (out) {
        snprintf(out, len, "%s%s%s", a, b, c);
    }
    return out;
}

/* Run one command through the two-call idiom. Caller frees. */
static char *run(WickraDarwin *handle, const char *cmd) {
    int32_t len = wickra_darwin_command(handle, cmd, NULL, 0);
    if (len < 0) {
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

int main(void) {
    if (GOLDEN_SPEC_COUNT == 0) {
        fprintf(stderr, "no golden specs were configured; this would test nothing\n");
        return 1;
    }

    char *raw_data = slurp(GOLDEN_DIR "/data.json");
    if (!raw_data) {
        return 1;
    }
    char *data = trim(raw_data);
    char *evolve_cmd = join("{\"cmd\":\"evolve\",\"data\":", data, "}");
    if (!evolve_cmd) {
        free(raw_data);
        return 1;
    }

    int failures = 0;
    size_t checked = 0;
    for (size_t i = 0; i < GOLDEN_SPEC_COUNT; i++) {
        const char *name = GOLDEN_SPECS[i];

        char *spec_path = join(GOLDEN_DIR "/specs/", name, "");
        char *expected_path = join(GOLDEN_DIR "/expected/", name, "");
        char *spec = spec_path ? slurp(spec_path) : NULL;
        char *expected_raw = expected_path ? slurp(expected_path) : NULL;
        free(spec_path);
        free(expected_path);

        if (!spec || !expected_raw) {
            fprintf(stderr, "%s: missing spec or expected file\n", name);
            failures++;
            free(spec);
            free(expected_raw);
            continue;
        }

        WickraDarwin *handle = wickra_darwin_new(spec);
        char *got_raw = handle ? run(handle, evolve_cmd) : NULL;
        if (!got_raw) {
            fprintf(stderr, "%s: the search did not run\n", name);
            failures++;
        } else {
            char *got = trim(got_raw);
            char *expected = trim(expected_raw);
            if (strcmp(got, expected) != 0) {
                fprintf(stderr, "%s: mismatch\n  expected: %.140s\n  got:      %.140s\n",
                        name, expected, got);
                failures++;
            } else {
                checked++;
            }
        }

        free(got_raw);
        wickra_darwin_free(handle);
        free(spec);
        free(expected_raw);
    }

    free(evolve_cmd);
    free(raw_data);

    if (failures > 0) {
        fprintf(stderr, "%d of %zu golden specs did not match\n", failures, GOLDEN_SPEC_COUNT);
        return 1;
    }
    printf("all %zu golden specs are byte-identical from C\n", checked);
    return 0;
}
