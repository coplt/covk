using Gen.Parsers.C;
using Pidgin;

namespace TestParser;

public class Tests
{
    [SetUp]
    public void Setup() { }

    [Test]
    public void Test1()
    {
        var code = "const char* const* a";
        var result = CParser.VarDef.ParseOrThrow(code).Type;
        Console.WriteLine(result);
        Console.WriteLine(result.ToC());
        Console.WriteLine(result.ToRust());
        Assert.That(result, Is.EqualTo(new Ptr(new Ptr(new Ident("char"), true), true)));
    }

    [Test]
    public void Test2()
    {
        var code = "struct VkBaseOutStructure* a";
        var result = CParser.VarDef.ParseOrThrow(code).Type;
        Console.WriteLine(result);
        Console.WriteLine(result.ToC());
        Console.WriteLine(result.ToRust());
        Assert.That(result, Is.EqualTo(new Ptr(new Ident("VkBaseOutStructure"), false)));
    }

    [Test]
    public void Test3()
    {
        var code = "const struct VkBaseOutStructure* a";
        var result = CParser.VarDef.ParseOrThrow(code).Type;
        Console.WriteLine(result);
        Console.WriteLine(result.ToC());
        Console.WriteLine(result.ToRust());
        Assert.That(result, Is.EqualTo(new Ptr(new Ident("VkBaseOutStructure"), true)));
    }

    [Test]
    public void Test4()
    {
        var code = "const void* a";
        var result = CParser.VarDef.ParseOrThrow(code).Type;
        Console.WriteLine(result);
        Console.WriteLine(result.ToC());
        Console.WriteLine(result.ToRust());
        Assert.That(result, Is.EqualTo(new Ptr(new Ident("void"), true)));
    }

    [Test]
    public void Test5()
    {
        var code = "VkStructureType a";
        var result = CParser.VarDef.ParseOrThrow(code).Type;
        Console.WriteLine(result);
        Console.WriteLine(result.ToC());
        Console.WriteLine(result.ToRust());
        Assert.That(result, Is.EqualTo(new Ident("VkStructureType")));
    }

    [Test]
    public void Test6()
    {
        var code = "VkBaseOutStructure* a";
        var result = CParser.VarDef.ParseOrThrow(code).Type;
        Console.WriteLine(result);
        Console.WriteLine(result.ToC());
        Console.WriteLine(result.ToRust());
        Assert.That(result, Is.EqualTo(new Ptr(new Ident("VkBaseOutStructure"), false)));
    }

    [Test]
    public void Test7()
    {
        var code = "int a";
        var result = CParser.VarDef.ParseOrThrow(code);
        Console.WriteLine(result);
        Console.WriteLine(result.ToC());
        Console.WriteLine(result.ToRust());
        Assert.That(result, Is.EqualTo(new VarDef(new Ident("int"), "a")));
    }

    [Test]
    public void Test8()
    {
        var code = "VkOffset3D srcOffsets[2]";
        var result = CParser.VarDef.ParseOrThrow(code);
        Console.WriteLine(result);
        Console.WriteLine(result.ToC());
        Console.WriteLine(result.ToRust());
        Assert.That(result, Is.EqualTo(
            new VarDef(new Ident("VkOffset3D"), "srcOffsets", new Number("2"))
        ));
    }

    [Test]
    public void Test9()
    {
        var code = "VkMemoryType memoryTypes[VK_MAX_MEMORY_TYPES]";
        var result = CParser.VarDef.ParseOrThrow(code);
        Console.WriteLine(result);
        Console.WriteLine(result.ToC());
        Console.WriteLine(result.ToRust());
        Assert.That(result, Is.EqualTo(
            new VarDef(new Ident("VkMemoryType"), "memoryTypes", new Ident("VK_MAX_MEMORY_TYPES"))
        ));
    }

    [Test]
    public void Test10()
    {
        var code = "char           deviceName[VK_MAX_PHYSICAL_DEVICE_NAME_SIZE]";
        var result = CParser.VarDef.ParseOrThrow(code);
        Console.WriteLine(result);
        Console.WriteLine(result.ToC());
        Console.WriteLine(result.ToRust());
        Assert.That(result, Is.EqualTo(
            new VarDef(new Ident("char"), "deviceName", new Ident("VK_MAX_PHYSICAL_DEVICE_NAME_SIZE"))
        ));
    }

    [Test]
    public void Test11()
    {
        var code = "VkComponentMapping     components";
        var result = CParser.VarDef.ParseOrThrow(code);
        Console.WriteLine(result);
        Console.WriteLine(result.ToC());
        Console.WriteLine(result.ToRust());
        Assert.That(result, Is.EqualTo(
            new VarDef(new Ident("VkComponentMapping"), "components")
        ));
    }

    [Test]
    public void Test12()
    {
        var code = "uint32_t                     constantID";
        var result = CParser.VarDef.ParseOrThrow(code);
        Console.WriteLine(result);
        Console.WriteLine(result.ToC());
        Console.WriteLine(result.ToRust());
        Assert.That(result, Is.EqualTo(
            new VarDef(new Ident("uint32_t"), "constantID")
        ));
    }

    [Test]
    public void Test13()
    {
        var code = "const float blendConstants[4]";
        var result = CParser.VarDef.ParseOrThrow(code);
        Console.WriteLine(result);
        Console.WriteLine(result.ToC());
        Console.WriteLine(result.ToRust(true));
        Assert.That(result, Is.EqualTo(
            new VarDef(new Ident("float"), "blendConstants", new Number("4"), true)
        ));
    }
}
