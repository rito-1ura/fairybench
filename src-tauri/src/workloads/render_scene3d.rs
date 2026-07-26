use std::time::Instant;

use wgpu::util::DeviceExt;
use crate::orchestrator::{BenchModule, MetricSample, RunPolicy};

/// Render-3DScene: wgpuを使った3Dシーンレンダリング（FPS測定）
/// Cinebench/3DMark相当のGPUレンダリング負荷試験
pub struct Render3DScene {
    duration_secs: f64,
}

impl Render3DScene {
    pub fn new() -> Self {
        Self { duration_secs: 5.0 }
    }
}

impl BenchModule for Render3DScene {
    fn name(&self) -> &'static str { "Render-3DScene" }

    fn prepare(&mut self) -> Result<(), String> {
        log::info!("Render-3DScene: prepared ({}s duration)", self.duration_secs);
        Ok(())
    }

    fn run(&mut self, _policy: &RunPolicy) -> Result<Vec<MetricSample>, String> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })).ok_or("No GPU adapter")?;

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("3DScene Device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
            },
            None,
        )).map_err(|e| format!("Device: {e}"))?;

        // Simple textured quad: 4 vertices, 6 indices
        let vertices: &[f32] = &[
            -1.0, -1.0, 0.0, 0.0, 0.0, 1.0,
             1.0, -1.0, 0.0, 0.0, 1.0, 1.0,
             1.0,  1.0, 0.0, 0.0, 1.0, 0.0,
            -1.0,  1.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let indices: &[u16] = &[0, 1, 2, 0, 2, 3];

        let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vert"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("idx"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let vertex_attrs = &[
            wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x3, offset: 0, shader_location: 0 },
            wgpu::VertexAttribute { format: wgpu::VertexFormat::Float32x3, offset: 12, shader_location: 1 },
        ];
        let vlayout = wgpu::VertexBufferLayout {
            array_stride: 24,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: vertex_attrs,
        };

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("3DScene"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/render_scene3d.wgsl").into()),
        });

        // Uniform buffer for rotation + time
        let uniform_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("uniform"),
            size: 64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bind_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bind"),
            layout: &bind_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buf.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipe_layout"),
            bind_group_layouts: &[&bind_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("3dscene"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[vlayout],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8Unorm,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        // Output texture
        let tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("output"),
            size: wgpu::Extent3d { width: 512, height: 512, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let tex_view = tex.create_view(&wgpu::TextureViewDescriptor::default());

        let mut samples = Vec::new();
        let start = Instant::now();
        let mut frame_count: u64 = 0;
        let mut rng = 1.0f32;

        while start.elapsed().as_secs_f64() < self.duration_secs {
            let frame_start = Instant::now();

            // Update uniform: time + rotation
            let elapsed = start.elapsed().as_secs_f32();
            rng = (rng * 1.01 + 0.01).fract();
            let uniform_data: [f32; 16] = [
                elapsed, elapsed * 0.5, rng, 1.0,
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
            ];
            queue.write_buffer(&uniform_buf, 0, bytemuck::cast_slice(&uniform_data));

            // Render
            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("frame_enc"),
            });
            {
                let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("frame_pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &tex_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });
                rpass.set_pipeline(&pipeline);
                rpass.set_bind_group(0, Some(&bind_group), &[]);
                rpass.set_vertex_buffer(0, vertex_buf.slice(..));
                rpass.set_index_buffer(index_buf.slice(..), wgpu::IndexFormat::Uint16);
                rpass.draw_indexed(0..6, 0, 0..1);
            }
            queue.submit(Some(encoder.finish()));

            let frame_elapsed = frame_start.elapsed().as_secs_f64();
            let fps = if frame_elapsed > 0.0 { 1.0 / frame_elapsed } else { 0.0 };
            frame_count += 1;

            // Poll (required for wgpu)
            device.poll(wgpu::Maintain::Wait);

            // Every 30 frames, record a sample
            if frame_count % 30 == 0 {
                samples.push(MetricSample {
                    timestamp_ms: chrono::Utc::now().timestamp_millis(),
                    value: fps,
                    label: "scene_fps".into(),
                });
            }
        }

        let total_elapsed = start.elapsed().as_secs_f64();
        let avg_fps = if total_elapsed > 0.0 { frame_count as f64 / total_elapsed } else { 0.0 };
        log::info!("Render-3DScene: {:.0} frames in {:.1}s = avg {:.1} FPS", frame_count, total_elapsed, avg_fps);

        Ok(samples)
    }

    fn teardown(&mut self) -> Result<(), String> {
        log::info!("Render-3DScene: teardown complete");
        Ok(())
    }
}
