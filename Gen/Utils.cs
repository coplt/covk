using System.Collections.Frozen;
using System.Text;
using System.Text.RegularExpressions;

namespace Gen;

public static partial class Utils
{
    [GeneratedRegex(@"(_BIT)(_\w+)?$", RegexOptions.Compiled)]
    private static partial Regex BitSuffix();

    public static string RemoveBitSuffix(this string str)
    {
        var match = BitSuffix().Match(str);
        if (match.Success)
        {
            var left = str[..match.Groups[1].Index];
            var right = str[(match.Groups[1].Index + match.Groups[1].Length)..];
            return left + right;
        }
        return str;
    }

    public static string LongestCommonPrefix(List<string> strs)
    {
        if (strs.Count == 0) return "";
        if (strs.Count == 1) return strs[0];

        strs.Sort();

        var first = strs[0];
        var last = strs[^1];
        var i = 0;

        while (i < first.Length && i < last.Length && first[i] == last[i])
        {
            i++;
        }

        return first[..i];
    }

    public static string LongestCommonPrefix(params Span<string> strs)
    {
        if (strs.IsEmpty) return "";
        if (strs.Length == 1) return strs[0];

        strs.Sort();

        var first = strs[0];
        var last = strs[^1];
        var i = 0;

        while (i < first.Length && i < last.Length && first[i] == last[i])
        {
            i++;
        }

        return first[..i];
    }

    public static string ProcessComment(this string Comment)
    {
        return Comment.Replace("<<", "").Replace(">>", "");
    }

    private static readonly FrozenSet<string> RustKeywords =
    [
        "type", "match", "self", "super", "crate", "mod", "fn", "struct", "enum", "union",
        "impl", "trait", "const", "static", "let", "mut", "ref", "return", "break",
        "continue", "async", "await",
    ];

    public static string CheckRustKeyword(this string name)
    {
        return RustKeywords.Contains(name) ? $"r#{name}" : name;
    }

    public static string ApiConstantToRust(string value)
    {
        return value.Replace("f", "f32").Replace("F", "f32").Replace("ULL", "u64").Replace("U", "u32").Replace("~", "!");
    }

    public static string NumberToLetters(this int n)
    {
        var result = new StringBuilder();

        do
        {
            var remainder = n % 26;
            result.Insert(0, (char)('a' + remainder));
            n = (n / 26) - 1;
        } while (n >= 0);

        return result.ToString();
    }
}
