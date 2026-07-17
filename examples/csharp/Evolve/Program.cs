// A runnable C# example: evolve strategy specs over a small deterministic
// universe and print the search summary.
//
//   dotnet run --project examples/csharp/Evolve
//
// Every language example builds the same universe and prints the same summary.
using System.Globalization;
using System.Text;
using System.Text.Json;
using Wickra.Darwin;

const string spec =
    "{\"seed\":7,\"population\":10,\"generations\":4," +
    "\"mutation_rate\":0.2,\"crossover_rate\":0.6,\"fitness\":\"sharpe\"," +
    "\"search_space\":{\"indicators\":[{\"name\":\"rsi\",\"param_ranges\":[{\"min\":2,\"max\":30}]}]," +
    "\"rules\":\"single_threshold\",\"max_conditions\":1},\"elitism\":1,\"top\":3}";

static string EvolveCommand()
{
    var sb = new StringBuilder("{\"cmd\":\"evolve\",\"data\":{\"SYM\":[");
    for (int i = 0; i < 16; i++)
    {
        double close = 100.0 + 8.0 * Math.Sin(i / 4.0) + 0.1 * i;
        double open = 100.0 + 8.0 * Math.Sin((i - 1) / 4.0) + 0.1 * (i - 1);
        if (i > 0) sb.Append(',');
        sb.Append(string.Format(CultureInfo.InvariantCulture,
            "{{\"time\":{0},\"open\":{1:F3},\"high\":{2:F3},\"low\":{3:F3},\"close\":{4:F3},\"volume\":1000}}",
            1700000000 + i * 3600, open, Math.Max(close, open) + 1.0, Math.Min(close, open) - 1.0, close));
    }
    sb.Append("]}}");
    return sb.ToString();
}

using var darwin = new Darwin(spec);
JsonElement report = JsonDocument.Parse(darwin.Command(EvolveCommand())).RootElement;
Console.WriteLine($"wickra-darwin {Darwin.Version()}");
Console.WriteLine($"generations: {report.GetProperty("history").GetArrayLength()}");
Console.WriteLine($"hall of fame: {report.GetProperty("best").GetArrayLength()}");
