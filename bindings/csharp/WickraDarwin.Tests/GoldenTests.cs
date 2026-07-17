using System.Text.Json;
using Wickra.Darwin;
using Xunit;

namespace WickraDarwin.Tests;

// The cross-language golden invariant seen from C#: the same seed yields
// byte-identical output across calls and across instances. The response bytes are
// what every other binding produces too, because the whole search lives once in
// the Rust core and this binding forwards its JSON verbatim.
public class GoldenTests
{
    [Fact]
    public void Evolve_IsByteIdenticalAcrossInstances()
    {
        string cmd = DarwinTests.EvolveCmd();
        using var a = new Darwin(DarwinTests.Spec);
        using var b = new Darwin(DarwinTests.Spec);
        Assert.Equal(a.Command(cmd), b.Command(cmd));
    }

    [Fact]
    public void DifferentSeed_StillValid()
    {
        const string other =
            "{\"seed\":99,\"population\":8,\"generations\":3," +
            "\"mutation_rate\":0.2,\"crossover_rate\":0.6,\"fitness\":\"sharpe\"," +
            "\"search_space\":{\"indicators\":[{\"name\":\"rsi\",\"param_ranges\":[{\"min\":2,\"max\":30,\"step\":1}]}]," +
            "\"rules\":\"single_threshold\",\"max_conditions\":1},\"elitism\":1,\"top\":3}";

        using var darwin = new Darwin(other);
        JsonElement outp = JsonDocument.Parse(darwin.Command(DarwinTests.EvolveCmd())).RootElement;
        Assert.True(outp.GetProperty("history").GetArrayLength() > 0);
    }
}
