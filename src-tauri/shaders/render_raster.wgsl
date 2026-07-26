// Render-Raster Benchmark Compute Shader
// Simulates rasterization pipeline workload:
//   1. Vertex transformation (4x4 matrix multiply)
//   2. Perspective divide + viewport transform
//   3. Triangle assembly + edge equation evaluation
//   4. Fragment color accumulation
//
// Workgroup size: 256 threads

struct VertexInput {
    pos_x: f32,
    pos_y: f32,
    pos_z: f32,
    color_r: f32,
    color_g: f32,
    color_b: f32,
}

struct Mat4 {
    cols: array<vec4<f32>, 4>,
}

@group(0) @binding(0) var<uniform> model_view_proj: Mat4;
@group(0) @binding(1) var<uniform> viewport: vec4<f32>;   // (width, height, half_w, half_h)
@group(0) @binding(2) var<uniform> params: vec4<f32>;     // (num_vertices, num_triangles, dispatch_id, 0)
@group(0) @binding(3) var<storage, read> vertices: array<VertexInput>;
@group(0) @binding(4) var<storage, read_write> framebuffer: array<f32>;

// 4x4 matrix × vec4
fn transform(v: vec4<f32>, m: Mat4) -> vec4<f32> {
    return m.cols[0] * v.x + m.cols[1] * v.y + m.cols[2] * v.z + m.cols[3] * v.w;
}

// Edge function for triangle rasterization
fn edge(a: vec2<f32>, b: vec2<f32>, c: vec2<f32>) -> f32 {
    return (c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x);
}

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let vertex_count = u32(params.x);
    let triangle_count = u32(params.y);
    let idx = id.x;

    // === Phase 1: Vertex Transformation ===
    // Each thread transforms one vertex
    if (idx < vertex_count) {
        let v = vertices[idx];
        let pos = vec4<f32>(v.pos_x, v.pos_y, v.pos_z, 1.0);
        let clip = transform(pos, model_view_proj);

        // Perspective divide
        let ndc = clip.xyz / clip.w;

        // Viewport transform
        let sx = ndc.x * viewport.z + viewport.z;
        let sy = -ndc.y * viewport.w + viewport.w;

        // Store transformed position + color back (simulating VS output)
        // We pack into framebuffer as a simple proxy
        let fb_idx = idx * 4u;
        if (fb_idx + 3u < arrayLength(&framebuffer)) {
            framebuffer[fb_idx]     = sx;
            framebuffer[fb_idx + 1u] = sy;
            framebuffer[fb_idx + 2u] = ndc.z;  // depth
            framebuffer[fb_idx + 3u] = 0.0;
        }
    }

    // === Phase 2: Triangle Processing + Fragment Simulation ===
    // Each thread "processes" one triangle's worth of work
    if (idx < triangle_count) {
        let tri_base = (idx * 3u) % vertex_count;
        let v0_idx = tri_base;
        let v1_idx = (tri_base + 1u) % vertex_count;
        let v2_idx = (tri_base + 2u) % vertex_count;

        let v0_fb = v0_idx * 4u;
        let v1_fb = v1_idx * 4u;
        let v2_fb = v2_idx * 4u;

        // Read transformed positions
        var p0 = vec2<f32>(framebuffer[v0_fb], framebuffer[v0_fb + 1u]);
        var p1 = vec2<f32>(framebuffer[v1_fb], framebuffer[v1_fb + 1u]);
        var p2 = vec2<f32>(framebuffer[v2_fb], framebuffer[v2_fb + 1u]);

        // Compute bounding box
        let min_x = max(0.0, min(p0.x, min(p1.x, p2.x)));
        let min_y = max(0.0, min(p0.y, min(p1.y, p2.y)));
        let max_x = min(viewport.x - 1.0, max(p0.x, max(p1.x, p2.x)));
        let max_y = min(viewport.y - 1.0, max(p0.y, max(p1.y, p2.y)));

        // Edge equations for rasterization
        // Simulate iterating over pixels in bounding box
        // Each thread processes a stripe of the bounding box
        // For benchmark purposes, we do a bounded number of iterations
        let step_count = min(64u, u32(max_x - min_x) * u32(max_y - min_y));
        let base_pixel = u32(viewport.x) * u32(min_y) + u32(min_x);

        var r_acc = 0.0;
        var g_acc = 0.0;
        var b_acc = 0.0;
        var pixel_count = 0u;

        for (var i = 0u; i < step_count; i = i + 1u) {
            let px = u32(min_x) + (i % 8u) * 8u;
            let py = u32(min_y) + (i / 8u) * 8u;
            if (px >= u32(viewport.x) || py >= u32(viewport.y)) {
                continue;
            }

            let p = vec2<f32>(f32(px) + 0.5, f32(py) + 0.5);

            // Edge function tests (simulating rasterizer)
            let e0 = edge(p1, p2, p);
            let e1 = edge(p2, p0, p);
            let e2 = edge(p0, p1, p);

            if (e0 >= 0.0 && e1 >= 0.0 && e2 >= 0.0) {
                // Barycentric-like weighting (simplified)
                let total = e0 + e1 + e2;
                if (total > 0.0) {
                    let w0 = e0 / total;
                    let w1 = e1 / total;
                    let w2 = e2 / total;

                    // Read vertex colors for interpolation
                    let c0_r = vertices[tri_base].color_r;
                    let c0_g = vertices[tri_base].color_g;
                    let c0_b = vertices[tri_base].color_b;
                    let c1_r = vertices[(tri_base + 1u) % vertex_count].color_r;
                    let c1_g = vertices[(tri_base + 1u) % vertex_count].color_g;
                    let c1_b = vertices[(tri_base + 1u) % vertex_count].color_b;
                    let c2_r = vertices[(tri_base + 2u) % vertex_count].color_r;
                    let c2_g = vertices[(tri_base + 2u) % vertex_count].color_g;
                    let c2_b = vertices[(tri_base + 2u) % vertex_count].color_b;

                    // Interpolate colors
                    r_acc = r_acc + w0 * c0_r + w1 * c1_r + w2 * c2_r;
                    g_acc = g_acc + w0 * c0_g + w1 * c1_g + w2 * c2_g;
                    b_acc = b_acc + w0 * c0_b + w1 * c1_b + w2 * c2_b;
                    pixel_count = pixel_count + 1u;
                }
            }
        }

        // Write accumulated fragment results to framebuffer (tile-based)
        let out_base = (vertex_count * 4u) + (idx * 4u);
        if (out_base + 3u < arrayLength(&framebuffer)) {
            framebuffer[out_base]     = r_acc / max(1.0, f32(pixel_count));
            framebuffer[out_base + 1u] = g_acc / max(1.0, f32(pixel_count));
            framebuffer[out_base + 2u] = b_acc / max(1.0, f32(pixel_count));
            framebuffer[out_base + 3u] = f32(pixel_count);
        }
    }
}
