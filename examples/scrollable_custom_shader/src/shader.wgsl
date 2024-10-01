@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var sampler1: sampler;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 4>(
        vec2<f32>(-1.0, -1.0), // bottom left
        vec2<f32>(1.0, -1.0),  // bottom right
        vec2<f32>(-1.0, 1.0),  // top left
        vec2<f32>(1.0, 1.0)    // top right
    );

    var tex_coords = array<vec2<f32>, 4>(
        vec2<f32>(0.0, 1.0), // bottom left
        vec2<f32>(1.0, 1.0), // bottom right
        vec2<f32>(0.0, 0.0), // top left
        vec2<f32>(1.0, 0.0)  // top right
    );

    let pos = positions[vertex_index];
    let tex = tex_coords[vertex_index];

    var output: VertexOutput;
    output.position = vec4<f32>(pos, 0.0, 1.0);
    output.tex_coords = tex;
    return output;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(texture, sampler1, in.tex_coords);
    return color;
}