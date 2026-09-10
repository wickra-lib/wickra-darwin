package org.wickra.darwin;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

/**
 * The batch search, driven through the JSON command boundary.
 *
 * <p>There is no streaming half in DARWIN: a search is a batch computation over
 * a whole dataset. What stands in its place is the property that makes the batch
 * usable — the same seed reproduces the same search — and it has to hold through
 * this binding, not only in Rust.
 *
 * <p>The second test names {@code macd}, which takes three parameters. Until the
 * search space resolved through the registry, {@code indicator_kind} was a
 * four-name allowlist that declared every indicator as taking one, so this spec
 * was unreachable twice over.
 */
class BatchSearchTest {
    private static final String ONE_PARAMETER =
            "{\"name\":\"rsi\",\"param_ranges\":[{\"min\":2,\"max\":30,\"step\":1}]}";

    private static final String THREE_PARAMETERS =
            "{\"name\":\"macd\",\"param_ranges\":["
                    + "{\"min\":8,\"max\":16,\"step\":2},"
                    + "{\"min\":20,\"max\":30,\"step\":2},"
                    + "{\"min\":5,\"max\":12,\"step\":1}]}";

    private static String spec(String indicators) {
        return "{\"seed\":7,\"population\":8,\"generations\":3,"
                + "\"mutation_rate\":0.2,\"crossover_rate\":0.6,\"fitness\":\"sharpe\","
                + "\"elitism\":1,\"top\":3,\"search_space\":{\"indicators\":[" + indicators
                + "],\"rules\":\"single_threshold\",\"max_conditions\":2}}";
    }

    private static String candles(int n) {
        StringBuilder out = new StringBuilder("[");
        for (int i = 0; i < n; i++) {
            if (i > 0) {
                out.append(',');
            }
            double f = i;
            double close = 100.0 + 10.0 * Math.sin(f * 0.1) + 0.05 * f;
            double opn = 100.0 + 10.0 * Math.sin((f - 1) * 0.1) + 0.05 * (f - 1);
            out.append("{\"time\":").append(1_700_000_000L + (long) i * 3600)
                    .append(",\"open\":").append(opn)
                    .append(",\"high\":").append(Math.max(close, opn) + 1.0)
                    .append(",\"low\":").append(Math.min(close, opn) - 1.0)
                    .append(",\"close\":").append(close)
                    .append(",\"volume\":1000}");
        }
        return out.append(']').toString();
    }

    private static String search(String spec) {
        String data = "{\"AAA\":" + candles(250) + ",\"BBB\":" + candles(250) + "}";
        try (Darwin darwin = new Darwin(spec)) {
            return darwin.command("{\"cmd\":\"evolve\",\"data\":" + data + "}");
        }
    }

    @Test
    void theBatchSearchIsReproducibleFromItsSeed() {
        String s = spec(ONE_PARAMETER);
        assertEquals(search(s), search(s), "the same seed produced two different searches");
    }

    @Test
    void aThreeParameterIndicatorIsSearchable() {
        String report = search(spec(THREE_PARAMETERS));
        assertTrue(report.contains("\"history\""), report);
        assertTrue(report.contains("\"best\""), report);
    }

    @Test
    void aNameTheRegistryDoesNotKnowIsRefused() {
        String bad = spec(
                "{\"name\":\"notanindicator\",\"param_ranges\":[{\"min\":2,\"max\":30,\"step\":1}]}");
        assertThrows(RuntimeException.class, () -> new Darwin(bad));
    }
}
