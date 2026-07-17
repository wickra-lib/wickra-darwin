package org.wickra.darwin;

import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;

import java.util.Locale;
import org.junit.jupiter.api.Test;

class DarwinTest {
    static final String SPEC =
            "{\"seed\":1,\"population\":8,\"generations\":3,"
                    + "\"mutation_rate\":0.2,\"crossover_rate\":0.6,\"fitness\":\"sharpe\","
                    + "\"search_space\":{\"indicators\":[{\"name\":\"rsi\",\"param_ranges\":"
                    + "[{\"min\":2,\"max\":30,\"step\":1}]}],\"rules\":\"single_threshold\","
                    + "\"max_conditions\":1},\"elitism\":1,\"top\":3}";

    // A deterministic sine price path over 250 bars.
    static String evolveCmd() {
        StringBuilder sb = new StringBuilder();
        sb.append("{\"cmd\":\"evolve\",\"data\":{\"AAA\":[");
        for (int i = 0; i < 250; i++) {
            double close = 100.0 + 10.0 * Math.sin(i * 0.1) + 0.05 * i;
            double open = 100.0 + 10.0 * Math.sin((i - 1) * 0.1) + 0.05 * (i - 1);
            double high = Math.max(close, open) + 1.0;
            double low = Math.min(close, open) - 1.0;
            if (i > 0) {
                sb.append(',');
            }
            sb.append(String.format(
                    Locale.ROOT,
                    "{\"time\":%d,\"open\":%s,\"high\":%s,\"low\":%s,\"close\":%s,\"volume\":1000.0}",
                    1700000000L + (long) i * 3600,
                    Double.toString(open), Double.toString(high),
                    Double.toString(low), Double.toString(close)));
        }
        sb.append("]}}");
        return sb.toString();
    }

    @Test
    void versionIsNonEmpty() {
        assertFalse(Darwin.version().isEmpty());
    }

    @Test
    void evolveReturnsHistory() {
        try (Darwin darwin = new Darwin(SPEC)) {
            String out = darwin.command(evolveCmd());
            assertTrue(out.contains("\"history\""), out);
            assertTrue(out.contains("\"best\""), out);
        }
    }

    @Test
    void invalidSpecThrows() {
        assertThrows(IllegalArgumentException.class, () -> new Darwin("{ not valid json"));
    }

    @Test
    void setSpecThenEvolve() {
        try (Darwin darwin = new Darwin("{}")) {
            String ok = darwin.command("{\"cmd\":\"set_spec\",\"spec\":" + SPEC + "}");
            assertTrue(ok.contains("\"ok\":true"), ok);
            assertTrue(darwin.command(evolveCmd()).contains("\"history\""));
        }
    }
}
