using System.Collections.Frozen;
using System.Collections.Immutable;
using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json.Serialization;
using System.Text.RegularExpressions;
using System.Xml.Linq;
using Gen.Emitter;
using Gen.Parsers.C;
using Pidgin;
using Serilog;

namespace Gen;

public partial record Symbols
{
    public required string VulkanVersion { get; set; }
    public required FrozenDictionary<string, SymbolType> Types { get; set; }
    public required FrozenDictionary<string, SymbolCommand> Commands { get; set; }
    public required ImmutableArray<ApiConstantItem> ApiConstants { get; set; }
    public required FrozenDictionary<string, SymbolFeature> Features { get; set; }
    public required FrozenDictionary<string, SymbolExtension> Extensions { get; set; }

    [GeneratedRegex(@".*#define VK_HEADER_VERSION (\d+).*")]
    public static partial Regex GetHeaderVersion();
    [GeneratedRegex(@".*#define VK_HEADER_VERSION_COMPLETE VK_MAKE_API_VERSION\(\d+, (\d+), (\d+), VK_HEADER_VERSION\).*")]
    public static partial Regex GetHeaderVersionComplete();

    public static Symbols Load(XDocument doc)
    {
        var registry = doc.Element("registry")!;

        var defines = registry.Element("types")!.Elements("type")
            .Where(a => a.Attribute("category")?.Value is "define")
            .Select(a => (name: a.Element("name")?.Value, a))
            .Where(a => a.name is not null)
            .DistinctBy(a => a.name)
            .ToDictionary(a => a.name!, a => a);
        var header_versions = GetHeaderVersionComplete().Match(defines["VK_HEADER_VERSION_COMPLETE"].a.Value);
        var vk_version1 = header_versions.Groups[1].Value;
        var vk_version2 = header_versions.Groups[2].Value;
        var vk_version3 = GetHeaderVersion().Match(defines["VK_HEADER_VERSION"].a.Value).Groups[1].Value;
        var vk_version = $"{vk_version1}.{vk_version2}.{vk_version3}";
        Log.Information("VulkanVersion: {Version}", vk_version);

        var enums = registry.Elements("enums")
            .ToDictionary(a => a.Attribute("name")?.Value!);

        var type_nodes = registry.Element("types")!.Elements("type")
            .Where(a => a.Attribute("category")?.Value is null or not "define")
            .Select(a => new KeyValuePair<string, XElement>(
                a.Attribute("name")?.Value
                ?? a.Element("name")?.Value
                ?? a.Element("proto")!.Element("name")!.Value,
                a
            ))
            .DistinctBy(a => a.Key)
            .ToDictionary();

        var types = new Dictionary<string, SymbolType>();
        var commands = new Dictionary<string, SymbolCommand>();
        foreach (var node in type_nodes)
        {
            LoadType(node.Value, node.Key);
        }

        var Cmds = registry.Element("commands")!.Elements("command")
            .Select(a => (k: a.Element("proto")?.Element("name")?.Value ?? a.Attribute("name")?.Value, v: a))
            .Where(a =>
            {
                if (a.k is null) Log.Warning("Invalid command: {Command}", a.v);
                return a.k is not null;
            })
            .GroupBy(a => a.k)
            .ToDictionary(a => a.Key!, a => a.Select(b => b.v).ToList());
        foreach (var (name, cmds) in Cmds)
        {
            foreach (var command in cmds)
            {
                LoadCommand(command, name);
            }
        }

        var features = new Dictionary<string, SymbolFeature>();
        foreach (var feature in registry.Elements("feature"))
        {
            LoadFeature(feature, feature.Attribute("name")!.Value);
        }

        var extensions = new Dictionary<string, SymbolExtension>();
        foreach (var extension in registry.Element("extensions")!.Elements("extension"))
        {
            LoadExtension(extension, extension.Attribute("name")!.Value);
        }

        return new()
        {
            VulkanVersion = vk_version,
            ApiConstants = LoadApiConstants(),
            Types = types.ToFrozenDictionary(),
            Commands = commands.ToFrozenDictionary(),
            Features = features.ToFrozenDictionary(),
            Extensions = extensions.ToFrozenDictionary(),
        };

        ImmutableArray<ApiConstantItem> LoadApiConstants()
        {
            var enum_node = enums["API Constants"];
            var items = ImmutableArray.CreateBuilder<ApiConstantItem>();
            foreach (var item in enum_node.Elements("enum"))
            {
                var item_name = item.Attribute("name")!.Value;
                var value = item.Attribute("value")!.Value;
                var item_type = item.Attribute("type")!.Value;
                var comment = item.Attribute("comment")?.Value;
                items.Add(new()
                {
                    Name = item_name,
                    Type = item_type,
                    Value = value,
                    Comment = comment,
                });
            }
            return items.ToImmutable();
        }

        void LoadType(XElement src, string name)
        {
            var cat = src.Attribute("category")?.Value;
            switch (cat)
            {
                case "bitmask":
                {
                    if (src.Attribute("alias") is { } alias_attr)
                    {
                        var alias = alias_attr.Value;
                        types.Add(name, new SymbolTypeAlias
                        {
                            Name = name,
                            Target = alias,
                        });
                        return;
                    }
                    var enum_name = src.Attribute("requires")?.Value ?? src.Attribute("bitvalues")?.Value;
                    var type = src.Element("type")?.Value;
                    var is_64 = type is "VkFlags64";
                    if (enum_name is null)
                    {
                        Log.Information("Find empty enum {Name}", name);
                        types.Add(name, new SymbolTypeEnum
                        {
                            Name = name,
                            Items = [],
                            Flags = true,
                            x64 = is_64,
                        });
                        return;
                    }
                    if (!enums.TryGetValue(enum_name, out var enum_node))
                    {
                        Log.Warning("Can not find enum define {Name} => {Enum}", name, enum_name);
                        return;
                    }
                    var enum_comment = enum_node.Attribute("comment")?.Value;
                    var items = new Dictionary<string, SymbolEnumItem>();
                    foreach (var item in enum_node.Elements("enum"))
                    {
                        var item_name = item.Attribute("name")!.Value;
                        var value = item.Attribute("value")?.Value;
                        var bitpos = item.Attribute("bitpos")?.Value;
                        var alias = item.Attribute("alias")?.Value;
                        var comment = item.Attribute("comment")?.Value;
                        EnumValue? item_value = null;
                        if (value is not null) item_value = new EnumValue.Expr(value);
                        else if (bitpos is not null) item_value = new EnumValue.BitPos(long.Parse(bitpos));
                        else if (alias is not null) item_value = new EnumValue.Alias(alias);
                        else
                        {
                            Log.Warning("No enum item value {Name} => {Enum} :: {Item}", name, enum_name, item_name);
                            continue;
                        }
                        items.Add(item_name, new()
                        {
                            Name = item_name,
                            Value = item_value!,
                            Comment = comment,
                        });
                    }
                    types.Add(name, new SymbolTypeEnum
                    {
                        Name = name,
                        Items = items,
                        Flags = true,
                        x64 = is_64,
                        Comment = enum_comment,
                    });
                    return;
                }
                case "enum":
                {
                    if (src.Attribute("alias") is { } alias_attr)
                    {
                        var alias = alias_attr.Value;
                        types.Add(name, new SymbolTypeAlias
                        {
                            Name = name,
                            Target = alias,
                        });
                        return;
                    }
                    if (!enums.TryGetValue(name, out var enum_node))
                    {
                        Log.Warning("Can not find enum define {Name} => {Enum}", name, name);
                        return;
                    }
                    var is_flags = enum_node.Attribute("type")?.Value is "bitmask";
                    if (is_flags) return;
                    {
                        if (src.Attribute("alias") is { Value: { } alias })
                        {
                            types.Add(name, new SymbolTypeAlias
                            {
                                Name = name,
                                Target = alias,
                            });
                            return;
                        }
                    }
                    var enum_comment = enum_node.Attribute("comment")?.Value;
                    var is_x64 = enum_node.Attribute("bitwidth")?.Value is "64";
                    var items = new Dictionary<string, SymbolEnumItem>();
                    foreach (var item in enum_node.Elements("enum"))
                    {
                        var item_name = item.Attribute("name")!.Value;
                        var value = item.Attribute("value")?.Value;
                        var bitpos = item.Attribute("bitpos")?.Value;
                        var alias = item.Attribute("alias")?.Value;
                        var comment = item.Attribute("comment")?.Value;
                        EnumValue? item_value = null;
                        if (value is not null) item_value = new EnumValue.Expr(value);
                        else if (bitpos is not null) item_value = new EnumValue.BitPos(long.Parse(bitpos));
                        else if (alias is not null) item_value = new EnumValue.Alias(alias);
                        else
                        {
                            Log.Warning("No enum item value {Name} :: {Item}", name, item_name);
                            continue;
                        }
                        items.Add(item_name, new()
                        {
                            Name = item_name,
                            Value = item_value!,
                            Comment = comment,
                        });
                    }
                    types.Add(name, new SymbolTypeEnum
                    {
                        Name = name,
                        Items = items,
                        Flags = false,
                        x64 = is_x64,
                        Comment = enum_comment,
                    });
                    return;
                }
                case "handle":
                {
                    if (src.Attribute("alias") is { } alias_attr)
                    {
                        var alias = alias_attr.Value;
                        types.Add(name, new SymbolTypeAlias
                        {
                            Name = name,
                            Target = alias,
                        });
                        return;
                    }
                    var parent = src.Attribute("parent")?.Value;
                    var dispatchable = src.Element("type")?.Value is "VK_DEFINE_HANDLE";
                    var object_type = src.Attribute("objtypeenum")!.Value;
                    types.Add(name, new SymbolTypeHandle
                    {
                        Name = name,
                        Parent = parent,
                        Dispatchable = dispatchable,
                        ObjectType = object_type,
                    });
                    return;
                }
                case "funcpointer":
                {
                    var comment = src.Element("comment")?.Value;
                    var proto = src.Element("proto");
                    var return_type = proto!.Element("type")!.Value;
                    var proto_def = ParseCSyntax(proto);
                    var Params = ImmutableArray.CreateBuilder<SymbolFuncParam>();
                    foreach (var param in src.Elements("param"))
                    {
                        var param_type = param.Element("type")!.Value;
                        var param_name = param.Element("name")!.Value;
                        var param_def = ParseCSyntax(param);
                        var param_comment = param.Element("comment")?.Value;
                        var param_len_raw = param.Attribute("len")?.Value;
                        var param_len = param_len_raw is null ? [] :
                            param_len_raw.StartsWith("latexmath") ? [] :
                            param_len_raw.Split(",").ToImmutableArray();
                        var object_type = param.Attribute("objecttype")?.Value;
                        Params.Add(new()
                        {
                            Name = param_name,
                            Type = param_type,
                            Def = param_def,
                            Len = param_len,
                            Comment = param_comment,
                            ObjectType = object_type,
                        });
                    }
                    types.Add(name, new SymbolTypeFuncPtr
                    {
                        Name = name,
                        Def = proto_def,
                        Return = return_type,
                        Params = Params.ToImmutable(),
                        Comment = comment,
                    });
                    return;
                }
                case "struct" or "union":
                {
                    if (src.Attribute("alias") is { } alias_attr)
                    {
                        var alias = alias_attr.Value;
                        types.Add(name, new SymbolTypeAlias
                        {
                            Name = name,
                            Target = alias,
                        });
                        return;
                    }
                    var is_union = cat is "union";
                    var comment = src.Attribute("comment")?.Value;
                    var returned_only = src.Attribute("returnedonly")?.Value is "true";
                    var struct_extends = src.Attribute("structextends")?.Value
                        .Split(",").ToImmutableArray() ?? [];
                    string? s_type = null;
                    var fields = ImmutableArray.CreateBuilder<SymbolStructField>();
                    foreach (var member in src.Elements("member"))
                    {
                        var api = member.Attribute("api")?.Value;
                        if (api is "vulkansc") continue;
                        var var_def = ParseCSyntax(member);
                        var member_type = member.Element("type")!.Value;
                        var member_name = member.Element("name")!.Value;
                        if (member_name is "sType")
                        {
                            s_type = member.Attribute("values")?.Value;
                            if (s_type is null)
                                Log.Warning("No sType values for struct {Name}", name);
                        }
                        var member_comment = member.Element("comment")?.Value;
                        var member_len_raw = member.Attribute("len")?.Value;
                        var member_len = member_len_raw is null ? [] :
                            member_len_raw.StartsWith("latexmath") ? [] :
                            member_len_raw.Split(",").ToImmutableArray();
                        var optional = member.Attribute("optional")?.Value.Split(",").Select(a => a is "true").ToImmutableArray() ?? [];
                        var object_type = member.Attribute("objecttype")?.Value;
                        fields.Add(new()
                        {
                            Name = member_name,
                            Type = member_type,
                            Def = var_def,
                            Len = member_len,
                            Comment = member_comment,
                            ObjectType = object_type,
                            Optional = optional,
                        });
                    }
                    types.Add(name, new SymbolTypeStruct
                    {
                        Name = name,
                        SType = s_type,
                        Fields = fields.ToImmutable(),
                        Extends = struct_extends,
                        Comment = comment,
                        IsUnion = is_union,
                        ReturnedOnly = returned_only,
                    });
                    return;
                }
                case null or "include" or "basetype":
                {
                    Log.Information("Skipped type {Name} ; Category={Category}", name, cat);
                    return;
                }
                default:
                    Log.Warning("TODO {Cat} \t {Name}", cat, name);
                    break;
            }
        }

        void LoadCommand(XElement src, string name)
        {
            if (src.Attribute("alias") is { } alias_attr)
            {
                var alias = alias_attr.Value;
                commands.Add(name, new()
                {
                    Name = name,
                    Alias = alias,
                    Type = null,
                    SuccessCodes = [],
                });
                return;
            }
            var api = src.Attribute("api")?.Value;
            if (api == "vulkansc") return;
            var export = src.Attribute("export")?.Value;
            var comment = src.Attribute("comment")?.Value;
            var successcodes = src.Attribute("successcodes")?.Value.Split(",").ToImmutableArray() ?? [];
            var proto = src.Element("proto");
            var return_type = proto!.Element("type")!.Value;
            var proto_def = ParseCSyntax(proto);
            var Params = ImmutableArray.CreateBuilder<SymbolFuncParam>();
            foreach (var param in src.Elements("param"))
            {
                var param_type = param.Element("type")!.Value;
                var param_name = param.Element("name")!.Value;
                var param_def = ParseCSyntax(param);
                var param_comment = param.Element("comment")?.Value;
                var param_len_raw = param.Attribute("len")?.Value;
                var param_len = param_len_raw is null ? [] :
                    param_len_raw.StartsWith("latexmath") ? [] :
                    param_len_raw.Split(",").ToImmutableArray();
                var param_optional = param.Attribute("optional")?.Value.Split(",").Select(a => a is "true").ToImmutableArray() ?? [];
                var object_type = param.Attribute("objecttype")?.Value;
                var param_api = param.Attribute("api")?.Value ?? "";
                if (param_api == "vulkansc") continue;
                Params.Add(new()
                {
                    Name = param_name,
                    Type = param_type,
                    Def = param_def,
                    Len = param_len,
                    Comment = param_comment,
                    ObjectType = object_type,
                    Optional = param_optional,
                });
            }
            commands.Add(name, new()
            {
                Name = name,
                Alias = null,
                Type = new()
                {
                    Name = name,
                    Def = proto_def,
                    Return = return_type,
                    Params = Params.ToImmutable(),
                    Comment = comment,
                },
                SuccessCodes = successcodes,
            });
        }

        void LoadFeature(XElement src, string name)
        {
            var api = src.Attribute("api")!.Value;
            var is_sc = api is "vulkansc";
            var Internal = src.Attribute("apitype")?.Value is "internal";
            var deps = src.Attribute("depends")?.Value.Split(",").ToImmutableArray() ?? [];
            var number = src.Attribute("number")!.Value;
            var fe_cmds = new HashSet<string>();
            foreach (var require in src.Elements("require"))
            {
                var req_comment = require.Attribute("comment")?.Value;
                if (req_comment is "API constants") continue;
                var Enums = require.Elements("enum");
                foreach (var Enum in Enums)
                {
                    var extends = Enum.Attribute("extends")?.Value;
                    if (extends is null) continue;
                    LoadExtendEnums(name, Enum, Enum.Attribute("name")!.Value, extends, null);
                }
                foreach (var cmd in require.Elements("command"))
                {
                    fe_cmds.Add(cmd.Attribute("name")!.Value);
                }
            }
            if (is_sc) return;
            features.Add(name, new()
            {
                Name = name,
                Commands = fe_cmds.ToFrozenSet(),
                Number = number,
                Depends = deps,
                Internal = Internal,
            });
        }

        void LoadExtension(XElement src, string name)
        {
            var disabled = src.Attribute("supported")?.Value is "disabled";
            var ext_number = long.Parse(src.Attribute("number")!.Value);
            var ex_cmds = new HashSet<string>();
            foreach (var require in src.Elements("require"))
            {
                var Enums = require.Elements("enum");
                foreach (var Enum in Enums)
                {
                    var extends = Enum.Attribute("extends")?.Value;
                    if (extends is null) continue;
                    LoadExtendEnums(name, Enum, Enum.Attribute("name")!.Value, extends, ext_number);
                }
                if (disabled) break;
                foreach (var cmd in require.Elements("command"))
                {
                    ex_cmds.Add(cmd.Attribute("name")!.Value);
                }
            }
            if (disabled) return;
            var type = src.Attribute("type")!.Value is "device" ? SymbolExtensionType.Device : SymbolExtensionType.Instance;
            var prefix = name.Split("_")[1];
            extensions.Add(name, new()
            {
                Name = name,
                Prefix = prefix,
                TailName = name[(4 + prefix.Length)..],
                Number = ext_number,
                Type = type,
                Commands = ex_cmds.ToFrozenSet(),
            });
        }

        void LoadExtendEnums(string ext, XElement src, string name, string extends, long? ext_num)
        {
            extends = extends.Replace("FlagBits", "Flags");
            var symbol = types.TryGetValue(extends, out var symbol_) ? (SymbolTypeEnum)symbol_ : null;
            if (symbol is null)
            {
                Log.Warning("Can not find enum {Extends} when enum item {Name} for extending in {Ext}",
                    extends, name, ext);
                return;
            }
            var comment = src.Attribute("comment")?.Value;
            EnumValue? value = null;
            if (src.Attribute("alias")?.Value is { } alias) value = new EnumValue.Alias(alias);
            else if (src.Attribute("bitpos")?.Value is { } bitpos) value = new EnumValue.BitPos(long.Parse(bitpos));
            else if (src.Attribute("value")?.Value is { } Value) value = new EnumValue.Expr(Value);
            else
            {
                var ex = ext_num;
                var extnumber = src.Attribute("extnumber")?.Value;
                if (extnumber is not null) ex = long.Parse(extnumber);
                if (ex is not null)
                {
                    var offset = long.Parse(src.Attribute("offset")!.Value);
                    var neg = src.Attribute("dir")?.Value is "-";
                    var val = 1_000_000_000 + (ex - 1) * 1_000 + offset;
                    if (neg) val = -val;
                    value = new EnumValue.Expr($"{val}");
                }
            }
            if (value is null)
            {
                Log.Warning("Invalid enum item {Extends}::{Name} for extending in {Ext}, no value",
                    extends, name, ext);
                return;
            }
            symbol.Items.TryAdd(name, new()
            {
                Name = name,
                Value = value,
                Comment = comment,
            });
        }
    }

