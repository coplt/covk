using System.Text;
using System.Xml.Linq;
using CliWrap;
using Gen;
using Gen.Emitter;
using Serilog;

Log.Logger = new LoggerConfiguration()
    .WriteTo.Console()
    .CreateLogger();

var cur_dir = Directory.GetCurrentDirectory();
var vk_path = Path.Join(cur_dir, "./deps/Vulkan-Docs/xml/vk.xml");
var sys_path = Path.Join(cur_dir, "./covk_sys/src/generated.rs");
var rust_path = Path.Join(cur_dir, "./covk/src/generated.rs");
var rust_path_sub_dir = Path.Join(cur_dir, "./covk/src/generated/");

Log.Information("CurDir: {CurDir}", cur_dir);
Log.Information("vk.xml: {Path}", vk_path);

await using var file = File.Open(vk_path, FileMode.Open, FileAccess.Read, FileShare.ReadWrite);
var doc = await XDocument.LoadAsync(file, LoadOptions.PreserveWhitespace, CancellationToken.None);
var symbols = Symbols.Load(doc);

var misc_dir = Path.Join(cur_dir, "./.misc/");
Directory.CreateDirectory(misc_dir);

var version = "0.1.2";
var target_version_path = Path.Join(misc_dir, "./target_version.txt");
await File.WriteAllTextAsync(target_version_path, symbols.VulkanVersion, Encoding.UTF8);

var set_version = Cli.Wrap("cargo")
    .WithArguments(["set-version", $"{version}-{symbols.VulkanVersion}", "--workspace"])
    .WithStandardErrorPipe(PipeTarget.ToDelegate(msg => Log.Information("[set-version] {Msg}", msg)))
    .WithStandardOutputPipe(PipeTarget.ToDelegate(msg => Log.Information("[set-version] {Msg}", msg)))
    .ExecuteAsync();

{
    var sb = new StringBuilder();
    sb.AppendLine("// generated file, do not modify manually");
    SysEmitter.Emit(sb, symbols);
    sb.AppendLine();
    await File.WriteAllTextAsync(sys_path, sb.ToString(), Encoding.UTF8);
}

{
    Directory.CreateDirectory(rust_path_sub_dir);
    var re = new RustEmitter();
    await re.Emit(symbols, rust_path, rust_path_sub_dir);
}

await set_version;
