package org.wickra.darwin;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

import org.junit.jupiter.api.Test;

// The cross-language golden invariant seen from Java: the same seed yields
// byte-identical output across calls and across instances. The response bytes are
// what every other binding produces too, because the whole search lives once in
// the Rust core and this binding forwards its JSON verbatim.
class GoldenTest {
    @Test
    void evolveIsByteIdenticalAcrossInstances() {
        String cmd = DarwinTest.evolveCmd();
        try (Darwin a = new Darwin(DarwinTest.SPEC);
                Darwin b = new Darwin(DarwinTest.SPEC)) {
            assertEquals(a.command(cmd), b.command(cmd));
        }
    }

    @Test
    void differentSeedStillValid() {
        String other = DarwinTest.SPEC.replace("\"seed\":1,", "\"seed\":99,");
        try (Darwin darwin = new Darwin(other)) {
            String out = darwin.command(DarwinTest.evolveCmd());
            assertTrue(out.contains("\"history\""), out);
        }
    }
}
