/// Options for rendering
struct OptionsUniform {
    /// The colored palette to get the final drawn pixel color from
    palette: array<vec4<f32>, 4>,
    /// The constant size of the Game Boy display (160x144)
    display_size: vec2<u32>,
    /// The varying size of the canvas to be drawn to
    canvas_size: vec2<u32>
}

@group(0) @binding(0)
var<uniform> options: OptionsUniform;

// Contains the display pixel data for rendering.
// Structured as an array of 4D vectors to
// satisfy the memory alignment system.
// 1440 integers in 4D vectors = 360 vectors
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
    // Calculate pixel scale as the possible largest integer scale
    // which still fits display in both dimensions
    let scale = min(
        options.canvas_size.x / options.display_size.x,
        options.canvas_size.y / options.display_size.y
    );
    // Calculate top-left origin in pixel space for centered canvas
    let pixel_origin = options.canvas_size - (options.display_size * scale) / 2u;
    // Convert to clip space
    let vertex_origin = (vec2f(pixel_origin) / vec2f(options.canvas_size)) * 2.0 - 1.0;

    // List of vertices that form a full clip space quad
    var square_vertices = array<vec2<f32>, 6>(
        vec2(-1.0, -1.0),
        vec2(1.0, -1.0),
        vec2(-1.0, 1.0),
        vec2(1.0, 1.0),
        vec2(-1.0, 1.0),
        vec2(1.0, -1.0),
    );
    // Mirror origin across axes based on vertex coordinates.
    // This transforms the full clip space quad into a centered quad
    // which the display will be drawn on
    let vertex = square_vertices[in_vertex_index];
    let pos = vec4<f32>(
        -vertex * vertex_origin,
        0.0,
        1.0
    );
    
    // Construct output struct
    var out: VertexOutput;
    out.pos = pos;
    out.pixel_scale = scale;
    out.display_origin = pixel_origin;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Convert pixel coordinates into coordinates on Game Boy display
    let pixel = (vec2u(in.pos.xy) - in.display_origin) / in.pixel_scale;
    // Crop out pixels outside of display boundary.
    // There's often one stray pixel column and row since the conversion
    // from integer pixel space to floating point clip space doesn't seem to be perfect
    if pixel.x >= options.display_size.x || pixel.y >= options.display_size.y {
        return vec4f(0.0, 0.0, 0.0, 1.0);
    }
    
    // Calculate index of pixel on display
    let pixel_i = pixel.y * options.display_size.x + pixel.x;
    // Calculate index of the two color bits in display buffer
    let bit_i = 2u * pixel_i;
    // Calculate index of vector containing bit
    // (vec4 contains 128 bits of data)
    let vec_i = bit_i / 128u;
    // Calculate index of vector integer containing bit
    let vec_bit_i = bit_i % 128u;
    let int_i = vec_bit_i / 32u;
    // Calculate index of color bits inside integer 
    let int_bit_i = ((vec_bit_i % 32u) / 2u) * 2u;

    // Get integer value and bit mask the color value
    let int = pixels.pixels[vec_i][int_i];
    let color = (int >> int_bit_i) & 3u;
    // Return color from current palette
    return options.palette[color];
}