    public static VarDef ParseCSyntax(XElement Node)
    {
        var strs = new StringBuilder();
        var cur_node = Node.FirstNode;
        while (cur_node is { } node)
        {
            cur_node = node.NextNode;
            if (node is XText text)
            {
                strs.Append(text.Value);
            }
            else if (node is XElement element)
            {
                if (element.Name == "comment") break;
                strs.Append(element.Value);
            }
            else throw new InvalidDataException(node.GetType().Name);
        }
        var code = strs.ToString().Trim();
        try
        {
            return (VarDef)CParser.VarDef.ParseOrThrow(code);
        }
        catch (Exception e)
        {
            throw new($"Parse failed on: \"{code}\"", e);
        }
    }

    public SymbolCommandScope GetCommandScope(SymbolCommand command)
    {
        if (command.Name == "vkGetDeviceProcAddr") return SymbolCommandScope.Instance;
        string type_name;
        if (command.Alias is { } alias)
        {
            var target = Commands[alias];
            type_name = target.Type!.Params[0].Type;
        }
        else
        {
            type_name = command.Type!.Params[0].Type;
        }
        var count = 0;
        for (;; count++)
        {
            if (!Types.TryGetValue(type_name, out var type))
                return SymbolCommandScope.Global;
            if (type is SymbolTypeAlias type_alias)
                if (!Types.TryGetValue(type_alias.Name, out type))
                    return SymbolCommandScope.Global;
            if (type is not SymbolTypeHandle handle)
                return SymbolCommandScope.Global;
            if (handle.Name is "VkDevice")
                return SymbolCommandScope.Device;
            if (handle.Name is "VkInstance")
                return SymbolCommandScope.Instance;
            if (handle.Parent is null) return SymbolCommandScope.Global;
            type_name = handle.Parent;
        }
    }

