use std::time::Instant;
use wgpu::util::DeviceExt;

use crate::orchestrator::{BenchModule, MetricSample, RunPolicy};

/// GPUレイトレーシング/パストレーシング性能を測定するRender-PathTraceモジュール
pub struct RenderPathTrace {
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    num_spheres: u32,
    image_size: (u32, u32),
    bounces: u32,
}

impl RenderPathTrace {
    pub fn new() -> Self {
        Self {
            device: None,
            queue: None,
            num_spheres: 50,
            image_size: (512, 512),
            bounces: 4,
        }
    }
}

impl BenchModule for RenderPathTrace {
    fn name(&self) -> &'static str {
        "Render-PathTrace"
    }

    fn prepare(&mut self) -> Result<(), String> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }))
        .ok_or_else(|| "Failed to get wgpu adapter".to_string())?;

        let adapter_info = adapter.get_info();
        log::info!(
            "Render-PathTrace: using adapter '{}' ({:?})",
            adapter_info.name,
            adapter_info.backend
        );

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("FairyBench Render-PathTrace Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
            },
            None,
        ))
        .map_err(|e| format!("Failed to request device: {e}"))?;

        self.device = Some(device);
        self.queue = Some(queue);

        log::info!(
            "Render-PathTrace: prepared {} spheres, {}x{} image, {} bounces",
            self.num_spheres, self.image_size.0, self.image_size.1, self.bounces,
        );
        Ok(())
    }

    fn run(&mut self, policy: &RunPolicy) -> Result<Vec<MetricSample>, String> {
        let device = self.device.as_ref().ok_or("Device not initialized")?;
        let queue = self.queue.as_ref().ok_or("Queue not initialized")?;

        let iterations = match policy {
            RunPolicy::FixedIterations(n) => *n,
            RunPolicy::MinDuration { .. } => 3,
            RunPolicy::Quick => 1,
        };

        let shader_source = include_str!("../../shaders/render_pathtrace.wgsl");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Render-PathTrace Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // === Camera uniforms ===
        let cam_data: [f32; 4] = [0.0, 0.0, -5.0, 0.5]; // origin, fov_scale
        let cam_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera"),
            contents: bytemuck::cast_slice(&cam_data),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let cam_dir_data: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        let cam_dir_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Target"),
            contents: bytemuck::cast_slice(&cam_dir_data),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let params_data: [f32; 4] = [
            self.num_spheres as f32,
            self.image_size.0 as f32,
            self.image_size.1 as f32,
            self.bounces as f32,
        ];
        let params_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Params"),
            contents: bytemuck::cast_slice(&params_data),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        // === Sphere data (50 spheres) ===
        let mut sphere_data = Vec::with_capacity(self.num_spheres as usize * 8);
        let mut rng_state = 99999u32;
        for _ in 0..self.num_spheres {
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let sx = ((rng_state >> 16) as f32 / 65536.0) * 6.0 - 3.0;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let sy = ((rng_state >> 16) as f32 / 65536.0) * 4.0 - 2.0;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let sz = ((rng_state >> 16) as f32 / 65536.0) * 6.0 - 3.0;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let radius = ((rng_state >> 16) as f32 / 65536.0) * 0.5 + 0.2;
            sphere_data.extend_from_slice(&[sx, sy, sz, radius]);

            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let cr = ((rng_state >> 16) as f32 / 65536.0) * 0.8 + 0.2;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let cg = ((rng_state >> 16) as f32 / 65536.0) * 0.8 + 0.2;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let cb = ((rng_state >> 16) as f32 / 65536.0) * 0.8 + 0.2;
            sphere_data.extend_from_slice(&[cr, cg, cb, 1.0]);
        }

        let sphere_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Spheres"),
            contents: bytemuck::cast_slice(&sphere_data),
            usage: wgpu::BufferUsages::STORAGE,
        });

        // === Framebuffer ===
        let fb_size = (self.image_size.0 * self.image_size.1 * 4) as u64;
        let fb_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Framebuffer"),
            size: fb_size * std::mem::size_of::<f32>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // === Bind group ===
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("PathTrace Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry { binding: 0, visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 1, visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 2, visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 3, visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None }, count: None },
                wgpu::BindGroupLayoutEntry { binding: 4, visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None }, count: None },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("PathTrace Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: cam_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: cam_dir_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: params_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: sphere_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 4, resource: fb_buf.as_entire_binding() },
            ],
        });

        // === Pipeline ===
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("PathTrace Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("PathTrace Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });

        // === Readback buffer ===
        let readback_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("PathTrace Readback"),
            size: fb_size * std::mem::size_of::<f32>() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // === Run iterations ===
        let total_rays = self.image_size.0 * self.image_size.1;
        let workgroup_count = ((total_rays + 63) / 64).max(1);
        let mut samples = Vec::with_capacity(iterations as usize);

        for iter in 0..iterations {
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("PathTrace Encoder"),
            });

            let start = Instant::now();

            {
                let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("PathTrace Pass"),
                    timestamp_writes: None,
                });
                cpass.set_pipeline(&compute_pipeline);
                cpass.set_bind_group(0, &bind_group, &[]);
                cpass.dispatch_workgroups(workgroup_count, 1, 1);
            }

            encoder.copy_buffer_to_buffer(&fb_buf, 0, &readback_buf, 0, fb_size * std::mem::size_of::<f32>() as u64);
            queue.submit(std::iter::once(encoder.finish()));

            // GPU sync via readback
            {
                let slice = readback_buf.slice(..);
                let (tx, rx) = std::sync::mpsc::channel();
                slice.map_async(wgpu::MapMode::Read, move |result| {
                    let _ = tx.send(result);
                });
                device.poll(wgpu::Maintain::Wait);
                rx.recv().map_err(|_| "GPU map timeout".to_string())?
                    .map_err(|e| format!("GPU map error: {e}"))?;
                readback_buf.unmap();
            }

            let elapsed = start.elapsed();

            // Metric: rays per second (each ray = pixel × bounces)
            let rays_per_sec = (total_rays as f64 * self.bounces as f64) / elapsed.as_secs_f64();

            samples.push(MetricSample {
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
                value: rays_per_sec,
                label: "rays/sec".into(),
            });

            log::info!(
                "Render-PathTrace iteration {}: {:.2}M rays/sec ({}ms)",
                iter + 1,
                rays_per_sec / 1_000_000.0,
                elapsed.as_millis()
            );
        }

        Ok(samples)
    }

    fn teardown(&mut self) -> Result<(), String> {
        self.device = None;
        self.queue = None;
        log::info!("Render-PathTrace: teardown complete");
        Ok(())
    }
}
