using System.Text;
using System.Text.Json;
using Wickra.Darwin;
using Xunit;

namespace WickraDarwin.Tests;

public class DarwinTests
{
    internal const string Spec =
        "{\"seed\":1,\"population\":8,\"generations\":3," +
        "\"mutation_rate\":0.2,\"crossover_rate\":0.6,\"fitness\":\"sharpe\"," +
        "\"search_space\":{\"indicators\":[{\"name\":\"rsi\",\"param_ranges\":[{\"min\":2,\"max\":30,\"step\":1}]}]," +
        "\"rules\":\"single_threshold\",\"max_conditions\":1},\"elitism\":1,\"top\":3}";

    // A deterministic sine price path over 250 bars.
    internal static string EvolveCmd()
    {
        var sb = new StringBuilder();
        sb.Append("{\"cmd\":\"evolve\",\"data\":{\"AAA\":[");
        for (int i = 0; i < 250; i++)
        {
            double close = 100.0 + 10.0 * Math.Sin(i * 0.1) + 0.05 * i;
            double open = 100.0 + 10.0 * Math.Sin((i - 1) * 0.1) + 0.05 * (i - 1);
            double high = Math.Max(close, open) + 1.0;
            double low = Math.Min(close, open) - 1.0;
            if (i > 0)
            {
                sb.Append(',');
            }
            sb.Append(System.Globalization.CultureInfo.InvariantCulture,
                $"{{\"time\":{1700000000 + i * 3600},\"open\":{open:R},\"high\":{high:R},\"low\":{low:R},\"close\":{close:R},\"volume\":1000.0}}");
        }
        sb.Append("]}}");
        return sb.ToString();
    }

    [Fact]
    public void Version_IsNonEmpty()
    {
        Assert.False(string.IsNullOrEmpty(Darwin.Version()));
    }

    [Fact]
    public void Evolve_ReturnsHistory()
    {
        using var darwin = new Darwin(Spec);
        JsonElement outp = JsonDocument.Parse(darwin.Command(EvolveCmd())).RootElement;

        Assert.Equal(4, outp.GetProperty("history").GetArrayLength());
    }

    [Fact]
    public void InvalidSpec_Throws()
    {
        Assert.Throws<ArgumentException>(() => new Darwin("{ not valid json"));
    }

    [Fact]
    public void SetSpec_ThenEvolve()
    {
        using var darwin = new Darwin("{}");
        string ok = darwin.Command("{\"cmd\":\"set_spec\",\"spec\":" + Spec + "}");
        Assert.Contains("\"ok\":true", ok);
        Assert.Contains("history", darwin.Command(EvolveCmd()));
    }
}
