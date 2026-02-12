using System.Collections.Immutable;
using System.Diagnostics;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Text;
using System.Text.RegularExpressions;
using CaseConverter;
using Gen.Parsers.C;
using Serilog;

namespace Gen.Emitter;

public partial class RustEmitter
{
    private const string GeneratedFile = "// generated file, do not modify manually";

    private NameMapper name_map = new();
    public Task Emit(Symbols symbols, string mod_path, string mod_dir)
    {
        var tasks = new List<Task>();

        #region Mod

        {
            var sb = new StringBuilder();
            sb.AppendLine(GeneratedFile);
            sb.AppendLine();
            sb.AppendLine($"pub mod raw_hnds;");
            sb.AppendLine($"pub mod fn_ptrs;");
            sb.AppendLine($"pub mod enums;");
            sb.AppendLine($"pub mod structs;");
            sb.AppendLine($"/// Rusty vulkan struct builders");
            sb.AppendLine($"pub mod new;");
            sb.AppendLine($"pub mod setter;");
            sb.AppendLine($"/// Vulkan commands");
            sb.AppendLine($"pub mod main_commands;");
            sb.AppendLine($"/// Vulkan extensions");
            sb.AppendLine($"pub mod extensions;");
            sb.AppendLine($"/// Object handles");
            sb.AppendLine($"pub mod hnd;");
            sb.AppendLine($"pub mod ext_commands;");
            sb.AppendLine($"pub mod prelude;");
            sb.AppendLine();
            tasks.Add(File.WriteAllTextAsync(mod_path, sb.ToString(), Encoding.UTF8));
        }

        #endregion

        #region Raw Handles

        var hnds = symbols.Types.Values
            .Where(a =>
            {
                if (a is SymbolTypeHandle) return true;
                if (a is SymbolTypeAlias alias && symbols.Types.TryGetValue(alias.Target, out var target))
                {
                    return target is SymbolTypeHandle;
                }
                return false;
            })
            .OrderBy(x => x.Name, StringComparer.Ordinal).ToList();

        {
            var sb = new StringBuilder();
            sb.AppendLine(GeneratedFile);

            sb.AppendLine();
            sb.AppendLine($"");
            sb.AppendLine($"use crate::vk::*;");
            sb.AppendLine($"use ::core::ptr::NonNull;");
            sb.AppendLine($"use ::core::num::NonZeroU64;");
            sb.AppendLine($"use crate::sys::ffi::*;");
            sb.AppendLine($"use crate::sys;");

            foreach (var symbol in hnds)
            {
                switch (symbol)
                {
                    case SymbolTypeAlias alias:
                    {
                        sb.AppendLine();
                        sb.AppendLine($"/// `{alias.Name}` = `{alias.Target}`");
                        sb.AppendLine($"pub type {name_map.Map(alias.Name)} = {name_map.Map(alias.Target)};");
                        continue;
                    }
                    case SymbolTypeHandle handle:
                    {
                        sb.AppendLine();
                        var name = name_map.Map(handle.Name);
                        var parent_name = handle.Parent is null ? null : name_map.Map(handle.Parent);
                        sb.Append($"/// `{handle.Name}`");
                        if (handle.Parent is not null)
                            sb.Append($" : `{handle.Parent}`");
                        sb.AppendLine();
                        sb.AppendLine("#[repr(transparent)]");
                        sb.AppendLine("#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]");
                        sb.Append($"pub struct {name}(pub ");
                        sb.Append(handle.Dispatchable ? "NonNull<void>" : "NonZeroU64");
                        sb.AppendLine($");");
                        sb.AppendLine($"from_by_transmute!({name} => sys::{handle.Name});");
                        sb.AppendLine($"impl_raw_handle!({name}: {handle.ObjectType} => sys::{handle.Name}; {(
                            handle.Parent is null ? "RawRootHandle" : $"RawSubHandle = {parent_name}")});");
                        if (handle.Dispatchable)
                            sb.AppendLine($"impl_debug_for_raw_dispatchable_handle!({name});");
                        else
                            sb.AppendLine($"impl_debug_for_raw_non_dispatchable_handle!({name});");

                        sb.AppendLine();
                        sb.AppendLine($"impl {name} {{");
                        if (handle.Dispatchable)
                        {
                            sb.AppendLine($"    /// Creates a new `{name}` from a raw handle value. Returns `None` if the value is null.");
                            sb.AppendLine($"    pub fn new(raw: *mut void) -> Option<Self> {{");
                            sb.AppendLine($"        NonNull::new(raw).map(Self)");
                            sb.AppendLine($"    }}");
                            sb.AppendLine($"    /// Creates a new `{name}` from a raw handle value without checking if it's null. The caller must ensure that the value is not null.");
                            sb.AppendLine($"    pub unsafe fn new_unchecked(raw: *mut void) -> Self {{");
                            sb.AppendLine($"        Self(unsafe {{ NonNull::new_unchecked(raw) }})");
                            sb.AppendLine($"    }}");
                        }
                        else
                        {
                            sb.AppendLine($"   /// Creates a new `{name}` from a raw handle value. Returns `None` if the value is zero.");
                            sb.AppendLine($"   pub fn new(raw: u64) -> Option<Self> {{");
                            sb.AppendLine($"       NonZeroU64::new(raw).map(Self)");
                            sb.AppendLine($"   }}");
                            sb.AppendLine($"   /// Creates a new `{name}` from a raw handle value without checking if it's zero. The caller must ensure that the value is not zero.");
                            sb.AppendLine($"   pub unsafe fn new_unchecked(raw: u64) -> Self {{");
                            sb.AppendLine($"       Self(unsafe {{ NonZeroU64::new_unchecked(raw) }})");
                            sb.AppendLine($"   }}");
                        }
                        sb.AppendLine($"}}");

                        continue;
                    }
                    default:
                        continue;
                }
            }

            sb.AppendLine();

