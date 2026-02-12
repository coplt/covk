import Core;

[vk_push_constant]
uniform Entry entry;
struct Entry
{
    Scene* scene;
    Parameters* params;
}

struct Scene
{
    float4x4 view;
    float4x4 proj;
    float4 light_dir;
    float time;
}

struct Parameters
{
    float4x4 model;
    float4x4 inv_model;
    float4 color;
    Hnd<Texture2D<float4>> texture;
    Hnd<SamplerState> sampler;
};

struct Attribute
{
    [vk_location(0)]
    float3 position : POSITION;
    [vk_location(1)]
    float3 normal : NORMAL;
    [vk_location(2)]
    float2 uv : UV0;
}

[shader("vertex")]
Varying Vertex(Attribute input)
{
    let params = entry.params;
    let scene = entry.scene;

    let world_position = mul(params.model, float4(input.position, 1.0));
    let view_position = mul(scene.view, world_position);
    let world_normal = normalize(mul((float3x3)params.inv_model, input.normal));

    Varying output;
    output.position_cs = mul(scene.proj, view_position);
    output.normal = world_normal;
    output.uv = input.uv;
    return output;
}

struct Varying
{
    float4 position_cs : SV_POSITION;
    float3 normal;
    float2 uv : UV0;
}

[shader("pixel")]
float4 Pixel(Varying input) : SV_TARGET
{
    let params = entry.params;
    let scene = entry.scene;

    let normal = normalize(input.normal);
    let light = lerp(0.5, 1, clamp(dot(scene.light_dir.xyz, normal), 0, 1));

    let base_color = hsv2rgb(params.color);
    let texture_color = params.texture.get().Sample(params.sampler.get(), input.uv);
    var output_color = alpha_blend(texture_color, base_color);
    output_color.rgb *= light;

    return output_color;
}

float4 alpha_blend(float4 src, float4 dst)
{
    return float4(
        src.rgb * src.a + dst.rgb * (1.0 - src.a),
        max(src.a, dst.a)
    );
}

float4 hsv2rgb(float4 hsva)
{
    let p = abs(fms(frac(hsva.xxx + float3(1.0, 2.0 / 3.0, 1.0 / 3.0)), 6.0, 3.0));
    let x = hsva.zzz * lerp(1.0, saturate(p - 1.0), hsva.yyy);
    return float4(x.rgb, hsva.w);
}

T fms<T: IArithmetic>(T a, T b, T c) { return a * b - c;}
