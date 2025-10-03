struct OptionsUniform {
    palette: array<vec4<f32>, 4>,
    width: u32,
    height: u32,
    canvas_width: u32,
    canvas_height: u32
}

@group(0) @binding(0)
var<uniform> options: OptionsUniform;

// Contains the display pixel data for rendering.
// Structured as an array of 4D vectors to
// satisfy the memory alignment system
struct PixelUniform {
    pixels: array<vec4<u32>, 360>
};

@group(1) @binding(0)
var<uniform> pixels: PixelUniform;

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) pixel_scale: u32,
    @location(1) display_origin: vec2<u32>,
}

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var square_vertices = array<vec2<f32>, 6>(
        vec2(-1.0, -1.0),
        vec2(1.0, -1.0),
        vec2(-1.0, 1.0),
        vec2(1.0, 1.0),
        vec2(-1.0, 1.0),
        vec2(1.0, -1.0),
    );

    let scale = min(options.canvas_width / options.width, options.canvas_height / options.height);
    let origin = vec2<u32>(
        (options.canvas_width - (options.width * scale)) / 2u,
        (options.canvas_height - (options.height * scale)) / 2u
    );

    let vertex = square_vertices[in_vertex_index];
    let pos = vec4<f32>(
        -vertex.x * ((f32(origin.x) / f32(options.canvas_width)) * 2.0 - 1.0),
        -vertex.y * ((f32(origin.y) / f32(options.canvas_height)) * 2.0 - 1.0),
        0.0,
        1.0
    );

    var out: VertexOutput;
    out.pos = pos;
    out.pixel_scale = scale;
    out.display_origin = origin;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let x = (u32(in.pos.x) - in.display_origin.x) / in.pixel_scale;
    let y = (u32(in.pos.y) - in.display_origin.y) / in.pixel_scale;

    let pixel_i = y * options.width + x;
    let bit_i = 2u * pixel_i;
    let vec_i = bit_i / 128u;
    let vec_bit_i = bit_i % 128u;
    let int_i = vec_bit_i / 32u;

    let int = pixels.pixels[vec_i][int_i];
    let int_col_i = ((vec_bit_i % 32u) / 2u) * 2u;

    let color = (int >> int_col_i) & 3u;
    return options.palette[color];
}

