@group(0) @binding(0)
var display_texture: texture_2d<f32>;
@group(0) @binding(1)
var display_sampler: sampler;
@group(1) @binding(0)
var effect_texture: texture_2d<f32>;
@group(1) @binding(1)
var effect_sampler: sampler;

struct Options {
    glow_strength: f32,
    pad1: f32,
    pad2: f32,
    pad3: f32
}

@group(2) @binding(0)
var<uniform> options: Options;


struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    // List of vertices that form a full clip space quad
    var square_vertices = array<vec2<f32>, 6>(
        vec2(-1.0, -1.0),
        vec2(1.0, -1.0),
        vec2(-1.0, 1.0),
        vec2(1.0, 1.0),
        vec2(-1.0, 1.0),
        vec2(1.0, -1.0),
    );
    let vertex = square_vertices[in_vertex_index];
    var uv = (vertex + 1.0) / 2.0;
    uv.y = 1.0 - uv.y;

    var out: VertexOutput;
    out.uv = uv;
    out.pos = vec4f(vertex, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var color = textureSample(display_texture, display_sampler, in.uv);
    color += textureSample(effect_texture, effect_sampler, in.uv) * options.glow_strength;
    return color;
}