    public SymbolHandleScope GetHandleScope(SymbolTypeHandle handle)
    {
        while (true)
        {
            if (handle.Name is "VkInstance") return SymbolHandleScope.Instance;
            if (handle.Name is "VkDevice") return SymbolHandleScope.Device;
            if (handle.Parent is null) return SymbolHandleScope.Instance;
            if (!Types.TryGetValue(handle.Parent, out var parent)) return SymbolHandleScope.Instance;
            handle = (SymbolTypeHandle)parent;
        }
    }
}

[JsonPolymorphic(TypeDiscriminatorPropertyName = "Kind")]
[JsonDerivedType(typeof(SymbolTypeAlias), typeDiscriminator: "Alias")]
[JsonDerivedType(typeof(SymbolTypeEnum), typeDiscriminator: "Enum")]
[JsonDerivedType(typeof(SymbolTypeStruct), typeDiscriminator: "Struct")]
[JsonDerivedType(typeof(SymbolTypeFuncPtr), typeDiscriminator: "FuncPtr")]
[JsonDerivedType(typeof(SymbolTypeHandle), typeDiscriminator: "Handle")]
public abstract record SymbolType
{
    public required string Name { get; set; }
}

public record SymbolTypeAlias : SymbolType
{
    public required string Target { get; set; }
}

