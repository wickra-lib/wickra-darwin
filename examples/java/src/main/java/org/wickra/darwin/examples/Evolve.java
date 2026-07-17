package org.wickra.darwin.examples;

import java.util.Locale;
import org.wickra.darwin.Darwin;

/**
 * A runnable Java example: evolve strategy specs over a small deterministic
 * universe and print the search summary.
 *
 * <pre>
 *   mvn -q compile exec:java -Dexec.mainClass=org.wickra.darwin.examples.Evolve
 * </pre>
 *
 * Every language example builds the same universe and prints the same summary.
 */
public final class Evolve {
    private Evolve() {}

    private static final String SPEC =
            "{\"seed\":7,\"population\":10,\"generations\":4,"
                    + "\"mutation_rate\":0.2,\"crossover_rate\":0.6,\"fitness\":\"sharpe\","
                    + "\"search_space\":{\"indicators\":[{\"name\":\"rsi\",\"param_ranges\":"
                    + "[{\"min\":2,\"max\":30}]}],\"rules\":\"single_threshold\","
                    + "\"max_conditions\":1},\"elitism\":1,\"top\":3}";

    private static String evolveCommand() {
        StringBuilder sb = new StringBuilder("{\"cmd\":\"evolve\",\"data\":{\"SYM\":[");
        for (int i = 0; i < 16; i++) {
            double close = 100.0 + 8.0 * Math.sin(i / 4.0) + 0.1 * i;
            double open = 100.0 + 8.0 * Math.sin((i - 1) / 4.0) + 0.1 * (i - 1);
            if (i > 0) {
                sb.append(',');
            }
            sb.append(String.format(Locale.ROOT,
                    "{\"time\":%d,\"open\":%.3f,\"high\":%.3f,\"low\":%.3f,\"close\":%.3f,\"volume\":1000}",
                    1700000000L + (long) i * 3600, open,
                    Math.max(close, open) + 1.0, Math.min(close, open) - 1.0, close));
        }
        sb.append("]}}");
        return sb.toString();
    }

    public static void main(String[] args) {
        try (Darwin darwin = new Darwin(SPEC)) {
            String report = darwin.command(evolveCommand());
            System.out.printf("wickra-darwin %s%n", Darwin.version());
            long generations = report.split("\"generation\"", -1).length - 1;
            System.out.printf("generations: %d%n", generations);
        }
    }
}
