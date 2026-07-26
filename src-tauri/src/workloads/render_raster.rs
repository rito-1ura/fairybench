use std::time::Instant;
use wgpu::util::DeviceExt;

use crate::orchestrator::{BenchModule, MetricSample, RunPolicy};

/// GPUラスタライズ性能を測定するRender-Rasterモジュール
pub struct RenderRaster {
    device: Option<wgpu::Device>,
    queue: Option<wgpu::Queue>,
    vertex_count: u32,
    triangle_count: u32,
    viewport_size: (u32, u32),
}

impl RenderRaster {
    pub fn new() -> Self {
        Self {
            device: None,
            queue: None,
            vertex_count: 65536,
            triangle_count: 32768,
            viewport_size: (1920, 1080),
        }
    }
}

impl BenchModule for RenderRaster {
    fn name(&self) -> &'static str {
        "Render-Raster"
    }

    fn prepare(&mut self) -> Result<(), String> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // wgpuのアダプタとデバイスを取得
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }))
        .ok_or_else(|| "Failed to get wgpu adapter".to_string())?;

        let adapter_info = adapter.get_info();
        log::info!(
            "Render-Raster: using adapter '{}' ({:?})",
            adapter_info.name,
            adapter_info.backend
        );

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("FairyBench Render-Raster Device"),
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
            "Render-Raster: prepared {} vertices, {} triangles, {}x{} viewport",
            self.vertex_count,
            self.triangle_count,
            self.viewport_size.0,
            self.viewport_size.1,
        );

        Ok(())
    }

    fn run(&mut self, policy: &RunPolicy) -> Result<Vec<MetricSample>, String> {
        let device = self.device.as_ref().ok_or("Device not initialized")?;
        let queue = self.queue.as_ref().ok_or("Queue not initialized")?;

        let iterations = match policy {
            RunPolicy::FixedIterations(n) => *n,
            RunPolicy::MinDuration { .. } => 5,   // v1 default
            RunPolicy::Quick => 1,
        };

        // === Build shader module ===
        let shader_source = include_str!("../../shaders/render_raster.wgsl");
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Render-Raster Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        // === Create uniform buffers ===

        // Matrix: simple perspective projection (static for benchmark)
        let aspect = self.viewport_size.0 as f32 / self.viewport_size.1 as f32;
        let fov = std::f32::consts::FRAC_PI_4;
        let near = 0.1;
        let far = 100.0;
        let f = 1.0 / (fov * 0.5).tan();

        let mvp: [f32; 16] = [
            f / aspect, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, far / (far - near), 1.0,
            0.0, 0.0, -(far * near) / (far - near), 0.0,
        ];
        let mvp_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("MVP Matrix"),
            contents: bytemuck::cast_slice(&mvp),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        // Viewport constants
        let vp: [f32; 4] = [
            self.viewport_size.0 as f32,
            self.viewport_size.1 as f32,
            self.viewport_size.0 as f32 * 0.5,
            self.viewport_size.1 as f32 * 0.5,
        ];
        let vp_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Viewport"),
            contents: bytemuck::cast_slice(&vp),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        // === Create vertex buffer ===
        // Generate random vertex data
        let mut vert_data = Vec::with_capacity(self.vertex_count as usize * 6);
        // Simple seeded random for reproducibility
        let mut rng_state = 12345u32;
        for _ in 0..self.vertex_count {
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let px = ((rng_state >> 16) as f32 / 65536.0) * 4.0 - 2.0;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let py = ((rng_state >> 16) as f32 / 65536.0) * 4.0 - 2.0;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let pz = ((rng_state >> 16) as f32 / 65536.0) * 4.0 - 2.0;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let cr = (rng_state >> 16) as f32 / 65536.0;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let cg = (rng_state >> 16) as f32 / 65536.0;
            rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
            let cb = (rng_state >> 16) as f32 / 65536.0;
            vert_data.extend_from_slice(&[px, py, pz, cr, cg, cb]);
        }

        let vert_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertices"),
            contents: bytemuck::cast_slice(&vert_data),
            usage: wgpu::BufferUsages::STORAGE,
        });

        // === Create framebuffer (storage) ===
        let fb_size = (self.vertex_count as usize * 4 + self.triangle_count as usize * 4) as u64;
        let fb_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Framebuffer"),
            size: fb_size * std::mem::size_of::<f32>() as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });

        // === Create params buffer (updated per invocation) ===
        let params_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Params"),
            size: 16,  // 4 x f32
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // === Create bind group ===
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Render-Raster Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0, visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1, visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2, visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3, visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: true }, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4, visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Storage { read_only: false }, has_dynamic_offset: false, min_binding_size: None },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Render-Raster Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: mvp_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: vp_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 2, resource: params_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 3, resource: vert_buf.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 4, resource: fb_buf.as_entire_binding() },
            ],
        });

        // === Create compute pipeline ===
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render-Raster Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Render-Raster Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });

        // === Staging buffer for readback ===
        let readback_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Readback"),
            size: fb_size * std::mem::size_of::<f32>() as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        // === Run benchmark iterations ===
        let mut samples = Vec::with_capacity(iterations as usize);
        let workgroup_count = ((self.vertex_count.max(self.triangle_count) + 255) / 256).max(1);

        for iter in 0..iterations {
            // Update params buffer with current iteration's data
            let params: [f32; 4] = [
                self.vertex_count as f32,
                self.triangle_count as f32,
                iter as f32,
                0.0,
            ];
            queue.write_buffer(&params_buf, 0, bytemuck::cast_slice(&params));

            // Timed dispatch
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render-Raster Encoder"),
            });

            let start = Instant::now();

            {
                let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: Some("Render-Raster Pass"),
                    timestamp_writes: None,
                });
                cpass.set_pipeline(&compute_pipeline);
                cpass.set_bind_group(0, &bind_group, &[]);
                cpass.dispatch_workgroups(workgroup_count, 1, 1);
            }

            // Copy framebuffer to readback to force GPU completion
            encoder.copy_buffer_to_buffer(&fb_buf, 0, &readback_buf, 0, fb_size * std::mem::size_of::<f32>() as u64);

            queue.submit(std::iter::once(encoder.finish()));

            // Wait for GPU to complete (blocking readback)
            {
                let slice = readback_buf.slice(..);
                let (tx, rx) = std::sync::mpsc::channel();
                slice.map_async(wgpu::MapMode::Read, move |result| {
                    let _ = tx.send(result);
                });
                device.poll(wgpu::Maintain::Wait);
                rx.recv().map_err(|_| "GPU map notification timeout".to_string())?
                    .map_err(|e| format!("GPU map error: {e}"))?;
                // Readback完了 → unmapして次の反復で再利用可能にする
                readback_buf.unmap();
            }
            let elapsed = start.elapsed();

            // Compute ops per second (vertices processed)
            let ops_per_sec = (self.vertex_count as f64 * 2.0) / elapsed.as_secs_f64();

            samples.push(MetricSample {
                timestamp_ms: chrono::Utc::now().timestamp_millis(),
                value: ops_per_sec,
                label: "verts/sec".into(),
            });

            log::info!(
                "Render-Raster iteration {}: {:.2}M verts/sec ({}ms)",
                iter + 1,
                ops_per_sec / 1_000_000.0,
                elapsed.as_millis()
            );
        }

        let _duration = if samples.is_empty() {
            0
        } else {
            samples.last().unwrap().timestamp_ms - samples.first().unwrap().timestamp_ms
        };

        Ok(samples)
    }

    fn teardown(&mut self) -> Result<(), String> {
        // wgpuデバイスとキューはドロップ時に自動解放
        self.device = None;
        self.queue = None;
        log::info!("Render-Raster: teardown complete");
        Ok(())
    }
}