public record SymbolTypeHandle : SymbolType
{
    public string? Parent { get; set; }
    public bool Dispatchable { get; set; }
    public required string ObjectType { get; set; }
}

public record struct ApiConstantItem
{
    public required string Name { get; set; }
    public required string Type { get; set; }
    public required string Value { get; set; }
    public string? Comment { get; set; }
}

public record SymbolTypeEnum : SymbolType
{
    public required Dictionary<string, SymbolEnumItem> Items { get; set; }
    public bool Flags { get; set; }
    public bool x64 { get; set; }
    public string? Comment { get; set; }
}

public record struct SymbolEnumItem
{
    public required string Name { get; set; }
    public required EnumValue Value { get; set; }
    public string? Comment { get; set; }
}

[JsonPolymorphic(TypeDiscriminatorPropertyName = "Kind")]
[JsonDerivedType(typeof(Expr), typeDiscriminator: "Expr")]
[JsonDerivedType(typeof(Alias), typeDiscriminator: "Alias")]
[JsonDerivedType(typeof(BitPos), typeDiscriminator: "BitPos")]
public abstract record EnumValue
{
    public record Expr(string Val) : EnumValue
    {
        public override string ToString() => Val;
    }

    public record Alias(string Taget) : EnumValue
    {
        public override string ToString() => Taget;
    }