            var path = Path.Join(mod_dir, "./raw_hnds.rs");
            tasks.Add(File.WriteAllTextAsync(path, sb.ToString(), Encoding.UTF8));
        }

        #endregion

        #region Enums

        var enums = symbols.Types.Values
            .Where(a =>
            {
                if (a is SymbolTypeEnum) return true;
                if (a is SymbolTypeAlias alias && symbols.Types.TryGetValue(alias.Target, out var target))
                {
                    return target is SymbolTypeEnum;
                }
                return false;
            })
            .OrderBy(x => x.Name, StringComparer.Ordinal).ToList();

        {
            var sb = new StringBuilder();
            sb.AppendLine(GeneratedFile);
            sb.AppendLine();
            sb.AppendLine($"use crate::vk::*;");
            sb.AppendLine($"use ::core::ptr::NonNull;");
            sb.AppendLine($"use ::core::num::NonZeroU64 ;");
            sb.AppendLine($"use crate::sys::ffi::*;");
            sb.AppendLine($"use crate::sys::ffi;");
            sb.AppendLine($"use crate::sys;");
            foreach (var symbol in enums)
            {
                switch (symbol)
                {
                    case SymbolTypeAlias alias:
                    {
                        if (!symbols.Types.TryGetValue(alias.Target, out var target)) continue;
                        sb.AppendLine();
                        sb.AppendLine($"/// `{alias.Name}` = `{alias.Target}`");
                        sb.AppendLine($"pub type {name_map.Map(alias.Name)} = {name_map.Map(alias.Target)};");
                        continue;
                    }
                    case SymbolTypeEnum Enum:
                    {
                        sb.AppendLine();
                        var enum_name = name_map.Map(Enum.Name);
                        var underlying = Enum.x64 ? "i64" : "i32";
                        sb.AppendLine($"/// `{Enum.Name}`");
                        if (Enum.Comment is not null)
                        {
                            sb.AppendLine($"/// ");
                            sb.AppendLine($"/// {Enum.Comment.Replace("\n", "\n    /// ")}");
                        }
                        sb.AppendLine("#[repr(transparent)]");
                        sb.AppendLine("#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]");
                        sb.AppendLine($"pub struct {enum_name}(pub {underlying});");
                        if (Enum.Flags) sb.AppendLine($"flags!({enum_name});");
                        sb.AppendLine($"from_into_transparent!({enum_name} : {underlying});");
                        sb.AppendLine($"from_by_transmute!({enum_name} => sys::{Enum.Name});");
                        sb.AppendLine($"from_by_transmute!(sys::{Enum.Name} => {enum_name});");
                        sb.AppendLine($"impl_to_sys!({enum_name} => sys::{Enum.Name});");
                        sb.AppendLine($"impl_enum!({enum_name} => sys::{Enum.Name});");

                        #region Items

                        if (Enum.Items.Count != 0)
                        {
                            sb.AppendLine();
                            sb.AppendLine($"impl {enum_name} {{");
                            var item_name_cache = new HashSet<string>();
                            var mapped_item_name_cache = new HashSet<string>();
                            foreach (var item in Enum.Items.Values)
                            {
                                if (!item_name_cache.Add(item.Name.RemoveBitSuffix())) continue;
                                var item_name = name_map.MapEnumItem(Enum.Name, item.Name, Enum.Items);
                                if (!mapped_item_name_cache.Add(item_name)) continue;
                                sb.AppendLine($"    /// `{item.Name}` = `{item.Value}`  ");
                                if (item.Comment is not null)
                                {
                                    sb.AppendLine($"    /// ");
                                    sb.AppendLine($"    /// {item.Comment.ProcessComment().Replace("\n", "\n        /// ")}");
                                }
                                sb.AppendLine($"    pub const {item_name}: Self = Self(sys::{item.Name}.0);");
                            }
                            sb.AppendLine($"}}");
                        }

                        #endregion

                        #region Display

                        {
                            sb.AppendLine();
                            sb.Append($"impl_{(Enum.Flags ? "flags" : "enum")}_display!({enum_name} = {Enum.Name} {{ ");
                            var first = true;
                            var item_name_cache = new HashSet<string>();
                            var mapped_item_name_cache = new HashSet<string>();
                            foreach (var item in Enum.Items.Values)
                            {
                                if (!item_name_cache.Add(item.Name.RemoveBitSuffix())) continue;
                                var item_name = name_map.MapEnumItem(Enum.Name, item.Name, Enum.Items);
                                if (!mapped_item_name_cache.Add(item_name)) continue;

                                if (first) first = false;
                                else sb.Append(", ");
                                sb.Append($"{item_name}");
                            }
                            sb.AppendLine($" }});");
                        }

                        #endregion

                        continue;
                    }
                    default:
                        continue;
                }
            }
            sb.AppendLine();

            var path = Path.Join(mod_dir, "./enums.rs");
            tasks.Add(File.WriteAllTextAsync(path, sb.ToString(), Encoding.UTF8));
        }

        #endregion

        #region FnPtrs

        var fn_pts = symbols.Types.Values
            .Where(a =>
            {
                if (a is SymbolTypeFuncPtr) return true;
                if (a is SymbolTypeAlias alias && symbols.Types.TryGetValue(alias.Target, out var target))
                {
                    return target is SymbolTypeFuncPtr;
                }
                return false;
            })
            .OrderBy(x => x.Name, StringComparer.Ordinal).ToList();

        {
            var sb = new StringBuilder();
            sb.AppendLine(GeneratedFile);
            sb.AppendLine($"#![allow(non_camel_case_types)]");
            sb.AppendLine();
            sb.AppendLine($"use crate::{{vk::*, vk::ffi::*}};");
            foreach (var symbol in fn_pts)
            {
                switch (symbol)
                {
                    case SymbolTypeAlias alias:
                    {
                        if (!symbols.Types.TryGetValue(alias.Target, out var target)) continue;
                        sb.AppendLine();
                        sb.AppendLine($"/// `{alias.Name}` = `{alias.Target}`");
                        sb.AppendLine($"pub type {alias.Name} = {alias.Target};");
                        sb.AppendLine($"/// `{alias.Name[1..]}` = `{alias.Target[1..]}`");
                        sb.AppendLine($"pub type {alias.Name[1..]} = {alias.Target[1..]};");
                        continue;
                    }
                    case SymbolTypeFuncPtr fp:
                    {
                        sb.AppendLine();
                        sb.AppendLine($"/// `{fp.Name}`");
                        sb.AppendLine($"pub type {fp.Name} = Option<{fp.Name[1..]}>;");
                        sb.AppendLine($"/// `{fp.Name[1..]}`");
                        sb.Append($"pub type {fp.Name[1..]} = unsafe extern \"system\" fn(");
                        if (!fp.Params.IsEmpty) sb.AppendLine();
                        foreach (var param in fp.Params)
                        {
                            sb.Append($"    {NameMapper.MapCommandName(param.Name)}: ");
                            EmitParamType(sb, param.Def);
                            sb.AppendLine(", ");
                        }
                        sb.Append($") -> ");
                        EmitReturnType(sb, fp.Def.Type);
                        sb.AppendLine($";");

                        continue;
                    }
                    default:
                        continue;
                }
            }
            sb.AppendLine();

            var path = Path.Join(mod_dir, "./fn_ptrs.rs");
            tasks.Add(File.WriteAllTextAsync(path, sb.ToString(), Encoding.UTF8));
        }

        #endregion

        #region Builder

        var structs = symbols.Types.Values
            .Where(a =>
            {
                if (a is SymbolTypeStruct) return true;
                if (a is SymbolTypeAlias alias && symbols.Types.TryGetValue(alias.Target, out var target))
                {
                    return target is SymbolTypeStruct;
                }
                return false;
            })
            .OrderBy(x => x.Name, StringComparer.Ordinal).ToList();
        var struct_info = new Dictionary<string, StructInfo>();

        foreach (var symbol in structs)
        {
            switch (symbol)
            {
                case SymbolTypeStruct Struct:
                {
                    var field_names = new HashSet<string>();
                    var len_targets = new Dictionary<string, int>();
                    var field_types = new Dictionary<string, MappedType>();
                    var builder_fields = new HashSet<string>();
                    foreach (var field in Struct.Fields)
                    {
                        if (field.Name is "sType" or "pNext") continue;
                        field_names.Add(field.Name);
                    }
                    foreach (var (field, i) in Struct.Fields.Select((a, b) => (a, b)))
                    {
                        if (field.Name is "sType" or "pNext") continue;
                        if (field.Type is "void") continue;
                        if (field.Len is [{ } tar and not ("null-terminated" or "1"), ..])
                        {
                            if (field_names.Contains(tar)) len_targets.TryAdd(tar, i);
                        }
                    }
                    var any_life_time = false;
                    var cur_field_is_ref = false;
                    var nth_ptr = 0;
                    var ctx = new NameMapper.MapBuilderTypeCtx
                    {
                        symbols = symbols,
                        field_names = field_names,
                        nth_ptr = ref nth_ptr,
                        cur_field_is_ref = ref cur_field_is_ref,
                    };
                    foreach (var field in Struct.Fields)
                    {
                        nth_ptr = 0;
                        cur_field_is_ref = false;
                        var type = name_map.MapBuilderFieldType(ctx, field);
                        if (type is MappedType.SimpleType("Bool")) type = new MappedType.SimpleType("bool");
                        field_types.Add(field.Name, type);
                        if (field.Name is "sType" or "pNext") continue;
                        if (field.Optional is [true, ..]) continue;
                        if (len_targets.ContainsKey(field.Name))
                        {
                            if (
                                type is MappedType.SimpleType
                                {
                                    Name: "uint32_t" or "uint64_t" or "int32_t" or "int64_t" or "size_t"
                                    or "uint8_t" or "uint16_t" or "int8_t" or "int16_t",
                                }
                            ) continue;
                            else len_targets.Remove(field.Name);
                        }
                        if (cur_field_is_ref) any_life_time = true;
                        builder_fields.Add(field.Name);
                    }
                    var has_s_type = Struct.Fields is [{ Name: "sType" }, ..];
                    struct_info.Add(Struct.Name, new()
                    {
                        Struct = Struct,
                        Fields = field_types,
                        BuilderFields = builder_fields,
                        LenTargets = len_targets,
                        RequiredFieldCount = builder_fields.Count,
                        NeedLifeTime = any_life_time || has_s_type,
                        BuilderNeedLifeTime = any_life_time,
                        NeedLifeTime_Collected = any_life_time,
                    });

                    break;
                }
                default: continue;
            }
        }

        foreach (var info in struct_info)
        {
            info.Value.SyncLifeTime(struct_info);
        }

        {
            var sb = new StringBuilder();
            sb.AppendLine(GeneratedFile);
            sb.AppendLine();
            sb.AppendLine($"use crate::vk::*;");
            sb.AppendLine($"use crate::generated::enums::*;");
            sb.AppendLine($"use ::core::ptr::NonNull;");
            sb.AppendLine($"use ::core::num::NonZeroU64;");
            sb.AppendLine($"use crate::sys::ffi::*;");
            sb.AppendLine($"use super::fn_ptrs::*;");
            sb.AppendLine($"use crate::sys;");
            sb.AppendLine($"use crate::vk;");
            sb.AppendLine($"use crate::Abi;");

            foreach (var symbol in structs)
            {
                switch (symbol)
                {
                    case SymbolTypeAlias alias: continue;
                    case SymbolTypeStruct Struct:
                    {
                        var name = name_map.Map(Struct.Name);
                        var info = struct_info[Struct.Name];
                        var new_lifetime = info.BuilderNeedLifeTime ? "<'a>" : "";
                        var vk_lifetime = info.NeedLifeTime ? "<'a>" : "";
                        var any_lifetime = info.NeedLifeTime || info.BuilderNeedLifeTime ? "<'a>" : "";
                        var vk_only_lifetime = info.NeedLifeTime && !info.BuilderNeedLifeTime ? "<'a>" : "";
                        var new_only_lifetime = !info.NeedLifeTime && info.BuilderNeedLifeTime ? "<'a>" : "";

                        sb.AppendLine();
                        if (info.RequiredFieldCount == 0)
                        {
                            sb.AppendLine($"impl{vk_lifetime} vk::{name}{vk_lifetime} {{");
                            sb.AppendLine($"    /// Create the default {name}");
                            sb.AppendLine($"    pub fn new() -> Self {{");
                            sb.AppendLine($"        Default::default()");
                            sb.AppendLine($"    }}");
                            sb.AppendLine($"}}");
                        }
                        else
                        {
                            if (Struct.IsUnion)
                            {
                                sb.AppendLine($"/// `{Struct.Name}` Builder");
                                if (Struct.Comment is not null)
                                {
                                    sb.AppendLine($"/// ");
                                    sb.AppendLine($"/// {Struct.Comment.Replace("\n", "\n    /// ")}");
                                }
                                sb.AppendLine($"#[derive(Debug)]");
                                sb.AppendLine($"pub enum {name}{new_lifetime} {{");
                                foreach (var field in Struct.Fields)
                                {
                                    var type = info.Fields[field.Name];
                                    if (field.Comment is not null)
                                    {
                                        sb.AppendLine($"    /// {field.Comment.Replace("\n", "\n    /// ")}");
                                        sb.AppendLine($"    /// ");
                                    }
                                    sb.AppendLine($"    /// `{field.Def.ToC()}`");
                                    sb.Append("    ");
                                    sb.Append(NameMapper.MapBuilderFieldName(field.Name, true));
                                    sb.Append("(");
                                    type.WriteBuilderField(sb, struct_info);
                                    sb.AppendLine("),");
                                }
                                sb.AppendLine($"}}");
                            }
                            else
                            {
                                sb.AppendLine($"/// `{Struct.Name}` Builder");
                                if (Struct.Comment is not null)
                                {
                                    sb.AppendLine($"/// ");
                                    sb.AppendLine($"/// {Struct.Comment.Replace("\n", "\n    /// ")}");
                                }
                                sb.AppendLine($"#[derive(Debug)]");
                                sb.AppendLine($"pub struct {name}{new_lifetime} {{");
                                foreach (var field in Struct.Fields)
                                {
                                    if (field.Name is "sType" or "pNext") continue;
                                    if (field.Optional is [true, ..]) continue;
                                    if (info.LenTargets.ContainsKey(field.Name)) continue;
                                    var type = info.Fields[field.Name];
                                    if (field.Comment is not null)
                                    {
                                        sb.AppendLine($"    /// {field.Comment.Replace("\n", "\n    /// ")}");
                                        sb.AppendLine($"    /// ");
                                    }
                                    sb.AppendLine($"    /// `{field.Def.ToC()}`");
                                    sb.Append("    pub ");
                                    sb.Append(NameMapper.MapBuilderFieldName(field.Name, false));
                                    sb.Append(": ");
                                    type.WriteBuilderField(sb, struct_info);
                                    sb.AppendLine(",");
                                }
                                sb.AppendLine($"}}");
                            }

                            sb.AppendLine();
                            sb.AppendLine($"impl{any_lifetime} From<{name}{new_lifetime}> for vk::{name}{vk_lifetime} {{");
                            sb.AppendLine($"    fn from(value: {name}{new_lifetime}) -> Self {{");
                            sb.AppendLine($"        value.new()");
                            sb.AppendLine($"    }}");
                            sb.AppendLine($"}}");
                            sb.AppendLine($"impl{vk_lifetime} vk::{name}{vk_lifetime} {{");
                            sb.AppendLine($"    /// Create the {name} with required fields");
                            sb.AppendLine($"    pub fn new{new_only_lifetime}(new: {name}{new_lifetime}) -> Self {{");
                            sb.AppendLine($"        new.new()");
                            sb.AppendLine($"    }}");
                            sb.AppendLine($"}}");
                            sb.AppendLine($"impl{new_lifetime} {name}{new_lifetime} {{");
                            sb.AppendLine($"    /// Create the {name} with required fields");
                            sb.AppendLine($"    pub fn new{vk_only_lifetime}(self) -> vk::{name}{vk_lifetime} {{");
                            sb.AppendLine($"        let mut r = vk::{name}::default();");
                            if (Struct.IsUnion)
                            {
                                sb.AppendLine($"        match self {{");
                                foreach (var field in Struct.Fields)
                                {
                                    sb.Append("            Self::");
                                    sb.Append(NameMapper.MapBuilderFieldName(field.Name, true));
                                    sb.Append("(a) => r.");
                                    sb.Append(NameMapper.MapRawFieldName(field.Name));
                                    sb.Append(" = a.abi()");
                                    sb.AppendLine(",");
                                }
                                sb.AppendLine($"        }}");
                            }
                            else
                            {
                                foreach (var field in Struct.Fields)
                                {
                                    if (field.Name is "sType" or "pNext") continue;
                                    if (field.Optional is [true, ..]) continue;
                                    if (info.LenTargets.ContainsKey(field.Name)) continue;
                                    var is_pfn = symbols.Types.GetValueOrDefault(field.Type) switch
                                    {
                                        SymbolTypeAlias alias => symbols.Types[alias.Target] is SymbolTypeFuncPtr,
                                        SymbolTypeFuncPtr => true,
                                        _ => false,
                                    };
                                    var type = info.Fields[field.Name];
                                    if (field.Len is [{ } len_target and not ("null-terminated" or "1"), ..] && info.LenTargets.ContainsKey(len_target))
                                    {
                                        sb.Append("        r.");
                                        sb.Append(NameMapper.MapRawFieldName(len_target));
                                        sb.Append(" = self.");
                                        sb.Append(NameMapper.MapBuilderFieldName(field.Name, false));
                                        sb.AppendLine(".len() as _;");
                                    }
                                    sb.Append("        r.");
                                    sb.Append(NameMapper.MapRawFieldName(field.Name));
                                    sb.Append(" = ");
                                    if (is_pfn) sb.Append("unsafe { ::core::mem::transmute(");
                                    sb.Append("self.");
                                    sb.Append(NameMapper.MapBuilderFieldName(field.Name, false));
                                    if (is_pfn)
                                    {
                                        sb.Append(") }");
                                    }
                                    else
                                    {
                                        sb.Append(".abi()");
                                    }
                                    sb.AppendLine(";");
                                }
                            }
                            sb.AppendLine($"        r");
                            sb.AppendLine($"    }}");
                            sb.AppendLine($"}}");
                        }

                        break;
                    }
                    default:
                        continue;
                }
            }

            var path = Path.Join(mod_dir, "./new.rs");
            tasks.Add(File.WriteAllTextAsync(path, sb.ToString(), Encoding.UTF8));
        }

        #endregion

        #region Structs

        {
            var sb = new StringBuilder();
            sb.AppendLine(GeneratedFile);
            sb.AppendLine();
            sb.AppendLine($"use crate::vk::*;");
            sb.AppendLine($"use crate::generated::enums::*;");
            sb.AppendLine($"use ::core::ptr::NonNull;");
            sb.AppendLine($"use ::core::num::NonZeroU64;");
            sb.AppendLine($"use crate::sys::ffi::*;");
            sb.AppendLine($"use crate::sys;");

            foreach (var symbol in structs)
            {
                switch (symbol)
                {
                    case SymbolTypeAlias alias:
                    {
                        if (!symbols.Types.TryGetValue(alias.Target, out var sym)) continue;
                        var name = name_map.Map(alias.Name);
                        var target = name_map.Map(alias.Target);
                        var info = struct_info.GetValueOrDefault(alias.Target);
                        var lifetime = info?.NeedLifeTime ?? false ? "<'a>" : "";
                        sb.AppendLine();
                        sb.AppendLine($"/// `{alias.Name}` = `{alias.Target}`");
                        sb.AppendLine($"pub type {name}{lifetime} = {target}{lifetime};");
                        continue;
                    }
                    case SymbolTypeStruct Struct:
                    {
                        var name = name_map.Map(Struct.Name);

                        var info = struct_info.GetValueOrDefault(Struct.Name);
                        var lifetime = info?.NeedLifeTime ?? false ? "<'a>" : "";

                        sb.AppendLine();
                        sb.AppendLine($"/// `{Struct.Name}`");
                        if (Struct.Comment is not null)
                        {
                            sb.AppendLine($"/// ");
                            sb.AppendLine($"/// {Struct.Comment.Replace("\n", "\n    /// ")}");
                        }
                        if (!Struct.Extends.IsEmpty)
                        {
                            sb.AppendLine($"/// ");
                            sb.Append($"/// Extends: ");
                            var first = true;
                            foreach (var extend in Struct.Extends)
                            {
                                if (first) first = false;
                                else sb.Append(", ");
                                sb.Append($"[{name_map.Map(extend)}]");
                            }
                            sb.AppendLine();
                        }
                        sb.AppendLine($"#[repr(C)]");
                        var can_eq = name is "Extent2D" or "Extent3D" or "Offset2D" or "Offset3D" or "Rect2D"
                            or "ImageSubresource" or "ImageSubresourceRange" or "ImageSubresourceLayers";
                        var eq = can_eq ? ", PartialEq, Eq, PartialOrd, Hash" : "";
                        sb.AppendLine($"#[derive({(Struct.IsUnion ? "" : "Debug, ")}Clone, Copy{eq})]");
                        sb.Append($"pub {(Struct.IsUnion ? "union" : "struct")} {name}{lifetime}");
                        sb.AppendLine($" {{");
                        var need_phantom = info?.NeedLifeTime ?? false;
                        foreach (var field in Struct.Fields)
                        {
                            if (field.Comment is not null)
                            {
                                sb.AppendLine($"    /// {field.Comment.Replace("\n", "\n    /// ")}");
                                sb.AppendLine($"    /// ");
                            }
                            sb.AppendLine($"    /// ```c");
                            sb.AppendLine($"    /// {field.Def.ToC()}");
                            sb.AppendLine($"    /// ```");
                            if (field.Optional is [true, ..]) sb.AppendLine($"    /// *Optional*  \n    /// ");
                            if (field.Len is [{ } len and not ("null-terminated" or "1"), ..])
                                sb.AppendLine($"    /// Length by: [{name}::{NameMapper.MapRawFieldName(len)}] (`{len}`)  \n    /// ");
                            if (field.Name is "sType" && Struct.SType is not null)
                                sb.AppendLine($"    /// Value: [StructureType::{name_map.GetEnumItem(Struct.SType)}]\n    /// ");
                            sb.Append("    pub ");
                            name_map.MapRawField(symbols, sb, field, struct_info, ref need_phantom);
                            sb.AppendLine(",");
                        }
                        if (need_phantom || Struct.Name is "VkBaseInStructure" or "VkBaseOutStructure")
                        {
                            sb.AppendLine("    pub _p: ::core::marker::PhantomData<&'a ()>,");
                        }
                        sb.AppendLine($"}}");
                        if (Struct.IsUnion) sb.AppendLine($"impl_union_debug!({name}{lifetime});");
                        sb.AppendLine($"impl_raw_struct!({name}{lifetime} => sys::{Struct.Name});");
                        if (Struct.SType is not null)
                        {
                            sb.AppendLine($"impl_raw_struct_s_type!({name}{lifetime} : {name_map.GetEnumItem(Struct.SType)});");
                        }
                        if (!Struct.Extends.IsEmpty)
                        {
                            sb.Append($"impl_extends!({lifetime} {name}{lifetime} : ");
                            var first = true;
                            foreach (var extend in Struct.Extends)
                            {
                                if (first) first = false;
                                else sb.Append(", ");
                                sb.Append(name_map.Map(extend));
                                sb.Append("<'a> ");
                            }
                            sb.AppendLine(" );");
                        }

                        break;
                    }
                    default:
                        continue;
                }
            }

            var path = Path.Join(mod_dir, "./structs.rs");
            tasks.Add(File.WriteAllTextAsync(path, sb.ToString(), Encoding.UTF8));
        }

        #endregion

        #region Setter

        {
            var sb = new StringBuilder();
            sb.AppendLine(GeneratedFile);
            sb.AppendLine();
            sb.AppendLine($"use crate::vk::*;");
            sb.AppendLine($"use crate::generated::enums::*;");
            sb.AppendLine($"use ::core::ptr::NonNull;");
            sb.AppendLine($"use ::core::num::NonZeroU64;");
            sb.AppendLine($"use crate::sys::ffi::*;");
            sb.AppendLine($"use crate::sys;");
            sb.AppendLine($"use crate::vk;");
            sb.AppendLine($"use crate::Abi;");

            foreach (var symbol in structs)
            {
                switch (symbol)
                {
                    case SymbolTypeAlias alias: continue;
                    case SymbolTypeStruct Struct:
                    {
                        var name = name_map.Map(Struct.Name);
                        var info = struct_info[Struct.Name];
                        var vk_lifetime = info.NeedLifeTime ? "<'a>" : "";
                        var new_lifetime = info.BuilderNeedLifeTime ? "<'a>" : "";
                        var vk_only_lifetime = info.NeedLifeTime && !info.BuilderNeedLifeTime ? "<'a>" : "";

                        sb.AppendLine();
                        sb.AppendLine($"impl{vk_lifetime} vk::{name}{vk_lifetime} {{");
                        if (Struct.IsUnion)
                        {
                            foreach (var field in Struct.Fields)
                            {
                                var type = info.Fields[field.Name];
                                var arg = NameMapper.MapBuilderFieldName(field.Name, false);
                                var raw_arg = NameMapper.MapRawFieldName(field.Name);

                                if (field.Comment is not null)
                                {
                                    sb.AppendLine($"    /// {field.Comment.Replace("\n", "\n    /// ")}");
                                    sb.AppendLine($"    /// ");
                                }
                                sb.AppendLine($"    /// ```c");
                                sb.AppendLine($"    /// {field.Def.ToC()}");
                                sb.AppendLine($"    /// ```");

                                sb.Append($"    pub fn {arg}(mut self, {arg}: ");
                                type.WriteBuilderField(sb, struct_info);
                                sb.AppendLine($") -> Self {{");
                                sb.AppendLine($"        self.{raw_arg} = {arg}.abi();");
                                sb.AppendLine($"        self");
                                sb.AppendLine($"    }}");
                            }
                        }
                        else
                        {
                            foreach (var field in Struct.Fields)
                            {
                                if (field.Name is "sType" or "pNext") continue;
                                var is_pfn = symbols.Types.GetValueOrDefault(field.Type) switch
                                {
                                    SymbolTypeAlias alias => symbols.Types[alias.Target] is SymbolTypeFuncPtr,
                                    SymbolTypeFuncPtr => true,
                                    _ => false,
                                };
                                var type = info.Fields[field.Name];
                                if (type is MappedType.Option opt) type = opt.Target;
                                var arg = NameMapper.MapBuilderFieldName(field.Name, false);
                                var raw_arg = NameMapper.MapRawFieldName(field.Name);

                                if (field.Comment is not null)
                                {
                                    sb.AppendLine($"    /// {field.Comment.Replace("\n", "\n    /// ")}");
                                    sb.AppendLine($"    /// ");
                                }
                                sb.AppendLine($"    /// ```c");
                                sb.AppendLine($"    /// {field.Def.ToC()}");
                                sb.AppendLine($"    /// ```");
                                if (field.Optional is [true, ..]) sb.AppendLine($"    /// *Optional*  \n    /// ");
                                if (field.Len is [{ } len and not ("null-terminated" or "1"), ..])
                                    sb.AppendLine($"    /// Length by: [vk::{name}::{NameMapper.MapRawFieldName(len)}] (`{len}`)  \n    /// ");

                                sb.Append($"    pub fn {arg}(mut self, {arg}: ");
                                type.WriteBuilderField(sb, struct_info);
                                sb.AppendLine($") -> Self {{");
                                if (field.Len is [{ } len_target and not ("null-terminated" or "1"), ..] && info.LenTargets.ContainsKey(len_target))
                                {
                                    sb.Append("        self.");
                                    sb.Append(NameMapper.MapRawFieldName(len_target));
                                    sb.AppendLine($" = {arg}.len() as _;");
                                }
                                if (is_pfn)
                                {
                                    sb.AppendLine($"        self.{raw_arg} = unsafe {{ ::core::mem::transmute({arg}) }};");
                                }
                                else
                                {
                                    sb.AppendLine($"        self.{raw_arg} = {arg}.abi();");
                                }
                                sb.AppendLine($"        self");
                                sb.AppendLine($"    }}");
                            }
                        }
                        sb.AppendLine($"}}");

                        if (info.RequiredFieldCount != 0 && !Struct.IsUnion)
                        {
                            sb.AppendLine();
                            sb.AppendLine($"impl{new_lifetime} crate::new::{name}{new_lifetime} {{");
                            foreach (var field in Struct.Fields)
                            {
                                if (field.Name is "sType" or "pNext") continue;
                                if (info.LenTargets.ContainsKey(field.Name)) continue;
                                var type = info.Fields[field.Name];
                                if (type is MappedType.Option opt) type = opt.Target;
                                var arg = NameMapper.MapBuilderFieldName(field.Name, false);

                                if (field.Comment is not null)
                                {
                                    sb.AppendLine($"    /// {field.Comment.Replace("\n", "\n    /// ")}");
                                    sb.AppendLine($"    /// ");
                                }
                                sb.AppendLine($"    /// ```c");
                                sb.AppendLine($"    /// {field.Def.ToC()}");
                                sb.AppendLine($"    /// ```");
                                if (field.Optional is [true, ..]) sb.AppendLine($"    /// *Optional*  \n    /// ");
                                if (field.Len is [{ } len and not ("null-terminated" or "1"), ..])
                                    sb.AppendLine($"    /// Length by: [vk::{name}::{NameMapper.MapRawFieldName(len)}] (`{len}`)  \n    /// ");

                                sb.Append($"    pub fn {arg}{vk_only_lifetime}(self, {arg}: ");
                                type.WriteBuilderField(sb, struct_info);
                                sb.AppendLine($") -> vk::{name}{vk_lifetime} {{");
                                sb.AppendLine($"        self.new().{arg}({arg})");
                                sb.AppendLine($"    }}");
                            }
                            sb.AppendLine($"}}");
                        }

                        break;
                    }
                    default:
                        continue;
                }
            }

            var path = Path.Join(mod_dir, "./setter.rs");
            tasks.Add(File.WriteAllTextAsync(path, sb.ToString(), Encoding.UTF8));
        }

        #endregion

        #region Commands

        var cmd_info = new Dictionary<string, CommandInfo>();
        foreach (var (name, command) in symbols.Commands)
        {
            if (command.Alias is not null) continue;
            var sig = command.Type!;
            var arg_names = new HashSet<string>();
            var len_targets = new Dictionary<string, int>();
            var args = new List<ParamInfo>();
            var param_types = new Dictionary<string, MappedType>();
            var required_params = new HashSet<string>();
            var any_life_time = false;
            var cur_arg_is_ref = false;
            var nth_ptr = 0;
            var ctx = new NameMapper.MapCommandTypeCtx
            {
                symbols = symbols,
                arg_names = arg_names,
                nth_ptr = ref nth_ptr,
                cur_arg_is_ref = ref cur_arg_is_ref,
            };
            var IsGetProcAddr = command.Name is "vkGetInstanceProcAddr" or "vkGetDeviceProcAddr";
            var ret = IsGetProcAddr
                ? new MappedType.Option(new MappedType.SimpleType("crate::ProcAddr"))
                : name_map.MapCommandReturnType(ctx, sig);
            foreach (var arg in sig.Params)
            {
                arg_names.Add(arg.Name);
            }
            foreach (var (arg, i) in sig.Params.Select((a, b) => (a, b)))
            {
                if (arg.Type is "void") continue;
                if (arg.Len is [{ } tar and not ("null-terminated" or "1"), ..])
                {
                    if (arg_names.Contains(tar)) len_targets.TryAdd(tar, i);
                }
            }
            foreach (var arg in sig.Params)
            {
                nth_ptr = 0;
                cur_arg_is_ref = false;
                var type = name_map.MapCommandParamType(ctx, arg);
                if (type is MappedType.SimpleType("Bool")) type = new MappedType.SimpleType("bool");
                args.Add(new()
                {
                    Name = arg.Name,
                    Type = type,
                });
                param_types.Add(arg.Name, type);
                if (len_targets.ContainsKey(arg.Name))
                {
                    if (
                        type is MappedType.SimpleType
                        {
                            Name: "uint32_t" or "uint64_t" or "int32_t" or "int64_t" or "size_t"
                            or "uint8_t" or "uint16_t" or "int8_t" or "int16_t",
                        }
                    ) continue;
                    else len_targets.Remove(arg.Name);
                }
                if (arg.Optional is [true, ..]) continue;
                if (cur_arg_is_ref) any_life_time = true;
                required_params.Add(arg.Name);
            }
            var IsAllocateMulti = name is "vkAllocateCommandBuffers" or "vkAllocateDescriptorSets";
            var LastReturn = false;
            var LastReturnHandle = false;
            var IsEnumerate = false;
            if (ret is MappedType.Enum { Name: "Result" } or MappedType.SimpleType { Name: "()" })
            {
                if (sig.Params.Length > 1)
                {
                    var type = args[^1].Type;
                    if (type is MappedType.Ref { Target: MappedType.Option { Target: MappedType.Handle }, Const: false })
                    {
                        LastReturn = true;
                        LastReturnHandle = true;
                    }
                    else if (name.StartsWith("vkGet") && type is MappedType.Ref { Const: false } or MappedType.Ptr { Const: false })
                    {
                        var is_void = type is MappedType.Ptr { Target: MappedType.SimpleType("void") };
                        var target = type switch
                        {
                            MappedType.Ptr { Target: var target1 } => target1,
                            MappedType.Ref { Target: var target2 } => target2,
                            _ => throw new UnreachableException(),
                        };
                        var is_chain = target is MappedType.Struct { Symbol.SType: not null };
                        var param = sig.Params[^1];
                        if (param.Len.IsEmpty && !is_void && !is_chain)
                        {
                            LastReturn = true;
                        }
                    }
                }
                if (
                    sig.Params.Length > 2 && (
                        name.StartsWith("vkEnumerate")
                        || (name.StartsWith("vkGet"))
                    )
                )
                {
                    var data_type = args[^1].Type;
                    var count_type = args[^2].Type;
                    if (
                        command.Name is not "vkGetEncodedVideoSessionParametersKHR" &&
                        data_type is MappedType.Option { Target: MappedType.Slice { Const: false } } &&
                        count_type is MappedType.Ptr
                        {
                            Target: MappedType.SimpleType { Name: "uint32_t" or "uint64_t" or "size_t" },
                            Const: false,
                        }
                    )
                    {
                        IsEnumerate = true;
                    }
                }
            }
            cmd_info.Add(name, new()
            {
                Command = command,
                Return = ret,
                Params = args,
                ParamTypes = param_types,
                LenTargets = len_targets,
                RequiredParams = required_params,
                NeedLifeTime = any_life_time,
                ReturnResult = ret is MappedType.Enum { Name: "Result" },
                LastReturn = LastReturn,
                LastReturnHandle = LastReturnHandle,
                IsEnumerate = IsEnumerate,
                IsAllocateMulti = IsAllocateMulti,
                IsGetProcAddr = IsGetProcAddr,
            });
        }

        var destroy_hnds = hnds.Where(hnd =>
        {
            var name = hnd.Name[2..];
            return symbols.Commands.ContainsKey($"vkDestroy{name}");
        }).ToList();

        #endregion

        #region Main Commands

        var hnd_default_scopes = new Dictionary<string, string>();
        {
            var sb = new StringBuilder();
            sb.AppendLine(GeneratedFile);
            sb.AppendLine($"#![allow(unused_qualifications)]");
            sb.AppendLine($"#![allow(mismatched_lifetime_syntaxes)]");
            sb.AppendLine($"#![allow(ambiguous_glob_reexports)]");
            sb.AppendLine();
            sb.AppendLine($"use crate::sys;");
            sb.AppendLine($"use crate::sys::ffi::*;");
            sb.AppendLine($"use crate::vk::*;");
            sb.AppendLine($"use crate::vk;");
            sb.AppendLine($"use crate::{{Abi, Vk, Sys}};");
            sb.AppendLine($"pub use super::ext_commands::*;");

            ReadOnlySpan<SymbolCommandScope> groups = [SymbolCommandScope.Instance, SymbolCommandScope.Device];

            foreach (var group in groups)
            {
                var grouped_features = symbols.Features.Values.GroupBy(a => a.Number)
                    .OrderBy(a => a.Key)
                    .ToDictionary(a => a.Key, a => a.ToList());

                sb.AppendLine();
                sb.AppendLine($"/// {group} commands");
                sb.AppendLine($"#[derive(Debug, Clone, Copy)]");
                sb.AppendLine($"pub struct {group} {{");
                foreach (var (number, features) in grouped_features)
                {
                    var ver = number.Replace(".", "_");
                    sb.AppendLine($"     pub _{ver}: sys::vtbl::{group}Commands_{ver}, ");
                }
                sb.AppendLine($"}}");

                sb.AppendLine();
                sb.AppendLine($"impl {group} {{");
                sb.AppendLine($"    pub fn load(mut get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {{");
                sb.AppendLine($"        Self {{");
                foreach (var (number, features) in grouped_features)
                {
                    var ver = number.Replace(".", "_");
                    sb.AppendLine($"            _{ver}: unsafe {{ sys::vtbl::{group}Commands_{ver}::load(&mut get) }},");
                }
                sb.AppendLine($"        }}");
                sb.AppendLine($"    }}");
                sb.AppendLine($"}}");

                foreach (var (number, features) in grouped_features)
                {
                    var ver = number.Replace(".", "_");

                    var commands = features.SelectMany(a => a.Commands)
                        .OrderBy(a => a, StringComparer.Ordinal)
                        .Select(a => symbols.Commands[a])
                        .Where(a => symbols.GetCommandScope(a) == group)
                        .ToList();

                    sb.AppendLine();
                    sb.AppendLine($"/// `Vulkan {number}` {group}Commands");
                    sb.AppendLine($"impl {group} {{");
                    foreach (var command in commands)
                    {
                        GenCommand(sb, command, symbols, cmd_info, struct_info, $"_{ver}", null);
                    }
                    sb.AppendLine($"}}");
                }

                #region Object

                sb.AppendLine();
                sb.AppendLine($"/// {group} object");
                sb.AppendLine($"pub trait Core{group} {{");
                sb.AppendLine($"    fn raw(&self) -> vk::{group};");
                sb.AppendLine($"    fn commands(&self) -> &{group};");
                sb.AppendLine();
                foreach (var (number, features) in grouped_features)
                {
                    var commands = features.SelectMany(a => a.Commands)
                        .OrderBy(a => a, StringComparer.Ordinal)
                        .Select(a => symbols.Commands[a])
                        .Where(a => symbols.GetCommandScope(a) == group)
                        .ToList();
                    foreach (var command in commands)
                    {
                        GenCommandForward(sb, command, symbols, cmd_info, struct_info,
                            $"{group}", "self.raw()", "self.commands()", null);
                    }
                }
                sb.AppendLine($"}}");

                #endregion

                #region Owner

                sb.AppendLine();
                sb.AppendLine($"impl crate::Owner<vk::{group}, vk::core> for {group} {{");
                sb.AppendLine($"    fn drop(&mut self, raw: vk::{group}) {{");
                sb.AppendLine($"        unsafe {{");
                sb.AppendLine($"            self.destroy_{$"{group}".ToLower()}(Some(raw))");
                sb.AppendLine($"        }}");
                sb.AppendLine($"    }}");
                sb.AppendLine($"}}");
                sb.AppendLine();
                sb.AppendLine($"impl crate::Owner<vk::{group}, vk::core> for ::alloc::sync::Arc<{group}> {{");
                sb.AppendLine($"    fn drop(&mut self, raw: vk::{group}) {{");
                sb.AppendLine($"        unsafe {{");
                sb.AppendLine($"            self.destroy_{$"{group}".ToLower()}(Some(raw))");
                sb.AppendLine($"        }}");
                sb.AppendLine($"    }}");
                sb.AppendLine($"}}");
                sb.AppendLine();
                sb.AppendLine($"impl Core{group} for crate::Unique<vk::{group}, {group}, vk::core> {{");
                sb.AppendLine($"    fn raw(&self) -> vk::{group} {{ self.raw }}");
                sb.AppendLine($"    fn commands(&self) -> &{group} {{ &self.owner }}");
                sb.AppendLine($"}}");
                sb.AppendLine();
                sb.AppendLine($"impl Core{group} for crate::Unique<vk::{group}, ::alloc::sync::Arc<{group}>, vk::core> {{");
                sb.AppendLine($"    fn raw(&self) -> vk::{group} {{ self.raw }}");
                sb.AppendLine($"    fn commands(&self) -> &{group} {{ &self.owner }}");
                sb.AppendLine($"}}");
                sb.AppendLine();
                sb.AppendLine($"impl crate::HndCtx<vk::core, vk::{group}> for ::alloc::sync::Arc<crate::Unique<vk::{group}, ::alloc::sync::Arc<{group}>, vk::core>> {{");
                sb.AppendLine($"    type Ctx = Self;");
                sb.AppendLine($"    fn ctx(&self) -> Self::Ctx {{ self.clone() }}");
                sb.AppendLine($"    fn raw(&self) -> vk::{group} {{ self.raw }}");
                sb.AppendLine($"    fn commands(&self) -> &::alloc::sync::Arc<{group}> {{ &self.owner }}");
                sb.AppendLine($"}}");

                #endregion

                #region Hnd

                sb.AppendLine();
                sb.AppendLine($"impl crate::HndScope<vk::{group}> for vk::core {{");
                sb.AppendLine($"    type Impl = _hs_{group}::{group};");
                sb.AppendLine($"}}");
                sb.AppendLine();
                sb.AppendLine($"mod _hs_{group} {{");
                hnd_default_scopes.Add($"{group}", "vk::core");

                #region inst

                var scope = group is SymbolCommandScope.Device ? $"(::alloc::sync::Arc<super::Instance>, ::alloc::sync::Arc<super::{group}>)" : $"::alloc::sync::Arc<super::{group}>";
                sb.AppendLine($"    use super::*;");
                sb.AppendLine($"    #[repr(transparent)]");
                sb.AppendLine($"    #[derive(Debug)]");
                sb.AppendLine($"    pub struct {group}(pub(crate) crate::handle::Hnd<vk::{group}, {scope}>);");
                sb.AppendLine();
                sb.AppendLine($"    impl Clone for {group} {{");
                sb.AppendLine($"        fn clone(&self) -> Self {{ Self(self.0.clone()) }}");
                sb.AppendLine($"    }}");
                sb.AppendLine();
                sb.AppendLine($"    #[derive(Debug, Clone, Copy)]");
                sb.AppendLine($"    struct Inst;");
                sb.AppendLine($"    impl<Dep> crate::handle::HndInst<vk::{group}, {scope}, Dep> for Inst {{");
                sb.AppendLine($"        fn drop(this: &mut crate::handle::HndData<vk::{group}, {scope}, Dep>) {{");
                sb.AppendLine($"            unsafe {{");
                sb.AppendLine($"                this.scope{(group is SymbolCommandScope.Device ? ".1" : "")}.destroy_{$"{group}".ToLower()}(Some(this.raw))");
                sb.AppendLine($"            }}");
                sb.AppendLine($"        }}");
                sb.AppendLine($"    }}");

                #endregion

                #region hnd

                var ctx = group is SymbolCommandScope.Device ? $"(impl crate::HndCtx<vk::core, vk::Instance>)" : "crate::Vulkan";
                sb.AppendLine();
                sb.AppendLine($"    impl crate::hnd::{group}<vk::core> {{");
                sb.AppendLine($"        pub unsafe fn new(ctx: &{ctx}, raw: vk::{group}) -> Self {{");
                sb.AppendLine($"            unsafe {{ Self::new_with(ctx, raw, || ()) }}");
                sb.AppendLine($"        }}");
                sb.AppendLine($"        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &{ctx}, raw: vk::{group}, dep: impl FnOnce() -> Dep) -> Self {{");
                sb.AppendLine($"            Self({group}(crate::handle::Hnd::new(");
                if (group is SymbolCommandScope.Device)
                    sb.AppendLine($"                (ctx.commands().clone(), ::alloc::sync::Arc::new(super::{group}::load(|name| unsafe {{ ctx.commands().get_device_proc_addr(raw, name) }}))),");
                else
                    sb.AppendLine($"                ::alloc::sync::Arc::new(super::{group}::load(|name| unsafe {{ ctx.get_instance_proc_addr(Some(raw), name) }})),");
                sb.AppendLine($"                raw,");
                var ctx0 = group is SymbolCommandScope.Device ? "ctx.ctx()" : "ctx.clone()";
                sb.AppendLine($"                ({ctx0}, dep()),");
                sb.AppendLine($"                Inst,");
                sb.AppendLine($"            )))");
                sb.AppendLine($"        }}");
                sb.AppendLine($"    }}");

                var ctx1 = group is SymbolCommandScope.Device ? "Ctx" : "crate::Vulkan";
                sb.AppendLine();
                if (group is SymbolCommandScope.Device)
                    sb.AppendLine($"    impl<Ctx: crate::HndCtx<vk::core, vk::Instance>> crate::MakeHnd<Ctx, vk::core> for vk::{group}  {{");
                else
                    sb.AppendLine($"    impl crate::MakeHnd<crate::Vulkan, vk::core> for vk::{group} {{");
                sb.AppendLine($"        type Output = crate::hnd::{group}<vk::core>;");
                sb.AppendLine($"        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &{ctx1}, dep: impl FnOnce() -> Dep) -> Self::Output {{");
                sb.AppendLine($"            unsafe {{ crate::hnd::{group}::new_with(ctx, self, dep) }}");
                sb.AppendLine($"        }}");
                sb.AppendLine($"    }}");

                sb.AppendLine();
                sb.AppendLine($"    impl crate::hnd::{group}<vk::core> {{");
                sb.AppendLine($"        pub fn raw(&self) -> vk::{group} {{ self.0.0.raw }}");
                if (group is SymbolCommandScope.Device)
                {
                    sb.AppendLine($"        pub fn commands(&self) -> &::alloc::sync::Arc<super::{group}> {{ &self.0.0.scope.1 }}");
                    sb.AppendLine($"        fn inst_commands(&self) -> &::alloc::sync::Arc<super::Instance> {{ &self.0.0.scope.0 }}");
                    sb.AppendLine($"        pub fn get_proc_addr(&self, name: &::core::ffi::CStr) -> Option<crate::ProcAddr> {{");
                    sb.AppendLine($"            unsafe {{ self.inst_commands().get_device_proc_addr(self.raw(), name) }}");
                    sb.AppendLine($"        }}");
                }
                else
                {
                    sb.AppendLine($"        pub fn commands(&self) -> &::alloc::sync::Arc<super::{group}> {{ &self.0.0.scope }}");
                    sb.AppendLine($"        pub fn get_proc_addr(&self, name: &::core::ffi::CStr) -> Option<crate::ProcAddr> {{");
                    sb.AppendLine($"            unsafe {{ self.commands().get_instance_proc_addr(Some(self.raw()), name) }}");
                    sb.AppendLine($"        }}");
                }
                sb.AppendLine($"    }}");

                sb.AppendLine();
                sb.AppendLine($"    impl ::core::fmt::Debug for crate::hnd::{group}<vk::core> {{");
                sb.AppendLine($"        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {{");
                sb.AppendLine($"            f.write_fmt(format_args!(\"{group}({{:p}})\", self.raw()))");
                sb.AppendLine($"        }}");
                sb.AppendLine($"    }}");

                sb.AppendLine();
                sb.AppendLine($"    impl ::core::fmt::Pointer for crate::hnd::{group}<vk::core> {{");
                sb.AppendLine($"        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {{");
                sb.AppendLine($"            self.raw().fmt(f)");
                sb.AppendLine($"        }}");
                sb.AppendLine($"    }}");

                sb.AppendLine();
                sb.AppendLine($"    impl ::core::ops::Deref for crate::hnd::{group}<vk::core> {{");
                sb.AppendLine($"        type Target = super::{group};");
                sb.AppendLine($"        fn deref(&self) -> &Self::Target {{ self.commands() }}");
                sb.AppendLine($"    }}");

                sb.AppendLine();
                sb.AppendLine($"    impl super::Core{group} for crate::hnd::{group}<vk::core> {{");
                sb.AppendLine($"        fn raw(&self) -> vk::{group} {{ self.raw() }}");
                sb.AppendLine($"        fn commands(&self) -> &super::{group} {{ self.commands() }}");
                sb.AppendLine($"    }}");

                sb.AppendLine();
                sb.AppendLine($"    impl crate::HndCtx<vk::core, vk::{group}> for crate::hnd::{group}<vk::core> {{");
                sb.AppendLine($"        type Ctx = Self;");
                sb.AppendLine($"        fn ctx(&self) -> Self::Ctx {{ self.clone() }}");
                sb.AppendLine($"        fn raw(&self) -> vk::{group} {{ self.raw() }}");
                sb.AppendLine($"        fn commands(&self) -> &::alloc::sync::Arc<super::{group}> {{ self.commands() }}");
                sb.AppendLine($"    }}");

                #endregion

                sb.AppendLine($"}}");

                #endregion

                #region Other Object

                var grouped_commands = grouped_features.SelectMany(a => a.Value.SelectMany(static a => a.Commands))
                    .OrderBy(a => a, StringComparer.Ordinal)
                    .Select(a => symbols.Commands[a])
                    .Where(a => symbols.GetCommandScope(a) == group)
                    .Select(a => (cmd: a, info: cmd_info[a.Name]))
                    .Select(a => (a.cmd, a.info, first: a.info.Params[0]))
                    .Where(a => a.first.Type is MappedType.Handle)
                    .GroupBy(a => ((MappedType.Handle)a.first.Type).Name)
                    .ToDictionary(a => a.Key, a => a.ToList());
                foreach (var (hnd, commands) in grouped_commands)
                {
                    if (hnd is "Device" or "Instance") continue;

                    #region Object

                    sb.AppendLine();
                    sb.AppendLine($"/// {hnd} object");
                    sb.AppendLine($"pub trait Core{hnd} {{");
                    sb.AppendLine($"    fn raw(&self) -> vk::{hnd};");
                    sb.AppendLine($"    fn commands(&self) -> &{group};");
                    sb.AppendLine();
                    foreach (var command in commands)
                    {
                        GenCommandForward(sb, command.cmd, symbols, cmd_info, struct_info,
                            hnd, "self.raw()", "self.commands()", null);
                    }
                    sb.AppendLine($"}}");

                    #endregion
                }

                #endregion

                #region Destroy

                var destroy_commands = grouped_features.SelectMany(a => a.Value.SelectMany(static a => a.Commands))
                    .OrderBy(a => a, StringComparer.Ordinal)
                    .Select(a => symbols.Commands[a])
                    .Where(a => symbols.GetCommandScope(a) == group)
                    .Where(a => a.Name.StartsWith("vkDestroy") && a.Name is not ("vkDestroyInstance" or "vkDestroyDevice"))
                    .ToList();

                foreach (var destroy_command in destroy_commands)
                {
                    #region Ready

                    var info = cmd_info[destroy_command.Name];
                    string hnd;
                    string first_arg = "";
                    if (info.Params[0].Type is MappedType.Handle { Name: var first_hnd })
                    {
                        if (first_hnd == $"{group}")
                        {
                            first_arg = $"self.raw(), ";
                            if (info.Params[1].Type is MappedType.Option { Target: MappedType.Handle { Name: var second_hnd } })
                            {
                                hnd = second_hnd;
                            }
                            else continue;
                        }
                        else hnd = first_hnd;
                    }
                    else if (info.Params[0].Type is MappedType.Option { Target: MappedType.Handle { Name: var first_hnd1 } })
                    {
                        hnd = first_hnd1;
                    }
                    else continue;

                    #endregion

                    #region Owner

                    sb.AppendLine();
                    sb.AppendLine($"impl<O: crate::HndCtx<vk::core, vk::{group}>> crate::Owner<vk::{hnd}, vk::core> for O {{");
                    sb.AppendLine($"    fn drop(&mut self, raw: vk::{hnd}) {{");
                    sb.AppendLine($"        unsafe {{");
                    sb.AppendLine($"            self.commands().destroy_{hnd.ToSnakeCase().ToLower()}({first_arg}Some(raw))");
                    sb.AppendLine($"        }}");
                    sb.AppendLine($"    }}");
                    sb.AppendLine($"}}");

                    #endregion

                    #region Ctx

                    sb.AppendLine($"impl<O> crate::HndCtx<vk::core, vk::{group}> for ::alloc::sync::Arc<crate::Unique<vk::{hnd}, O, vk::core>>");
                    sb.AppendLine($"    where O: crate::HndCtx<vk::core, vk::{group}> + Send + Sync + 'static,");
                    sb.AppendLine($"{{");
                    sb.AppendLine($"    type Ctx = Self;");
                    sb.AppendLine($"    fn ctx(&self) -> Self::Ctx {{ self.clone() }}");
                    sb.AppendLine($"    fn raw(&self) -> vk::{group} {{ self.owner.raw() }}");
                    sb.AppendLine($"    fn commands(&self) -> &::alloc::sync::Arc<{group}> {{ self.owner.commands() }}");
                    sb.AppendLine($"}}");

                    #endregion

                    #region hnd

                    sb.AppendLine();
                    sb.AppendLine($"impl crate::HndScope<vk::{hnd}> for vk::core {{");
                    sb.AppendLine($"    type Impl = _hs_{hnd}::{hnd};");
                    sb.AppendLine($"}}");
                    sb.AppendLine();
                    sb.AppendLine($"mod _hs_{hnd} {{");
                    sb.AppendLine($"    use super::*;");
                    hnd_default_scopes.Add(hnd, "vk::core");

                    #region inst

                    sb.AppendLine();
                    sb.AppendLine($"    #[repr(transparent)]");
                    sb.AppendLine($"    #[derive(Debug)]");
                    sb.AppendLine($"    pub struct {hnd}(pub(crate) crate::handle::Hnd<vk::{hnd}, ::alloc::sync::Arc<super::{group}>>);");
                    sb.AppendLine();
                    sb.AppendLine($"    impl Clone for {hnd} {{");
                    sb.AppendLine($"        fn clone(&self) -> Self {{ Self(self.0.clone()) }}");
                    sb.AppendLine($"    }}");
                    sb.AppendLine();
                    sb.AppendLine($"    #[derive(Debug, Clone, Copy)]");
                    sb.AppendLine($"    struct Inst;");
                    sb.AppendLine($"    impl<Ctx, Dep> crate::handle::HndInst<vk::{hnd}, ::alloc::sync::Arc<super::{group}>, (Ctx, Dep)> for Inst");
                    sb.AppendLine($"        where Ctx: crate::HndCtx<vk::core, vk::{group}>,");
                    sb.AppendLine($"    {{");
                    sb.AppendLine($"        fn drop(this: &mut crate::handle::HndData<vk::{hnd}, ::alloc::sync::Arc<super::{group}>, (Ctx, Dep)>) {{");
                    sb.AppendLine($"            unsafe {{");
                    sb.AppendLine($"                this.scope.destroy_{hnd.ToSnakeCase().ToLower()}(this.dep.0.raw(), Some(this.raw))");
                    sb.AppendLine($"            }}");
                    sb.AppendLine($"        }}");
                    sb.AppendLine($"    }}");

                    #endregion

                    #region hnd

                    sb.AppendLine();
                    sb.AppendLine($"    impl crate::hnd::{hnd}<vk::core>");
                    sb.AppendLine($"    {{");
                    sb.AppendLine($"        pub unsafe fn new(ctx: &impl crate::HndCtx<vk::core, vk::{group}>, raw: vk::{hnd}) -> Self {{");
                    sb.AppendLine($"            unsafe {{ Self::new_with(ctx, raw, || ()) }}");
                    sb.AppendLine($"        }}");
                    sb.AppendLine($"        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<vk::core, vk::{group}>, raw: vk::{hnd}, dep: impl FnOnce() -> Dep) -> Self {{");
                    sb.AppendLine($"            Self({hnd}(crate::handle::Hnd::new(");
                    sb.AppendLine($"                ctx.commands().clone(),");
                    sb.AppendLine($"                raw,");
                    sb.AppendLine($"                (ctx.ctx(), dep()),");
                    sb.AppendLine($"                Inst,");
                    sb.AppendLine($"            )))");
                    sb.AppendLine($"        }}");
                    sb.AppendLine($"    }}");
                    sb.AppendLine();
                    sb.AppendLine($"    impl crate::hnd::{hnd}<vk::core> {{");
                    sb.AppendLine($"        pub fn raw(&self) -> vk::{hnd} {{ self.0.0.raw }}");
                    sb.AppendLine($"        pub fn commands(&self) -> &::alloc::sync::Arc<super::{group}> {{ &self.0.0.scope }}");
                    sb.AppendLine($"    }}");

                    sb.AppendLine();
                    sb.AppendLine($"    impl ::core::fmt::Debug for crate::hnd::{hnd}<vk::core> {{");
                    sb.AppendLine($"        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {{");
                    sb.AppendLine($"            f.write_fmt(format_args!(\"{hnd}({{:p}})\", self.raw()))");
                    sb.AppendLine($"        }}");
                    sb.AppendLine($"    }}");

                    sb.AppendLine();
                    sb.AppendLine($"    impl ::core::fmt::Pointer for crate::hnd::{hnd}<vk::core> {{");
                    sb.AppendLine($"        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {{");
                    sb.AppendLine($"            self.raw().fmt(f)");
                    sb.AppendLine($"        }}");
                    sb.AppendLine($"    }}");

                    #endregion

                    #region make

                    sb.AppendLine();
                    sb.AppendLine($"    impl<Ctx> crate::MakeHnd<Ctx, vk::core> for vk::{hnd}");
                    sb.AppendLine($"        where Ctx: crate::HndCtx<vk::core, vk::{group}>,");
                    sb.AppendLine($"    {{");
                    sb.AppendLine($"        type Output = crate::hnd::{hnd}<vk::core>;");
                    sb.AppendLine($"        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {{");
                    sb.AppendLine($"            unsafe {{ crate::hnd::{hnd}::<vk::core>::new_with(ctx, self, dep) }}");
                    sb.AppendLine($"        }}");
                    sb.AppendLine($"    }}");

                    #endregion

                    sb.AppendLine($"}}");

                    #endregion
                }

                #endregion
            }

            var path = Path.Join(mod_dir, "./main_commands.rs");
            tasks.Add(File.WriteAllTextAsync(path, sb.ToString(), Encoding.UTF8));
        }

        #endregion

        #region Extensions

        var ext_object_uses = new HashSet<string>();
        var grouped_exts = symbols.Extensions.Values.OrderBy(a => a.Number)
            .GroupBy(a => a.Prefix)
            .OrderBy(a => a.Key)
            .ToDictionary(a => a.Key, a => a.ToList());

        #region extensions

        var ext_dir = Path.Join(mod_dir, "./extensions");
        if (Directory.Exists(ext_dir)) Directory.Delete(ext_dir, true);
        Directory.CreateDirectory(ext_dir);

        {
            var ext_sb = new StringBuilder();
            ext_sb.AppendLine(GeneratedFile);
            foreach (var ext_group in grouped_exts.Keys.OrderBy(a => a))
            {
                ext_sb.AppendLine($"/// `{ext_group}` prefix extensions");
                ext_sb.AppendLine($"pub mod {ext_group.ToLower()};");
            }
            var path = Path.Join(mod_dir, "./extensions.rs");
            tasks.Add(File.WriteAllTextAsync(path, ext_sb.ToString(), Encoding.UTF8));
        }

        #endregion

        #region ext_commands

        var ext_cmd_dir = Path.Join(mod_dir, "./ext_commands");
        if (Directory.Exists(ext_cmd_dir)) Directory.Delete(ext_cmd_dir, true);
        Directory.CreateDirectory(ext_cmd_dir);

        {
            var ext_sb = new StringBuilder();
            ext_sb.AppendLine(GeneratedFile);
            foreach (var ext_group in grouped_exts.Keys.OrderBy(a => a))
            {
                ext_sb.AppendLine($"/// `{ext_group}` prefix extensions");
                ext_sb.AppendLine($"pub mod {ext_group.ToLower()};");
            }
            var path = Path.Join(mod_dir, "./ext_commands.rs");
            tasks.Add(File.WriteAllTextAsync(path, ext_sb.ToString(), Encoding.UTF8));
        }

        #endregion

        foreach (var (ext_group, exts) in grouped_exts.OrderBy(a => a.Key))
        {
            var ext_group_name = ext_group.ToLower();
            var ext_cmd_group_dir = Path.Join(ext_cmd_dir, $"./{ext_group_name}");
            Directory.CreateDirectory(ext_cmd_group_dir);
            var ext_group_sb = new StringBuilder();
            var ext_cmd_group_sb = new StringBuilder();
            ext_group_sb.AppendLine(GeneratedFile);
            ext_cmd_group_sb.AppendLine(GeneratedFile);

            foreach (var ext in exts)
            {
                var prefix = char.IsNumber(ext.TailName[0]) ? "_" : "";
                var ext_name = $"{prefix}{ext.TailName.ToLower()}";

                #region Ext

                ext_group_sb.AppendLine($"/// `{ext.Name}`");
                ext_group_sb.AppendLine($"#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]");
                ext_group_sb.AppendLine($"pub struct {ext_name};");
                ext_group_sb.AppendLine($"impl {ext_name} {{");
                ext_group_sb.AppendLine($"    /// `{ext.Name}`;");
                ext_group_sb.AppendLine($"    pub const NAME: crate::ExtName<'static> = crate::e!(\"{ext.Name}\");");
                ext_group_sb.AppendLine($"}}");
                ext_group_sb.AppendLine($"impl_ext! {{ {ext_name} }}");

                #endregion

                var cmd_first = true;

                var sb = new StringBuilder();

                ReadOnlySpan<SymbolCommandScope> groups = [SymbolCommandScope.Instance, SymbolCommandScope.Device];
                foreach (var group in groups)
                {
                    #region Cmd Base

                    var commands = ext.Commands
                        .OrderBy(a => a, StringComparer.Ordinal)
                        .Select(a => symbols.Commands[a])
                        .Where(a => symbols.GetCommandScope(a) == group)
                        .ToList();
                    if (commands.Count == 0) continue;

                    if (cmd_first)
                    {
                        cmd_first = false;

                        ext_cmd_group_sb.AppendLine($"/// `{ext.Name}`");
                        ext_cmd_group_sb.AppendLine($"pub mod {ext_name};");

                        sb.AppendLine(GeneratedFile);
                        sb.AppendLine($"#![allow(unused_qualifications)]");
                        sb.AppendLine($"#![allow(mismatched_lifetime_syntaxes)]");
                        sb.AppendLine($"use crate::sys;");
                        sb.AppendLine($"use crate::sys::ffi::*;");
                        sb.AppendLine($"use crate::vk::*;");
                        sb.AppendLine($"use crate::vk;");
                        sb.AppendLine($"use crate::{{Abi, Vk, Sys}};");
                    }

                    #endregion

                    #region Cmd

                    sb.AppendLine();
                    sb.AppendLine($"/// `{ext.Name}` {group}Commands");
                    sb.AppendLine($"#[derive(Debug, Clone, Copy)]");
                    sb.AppendLine($"pub struct {group}(pub sys::{ext_group_name}::{ext_name}::{group}Commands);");

                    sb.AppendLine();
                    sb.AppendLine($"impl {group} {{");
                    sb.AppendLine($"    pub fn load(get: impl FnMut(&::core::ffi::CStr) -> Option<crate::ProcAddr>) -> Self {{");
                    sb.AppendLine($"        Self(unsafe {{ sys::{ext_group_name}::{ext_name}::{group}Commands::load(get) }})");
                    sb.AppendLine($"    }}");
                    sb.AppendLine($"}}");

                    sb.AppendLine();
                    sb.AppendLine($"impl {group} {{");
                    foreach (var command in commands)
                    {
                        GenCommand(sb, command, symbols, cmd_info, struct_info, "0", ext_group);
                    }
                    sb.AppendLine($"}}");

                    sb.AppendLine();
                    sb.AppendLine($"impl crate::CommandScope<vk::{group}> for vk::extensions::{ext_group_name}::{ext_name} {{");
                    sb.AppendLine($"    type Commands = {group};");
                    sb.AppendLine($"}}");

                    #endregion

                    #region Object

                    var obj_name = $"{$"{ext_group}_{ext.TailName}".ToPascalCase()}";

                    sb.AppendLine();
                    sb.AppendLine($"/// {group} object");
                    sb.AppendLine($"pub trait {obj_name}{group} {{");
                    sb.AppendLine($"    fn raw(&self) -> vk::{group};");
                    sb.AppendLine($"    fn commands(&self) -> &{group};");
                    sb.AppendLine();
                    foreach (var command in commands)
                    {
                        GenCommandForward(sb, command, symbols, cmd_info, struct_info,
                            $"{group}", "self.raw()", "self.commands()", ext_group);
                    }
                    sb.AppendLine($"}}");

                    ext_object_uses.Add($"crate::generated::ext_commands::{ext_group_name}::{ext_name}::{obj_name}{group}");

                    #endregion

                    #region Hnd

                    var hnd_scope_name = $"vk::extensions::{ext_group_name}::{ext_name}";

                    sb.AppendLine();
                    sb.AppendLine($"impl crate::HndScope<vk::{group}> for {hnd_scope_name} {{");
                    sb.AppendLine($"    type Impl = _hs_{group}::{group};");
                    sb.AppendLine($"}}");
                    sb.AppendLine();
                    sb.AppendLine($"mod _hs_{group} {{");

                    #region inst

                    sb.AppendLine($"    use super::*;");
                    sb.AppendLine($"    #[derive(Debug)]");
                    sb.AppendLine($"    pub struct {group}(pub(crate) ::alloc::sync::Arc<super::{group}>, pub(crate) crate::hnd::{group}<vk::core>);");
                    sb.AppendLine();
                    sb.AppendLine($"    impl Clone for {group} {{");
                    sb.AppendLine($"        fn clone(&self) -> Self {{ Self(self.0.clone(), self.1.clone()) }}");
                    sb.AppendLine($"    }}");

                    #endregion

                    #region hnd

                    sb.AppendLine();
                    sb.AppendLine($"    impl crate::hnd::{group}<{hnd_scope_name}> {{");
                    sb.AppendLine($"        pub unsafe fn new(base: &crate::hnd::{group}<vk::core>) -> Self {{");
                    sb.AppendLine($"            unsafe {{");
                    sb.AppendLine($"                Self({group}(");
                    sb.AppendLine($"                   ::alloc::sync::Arc::new(super::{group}::load(|name| unsafe {{ base.get_proc_addr(name) }})),");
                    sb.AppendLine($"                   base.clone(),");
                    sb.AppendLine($"                ))");
                    sb.AppendLine($"            }}");
                    sb.AppendLine($"        }}");
                    sb.AppendLine($"    }}");

                    sb.AppendLine();
                    sb.AppendLine($"    impl crate::Extension<crate::hnd::{group}<vk::core>> for {hnd_scope_name} {{");
                    sb.AppendLine($"        type Output = crate::hnd::{group}<{hnd_scope_name}>;");
                    sb.AppendLine($"        unsafe fn make(target: &crate::hnd::{group}<vk::core>) -> Self::Output {{");
                    sb.AppendLine($"            unsafe {{ crate::hnd::{group}::<{hnd_scope_name}>::new(target) }}");
                    sb.AppendLine($"        }}");
                    sb.AppendLine($"    }}");

                    sb.AppendLine();
                    sb.AppendLine($"    impl crate::hnd::{group}<{hnd_scope_name}> {{");
                    sb.AppendLine($"        pub fn raw(&self) -> vk::{group} {{ self.0.1.raw() }}");
                    sb.AppendLine($"        pub fn commands(&self) -> &::alloc::sync::Arc<super::{group}> {{ &self.0.0 }}");
                    sb.AppendLine($"        pub fn core(&self) -> &crate::hnd::{group}<vk::core> {{ &self.0.1 }}");
                    sb.AppendLine($"    }}");

                    sb.AppendLine();
                    sb.AppendLine($"    impl ::core::fmt::Debug for crate::hnd::{group}<{hnd_scope_name}> {{");
                    sb.AppendLine($"        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {{");
                    sb.AppendLine($"            f.write_fmt(format_args!(\"{group}({{:p}})\", self.raw()))");
                    sb.AppendLine($"        }}");
                    sb.AppendLine($"    }}");

                    sb.AppendLine();
                    sb.AppendLine($"    impl ::core::fmt::Pointer for crate::hnd::{group}<{hnd_scope_name}> {{");
                    sb.AppendLine($"        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {{");
                    sb.AppendLine($"            self.raw().fmt(f)");
                    sb.AppendLine($"        }}");
                    sb.AppendLine($"    }}");

                    sb.AppendLine();
                    sb.AppendLine($"    impl ::core::ops::Deref for crate::hnd::{group}<{hnd_scope_name}> {{");
                    sb.AppendLine($"        type Target = super::{group};");
                    sb.AppendLine($"        fn deref(&self) -> &Self::Target {{ self.commands() }}");
                    sb.AppendLine($"    }}");

                    sb.AppendLine();
                    sb.AppendLine($"    impl super::{obj_name}{group} for crate::hnd::{group}<{hnd_scope_name}> {{");
                    sb.AppendLine($"        fn raw(&self) -> vk::{group} {{ self.raw() }}");
                    sb.AppendLine($"        fn commands(&self) -> &super::{group} {{ self.commands() }}");
                    sb.AppendLine($"    }}");

                    sb.AppendLine();
                    sb.AppendLine($"    impl crate::HndCtx<{hnd_scope_name}, vk::{group}> for crate::hnd::{group}<{hnd_scope_name}> {{");
                    sb.AppendLine($"        type Ctx = Self;");
                    sb.AppendLine($"        fn ctx(&self) -> Self::Ctx {{ self.clone() }}");
                    sb.AppendLine($"        fn raw(&self) -> vk::{group} {{ self.raw() }}");
                    sb.AppendLine($"        fn commands(&self) -> &::alloc::sync::Arc<super::{group}> {{ self.commands() }}");
                    sb.AppendLine($"    }}");

                    #endregion

                    sb.AppendLine($"}}");

                    #endregion

                    #region Other Object

                    var grouped_object_commands = commands.Select(a => (cmd: a, info: cmd_info[a.Alias ?? a.Name]))
                        .Select(a => (a.cmd, a.info, first: a.info.Params[0]))
                        .Where(a => a.first.Type is MappedType.Handle)
                        .GroupBy(a => ((MappedType.Handle)a.first.Type).Name)
                        .ToDictionary(a => a.Key, a => a.ToList());
                    foreach (var (hnd, object_commands) in grouped_object_commands)
                    {
                        if (hnd is "Device" or "Instance") continue;

                        #region Object

                        sb.AppendLine();
                        sb.AppendLine($"/// {hnd} object");
                        sb.AppendLine($"pub trait {obj_name}{hnd} {{");
                        sb.AppendLine($"    fn raw(&self) -> vk::{hnd};");
                        sb.AppendLine($"    fn commands(&self) -> &{group};");
                        sb.AppendLine();
                        foreach (var command in object_commands)
                        {
                            GenCommandForward(sb, command.cmd, symbols, cmd_info, struct_info,
                                hnd, "self.raw()", "self.commands()", ext_group);
                        }
                        sb.AppendLine($"}}");
                        ext_object_uses.Add($"crate::generated::ext_commands::{ext_group_name}::{ext_name}::{obj_name}{hnd}");

                        #endregion
                    }
                    
                    #endregion

                    #region Destroy

                    var destroy_commands = ext.Commands
                        .OrderBy(a => a, StringComparer.Ordinal)
                        .Select(a => symbols.Commands[a])
                        .Where(a => symbols.GetCommandScope(a) == group)
                        .Where(a => a.Name.StartsWith("vkDestroy"))
                        .ToList();

                    foreach (var destroy_command in destroy_commands)
                    {
                        #region Ready

                        var method_name = destroy_command.Name;
                        if (ext_group is not null && method_name.EndsWith(ext_group))
                            method_name = method_name[..^ext_group.Length];
                        if (method_name.StartsWith("vk", StringComparison.OrdinalIgnoreCase))
                            method_name = method_name[2..];
                        var info = cmd_info[destroy_command.Alias ?? destroy_command.Name];
                        string hnd;
                        string first_arg = "";
                        if (info.Params[0].Type is MappedType.Handle { Name: var first_hnd })
                        {
                            if (first_hnd == $"{group}")
                            {
                                first_arg = $"self.raw(), ";
                                if (info.Params[1].Type is MappedType.Option { Target: MappedType.Handle { Name: var second_hnd } })
                                {
                                    hnd = second_hnd;
                                }
                                else continue;
                            }
                            else hnd = first_hnd;
                        }
                        else if (info.Params[0].Type is MappedType.Option { Target: MappedType.Handle { Name: var first_hnd1 } })
                        {
                            hnd = first_hnd1;
                        }
                        else continue;

                        var method_name_call = NameMapper.MapCommandName(method_name);

                        #endregion

                        #region Owner

                        sb.AppendLine();
                        sb.AppendLine($"impl<O: crate::HndCtx<{hnd_scope_name}, vk::{group}>> crate::Owner<vk::{hnd}, {hnd_scope_name}> for O {{");
                        sb.AppendLine($"    fn drop(&mut self, raw: vk::{hnd}) {{");
                        sb.AppendLine($"        unsafe {{");
                        sb.AppendLine($"            self.commands().{method_name_call}({first_arg}Some(raw))");
                        sb.AppendLine($"        }}");
                        sb.AppendLine($"    }}");
                        sb.AppendLine($"}}");

                        #endregion

                        #region Ctx

                        sb.AppendLine($"impl<O> crate::HndCtx<{hnd_scope_name}, vk::{group}> for ::alloc::sync::Arc<crate::Unique<vk::{hnd}, O, {hnd_scope_name}>>");
                        sb.AppendLine($"    where O: crate::HndCtx<{hnd_scope_name}, vk::{group}> + Send + Sync + 'static,");
                        sb.AppendLine($"{{");
                        sb.AppendLine($"    type Ctx = Self;");
                        sb.AppendLine($"    fn ctx(&self) -> Self::Ctx {{ self.clone() }}");
                        sb.AppendLine($"    fn raw(&self) -> vk::{group} {{ self.owner.raw() }}");
                        sb.AppendLine($"    fn commands(&self) -> &::alloc::sync::Arc<{group}> {{ self.owner.commands() }}");
                        sb.AppendLine($"}}");

                        #endregion

                        #region hnd

                        sb.AppendLine();
                        sb.AppendLine($"impl crate::HndScope<vk::{hnd}> for {hnd_scope_name} {{");
                        sb.AppendLine($"    type Impl = _hs_{hnd}::{hnd};");
                        sb.AppendLine($"}}");
                        sb.AppendLine();
                        sb.AppendLine();
                        sb.AppendLine($"mod _hs_{hnd} {{");
                        sb.AppendLine($"    use super::*;");
                        hnd_default_scopes.TryAdd(hnd, hnd_scope_name);

                        #region inst

                        var scope_type = $"::alloc::sync::Arc<super::{group}>";
                        sb.AppendLine();
                        sb.AppendLine($"    #[repr(transparent)]");
                        sb.AppendLine($"    #[derive(Debug)]");
                        sb.AppendLine($"    pub struct {hnd}(pub(crate) crate::handle::Hnd<vk::{hnd}, {scope_type}>);");
                        sb.AppendLine();
                        sb.AppendLine($"    impl Clone for {hnd} {{");
                        sb.AppendLine($"        fn clone(&self) -> Self {{ Self(self.0.clone()) }}");
                        sb.AppendLine($"    }}");
                        sb.AppendLine();
                        sb.AppendLine($"    #[derive(Debug, Clone, Copy)]");
                        sb.AppendLine($"    struct Inst;");
                        sb.AppendLine($"    impl<Ctx, Dep> crate::handle::HndInst<vk::{hnd}, {scope_type}, (Ctx, Dep)> for Inst");
                        sb.AppendLine($"        where Ctx: crate::HndCtx<{hnd_scope_name}, vk::{group}>,");
                        sb.AppendLine($"    {{");
                        sb.AppendLine($"        fn drop(this: &mut crate::handle::HndData<vk::{hnd}, {scope_type}, (Ctx, Dep)>) {{");
                        sb.AppendLine($"            unsafe {{");
                        sb.AppendLine($"                this.scope.{method_name_call}(this.dep.0.raw(), Some(this.raw))");
                        sb.AppendLine($"            }}");
                        sb.AppendLine($"        }}");
                        sb.AppendLine($"    }}");

                        #endregion

                        #region hnd

                        sb.AppendLine();
                        sb.AppendLine($"    impl crate::hnd::{hnd}<{hnd_scope_name}>");
                        sb.AppendLine($"    {{");
                        sb.AppendLine($"        pub unsafe fn new(ctx: &impl crate::HndCtx<{hnd_scope_name}, vk::{group}>, raw: vk::{hnd}) -> Self {{");
                        sb.AppendLine($"            unsafe {{ Self::new_with(ctx, raw, || ()) }}");
                        sb.AppendLine($"        }}");
                        sb.AppendLine($"        pub unsafe fn new_with<Dep: Send + Sync + 'static>(ctx: &impl crate::HndCtx<{hnd_scope_name}, vk::{group}>, raw: vk::{hnd}, dep: impl FnOnce() -> Dep) -> Self {{");
                        sb.AppendLine($"            Self({hnd}(crate::handle::Hnd::new(");
                        sb.AppendLine($"                ctx.commands().clone(),");
                        sb.AppendLine($"                raw,");
                        sb.AppendLine($"                (ctx.ctx(), dep()),");
                        sb.AppendLine($"                Inst,");
                        sb.AppendLine($"            )))");
                        sb.AppendLine($"        }}");
                        sb.AppendLine($"    }}");
                        sb.AppendLine();
                        sb.AppendLine($"    impl crate::hnd::{hnd}<{hnd_scope_name}> {{");
                        sb.AppendLine($"        pub fn raw(&self) -> vk::{hnd} {{ self.0.0.raw }}");
                        sb.AppendLine($"        pub fn commands(&self) -> &::alloc::sync::Arc<super::{group}> {{ &self.0.0.scope }}");
                        sb.AppendLine($"    }}");

                        sb.AppendLine();
                        sb.AppendLine($"    impl ::core::fmt::Debug for crate::hnd::{hnd}<{hnd_scope_name}> {{");
                        sb.AppendLine($"        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {{");
                        sb.AppendLine($"            f.write_fmt(format_args!(\"{hnd}({{:p}})\", self.raw()))");
                        sb.AppendLine($"        }}");
                        sb.AppendLine($"    }}");

                        sb.AppendLine();
                        sb.AppendLine($"    impl ::core::fmt::Pointer for crate::hnd::{hnd}<{hnd_scope_name}> {{");
                        sb.AppendLine($"        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {{");
                        sb.AppendLine($"            self.raw().fmt(f)");
                        sb.AppendLine($"        }}");
                        sb.AppendLine($"    }}");

                        #endregion

                        #region make

                        sb.AppendLine();
                        sb.AppendLine($"    impl<Ctx> crate::MakeHnd<Ctx, {hnd_scope_name}> for vk::{hnd}");
                        sb.AppendLine($"        where Ctx: crate::HndCtx<{hnd_scope_name}, vk::{group}>,");
                        sb.AppendLine($"    {{");
                        sb.AppendLine($"        type Output = crate::hnd::{hnd}<{hnd_scope_name}>;");
                        sb.AppendLine($"        unsafe fn hnd_with<Dep: Send + Sync + 'static>(self, ctx: &Ctx, dep: impl FnOnce() -> Dep) -> Self::Output {{");
                        sb.AppendLine($"            unsafe {{ crate::hnd::{hnd}::<{hnd_scope_name}>::new_with(ctx, self, dep) }}");
                        sb.AppendLine($"        }}");
                        sb.AppendLine($"    }}");

                        #endregion

                        sb.AppendLine($"}}");

                        #endregion
                    }

                    #endregion
                }

                if (!cmd_first)
                {
                    var ext_path = Path.Join(ext_cmd_group_dir, $"./{ext_name}.rs");
                    tasks.Add(File.WriteAllTextAsync(ext_path, sb.ToString(), Encoding.UTF8));
                }
            }

            var ext_group_path = Path.Join(ext_dir, $"./{ext_group_name}.rs");
            tasks.Add(File.WriteAllTextAsync(ext_group_path, ext_group_sb.ToString(), Encoding.UTF8));
            var ext_cmd_group_path = Path.Join(ext_cmd_dir, $"./{ext_group_name}.rs");
            tasks.Add(File.WriteAllTextAsync(ext_cmd_group_path, ext_cmd_group_sb.ToString(), Encoding.UTF8));
        }

        #endregion

        #region Hnds

        {
            var sb = new StringBuilder();
            sb.AppendLine(GeneratedFile);
            sb.AppendLine();
            sb.AppendLine($"use crate::vk::*;");
            sb.AppendLine($"use crate::generated::enums::*;");
            sb.AppendLine($"use ::core::ptr::NonNull;");
            sb.AppendLine($"use ::core::num::NonZeroU64;");
            sb.AppendLine($"use ::alloc::sync::Arc;");
            sb.AppendLine($"use crate::sys::ffi::*;");
            sb.AppendLine($"use crate::{{sys, vk, Sys, Vk}};");

            foreach (var symbol in destroy_hnds)
            {
                switch (symbol)
                {
                    case SymbolTypeAlias alias:
                    {
                        sb.AppendLine();
                        sb.AppendLine($"pub use {name_map.Map(alias.Target)} as {name_map.Map(alias.Name)};");
                        continue;
                    }
                    case SymbolTypeHandle handle:
                    {
                        sb.AppendLine();
                        var name = name_map.Map(handle.Name);
                        var parent_name = handle.Parent is null ? null : name_map.Map(handle.Parent);
                        var scope = symbols.GetHandleScope(handle);

                        var def_scope = hnd_default_scopes.GetValueOrDefault(name);
                        var defv = def_scope is null ? "" : $" = {def_scope}";

                        sb.Append($"/// `{handle.Name}`");
                        if (handle.Parent is not null)
                            sb.Append($" : `{handle.Parent}`");
                        sb.AppendLine();
                        sb.AppendLine("#[repr(transparent)]");
                        sb.AppendLine($"pub struct {name}<S: crate::HndScope<vk::{name}>{defv}>(pub(crate) S::Impl);");
                        sb.AppendLine($"impl_object! {{ {name}(vk::{name}) }}");

                        continue;
                    }
                    default:
                        continue;
                }
            }

            var path = Path.Join(mod_dir, "./hnd.rs");
            tasks.Add(File.WriteAllTextAsync(path, sb.ToString(), Encoding.UTF8));
        }

        #endregion

        #region Prelude

        {
            var sb = new StringBuilder();
            sb.AppendLine(GeneratedFile);

            foreach (var ext_object_use in ext_object_uses)
            {
                sb.AppendLine($"pub use {ext_object_use};");
            }

            sb.AppendLine();
            var path = Path.Join(mod_dir, "./prelude.rs");
            tasks.Add(File.WriteAllTextAsync(path, sb.ToString(), Encoding.UTF8));
        }

        #endregion

        return Task.WhenAll(tasks);
    }

    public void EmitParamType(StringBuilder sb, VarDef def)
    {
        if (def.ArraySize is not null)
        {
            sb.Append("*");
            if (def.ConstArray) sb.Append("const ");
            else sb.Append("mut ");
        }
        EmitParamType(sb, def.Type);
    }
    private void EmitParamType(StringBuilder sb, CSyntax syn)
    {
        while (true)
        {
            switch (syn)
            {
                case Ident a:
                {
                    if (a.Name is "char") sb.Append("c_");
                    sb.Append(name_map.Map(a.Name));
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

    public void EmitReturnType(StringBuilder sb, CSyntax syn, CSyntax? parent = null)
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
                sb.Append(name_map.Map(a.Name));
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

    void GenCommand(
        StringBuilder sb, SymbolCommand command1,
        Symbols symbols,
        Dictionary<string, CommandInfo> cmd_info,
        Dictionary<string, StructInfo> struct_info,
        string cmd_path, string? ext_group
    )
    {
        var multi_success = command1.SuccessCodes.Length > 1;
        var alias_target = command1.Alias is not null ? symbols.Commands[command1.Alias!] : null;
        var command_name = command1.Name;
        var info = cmd_info[command1.Alias ?? command1.Name];
        var method_name = command1.Name;
        if (ext_group is not null && method_name.EndsWith(ext_group))
            method_name = method_name[..^ext_group.Length];
        if (method_name.StartsWith("vk", StringComparison.OrdinalIgnoreCase))
            method_name = method_name[2..];
        method_name = NameMapper.MapCommandName(method_name);
        var sig = alias_target?.Type ?? command1.Type!;
        var success_codes = alias_target?.SuccessCodes ?? command1.SuccessCodes;

        sb.AppendLine($"    /// ```c");
        sb.AppendLine($"    /// {sig.Def.ToC()}({string.Join(", ", sig.Params.Select(a => a.Def.ToC()))})");
        sb.AppendLine($"    /// ```");
        if (multi_success)
        {
            sb.AppendLine($"    ///");
            sb.Append($"    /// SuccessCodes: ");
            var first = true;
            foreach (var code in success_codes)
            {
                if (first) first = false;
                else sb.Append(", ");
                sb.Append($"[Result::{name_map.GetEnumItem(code)}]");
            }
            sb.AppendLine();
        }
        sb.AppendLine($"    pub unsafe fn {method_name}(");

        #region Params

        sb.AppendLine($"        &self,");
        var enum_count_type = info.IsEnumerate ? ((MappedType.Ptr)info.Params[^2].Type).Target : null;
        for (int i = 0; i < sig.Params.Length; i++)
        {
            var param = sig.Params[i];
            if (param.Name is "pAllocator" && param.Type is "VkAllocationCallbacks") continue;
            if (info.LastReturn && i == sig.Params.Length - 1) continue;
            if (info.IsEnumerate && i >= sig.Params.Length - 2) continue;
            if (info.LenTargets.ContainsKey(param.Name)) continue;
            var param_info = info.Params[i];
            sb.Append($"        {NameMapper.MapCommandName(param_info.Name)}: ");
            param_info.Type.WriteCommand(sb, struct_info);
            sb.AppendLine($",");
        }
        if (info.IsAllocateMulti)
        {
            var param_info = info.Params[^1];
            sb.Append($"        {NameMapper.MapCommandName(param_info.Name)}: ");
            var type = (MappedType.Handle)((MappedType.Option)(((MappedType.Ref)param_info.Type).Target)).Target;
            sb.Append($"&mut [Option<");
            type.WriteCommand(sb, struct_info);
            sb.AppendLine($">],");
        }
        if (info.IsEnumerate)
        {
            var param_info = info.Params[^1];
            sb.Append($"        {NameMapper.MapCommandName(param_info.Name)}: ");
            var type = ((MappedType.Slice)((MappedType.Option)param_info.Type).Target).Target;
            sb.Append($"Option<&mut ::alloc::vec::Vec<");
            type.WriteCommand(sb, struct_info);
            sb.AppendLine($">>,");
        }

        #endregion

        sb.Append($"    ) -> ");

        #region Return

        var ret_is_void = info.Return is MappedType.SimpleType { Name: "()" };
        var result_replaced = false;
        if (info.LastReturn && !info.IsAllocateMulti || info.IsEnumerate) result_replaced = true;

        if (info.ReturnResult)
        {
            sb.Append($"crate::Result<");
            if (multi_success && result_replaced) sb.Append("(");
        }
        if (info.LastReturn && !info.IsAllocateMulti)
        {
            var param_info = info.Params[^1];
            if (param_info.Type is MappedType.Ref { Target: MappedType.Option { Target: MappedType.Handle hnd } })
            {
                var sym = hnd.Symbol;
                hnd.WriteCommand(sb, struct_info);
            }
            else if (param_info.Type is MappedType.Ref { Target: var typ0 })
            {
                typ0.WriteCommand(sb, struct_info);
            }
            else if (param_info.Type is MappedType.Ptr { Target: var typ1 })
            {
                typ1.WriteCommand(sb, struct_info);
            }
            else throw new($"type error {param_info.Type}");
        }
        else if (info.IsEnumerate)
        {
            enum_count_type!.WriteCommand(sb, struct_info);
        }
        else if (info.ReturnResult && !multi_success) sb.Append($"()");
        else info.Return.WriteCommand(sb, struct_info);
        if (info.ReturnResult)
        {
            if (multi_success && result_replaced) sb.Append(", Result)");
            sb.Append($">");
        }

        #endregion

        sb.AppendLine($" {{");

        #region Body

        {
            sb.AppendLine("        unsafe {");

            #region Ready

            if (result_replaced)
            {
                if (info.LastReturn && !info.IsAllocateMulti)
                {
                    sb.Append($"            let mut _v: ");
                    var param_info = info.Params[^1];
                    if (param_info.Type is MappedType.Ref { Target: MappedType.Option { Target: MappedType.Handle hnd } })
                    {
                        sb.Append("Option<");
                        hnd.WriteCommand(sb, struct_info);
                        sb.Append(">");
                    }
                    else if (param_info.Type is MappedType.Ref { Target: var typ0 })
                    {
                        typ0.WriteCommand(sb, struct_info);
                    }
                    else if (param_info.Type is MappedType.Ptr { Target: var typ1 })
                    {
                        typ1.WriteCommand(sb, struct_info);
                    }
                    else throw new($"type error {param_info.Type}");
                    sb.AppendLine(" = Default::default();");
                }
                else if (info.IsEnumerate)
                {
                    sb.Append($"            let mut _c: ");
                    enum_count_type!.WriteCommand(sb, struct_info);
                    sb.AppendLine(" = Default::default();");
                }
            }

            #endregion

            #region First Call

            var call_name = command_name[2..];
            {
                sb.Append($"            ");
                if (!ret_is_void) sb.Append($"let{(info.IsEnumerate ? " mut" : "")} _r = ");
                sb.AppendLine($"self.{cmd_path}.{call_name}(");
                var first = true;
                for (int i = 0; i < sig.Params.Length; i++)
                {
                    if (first) first = false;
                    else sb.AppendLine(", ");
                    sb.Append("                ");
                    var param = sig.Params[i];
                    if (param.Name is "pAllocator" && param.Type is "VkAllocationCallbacks")
                    {
                        sb.Append("Default::default()");
                        continue;
                    }
                    if (info.LastReturn && i == sig.Params.Length - 1)
                    {
                        if (command_name is "vkAllocateCommandBuffers" or "vkAllocateDescriptorSets")
                        {
                            if (command_name is "vkAllocateCommandBuffers")
                            {
                                sb.Append($"command_buffers.as_mut_ptr().cast()");
                            }
                            else if (command_name is "vkAllocateDescriptorSets")
                            {
                                sb.Append($"descriptor_sets.as_mut_ptr().cast()");
                            }
                            continue;
                        }
                        sb.Append("(&mut _v).abi()");
                        continue;
                    }
                    if (info.IsEnumerate && i == sig.Params.Length - 2)
                    {
                        sb.Append("&mut _c");
                        continue;
                    }
                    if (info.IsEnumerate && i == sig.Params.Length - 1)
                    {
                        sb.Append($"Default::default()");
                        continue;
                    }
                    if (info.LenTargets.TryGetValue(param.Name, out var len_target))
                    {
                        var target_info = info.Params[len_target];
                        if (target_info.Type is MappedType.Option) 
                            sb.Append($"{NameMapper.MapCommandName(target_info.Name)}.map(|a| a.len()).unwrap_or_default() as _");
                        else
                            sb.Append($"{NameMapper.MapCommandName(target_info.Name)}.len() as _");
                        continue;
                    }
                    {
                        var param_info = info.Params[i];
                        if (param_info.Type is MappedType.Array)
                        {
                            sb.Append($"{NameMapper.MapCommandName(param_info.Name)}.as_ref().abi()");
                            continue;
                        }
                        sb.Append($"{NameMapper.MapCommandName(param_info.Name)}.abi()");
                    }
                }
                sb.AppendLine(", ");
                sb.Append($"            )");
                if (info.ReturnResult) sb.Append($".vk()");
                sb.AppendLine($";");
            }

            #endregion

            #region IsEnumerate

            if (info.IsEnumerate)
            {
                var last_param_info = info.Params[^1];
                sb.AppendLine($"            if let Some(_b) = {NameMapper.MapCommandName(last_param_info.Name)} {{");
                if (info.ReturnResult) sb.AppendLine($"                _r.result(|| Some(()))?;");
                sb.AppendLine($"                _b.reserve(_c as usize);");

                #region Second Call

                sb.Append($"                ");
                if (info.ReturnResult) sb.Append($"_r = ");
                sb.AppendLine($"self.{cmd_path}.{call_name}(");
                var first = true;
                for (int i = 0; i < sig.Params.Length; i++)
                {
                    if (first) first = false;
                    else sb.AppendLine(", ");
                    sb.Append("                    ");
                    var param = sig.Params[i];
                    if (param.Name is "pAllocator" && param.Type is "VkAllocationCallbacks")
                    {
                        sb.Append("Default::default()");
                        continue;
                    }
                    if (info.LastReturn && i == sig.Params.Length - 1)
                    {
                        sb.Append("(&mut _v).abi()");
                        continue;
                    }
                    if (info.IsEnumerate && i == sig.Params.Length - 2)
                    {
                        sb.Append("&mut _c");
                        continue;
                    }
                    if (info.IsEnumerate && i == sig.Params.Length - 1)
                    {
                        sb.Append($"(&mut **_b).abi()");
                        continue;
                    }
                    if (info.LenTargets.TryGetValue(param.Name, out var len_target))
                    {
                        var target_info = info.Params[len_target];
                        if (target_info.Type is MappedType.Option) 
                            sb.Append($"{NameMapper.MapCommandName(target_info.Name)}.map(|a| a.len()).unwrap_or_default() as _");
                        else
                            sb.Append($"{NameMapper.MapCommandName(target_info.Name)}.len() as _");
                        continue;
                    }
                    {
                        var param_info = info.Params[i];
                        if (param_info.Type is MappedType.Array)
                        {
                            sb.Append($"{NameMapper.MapCommandName(param_info.Name)}.as_ptr()");
                            continue;
                        }
                        sb.Append($"{NameMapper.MapCommandName(param_info.Name)}.abi()");
                    }
                }
                sb.AppendLine(", ");
                sb.Append($"                )");
                if (info.ReturnResult) sb.Append($".vk()");
                sb.AppendLine($";");

                #endregion

                if (info.ReturnResult) sb.AppendLine($"                _r.result(|| Some(()))?;");
                sb.AppendLine($"                _b.set_len(_b.len() + _c as usize);");

                sb.AppendLine($"            }}");
            }

            #endregion

            #region Return

            if (command_name is "vkAllocateCommandBuffers" or "vkAllocateDescriptorSets")
            {
                sb.AppendLine("            Ok(())");
            }
            else if (result_replaced)
            {
                if (info.LastReturn)
                {
                    var param_info = info.Params[^1];
                    if (param_info.Type is MappedType.Ref { Target: MappedType.Option { Target: MappedType.Handle hnd } })
                    {
                        var sym = hnd.Symbol;
                        if (info.ReturnResult)
                        {
                            if (multi_success) sb.AppendLine($"            _r.result_multi(|| _v)");
                            else sb.AppendLine($"            _r.result(|| _v)");
                        }
                        else sb.AppendLine($"            unsafe {{ _v.unwrap_unchecked() }}");
                    }
                    else if (param_info.Type is MappedType.Ref { Target: var typ0 })
                    {
                        if (info.ReturnResult)
                        {
                            if (multi_success) sb.AppendLine($"            _r.result_multi(|| Some(_v))");
                            else sb.AppendLine($"            _r.result(|| Some(_v))");
                        }
                        else sb.AppendLine($"            _v");
                    }
                    else if (param_info.Type is MappedType.Ptr { Target: var typ1 })
                    {
                        if (info.ReturnResult)
                        {
                            if (multi_success) sb.AppendLine($"            _r.result_multi(|| Some(_v))");
                            else sb.AppendLine($"            _r.result(|| Some(_v))");
                        }
                        else sb.AppendLine($"            _v");
                    }
                    else throw new($"type error {param_info.Type}");
                }
                else if (info.IsEnumerate)
                {
                    if (info.ReturnResult)
                    {
                        if (multi_success) sb.AppendLine($"            _r.result_multi(|| Some(_c))");
                        else sb.AppendLine($"            _r.result(|| Some(_c))");
                    }
                    else sb.AppendLine($"            _c");
                }
                else throw new UnreachableException("never");
            }
            else if (info.ReturnResult)
            {
                if (multi_success) sb.AppendLine($"            _r.result(|| Some(_r))");
                else sb.AppendLine($"            _r.result(|| Some(()))");
            }
            else if (!ret_is_void)
            {
                if (
                    command_name is "vkGetInstanceProcAddr" or "vkGetDeviceProcAddr"
                    || info.Return is MappedType.SimpleType
                    {
                        Name: "DeviceAddress" or "DeviceSize" or "size_t"
                        or "uint8_t" or "uint16_t" or "uint32_t" or "uint64_t"
                        or "int8_t" or "int16_t" or "int32_t" or "int64_t",
                    }
                ) sb.AppendLine($"            _r");
                else sb.AppendLine($"            _r.vk()");
            }

            #endregion

            sb.AppendLine("        }");
        }

        #endregion

        sb.AppendLine($"    }}");
    }

    void GenCommandForward(
        StringBuilder sb, SymbolCommand command1,
        Symbols symbols,
        Dictionary<string, CommandInfo> cmd_info,
        Dictionary<string, StructInfo> struct_info,
        string Self, string self_path, string cmd_path, string? ext_group
    )
    {
        var multi_success = command1.SuccessCodes.Length > 1;
        var alias_target = command1.Alias is not null ? symbols.Commands[command1.Alias!] : null;
        var command_name = command1.Name;
        var info = cmd_info[command1.Alias ?? command1.Name];
        if (!(info.Params[0].Type is MappedType.Handle { Name: var hnd_name1 } && hnd_name1 == Self)) return;
        var method_name = command1.Name;
        if (ext_group is not null && method_name.EndsWith(ext_group))
            method_name = method_name[..^ext_group.Length];
        if (method_name.StartsWith("vk", StringComparison.OrdinalIgnoreCase))
            method_name = method_name[2..];
        method_name = NameMapper.MapCommandName(method_name);
        var target_method_name = method_name;
        if (Self is "CommandBuffer")
        {
            if (method_name.StartsWith("cmd_")) method_name = method_name[4..];
            if (method_name.EndsWith("_command_buffer")) method_name = method_name[..^"_command_buffer".Length];
        }
        else if (Self is "PhysicalDevice")
        {
            if (method_name.StartsWith("get_physical_device_")) method_name = $"get_{method_name["get_physical_device_".Length..]}";
        }
        else if (Self is "Device")
        {
            if (method_name.StartsWith("get_device_queue")) method_name = $"get_{method_name["get_device_".Length..]}";
        }
        else
        {
            var low_self = Self.ToSnakeCase().ToLower();
            if (method_name.StartsWith($"{low_self}_")) method_name = method_name[(low_self.Length + 1)..];
        }
        var sig = alias_target?.Type ?? command1.Type!;
        var success_codes = alias_target?.SuccessCodes ?? command1.SuccessCodes;

        sb.AppendLine($"    /// ```c");
        sb.AppendLine($"    /// {sig.Def.ToC()}({string.Join(", ", sig.Params.Select(a => a.Def.ToC()))})");
        sb.AppendLine($"    /// ```");
        if (multi_success)
        {
            sb.AppendLine($"    ///");
            sb.Append($"    /// SuccessCodes: ");
            var first = true;
            foreach (var code in success_codes)
            {
                if (first) first = false;
                else sb.Append(", ");
                sb.Append($"[Result::{name_map.GetEnumItem(code)}]");
            }
            sb.AppendLine();
        }
        sb.AppendLine($"    unsafe fn {method_name}(");

        #region Params

        sb.AppendLine($"        &self,");
        var enum_count_type = info.IsEnumerate ? ((MappedType.Ptr)info.Params[^2].Type).Target : null;
        for (int i = 0; i < sig.Params.Length; i++)
        {
            var param = sig.Params[i];
            if (param.Name is "pAllocator" && param.Type is "VkAllocationCallbacks") continue;
            if (info.LastReturn && i == sig.Params.Length - 1) continue;
            if (info.IsEnumerate && i >= sig.Params.Length - 2) continue;
            if (info.LenTargets.ContainsKey(param.Name)) continue;
            var param_info = info.Params[i];
            if (i == 0 && param_info.Type is MappedType.Handle { Name: var hnd_name } && hnd_name == Self)
                continue;
            sb.Append($"        {NameMapper.MapCommandName(param_info.Name)}: ");
            param_info.Type.WriteCommand(sb, struct_info);
            sb.AppendLine($",");
        }
        if (info.IsAllocateMulti)
        {
            var param_info = info.Params[^1];
            sb.Append($"        {NameMapper.MapCommandName(param_info.Name)}: ");
            var type = (MappedType.Handle)((MappedType.Option)(((MappedType.Ref)param_info.Type).Target)).Target;
            sb.Append($"&mut [Option<");
            type.WriteCommand(sb, struct_info);
            sb.AppendLine($">],");
        }
        if (info.IsEnumerate)
        {
            var param_info = info.Params[^1];
            sb.Append($"        {NameMapper.MapCommandName(param_info.Name)}: ");
            var type = ((MappedType.Slice)((MappedType.Option)param_info.Type).Target).Target;
            sb.Append($"Option<&mut ::alloc::vec::Vec<");
            type.WriteCommand(sb, struct_info);
            sb.AppendLine($">>,");
        }

        #endregion

        sb.Append($"    ) -> ");

        #region Return

        var ret_is_void = info.Return is MappedType.SimpleType { Name: "()" };
        var result_replaced = false;
        if (info.LastReturn && !info.IsAllocateMulti || info.IsEnumerate) result_replaced = true;

        if (info.ReturnResult)
        {
            sb.Append($"crate::Result<");
            if (multi_success && result_replaced) sb.Append("(");
        }
        if (info.LastReturn && !info.IsAllocateMulti)
        {
            var param_info = info.Params[^1];
            if (param_info.Type is MappedType.Ref { Target: MappedType.Option { Target: MappedType.Handle hnd } })
            {
                var sym = hnd.Symbol;
                hnd.WriteCommand(sb, struct_info);
            }
            else if (param_info.Type is MappedType.Ref { Target: var typ0 })
            {
                typ0.WriteCommand(sb, struct_info);
            }
            else if (param_info.Type is MappedType.Ptr { Target: var typ1 })
            {
                typ1.WriteCommand(sb, struct_info);
            }
            else throw new($"type error {param_info.Type}");
        }
        else if (info.IsEnumerate)
        {
            enum_count_type!.WriteCommand(sb, struct_info);
        }
        else if (info.ReturnResult && !multi_success) sb.Append($"()");
        else info.Return.WriteCommand(sb, struct_info);
        if (info.ReturnResult)
        {
            if (multi_success && result_replaced) sb.Append(", Result)");
            sb.Append($">");
        }

        #endregion

        sb.AppendLine($" {{");

        #region Body

        {
            sb.AppendLine("        unsafe {");

            {
                sb.Append($"            ");
                sb.AppendLine($"{cmd_path}.{target_method_name}(");
                for (int i = 0; i < sig.Params.Length; i++)
                {
                    var param = sig.Params[i];
                    if (param.Name is "pAllocator" && param.Type is "VkAllocationCallbacks")
                    {
                        continue;
                    }
                    if (info.LenTargets.ContainsKey(param.Name)) continue;
                    if (info.IsEnumerate && i == sig.Params.Length - 2)
                    {
                        continue;
                    }
                    if (info.LastReturn && i == sig.Params.Length - 1)
                    {
                        if (command_name is "vkAllocateCommandBuffers" or "vkAllocateDescriptorSets")
                        {
                            if (command_name is "vkAllocateCommandBuffers")
                            {
                                sb.AppendLine("                command_buffers,");
                            }
                            else if (command_name is "vkAllocateDescriptorSets")
                            {
                                sb.AppendLine("                descriptor_sets,");
                            }
                            continue;
                        }
                        continue;
                    }
                    {
                        var param_info = info.Params[i];
                        if (i == 0 && param_info.Type is MappedType.Handle { Name: var hnd_name } && hnd_name == Self)
                        {
                            sb.AppendLine($"                {self_path},");
                            continue;
                        }
                        sb.AppendLine($"                {NameMapper.MapCommandName(param_info.Name)},");
                    }
                }
                sb.Append($"            )");
                sb.AppendLine();
            }

            sb.AppendLine("        }");
        }

        #endregion

        sb.AppendLine($"    }}");
    }

    public partial class NameMapper
    {
        public Dictionary<string, string> type_mapper = new() { { "VkBool32", "Bool" } };
        public Dictionary<string, string> enum_item_mapper = new();
        public Dictionary<string, int> enum_prefix = new();

        public string Map(string name)
        {
            ref var slot = ref CollectionsMarshal.GetValueRefOrAddDefault(type_mapper, name, out var exists);
            if (exists) return slot!;
            if (name.StartsWith("vk", StringComparison.OrdinalIgnoreCase)) name = name[2..];
            name = name.Replace("FlagBits", "Flags");
            slot = name;
            return name;
        }
        public int GetEnumPrefix(string enum_name, Dictionary<string, SymbolEnumItem> items)
        {
            ref var slot = ref CollectionsMarshal.GetValueRefOrAddDefault(enum_prefix, enum_name, out var exists);
            if (exists) return slot!;
            enum_name = enum_name switch
            {
                _ => $"{enum_name.ToSnakeCase().ToUpper()}_",
            };
            var lcp = Utils.LongestCommonPrefix(
                items.Keys.Select(a => a.RemoveBitSuffix()).Concat(Enumerable.Repeat(enum_name, 1)).ToList()
            );
            slot = lcp.LastIndexOf('_') + 1;
            return slot;
        }
        [GeneratedRegex(".*(UNDEFINED|PACK|PLANE|SRGB|BLOCK|BOOL).*")]
        public static partial Regex ShouldPascalCase();
        public static string FormatToPascalCase(string name)
        {
            name = name.ToPascalCase();
            if (name.Length > 3 && char.IsDigit(name[0]))
            {
                name = $"{name[0]}{char.ToUpper(name[1])}{name[2..]}";
            }
            return name;
        }
        public string MapEnumItem(string enum_name, string name, Dictionary<string, SymbolEnumItem> items)
        {
            ref var slot = ref CollectionsMarshal.GetValueRefOrAddDefault(enum_item_mapper, name, out var exists);
            if (exists) return slot!;
            var lcp = GetEnumPrefix(enum_name, items);
            name = name.RemoveBitSuffix();
            name = name[lcp..];
            if (NumberNnd().IsMatch(enum_name))
            {
                var match = NumberNameStart().Match(name);
                if (match.Success) name = name[match.Groups[1].Length..];
            }
            if (enum_name is "VkFormat")
            {
                name = string.Join("_", name.Split("_").Select(a => a switch
                {
                    "UNORM" => "UNorm",
                    "UFLOAT" => "UFloat",
                    "UINT" => "UInt",
                    "USCALED" => "UScaled",
                    "SNORM" => "SNorm",
                    "SFLOAT" => "SFloat",
                    "SINT" => "SInt",
                    "SSCALED" => "SScaled",
                    "FPENCODING" => "FpEncoding",
                    "SFIXED5" => "SFixed5",
                    _ => ShouldPascalCase().IsMatch(a) ? FormatToPascalCase(a) : a,
                }));
            }
            else
            {
                name = name.ToPascalCase();
            }
            if (NumberStart().IsMatch(name)) name = $"_{name}";
            slot = name;
            return slot;
        }
        public string GetEnumItem(string name) => enum_item_mapper[name];

        [GeneratedRegex(@"(\d+)$", RegexOptions.Compiled)]
        private static partial Regex NumberNnd();
        [GeneratedRegex(@"^(\d+_)", RegexOptions.Compiled)]
        private static partial Regex NumberNameStart();
        [GeneratedRegex(@"^(\d+)", RegexOptions.Compiled)]
        private static partial Regex NumberStart();

        public void MapRawField(
            Symbols symbols, StringBuilder sb, SymbolStructField field,
            Dictionary<string, StructInfo> infos, ref bool need_phantom
        )
        {
            sb.Append(MapRawFieldName(field.Name));
            sb.Append(": ");
            if (field.Def.ArraySize is { } size)
            {
                var arr_len = size switch
                {
                    Number n => n.Value,
                    Ident id => $"sys::{id.Name} as usize",
                    _ => throw new($"Unknown array size syntax: {size}"),
                };
                sb.Append("[");
                MapRawField(symbols, sb, field.Def.Type, infos, ref need_phantom);
                sb.Append($"; {arr_len}]");
                return;
            }
            MapRawField(symbols, sb, field.Def.Type, infos, ref need_phantom);
        }

        private void MapRawField(
            Symbols symbols, StringBuilder sb, CSyntax type,
            Dictionary<string, StructInfo> infos, ref bool need_phantom
        )
        {
            switch (type)
            {
                case Ident id:
                {
                    if (id.Name is "char")
                    {
                        sb.Append("c_char");
                        break;
                    }
                    if (symbols.Types.TryGetValue(id.Name, out var sym))
                    {
                        if (sym is SymbolTypeHandle h)
                        {
                            sb.Append($"Option<{Map(id.Name)}>");
                            break;
                        }
                        if (sym is SymbolTypeStruct s)
                        {
                            sb.Append(Map(id.Name));
                            if (infos.TryGetValue(s.Name, out var info) && info.NeedLifeTime)
                            {
                                need_phantom = false;
                                sb.Append("<'a>");
                            }
                            break;
                        }
                        if (sym is SymbolTypeAlias alias)
                        {
                            var at = symbols.Types[alias.Name];
                            if (at is SymbolTypeHandle ath)
                            {
                                sb.Append($"Option<{Map(id.Name)}>");
                                break;
                            }
                            if (at is SymbolTypeStruct ats)
                            {
                                sb.Append(Map(id.Name));
                                if (infos.TryGetValue(ats.Name, out var info) && info.NeedLifeTime)
                                {
                                    need_phantom = false;
                                    sb.Append("<'a>");
                                }
                                break;
                            }
                        }
                    }
                    sb.Append(Map(id.Name));
                    break;
                }
                case Ptr ptr:
                {
                    sb.Append($"*");
                    sb.Append(ptr.Const ? "const " : "mut ");
                    MapRawField(symbols, sb, ptr.Target, infos, ref need_phantom);
                    break;
                }
                default: throw new($"Unknown syntax: {type}");
            }
        }

        public static string MapRawFieldName(string name)
        {
            name = name.ToSnakeCase().ToLower();
            if (name == "type") name = "typ";
            return name;
        }

        public static string MapBuilderFieldName(string name, bool Union)
        {
            name = name.ToSnakeCase().ToLower();
            if (!Union && name == "type") name = "typ";
            else if (name.StartsWith("pp_")) name = name[1..];
            else if (name.StartsWith("p_")) name = name[2..];
            if (Union) name = name.ToPascalCase();
            return name;
        }

        public static string MapCommandName(string name)
        {
            name = name.ToSnakeCase().ToLower();
            if (name == "type") name = "typ";
            else if (name.StartsWith("pp_")) name = name[1..];
            else if (name.StartsWith("p_")) name = name[2..];
            return name;
        }

        public ref struct MapBuilderTypeCtx
        {
            public required Symbols symbols;
            public required HashSet<string> field_names;
            public required ref int nth_ptr;
            public required ref bool cur_field_is_ref;
        }

        public MappedType MapBuilderFieldType(in MapBuilderTypeCtx ctx, SymbolStructField field) =>
            MapBuilderFieldType(ctx, field.Def, field.Optional.AsSpan(), field.Len.AsSpan());

        private MappedType MapBuilderFieldType(
            in MapBuilderTypeCtx ctx, VarDef def,
            ReadOnlySpan<bool> option, ReadOnlySpan<string> len
        )
        {
            if (def.ArraySize is { } size)
            {
                ctx.nth_ptr++;
                var next_option = option.IsEmpty ? [] : option[1..];
                var next_len = len.IsEmpty ? [] : len[1..];
                var cur_len = len.IsEmpty ? null : len[0];
                var arr_len = size switch
                {
                    Number n => n.Value,
                    Ident id => $"sys::{id.Name} as usize",
                    _ => throw new($"Unknown array size syntax: {size}"),
                };
                if (arr_len is "sys::VK_UUID_SIZE as usize")
                {
                    return new MappedType.Uuid();
                }
                if (cur_len is "null-terminated")
                {
                    var target = MapBuilderFieldType(ctx, def.Type, next_option, next_len, false);
                    if (target is MappedType.SimpleType { Name: "c_char" })
                    {
                        return new MappedType.ArrayStr(arr_len);
                    }
                }
                if (cur_len is { } cl and not ("1" or "null-terminated") && ctx.field_names.Contains(cl))
                {
                    var target = MapBuilderFieldType(ctx, def.Type, next_option, next_len, false);
                    return new MappedType.ArraySlice(target, arr_len);
                }
                else
                {
                    var target = MapBuilderFieldType(ctx, def.Type, next_option, next_len, false);
                    var r = new MappedType.Array(target, arr_len);
                    return r;
                }
            }
            else return MapBuilderFieldType(ctx, def.Type, option, len, false);
        }

        private MappedType MapBuilderFieldType(
            in MapBuilderTypeCtx ctx, CSyntax type,
            ReadOnlySpan<bool> option, ReadOnlySpan<string> len,
            bool parent_is_mut_ptr
        )
        {
            var next_option = option.IsEmpty ? [] : option[1..];
            var next_len = len.IsEmpty ? [] : len[1..];
            var cur_option = !option.IsEmpty && option[0];
            var cur_len = len.IsEmpty ? null : len[0];
            MappedType r;
            switch (type)
            {
                case Ident id:
                {
                    if (id.Name is "char")
                    {
                        r = new MappedType.SimpleType("c_char");
                        break;
                    }
                    if (ctx.symbols.Types.TryGetValue(id.Name, out var sym))
                    {
                        if (sym is SymbolTypeStruct s)
                        {
                            r = new MappedType.Struct(Map(id.Name), s);
                            break;
                        }
                        if (sym is SymbolTypeEnum e)
                        {
                            r = new MappedType.Enum(Map(id.Name), e);
                            break;
                        }
                        if (sym is SymbolTypeHandle h)
                        {
                            r = new MappedType.Handle(Map(id.Name), h);
                            if (parent_is_mut_ptr) r = new MappedType.Option(r);
                            break;
                        }
                        if (sym is SymbolTypeFuncPtr f)
                        {
                            r = new MappedType.FuncPtr(Map(id.Name), f);
                            break;
                        }
                        if (sym is SymbolTypeAlias alias)
                        {
                            var at = ctx.symbols.Types[alias.Name];
                            if (at is SymbolTypeStruct ats)
                            {
                                r = new MappedType.Struct(Map(at.Name), ats);
                                break;
                            }
                            if (at is SymbolTypeHandle ath)
                            {
                                r = new MappedType.Handle(Map(at.Name), ath);
                                if (parent_is_mut_ptr) r = new MappedType.Option(r);
                                break;
                            }
                            if (at is SymbolTypeEnum ate)
                            {
                                r = new MappedType.Enum(Map(id.Name), ate);
                                break;
                            }
                            if (at is SymbolTypeFuncPtr atf)
                            {
                                r = new MappedType.FuncPtr(Map(id.Name), atf);
                                break;
                            }
                        }
                    }
                    r = new MappedType.SimpleType(Map(id.Name));
                    break;
                }
                case Ptr ptr:
                {
                    var allow_ref = ctx.nth_ptr++ == 0 && !ctx.cur_field_is_ref;
                    var target = MapBuilderFieldType(ctx, ptr.Target, next_option, next_len, !ptr.Const);
                    if (allow_ref)
                    {
                        if (cur_len is "null-terminated" && target is MappedType.SimpleType { Name: "c_char" })
                        {
                            ctx.cur_field_is_ref = true;
                            r = new MappedType.Ref(new MappedType.CStr(), ptr.Const);
                            break;
                        }
                        if (cur_len is not null and not "1" && ctx.field_names.Contains(cur_len) && target is not MappedType.SimpleType { Name: "void" })
                        {
                            ctx.cur_field_is_ref = true;
                            r = new MappedType.Slice(target, ptr.Const);
                            break;
                        }
                        if (target is MappedType.Struct or MappedType.Handle or MappedType.Enum)
                        {
                            ctx.cur_field_is_ref = true;
                            r = new MappedType.Ref(target, ptr.Const);
                            break;
                        }
                    }
                    r = new MappedType.Ptr(target, ptr.Const);
                    break;
                }
                default: throw new($"Unknown syntax: {type}");
            }
            if (cur_option)
            {
                if (r is MappedType.Ref or MappedType.Slice or MappedType.Handle or MappedType.Struct)
                {
                    return new MappedType.Option(r);
                }
            }
            return r;
        }


        public ref struct MapCommandTypeCtx
        {
            public required Symbols symbols;
            public required HashSet<string> arg_names;
            public required ref int nth_ptr;
            public required ref bool cur_arg_is_ref;
        }

        public MappedType MapCommandParamType(in MapCommandTypeCtx ctx, SymbolFuncParam param) =>
            MapCommandType(ctx, param.Def, false, param.Optional.AsSpan(), param.Len.AsSpan());
        public MappedType MapCommandReturnType(in MapCommandTypeCtx ctx, SymbolTypeFuncPtr sig) =>
            MapCommandType(ctx, sig.Def, true, [], []);

        private MappedType MapCommandType(
            in MapCommandTypeCtx ctx, VarDef def, bool is_return,
            ReadOnlySpan<bool> option, ReadOnlySpan<string> len
        )
        {
            if (def.ArraySize is { } size)
            {
                ctx.nth_ptr++;
                var next_option = option.IsEmpty ? [] : option[1..];
                var next_len = len.IsEmpty ? [] : len[1..];
                var arr_len = size switch
                {
                    Number n => n.Value,
                    Ident id => $"sys::{id.Name} as usize",
                    _ => throw new($"Unknown array size syntax: {size}"),
                };
                if (arr_len is "sys::VK_UUID_SIZE as usize")
                {
                    return new MappedType.Uuid();
                }
                var target = MapCommandType(ctx, def.Type, is_return, next_option, next_len, false, 1);
                var r = new MappedType.Array(target, arr_len);
                return r;
            }
            else return MapCommandType(ctx, def.Type, is_return, option, len, false, 0);
        }

        private MappedType MapCommandType(
            in MapCommandTypeCtx ctx, CSyntax type, bool is_return,
            ReadOnlySpan<bool> option, ReadOnlySpan<string> len,
            bool parent_is_mut_ptr, int level
        )
        {
            var next_option = option.IsEmpty ? [] : option[1..];
            var next_len = len.IsEmpty ? [] : len[1..];
            var cur_option = !option.IsEmpty && option[0];
            var cur_len = len.IsEmpty ? null : len[0];
            MappedType r;
            switch (type)
            {
                case Ident id:
                {
                    if (is_return && level == 0 && id.Name is "void")
                    {
                        r = new MappedType.SimpleType("()");
                        break;
                    }
                    if (id.Name is "char")
                    {
                        r = new MappedType.SimpleType("c_char");
                        break;
                    }
                    if (ctx.symbols.Types.TryGetValue(id.Name, out var sym))
                    {
                        if (sym is SymbolTypeStruct s)
                        {
                            r = new MappedType.Struct(Map(id.Name), s);
                            break;
                        }
                        if (sym is SymbolTypeEnum e)
                        {
                            r = new MappedType.Enum(Map(id.Name), e);
                            break;
                        }
                        if (sym is SymbolTypeHandle h)
                        {
                            r = new MappedType.Handle(Map(id.Name), h);
                            if (parent_is_mut_ptr) r = new MappedType.Option(r);
                            break;
                        }
                        if (sym is SymbolTypeFuncPtr f)
                        {
                            r = new MappedType.FuncPtr(Map(id.Name), f);
                            break;
                        }
                        if (sym is SymbolTypeAlias alias)
                        {
                            var at = ctx.symbols.Types[alias.Name];
                            if (at is SymbolTypeStruct ats)
                            {
                                r = new MappedType.Struct(Map(at.Name), ats);
                                break;
                            }
                            if (at is SymbolTypeHandle ath)
                            {
                                r = new MappedType.Handle(Map(at.Name), ath);
                                if (parent_is_mut_ptr) r = new MappedType.Option(r);
                                break;
                            }
                            if (at is SymbolTypeEnum ate)
                            {
                                r = new MappedType.Enum(Map(id.Name), ate);
                                break;
                            }
                            if (at is SymbolTypeFuncPtr atf)
                            {
                                r = new MappedType.FuncPtr(Map(id.Name), atf);
                                break;
                            }
                        }
                    }
                    r = new MappedType.SimpleType(Map(id.Name));
                    break;
                }
                case Ptr ptr:
                {
                    var allow_ref = ctx.nth_ptr++ == 0 && !ctx.cur_arg_is_ref;
                    var target = MapCommandType(ctx, ptr.Target, is_return, next_option, next_len, !ptr.Const, level + 1);
                    if (allow_ref)
                    {
                        if (cur_len is "null-terminated" && target is MappedType.SimpleType { Name: "c_char" })
                        {
                            ctx.cur_arg_is_ref = true;
                            r = new MappedType.Ref(new MappedType.CStr(), ptr.Const);
                            break;
                        }
                        if (cur_len is not null and not "1" && ctx.arg_names.Contains(cur_len) && target is not MappedType.SimpleType { Name: "void" })
                        {
                            ctx.cur_arg_is_ref = true;
                            r = new MappedType.Slice(target, ptr.Const);
                            break;
                        }
                        if (target is MappedType.Struct or MappedType.Handle or MappedType.Enum)
                        {
                            ctx.cur_arg_is_ref = true;
                            r = new MappedType.Ref(target, ptr.Const);
                            break;
                        }
                        if (target is MappedType.Option { Target: MappedType.Struct or MappedType.Handle or MappedType.Enum })
                        {
                            ctx.cur_arg_is_ref = true;
                            r = new MappedType.Ref(target, ptr.Const);
                            break;
                        }
                    }
                    r = new MappedType.Ptr(target, ptr.Const);
                    break;
                }
                default: throw new($"Unknown syntax: {type}");
            }
            if (cur_option)
            {
                if (r is MappedType.Ref or MappedType.Slice or MappedType.Handle or MappedType.Struct)
                {
                    return new MappedType.Option(r);
                }
            }
            return r;
        }
    }

    public abstract record MappedType
    {
        public abstract void WriteBuilderField(StringBuilder sb, Dictionary<string, StructInfo> info);
        public abstract void WriteCommand(StringBuilder sb, Dictionary<string, StructInfo> info);
        public abstract (bool All, bool Builder) NeedLifeTime(Dictionary<string, StructInfo> info);

        public record SimpleType(string Name) : MappedType()
        {
            public override void WriteBuilderField(StringBuilder sb, Dictionary<string, StructInfo> info) => sb.Append(Name);
            public override void WriteCommand(StringBuilder sb, Dictionary<string, StructInfo> info) => sb.Append(Name);
            public override (bool All, bool Builder) NeedLifeTime(Dictionary<string, StructInfo> info) => default;
        }

        public record FuncPtr(string Name, SymbolTypeFuncPtr Symbol) : MappedType()
        {
            public override void WriteBuilderField(StringBuilder sb, Dictionary<string, StructInfo> info) => sb.Append(Name[1..]);
            public override void WriteCommand(StringBuilder sb, Dictionary<string, StructInfo> info) => sb.Append(Name[1..]);
            public override (bool All, bool Builder) NeedLifeTime(Dictionary<string, StructInfo> info) => default;
        }

        public record Enum(string Name, SymbolTypeEnum Symbol) : MappedType()
        {
            public override void WriteBuilderField(StringBuilder sb, Dictionary<string, StructInfo> info) => sb.Append(Name);
            public override void WriteCommand(StringBuilder sb, Dictionary<string, StructInfo> info) => sb.Append(Name);
            public override (bool All, bool Builder) NeedLifeTime(Dictionary<string, StructInfo> info) => default;
        }

        public record Handle(string Name, SymbolTypeHandle Symbol) : MappedType()
        {
            public override void WriteBuilderField(StringBuilder sb, Dictionary<string, StructInfo> info) => sb.Append(Name);
            public override void WriteCommand(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append("vk::");
                sb.Append(Name);
            }
            public override (bool All, bool Builder) NeedLifeTime(Dictionary<string, StructInfo> info) => default;
        }

        public record Struct(string Name, SymbolTypeStruct Symbol) : MappedType()
        {
            public override void WriteBuilderField(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                var i = info.GetValueOrDefault(Symbol.Name);
                var lifetime = i?.NeedLifeTime ?? false ? "<'a>" : "";
                sb.Append($"vk::{Name}{lifetime}");
            }
            public override void WriteCommand(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                var i = info.GetValueOrDefault(Symbol.Name);
                sb.Append($"{Name}");
            }
            public override (bool All, bool Builder) NeedLifeTime(Dictionary<string, StructInfo> info)
            {
                if (!info.TryGetValue(Symbol.Name, out var i)) return default;
                i.SyncLifeTime(info);
                return (i.NeedLifeTime, i.BuilderNeedLifeTime);
            }
        }

        public record Ptr(MappedType Target, bool Const) : MappedType()
        {
            public override void WriteBuilderField(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"*{(Const ? "const" : "mut")} ");
                Target.WriteBuilderField(sb, info);
            }
            public override void WriteCommand(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"*{(Const ? "const" : "mut")} ");
                Target.WriteCommand(sb, info);
            }
            public override (bool All, bool Builder) NeedLifeTime(Dictionary<string, StructInfo> info) => Target.NeedLifeTime(info);
        }

        public record Ref(MappedType Target, bool Const) : MappedType()
        {
            public override void WriteBuilderField(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"&'a");
                if (!Const) sb.Append($" mut");
                sb.Append(" ");
                Target.WriteBuilderField(sb, info);
            }
            public override void WriteCommand(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"&");
                if (!Const) sb.Append($"mut ");
                Target.WriteCommand(sb, info);
            }
            public override (bool All, bool Builder) NeedLifeTime(Dictionary<string, StructInfo> info) => (true, true);
        }

        public record Slice(MappedType Target, bool Const) : MappedType()
        {
            public override void WriteBuilderField(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"&'a");
                if (!Const) sb.Append($" mut");
                sb.Append($" [");
                Target.WriteBuilderField(sb, info);
                sb.Append($"]");
            }
            public override void WriteCommand(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"&");
                if (!Const) sb.Append($"mut ");
                sb.Append($"[");
                Target.WriteCommand(sb, info);
                sb.Append($"]");
            }
            public override (bool All, bool Builder) NeedLifeTime(Dictionary<string, StructInfo> info) => (true, true);
        }

        public record Option(MappedType Target) : MappedType()
        {
            public override void WriteBuilderField(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"Option<");
                Target.WriteBuilderField(sb, info);
                sb.Append($">");
            }
            public override void WriteCommand(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"Option<");
                Target.WriteCommand(sb, info);
                sb.Append($">");
            }
            public override (bool All, bool Builder) NeedLifeTime(Dictionary<string, StructInfo> info) => Target.NeedLifeTime(info);
        }

        public record Array(MappedType Target, string Length) : MappedType()
        {
            public override void WriteBuilderField(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"[");
                Target.WriteBuilderField(sb, info);
                sb.Append($"; {Length}");
                sb.Append($"]");
            }
            public override void WriteCommand(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"[");
                Target.WriteCommand(sb, info);
                sb.Append($"; {Length}");
                sb.Append($"]");
            }
            public override (bool All, bool Builder) NeedLifeTime(Dictionary<string, StructInfo> info) => Target.NeedLifeTime(info);
        }

        public record ArrayStr(string Length) : MappedType()
        {
            public override void WriteBuilderField(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"crate::ArrayStr<{{ {Length}");
                sb.Append($" }}>");
            }
            public override void WriteCommand(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"crate::ArrayStr<{{ {Length}");
                sb.Append($" }}>");
            }
            public override (bool All, bool Builder) NeedLifeTime(Dictionary<string, StructInfo> info) => default;
        }

        public record ArraySlice(MappedType Target, string Length) : MappedType()
        {
            public override void WriteBuilderField(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"crate::ArraySlice<");
                Target.WriteBuilderField(sb, info);
                sb.Append($", {{ {Length}");
                sb.Append($" }}>");
            }
            public override void WriteCommand(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"crate::ArraySlice<");
                Target.WriteCommand(sb, info);
                sb.Append($", {{ {Length}");
                sb.Append($" }}>");
            }
            public override (bool All, bool Builder) NeedLifeTime(Dictionary<string, StructInfo> info) => Target.NeedLifeTime(info);
        }

        public record Uuid() : MappedType()
        {
            public override void WriteBuilderField(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"uuid::Uuid");
            }
            public override void WriteCommand(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"uuid::Uuid");
            }
            public override (bool All, bool Builder) NeedLifeTime(Dictionary<string, StructInfo> info) => default;
        }

        public record CStr() : MappedType()
        {
            public override void WriteBuilderField(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"::core::ffi::CStr");
            }
            public override void WriteCommand(StringBuilder sb, Dictionary<string, StructInfo> info)
            {
                sb.Append($"::core::ffi::CStr");
            }
            public override (bool All, bool Builder) NeedLifeTime(Dictionary<string, StructInfo> info) => default;
        }
    }

    public record StructInfo
    {
        public required SymbolTypeStruct Struct;
        public required Dictionary<string, MappedType> Fields;
        public required Dictionary<string, int> LenTargets;
        public required HashSet<string> BuilderFields;
        public required int RequiredFieldCount;
        public bool NeedLifeTime;
        public bool BuilderNeedLifeTime;
        public bool NeedLifeTime_Collected;

        public void SyncLifeTime(Dictionary<string, StructInfo> infos)
        {
            if (NeedLifeTime_Collected) return;
            foreach (var (name, type) in Fields)
            {
                var (a, _) = type.NeedLifeTime(infos);
                if (a) NeedLifeTime = true;
                if (a && BuilderFields.Contains(name)) BuilderNeedLifeTime = true;
                if (NeedLifeTime && BuilderNeedLifeTime) break;
            }
            NeedLifeTime_Collected = true;
        }
    }

    public record CommandInfo
    {
        public required SymbolCommand Command;
        public required MappedType Return;
        public required List<ParamInfo> Params;
        public required Dictionary<string, MappedType> ParamTypes;
        public required Dictionary<string, int> LenTargets;
        public required HashSet<string> RequiredParams;
        public bool NeedLifeTime;
        public bool ReturnResult;
        public bool LastReturn;
        public bool LastReturnHandle;
        public bool IsEnumerate;
        public bool IsAllocateMulti;
        public bool IsGetProcAddr;
    }

    public record struct ParamInfo
    {
        public required string Name { get; set; }
        public required MappedType Type { get; set; }
    }
}
