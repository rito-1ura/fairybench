// Render-PathTrace Benchmark Compute Shader
// Simulates ray tracing workload:
//   1. Ray generation from virtual camera
//   2. Ray-sphere intersection tests (50 spheres)
//   3. Multi-bounce path tracing (up to 4 bounces)
//   4. Color accumulation
//
// Workgroup size: 64 threads

struct Sphere {
    center: vec4<f32>,   // xyz = center, w = radius
    color: vec4<f32>,    // rgb = color, a = padding
}

struct Ray {
    origin: vec4<f32>,
    dir: vec4<f32>,
}

// Camera uniform
@group(0) @binding(0) var<uniform> cam: vec4<f32>;  // (origin_x, origin_y, origin_z, fov_scale)
@group(0) @binding(1) var<uniform> cam_dir: vec4<f32>; // (target_x, target_y, target_z, 0)
@group(0) @binding(2) var<uniform> params: vec4<f32>;  // (num_spheres, img_width, img_height, bounces)

// Scene data
@group(0) @binding(3) var<storage, read> spheres: array<Sphere>;
@group(0) @binding(4) var<storage, read_write> framebuffer: array<f32>;

// Random number generator (simple LCG)
fn rand(seed: ptr<function, u32>) -> f32 {
    let s = 1664525u * (*seed) + 1013904223u;
    *seed = s;
    return f32(s & 0x007fffffu) / f32(0x007fffff);
}

// Ray-sphere intersection test
fn intersect_sphere(ray_origin: vec3<f32>, ray_dir: vec3<f32>, sphere: Sphere) -> f32 {
    let oc = ray_origin - sphere.center.xyz;
    let a = dot(ray_dir, ray_dir);
    let b = 2.0 * dot(oc, ray_dir);
    let c = dot(oc, oc) - sphere.center.w * sphere.center.w;
    let disc = b * b - 4.0 * a * c;
    if (disc < 0.0) {
        return -1.0;
    }
    let t = (-b - sqrt(disc)) / (2.0 * a);
    return t;
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let num_spheres = u32(params.x);
    let img_w = u32(params.y);
    let img_h = u32(params.z);
    let bounces = u32(params.w);

    let ray_idx = id.x;
    if (ray_idx >= img_w * img_h) {
        return;
    }

    let px = ray_idx % img_w;
    let py = ray_idx / img_w;

    // Setup RNG with deterministic seed
    var rng: u32 = ray_idx * 1664525u + 1013904223u;

    // Camera basis
    let origin = cam.xyz;
    let fwd = normalize(cam_dir.xyz - origin);
    let world_up = vec3<f32>(0.0, 1.0, 0.0);
    let right = normalize(cross(fwd, world_up));
    let up = cross(right, fwd);

    let aspect = f32(img_w) / f32(img_h);
    let fov_scale = cam.w;

    // Accumulated color
    var acc_color = vec3<f32>(0.0, 0.0, 0.0);

    // Multi-bounce path tracing
    var ray_origin = origin;
    var throughput = vec3<f32>(1.0, 1.0, 1.0);

    for (var bounce = 0u; bounce < bounces; bounce = bounce + 1u) {
        // Generate ray direction with jitter (anti-aliasing)
        let jitter_x = rand(&rng) - 0.5;
        let jitter_y = rand(&rng) - 0.5;
        let ndc_x = (f32(px) + 0.5 + jitter_x) / f32(img_w) * 2.0 - 1.0;
        let ndc_y = (f32(py) + 0.5 + jitter_y) / f32(img_h) * 2.0 - 1.0;
        var ray_dir = normalize(fwd + right * ndc_x * aspect * fov_scale + up * ndc_y * fov_scale);

        // Find closest intersection
        var closest_t = 1e10;
        var hit_color = vec3<f32>(0.0, 0.0, 0.0);
        var hit_normal = vec3<f32>(0.0, 0.0, 0.0);

        for (var si = 0u; si < num_spheres; si = si + 1u) {
            let t = intersect_sphere(ray_origin, ray_dir, spheres[si]);
            if (t > 0.001 && t < closest_t) {
                closest_t = t;
                let hit_pt = ray_origin + t * ray_dir;
                let norm = normalize(hit_pt - spheres[si].center.xyz);
                hit_normal = norm;
                hit_color = spheres[si].color.rgb;
            }
        }

        if (closest_t < 1e9) {
            // Diffuse bounce
            let r1 = rand(&rng);
            let r2 = rand(&rng);
            let theta = 2.0 * 3.14159 * r1;
            let phi = acos(2.0 * r2 - 1.0);
            let bounce_dir = normalize(
                hit_normal * cos(phi) +
                vec3<f32>(cos(theta) * sin(phi), sin(theta) * sin(phi), cos(phi))
            );

            acc_color = acc_color + throughput * hit_color * 0.5;
            throughput = throughput * hit_color * 0.5;

            // Russian roulette
            let rr = max(throughput.r, max(throughput.g, throughput.b));
            if (rr < 0.01 && bounces > 1u) {
                break;
            }

            ray_origin = ray_origin + closest_t * ray_dir + hit_normal * 0.001;
            ray_dir = bounce_dir;
        } else {
            // Sky (gradient)
            let sky = vec3<f32>(0.6, 0.8, 1.0) * max(ray_dir.y, 0.0) + vec3<f32>(0.2, 0.3, 0.5) * (1.0 - max(ray_dir.y, 0.0));
            acc_color = acc_color + throughput * sky;
            break;
        }
    }

    // Write pixel color
    let fb_idx = (py * img_w + px) * 4u;
    if (fb_idx + 3u < arrayLength(&framebuffer)) {
        framebuffer[fb_idx]     = acc_color.r;
        framebuffer[fb_idx + 1u] = acc_color.g;
        framebuffer[fb_idx + 2u] = acc_color.b;
        framebuffer[fb_idx + 3u] = 1.0;
    }
}
