using System.Collections.Immutable;
using Pidgin;
using static Pidgin.Parser;
using static Pidgin.Parser<char>;

namespace Gen.Parsers.C;

public abstract record CSyntax
{
    public abstract string ToC();
    public abstract string ToRust(bool on_arg = false);
}

public record Ident(string Name) : CSyntax
{
    public override string ToC() => Name;
    public override string ToRust(bool on_arg = false) => Name;
}

public record Number(string Value) : CSyntax
{
    public override string ToC() => Value;
    public override string ToRust(bool on_arg = false) => Value;
}

public record Ptr(CSyntax Target, bool Const) : CSyntax
{
    public override string ToC() => $"{Target.ToC()}{(Const ? " const" : "")}*";
    public override string ToRust(bool on_arg = false) => $"*{(Const ? "const " : "mut ")}{Target.ToRust()}";
}

public record VarDef(CSyntax Type, string Name, CSyntax? ArraySize = null, bool ConstArray = false) : CSyntax
{
    public static VarDef Create(bool LeftConst, CSyntax Type, ImmutableArray<bool> Ptrs, string Name, CSyntax? ArraySize)
    {
        var t = Type;
        for (var i = 0; i < Ptrs.Length; i++)
        {
            var p = Ptrs[i];
            if (i == 0 && LeftConst) p = true;
            t = new Ptr(t, p);
        }
        return new(t, Name, ArraySize, ArraySize != null && LeftConst && t is not Ptr);
    }

    public override string ToC() => $"{(ConstArray ? "const " : "")}{Type.ToC()} {Name}{(ArraySize is { } a ? $"[{a.ToC()}]" : "")}";
    public override string ToRust(bool on_arg = false) => $"{Name}: {(ArraySize is { } a ?
        $"{(on_arg ? $"&{(ConstArray ? "" : " mut")}" : "")}[{Type.ToRust()}; {a.ToRust()}]" : Type.ToRust())}";
}

public static class CParser
{
    private static readonly Parser<char, char> IdentHead = Token(c => char.IsLetter(c) || c is '_' or '$').Labelled("IdentHead");
    private static readonly Parser<char, char> IdentBody = Token(c => char.IsLetterOrDigit(c) || c is '_' or '$').Labelled("IdentBody");

    /// Ident = [\w_$][\w\d_$]*
    private static readonly Parser<char, CSyntax> Ident =
        IdentHead.Then(IdentBody.ManyString(), (h, t) => $"{h}{t}")
            .Select(CSyntax (a) => new Ident(a))
            .Labelled("Ident")
            .Before(SkipWhitespaces);

    /// Int = [\d]+
    private static readonly Parser<char, CSyntax> Int =
        Digit.AtLeastOnceString()
            .Select(CSyntax (a) => new Number(a))
            .Labelled("Number")
            .Before(SkipWhitespaces);

    /// TypeName = "struct"? Ident
    private static readonly Parser<char, CSyntax> TypeName =
        Try(String("struct")).Then(SkipWhitespaces).Optional().Then(Ident, CSyntax (_, a) => a)
            .Labelled("TypeName").Before(SkipWhitespaces);

    /// Ptr = "*"
    private static readonly Parser<char, char> Ptr =
        Char('*').Labelled("Ptr").Before(SkipWhitespaces);

    /// CPtr = "const" "*" => true | "*" => false
    private static readonly Parser<char, bool> CPtr =
        Try(String("const").Optional().Then(Ptr, (s, _) => s.HasValue)
            .Labelled("CPtr").Before(SkipWhitespaces));

    /// ArraySize = "[" (Int | Ident) "]"
    private static readonly Parser<char, CSyntax> ArraySize =
        Int.Or(Ident).Between(SkipWhitespaces).Between(Char('['), Char(']'))
            .Labelled("ArraySize")
            .Before(SkipWhitespaces);

    /// VarDef = "const"? TypeName CPtr* Ident ArraySize?
    public static readonly Parser<char, VarDef> VarDef =
        Try(String("const")).Then(SkipWhitespaces, (l, _) => l).Optional()
            .Then(TypeName, (lc, tn) => (lc: lc.HasValue, tn))
            .Then(CPtr.Many().Select(a => a.ToImmutableArray()),
                (l, ps) => (l.lc, l.tn, ps))
            .Then(Ident, (l, i) => (l.lc, l.tn, l.ps, i: (Ident)i))
            .Then(ArraySize.Optional(),
                (l, a) => Gen.Parsers.C.VarDef.Create(
                    l.lc, l.tn, l.ps, l.i.Name, a.GetValueOrDefault()))
            .Labelled("VarDef").Before(SkipWhitespaces);
}