    public record BitPos(long Pos) : EnumValue
    {
        public override string ToString() => $"1 << {Pos}";
    }
}

public record SymbolTypeStruct : SymbolType
{
    public string? SType { get; set; }
    public ImmutableArray<SymbolStructField> Fields { get; set; }
    public ImmutableArray<string> Extends { get; set; }
    public string? Comment { get; set; }
    public bool IsUnion { get; set; }
    public bool ReturnedOnly { get; set; }
}

public record struct SymbolStructField
{
    public required string Name { get; set; }
    public required string Type { get; set; }
    public required VarDef Def { get; set; }
    public ImmutableArray<string> Len { get; set; }
    public string? Comment { get; set; }
    public string? ObjectType { get; set; }
    public ImmutableArray<bool> Optional { get; set; }
}

public record SymbolTypeFuncPtr : SymbolType
{
    public required VarDef Def { get; set; }
    public required string Return { get; set; }
    public ImmutableArray<SymbolFuncParam> Params { get; set; }
    public string? Comment { get; set; }
    public bool ReturnOptional { get; set; }
}

public record struct SymbolFuncParam
{
    public required string Name { get; set; }
    public required string Type { get; set; }
    public required VarDef Def { get; set; }
    public ImmutableArray<string> Len { get; set; }
    public string? Comment { get; set; }
    public string? ObjectType { get; set; }
    public ImmutableArray<bool> Optional { get; set; }
}

public record SymbolCommand
{
    public required string Name { get; set; }
    public required string? Alias { get; set; }
    public required SymbolTypeFuncPtr? Type { get; set; }
    public required ImmutableArray<string> SuccessCodes { get; set; }
}

public record SymbolFeature
{
    public required string Name { get; set; }
    public required FrozenSet<string> Commands { get; set; }
    public required ImmutableArray<string> Depends { get; set; }
    public required string Number { get; set; }
    public required bool Internal { get; set; }
}

public record SymbolExtension
{
    public required string Name { get; set; }
    public required string Prefix { get; set; }
    public required string TailName { get; set; }
    public required long Number { get; set; }
    public required SymbolExtensionType Type { get; set; }
    public required FrozenSet<string> Commands { get; set; }
}

public enum SymbolExtensionType
{
    Instance,
    Device,
}

public enum SymbolCommandScope
{
    Global,
    Instance,
    Device,
}

public enum SymbolHandleScope
{
    Instance,
    Device,
}
