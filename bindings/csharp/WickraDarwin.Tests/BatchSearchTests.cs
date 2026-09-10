using System.Globalization;
using System.Text;
using Wickra.Darwin;
using Xunit;

namespace WickraDarwin.Tests;

/// <summary>
/// The batch search, driven through the JSON command boundary.
///
/// There is no streaming half in DARWIN: a search is a batch computation over a
/// whole dataset. What stands in its place is the property that makes the batch
/// usable — the same seed reproduces the same search — and it has to hold
/// through this binding, not only in Rust.
///
/// The second test names <c>macd</c>, which takes three parameters. Until the
/// search space resolved through the registry, <c>indicator_kind</c> was a
/// four-name allowlist that declared every indicator as taking one, so this spec
/// was unreachable twice over.
/// </summary>
public class BatchSearchTests
{
    private const string OneParameter =
        "{\"name\":\"rsi\",\"param_ranges\":[{\"min\":2,\"max\":30,\"step\":1}]}";

    private const string ThreeParameters =
        "{\"name\":\"macd\",\"param_ranges\":[" +
        "{\"min\":8,\"max\":16,\"step\":2}," +
        "{\"min\":20,\"max\":30,\"step\":2}," +
        "{\"min\":5,\"max\":12,\"step\":1}]}";

    private static string Spec(string indicators) =>
        "{\"seed\":7,\"population\":8,\"generations\":3," +
        "\"mutation_rate\":0.2,\"crossover_rate\":0.6,\"fitness\":\"sharpe\"," +
        "\"elitism\":1,\"top\":3,\"search_space\":{\"indicators\":[" + indicators +
        "],\"rules\":\"single_threshold\",\"max_conditions\":2}}";

    private static string Candles(int n)
    {
        string N(double v) => v.ToString("R", CultureInfo.InvariantCulture);
        var sb = new StringBuilder("[");
        for (int i = 0; i < n; i++)
        {
            if (i > 0)
            {
                sb.Append(',');
            }

            double f = i;
            double close = 100.0 + (10.0 * Math.Sin(f * 0.1)) + (0.05 * f);
            double opn = 100.0 + (10.0 * Math.Sin((f - 1) * 0.1)) + (0.05 * (f - 1));
            sb.Append("{\"time\":").Append((1_700_000_000L + (i * 3600L))
                    .ToString(CultureInfo.InvariantCulture))
              .Append(",\"open\":").Append(N(opn))
              .Append(",\"high\":").Append(N(Math.Max(close, opn) + 1.0))
              .Append(",\"low\":").Append(N(Math.Min(close, opn) - 1.0))
              .Append(",\"close\":").Append(N(close))
              .Append(",\"volume\":1000}");
        }

        return sb.Append(']').ToString();
    }

    private static string Search(string spec)
    {
        string data = "{\"AAA\":" + Candles(250) + ",\"BBB\":" + Candles(250) + "}";
        using var darwin = new Darwin(spec);
        return darwin.Command("{\"cmd\":\"evolve\",\"data\":" + data + "}");
    }

    [Fact]
    public void TheBatchSearchIsReproducibleFromItsSeed()
    {
        string spec = Spec(OneParameter);
        Assert.Equal(Search(spec), Search(spec));
    }

    [Fact]
    public void AThreeParameterIndicatorIsSearchable()
    {
        string report = Search(Spec(ThreeParameters));
        Assert.Contains("\"history\"", report);
        Assert.Contains("\"best\"", report);
    }

    [Fact]
    public void ANameTheRegistryDoesNotKnowIsRefused()
    {
        string bad = Spec(
            "{\"name\":\"notanindicator\",\"param_ranges\":[{\"min\":2,\"max\":30,\"step\":1}]}");
        Assert.ThrowsAny<Exception>(() => new Darwin(bad));
    }
}
