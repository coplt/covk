using System.Diagnostics;
using System.Text;
using Gen.Parsers.C;

namespace Gen.Emitter;

public static class SysEmitter
{
    public static void Emit(StringBuilder sb, Symbols symbols)
    {
        sb.AppendLine();
        sb.AppendLine($"use crate::ffi::*;");
        sb.AppendLine($"use crate::*;");
        foreach (var constant in symbols.ApiConstants)
        {
            sb.AppendLine();
            sb.AppendLine($"/// `{constant.Name}` = `{constant.Value}`");
            if (constant.Comment is not null)
            {
                sb.AppendLine($"/// ");
                sb.AppendLine($"/// {constant.Comment.Replace("\n", "\n    /// ")}");
            }
            sb.AppendLine($"pub const {constant.Name}: {constant.Type} = {Utils.ApiConstantToRust(constant.Value)};");
        }
        foreach (var symbol in symbols.Types.Values.OrderBy(x => x.Name, StringComparer.Ordinal))
        {
            switch (symbol)
            {
                case SymbolTypeAlias alias:
                {
                    if (!symbols.Types.ContainsKey(alias.Target)) continue;
                    sb.AppendLine();
                    sb.AppendLine($"/// `{alias.Target}`");
                    sb.AppendLine($"pub type {alias.Name} = {alias.Target};");
                    continue;
                }
                case SymbolTypeHandle handle:
                {
                    sb.AppendLine();
                    var underlying = handle.Dispatchable ? "*mut void" : "u64";
                    sb.Append($"/// `{handle.Name}`");
                    if (handle.Parent is not null)
                        sb.Append($" : `{handle.Parent}`");
                    sb.AppendLine();
                    sb.AppendLine("#[repr(transparent)]");
                    sb.AppendLine("#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]");
                    sb.AppendLine($"pub struct {handle.Name}(pub {underlying});");
                    sb.AppendLine($"from_into_transparent!({handle.Name}: {underlying});");
                    if (handle.Dispatchable) sb.AppendLine($"from_into_transparent!({handle.Name}: u64);");
                    sb.AppendLine($"impl_handle!({handle.Name}: {handle.ObjectType}; {(
                        handle.Parent is null ? "RootHandle" : $"SubHandle = {handle.Parent}")});");
                    continue;
                }
                case SymbolTypeEnum Enum:
                {
                    sb.AppendLine();
                    var underlying = Enum.x64 ? "i64" : "i32";
                    if (Enum.Flags) sb.AppendLine($"flags!({Enum.Name});");
                    sb.AppendLine($"from_into_transparent!({Enum.Name}: {underlying});");
                    sb.AppendLine($"/// `{Enum.Name}`");
                    if (Enum.Comment is not null)
                    {
                        sb.AppendLine($"/// ");
                        sb.AppendLine($"/// {Enum.Comment.Replace("\n", "\n/// ")}");
                    }
                    sb.AppendLine("#[repr(transparent)]");
                    sb.AppendLine("#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]");
                    sb.AppendLine($"pub struct {Enum.Name}(pub {underlying});");

                    if (Enum.Items.Count != 0)
                    {
                        sb.AppendLine();
                        foreach (var item in Enum.Items.Values)
                        {
                            sb.AppendLine($"/// `{item.Value}`  ");
                            if (item.Comment is not null)
                            {
                                sb.AppendLine($"/// ");
                                sb.AppendLine($"/// {item.Comment.ProcessComment().Replace("\n", "\n/// ")}");
                            }
                            sb.Append($"pub const {item.Name}: {Enum.Name} = ");
                            switch (item.Value)
                            {
                                case EnumValue.Expr expr:
                                {
                                    sb.Append($"{Enum.Name}({expr.Val})");
                                    break;
                                }
                                case EnumValue.Alias alias:
                                {
                                    sb.Append(alias.Taget);
                                    break;
                                }
                                case EnumValue.BitPos pos:
                                {
                                    sb.Append($"{Enum.Name}(1 << {pos.Pos})");
                                    break;
                                }
                                default:
                                    throw new InvalidOperationException($"Unknown enum value type: {item.Value.GetType().Name}");
                            }
                            sb.AppendLine($";");
                        }
                    }

                    sb.AppendLine();
                    sb.Append($"impl_{(Enum.Flags ? "flags" : "enum")}_display!({Enum.Name} {{ ");
                    var first = true;
                    foreach (var item in Enum.Items.Values)
                    {
                        if (first) first = false;
                        else sb.Append(", ");
                        sb.Append($"{item.Name}");
                    }
                    sb.AppendLine($" }});");
                    continue;
                }
                case SymbolTypeFuncPtr FuncPtr:
                {
                    {
                        sb.AppendLine();
                        sb.AppendLine($"/// `{FuncPtr.Name}`");
                        if (FuncPtr.Comment is not null)
                        {
                            sb.AppendLine($"/// ");
                            sb.AppendLine($"/// {FuncPtr.Comment.Replace("\n", "\n    /// ")}");
                        }
                        sb.AppendLine($"pub type {FuncPtr.Name} = Option<{FuncPtr.Name[1..]}>;");
                    }
                    {
                        sb.AppendLine();
                        sb.AppendLine($"/// `{FuncPtr.Name}`");
                        if (FuncPtr.Comment is not null)
                        {
                            sb.AppendLine($"/// ");
                            sb.AppendLine($"/// {FuncPtr.Comment.Replace("\n", "\n    /// ")}");
                        }
                        sb.Append($"pub type {FuncPtr.Name[1..]} = unsafe extern \"system\" fn(");
                        var first = true;
                        foreach (var param in FuncPtr.Params)
                        {
                            if (first) first = false;
                            else sb.Append(", ");
                            sb.Append($"{param.Name}: ");
                            EmitParamType(sb, param.Def);
                        }
                        sb.Append($") -> ");
                        EmitReturnType(sb, FuncPtr.Def.Type);
                        sb.AppendLine($";");
                    }
                    continue;
                }
                case SymbolTypeStruct Struct:
                {
                    sb.AppendLine();
                    sb.AppendLine($"/// `{Struct.Name}`");
                    if (Struct.Comment is not null)
                    {
                        sb.AppendLine($"/// ");
                        sb.AppendLine($"/// {Struct.Comment.Replace("\n", "\n    /// ")}");
                    }
                    sb.AppendLine("#[repr(C)]");
                    sb.AppendLine($"#[derive({(Struct.IsUnion ? "" : "Debug, ")}Clone, Copy)]");
                    sb.AppendLine($"pub {(Struct.IsUnion ? "union" : "struct")} {Struct.Name} {{");
                    foreach (var field in Struct.Fields)
                    {
                        if (field.Comment is not null)
                        {
                            sb.AppendLine($"    /// {field.Comment.Replace("\n", "\n    /// ")}");
                        }
                        sb.Append($"    pub {field.Name.CheckRustKeyword()}: ");
                        EmitFieldType(sb, field.Def, field.Def.Type);
                        sb.AppendLine($",");
                    }
                    sb.AppendLine($"}}");
                    if (Struct.IsUnion) sb.AppendLine($"impl_debug_for_union!({Struct.Name});");
                    if (Struct.SType is not null)
                    {
                        sb.AppendLine($"impl_default_zeroed_with_s_type!({Struct.Name} : {Struct.SType});");
                    }
                    else
                    {
                        sb.AppendLine($"impl_default_zeroed!({Struct.Name});");
                    }
                    if (!Struct.Extends.IsEmpty)
                    {
                        sb.Append($"impl_extends!({Struct.Name} : ");
                        var first = true;
                        foreach (var extend in Struct.Extends)
                        {
                            if (first) first = false;
                            else sb.Append(", ");
                            sb.Append(extend);
                        }
                        sb.AppendLine(");");
                    }
                    continue;
                }
                default:
                    throw new InvalidOperationException($"Unknown symbol type: {symbol.GetType().Name}");
            }
        }
        foreach (var command in symbols.Commands.Values.OrderBy(a => a.Name))
        {
            if (command.Alias is { } alias)
            {
                sb.AppendLine();
                sb.AppendLine($"/// `{command.Name}` = `{alias}`");
                sb.AppendLine($"pub type PFN_{command.Name} = PFN_{alias};");
                sb.AppendLine();
                sb.AppendLine($"/// `{command.Name}` = `{alias}`");
                sb.AppendLine($"pub type FN_{command.Name} = FN_{alias};");
                continue;
            }
            {
                var FuncPtr = command.Type!;
                sb.AppendLine();
                sb.AppendLine($"/// `{command.Name}`");
                if (FuncPtr.Comment is not null)
                {
                    sb.AppendLine($"/// ");
                    sb.AppendLine($"/// {FuncPtr.Comment.Replace("\n", "\n    /// ")}");
                }
                sb.AppendLine($"pub type PFN_{command.Name} = Option<FN_{command.Name}>;");
                sb.AppendLine();
                sb.AppendLine($"/// `{command.Name}`");
                if (FuncPtr.Comment is not null)
                {
                    sb.AppendLine($"/// ");
                    sb.AppendLine($"/// {FuncPtr.Comment.Replace("\n", "\n    /// ")}");
                }
                sb.Append($"pub type FN_{command.Name} = unsafe extern \"system\" fn(");
                var first = true;
                foreach (var param in FuncPtr.Params)
                {
                    if (first) first = false;
                    else sb.Append(", ");
                    sb.Append($"{param.Name.CheckRustKeyword()}: ");
                    EmitParamType(sb, param.Def);
                }
                sb.Append($") -> ");
                EmitReturnType(sb, FuncPtr.Def.Type);
                sb.AppendLine($";");
                sb.AppendLine();
            }
        }

        #region Features

        sb.AppendLine();
        sb.AppendLine($"pub(crate) mod vtbl_gen {{");
        sb.AppendLine($"    use core::ffi::CStr;");
        sb.AppendLine($"    use crate::loader::{{ProcAddr, LoadingError}};");
        sb.AppendLine($"    use crate::vtbl::*;");
        sb.AppendLine($"    use crate::ffi::*;");
        sb.AppendLine($"    use crate::*;");
        foreach (
            var g in
            symbols.Features.Values.GroupBy(a => a.Number).OrderBy(a => a.Key)
        )
        {
            var number = g.Key;
            var ver = number.Replace(".", "_");

            #region Commands

            var commands = g.SelectMany(a => a.Commands)
                .OrderBy(a => a, StringComparer.Ordinal)
                .Select(a => symbols.Commands[a])
                .GroupBy(symbols.GetCommandScope)
                .Where(a => a.Key is not SymbolCommandScope.Global)
                .OrderBy(a => a.Key)
                .ToDictionary(k => k.Key, v => v.ToList());

            ReadOnlySpan<SymbolCommandScope> groups = [SymbolCommandScope.Instance, SymbolCommandScope.Device];
            foreach (var group in groups)
            {
                var type = $"{group}Commands_{ver}";
                commands.TryGetValue(group, out var group_commands);
                group_commands ??= [];

                sb.AppendLine();
                sb.AppendLine($"    /// `Vulkan {number}` {group}Commands");
                sb.AppendLine($"    #[derive(Debug, Clone, Copy)]");
                sb.AppendLine($"    pub struct {type} {{");

                #region Fields

                foreach (var command in group_commands)
                {
                    sb.AppendLine($"        pub {command.Name}: PFN_{command.Name},");
                }

                #endregion

                sb.AppendLine($"    }}");

                #region Load

                sb.AppendLine();
                sb.AppendLine($"    impl {type} {{");
                sb.AppendLine($"        pub unsafe fn load(mut get: impl FnMut(&CStr) -> Option<ProcAddr>) -> Self {{");
                sb.AppendLine($"            unsafe {{ Self {{");

                #region Fields

                foreach (var command in group_commands)
                {
                    sb.AppendLine($"                {command.Name}: get(c!(\"{command.Name}\")).map(|a| a.cast()),");
                }

                #endregion

                sb.AppendLine($"            }} }}");
                sb.AppendLine($"        }}");
                sb.AppendLine($"    }}");

                #endregion

                #region Methods

                sb.AppendLine();
                sb.AppendLine($"    impl {type} {{");

                foreach (var command in group_commands)
                {
                    if (command.Name is "vkGetDeviceProcAddr" or "vkGetInstanceProcAddr") continue;
                    var method_name = command.Name;
                    if (method_name.StartsWith("vk", StringComparison.OrdinalIgnoreCase))
                        method_name = method_name[2..];
                    if (command.Alias is not null) throw new UnreachableException("Core never alias");
                    sb.AppendLine($"        /// `{command.Name}`");
                    sb.Append($"        pub unsafe fn {method_name}(&self");
                    var sig = command.Type!;
                    foreach (var param in sig.Params)
                    {
                        sb.Append(", ");
                        sb.Append($"{param.Name.CheckRustKeyword()}: ");
                        EmitParamType(sb, param.Def);
                    }
                    sb.Append($") -> ");
                    EmitReturnType(sb, sig.Def.Type);
                    sb.AppendLine($" {{");
                    sb.Append($"            unsafe {{ (self.{command.Name}.expect(\"Unable to load {command.Name}\"))(");
                    var first = true;
                    foreach (var param in sig.Params)
                    {
                        if (first) first = false;
                        else sb.Append(", ");
                        sb.Append($"{param.Name.CheckRustKeyword()}");
                    }
                    sb.AppendLine($") }}");
                    sb.AppendLine($"        }}");
                }

                sb.AppendLine($"    }}");

                #endregion
            }

            #endregion
        }
        sb.AppendLine($"}}");

        #endregion

        #region Extensions

        sb.AppendLine();
        sb.AppendLine($"pub use extensions::*;");
        sb.AppendLine($"/// Extensions");
        sb.AppendLine($"pub mod extensions {{");
        foreach (
            var ext_group in
            symbols.Extensions.Values.OrderBy(a => a.Number)
                .GroupBy(a => a.Prefix)
                .OrderBy(a => a.Key)
        )
        {
            sb.AppendLine($"    /// `{ext_group.Key}` prefix extensions");
            sb.AppendLine($"    pub mod {ext_group.Key.ToLower()} {{");
            sb.AppendLine($"        use core::ffi::CStr;");
            sb.AppendLine($"        use crate::loader::*;");
            sb.AppendLine($"        use crate::ffi::*;");
            sb.AppendLine($"        use crate::*;");
            foreach (var extension in ext_group)
            {
                var prefix = char.IsNumber(extension.TailName[0]) ? "_" : "";
                sb.AppendLine();
                sb.AppendLine($"        /// `{extension.Name}`");
                sb.AppendLine($"        pub mod {prefix}{extension.TailName.ToLower()} {{");
                sb.AppendLine($"            use super::*;");
                sb.AppendLine();
                sb.AppendLine($"            /// `{extension.Name}`");
                sb.AppendLine($"            pub const NAME: &'static CStr = c!(\"{extension.Name}\");");

                #region Commands

                var commands = extension.Commands
                    .OrderBy(a => a, StringComparer.Ordinal)
                    .Select(a => symbols.Commands[a])
                    .GroupBy(symbols.GetCommandScope)
                    .Where(a => a.Key is not SymbolCommandScope.Global)
                    .OrderBy(a => a.Key)
                    .ToDictionary(k => k.Key, v => v.ToList());

                ReadOnlySpan<SymbolCommandScope> groups = [SymbolCommandScope.Instance, SymbolCommandScope.Device];
                foreach (var group in groups)
                {
                    var type = $"{group}Commands";
                    commands.TryGetValue(group, out var group_commands);
                    if (group_commands == null) continue;

                    sb.AppendLine();
                    sb.AppendLine($"            /// {group}Commands");
                    sb.AppendLine($"            #[derive(Debug, Clone, Copy)]");
                    sb.AppendLine($"            pub struct {type} {{");

                    #region Fields

                    foreach (var command in group_commands)
                    {
                        sb.AppendLine($"                pub {command.Name}: PFN_{command.Name},");
                    }

                    #endregion

                    sb.AppendLine($"            }}");

                    #region Load

                    sb.AppendLine();
                    sb.AppendLine($"            impl {type} {{");
                    sb.AppendLine($"                pub unsafe fn load(mut get: impl FnMut(&CStr) -> Option<ProcAddr>) -> Self {{");
                    sb.AppendLine($"                    unsafe {{ Self {{");

                    #region Fields

                    foreach (var command in group_commands)
                    {
                        sb.AppendLine($"                        {command.Name}: get(c!(\"{command.Name}\")).map(|a| a.cast()),");
                    }

                    #endregion

                    sb.AppendLine($"                    }} }}");
                    sb.AppendLine($"                }}");
                    sb.AppendLine($"            }}");

                    #endregion

                    #region Methods

                    sb.AppendLine();
                    sb.AppendLine($"            impl {type} {{");

                    foreach (var command in group_commands)
                    {
                        var method_name = command.Name;
                        if (method_name.StartsWith("vk", StringComparison.OrdinalIgnoreCase))
                            method_name = method_name[2..];
                        var sig = command.Type;
                        if (command.Alias is { } alias)
                        {
                            var cmd = symbols.Commands[alias];
                            sig = cmd.Type;
                        }
                        sb.AppendLine($"                /// `{command.Name}`");
                        sb.Append($"                pub unsafe fn {method_name}(&self");
                        foreach (var param in sig!.Params)
                        {
                            sb.Append(", ");
                            sb.Append($"{param.Name.CheckRustKeyword()}: ");
                            EmitParamType(sb, param.Def);
                        }
                        sb.Append($") -> ");
                        EmitReturnType(sb, sig.Def.Type);
                        sb.AppendLine($" {{");
                        sb.Append($"                    unsafe {{ (self.{command.Name}.expect(\"Unable to load {command.Name}\"))(");
                        var first = true;
                        foreach (var param in sig.Params)
                        {
                            if (first) first = false;
                            else sb.Append(", ");
                            sb.Append($"{param.Name.CheckRustKeyword()}");
                        }
                        sb.AppendLine($") }}");
                        sb.AppendLine($"                }}");
                    }

                    sb.AppendLine($"            }}");

                    #endregion
                }

                #endregion

                sb.AppendLine($"        }}");
            }
            sb.AppendLine($"    }}");
        }
        sb.AppendLine($"}}");

        #endregion
    }

    public static void EmitParamType(StringBuilder sb, VarDef def)
    {
        if (def.ArraySize is not null)
        {
            sb.Append("*");
            if (def.ConstArray) sb.Append("const ");
            else sb.Append("mut ");
        }
        EmitParamType(sb, def.Type);
    }
    private static void EmitParamType(StringBuilder sb, CSyntax syn)
    {
        while (true)
        {
            switch (syn)
            {
                case Ident a:
                {
                    if (a.Name is "char") sb.Append("c_");
                    sb.Append(a.Name.Replace("FlagBits", "Flags"));
                    return;
                }
                case Ptr a:
                {
                    sb.Append('*');
                    sb.Append(a.Const ? "const " : "mut ");
                    syn = a.Target;
                    continue;
                }
                default:
                    throw new InvalidOperationException($"Unknown symbol type: {syn.GetType().Name}");
            }
            break;
        }
    }

    public static void EmitReturnType(StringBuilder sb, CSyntax syn, CSyntax? parent = null)
    {
        switch (syn)
        {
            case Ident a:
            {
                if (parent == null && a.Name is "void")
                {
                    sb.Append("()");
                    return;
                }
                if (a.Name is "char") sb.Append("c_");
                sb.Append(a.Name.Replace("FlagBits", "Flags"));
                return;
            }
            case Ptr a:
            {
                sb.Append('*');
                sb.Append(a.Const ? "const " : "mut ");
                EmitReturnType(sb, a.Target, syn);
                return;
            }
            default: throw new InvalidOperationException($"Unknown symbol type: {syn.GetType().Name}");
        }
    }

    public static void EmitFieldType(StringBuilder sb, VarDef def, CSyntax syn, CSyntax? parent = null)
    {
        if (parent is null)
        {
            if (def.ArraySize is not null)
            {
                sb.Append("[");
            }
        }
        switch (syn)
        {
            case Ident a:
            {
                if (a.Name is "char") sb.Append("c_");
                var name = a.Name.Replace("FlagBits", "Flags");
                sb.Append(name);
                break;
            }
            case Ptr a:
            {
                sb.Append('*');
                sb.Append(a.Const ? "const " : "mut ");
                EmitFieldType(sb, def, a.Target, syn);
                break;
            }
            default: throw new InvalidOperationException($"Unknown symbol type: {syn.GetType().Name}");
        }
        if (parent is null)
        {
            if (def.ArraySize is not null)
            {
                sb.Append($"; ");
                if (def.ArraySize is Number number)
                {
                    sb.Append(number.Value);
                }
                else if (def.ArraySize is Ident ident)
                {
                    sb.Append(ident.Name);
                }
                sb.Append($" as usize]");
            }
        }
    }
}
