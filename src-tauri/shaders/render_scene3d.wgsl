// Render-3DScene: シンプルな3Dシェーダー
struct Uniforms {
    time: f32,
    rotation: f32,
    scale: f32,
    _padding: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) position: vec3f,
    @location(1) color: vec3f,
};

struct VertexOutput {
    @builtin(position) pos: vec4f,
    @location(0) color: vec3f,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let angle = uniforms.time * 0.5;
    let s = sin(angle);
    let c = cos(angle);
    let rot = mat3x3f(
        c, 0.0, s,
        0.0, 1.0, 0.0,
        -s, 0.0, c,
    );
    let pos = rot * input.position;
    var out: VertexOutput;
    out.pos = vec4f(pos * uniforms.scale, 1.0);
    out.color = input.color;
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4f {
    let time_factor = sin(uniforms.time * 3.0) * 0.2 + 0.8;
    return vec4f(input.color * time_factor, 1.0);
}
