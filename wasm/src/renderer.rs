use wgpu::util::DeviceExt;

use super::*;
use dmg_2025_core::{DISPLAY_BUFFER_SIZE, DisplayBuffer};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct OptionsUniform {
    palette: Palette,
    display_width: u32,
    display_height: u32,
    canvas_width: u32,
    canvas_height: u32,
}

impl Default for OptionsUniform {
    fn default() -> Self {
        Self {
            palette: Palette::default(),
            display_width: 160,
            display_height: 144,
            canvas_width: 0,
            canvas_height: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
// Uniform of display data
struct DisplayUniform {
    pub pixels: DisplayBuffer,
}

impl Default for DisplayUniform {
    fn default() -> Self {
        Self {
            pixels: [0; DISPLAY_BUFFER_SIZE],
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct EffectOptionsUniform {
    pub direction: [f32; 2],
    pub resolution: [f32; 2],
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct FinalOptionsUniform {
    pub glow_strength: f32,
    _pad: [f32; 3],
}

#[derive(Debug)]
struct UniformBuffer<U> {
    pub uniform: U,
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

impl<U> UniformBuffer<U>
where
    U: bytemuck::NoUninit + Default,
{
    pub fn new(device: &wgpu::Device, name: &str) -> Self {
        let uniform = U::default();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{name} Buffer")),
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(&format!("{name} Bind Group Layout")),
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
            label: Some(&format!("{name} Bind Group")),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });
        Self {
            uniform,
            buffer,
            bind_group,
            bind_group_layout,
        }
    }

    pub fn update(&self, queue: &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }
}

#[derive(Debug)]
struct FrameTexture {
    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
    pub bind_group: wgpu::BindGroup,
    name: String,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl FrameTexture {
    pub fn new(
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
        size: &wgpu::Extent3d,
        name: &str,
    ) -> Self {
        let layout = bind_group_layout.clone();
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&format!("{name} Texture")),
            size: *size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let (texture_view, bind_group) = Self::get_components(&texture, device, &layout, name);

        Self {
            texture,
            texture_view,
            bind_group,
            bind_group_layout: layout,
            name: name.to_string(),
        }
    }

    pub fn update(&mut self, device: &wgpu::Device) {
        let (texture_view, bind_group) =
            Self::get_components(&self.texture, device, &self.bind_group_layout, &self.name);
        self.texture_view = texture_view;
        self.bind_group = bind_group;
    }

    fn get_components(
        texture: &wgpu::Texture,
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
        name: &str,
    ) -> (wgpu::TextureView, wgpu::BindGroup) {
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some(&format!("{name} Bind Group")),
        });
        (texture_view, bind_group)
    }
}

#[derive(Debug)]
pub struct Renderer {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub is_surface_configured: bool,
    pub window: Arc<Window>,

    display_render_pipeline: wgpu::RenderPipeline,
    effect_render_pipeline: wgpu::RenderPipeline,
    final_render_pipeline: wgpu::RenderPipeline,

    display_texture: Option<FrameTexture>,
    texture_a: Option<FrameTexture>,
    texture_b: Option<FrameTexture>,
    texture_bind_group_layout: wgpu::BindGroupLayout,

    options: UniformBuffer<OptionsUniform>,
    display: UniformBuffer<DisplayUniform>,
    effect_options: UniformBuffer<EffectOptionsUniform>,
    final_options: UniformBuffer<FinalOptionsUniform>,
}

impl Renderer {
    fn init_render_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        shader: &wgpu::ShaderModule,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> wgpu::RenderPipeline {
        let display_render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts,
                push_constant_ranges: &[],
            });
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&display_render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            cache: None,
            depth_stencil: None,
            multiview: None,
        })
    }

    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let mut limits = wgpu::Limits::downlevel_webgl2_defaults();
        // Increase max texture size so website can be ran on bigger screens
        limits.max_texture_dimension_2d = 4096;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_features: wgpu::Features::empty(),
                required_limits: limits,
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let options = UniformBuffer::<OptionsUniform>::new(&device, "Options");
        let display = UniformBuffer::<DisplayUniform>::new(&device, "Display");
        // Initialize render pipeline for rendering the raw display data
        let display_shader = device.create_shader_module(wgpu::include_wgsl!("display.wgsl"));
        let display_render_pipeline = Self::init_render_pipeline(
            &device,
            &config,
            &display_shader,
            &[&display.bind_group_layout, &options.bind_group_layout],
        );

        let effect_options = UniformBuffer::<EffectOptionsUniform>::new(&device, "Effect Options");
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("Texture Bind Group Layout"),
            });
        // Initialize render pipeline for post processing effects
        let effect_shader = device.create_shader_module(wgpu::include_wgsl!("effect.wgsl"));
        let effect_render_pipeline = Self::init_render_pipeline(
            &device,
            &config,
            &effect_shader,
            &[
                &texture_bind_group_layout,
                &effect_options.bind_group_layout,
            ],
        );

        let final_options = UniformBuffer::<FinalOptionsUniform>::new(&device, "Final Options");
        // Initialize render pipeline for final composite pass
        let final_shader = device.create_shader_module(wgpu::include_wgsl!("final.wgsl"));
        let final_render_pipeline = Self::init_render_pipeline(
            &device,
            &config,
            &final_shader,
            &[
                &texture_bind_group_layout,
                &texture_bind_group_layout,
                &final_options.bind_group_layout,
            ],
        );

        Ok(Self {
            surface,
            device,
            queue,
            config,
            is_surface_configured: false,
            window,

            display_render_pipeline,
            effect_render_pipeline,
            final_render_pipeline,

            display_texture: None,
            texture_a: None,
            texture_b: None,
            texture_bind_group_layout,

            options,
            display,
            effect_options,
            final_options,
        })
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // We can't render unless the surface is configured
        if !self.is_surface_configured {
            return Ok(());
        }

        // Render the Game Boy display onto texture
        let mut display_encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Display Render Encoder"),
                });
        let mut display_render_pass =
            display_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Display Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.display_texture.as_ref().unwrap().texture_view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        display_render_pass.set_pipeline(&self.display_render_pipeline);
        display_render_pass.set_bind_group(0, &self.options.bind_group, &[]);
        display_render_pass.set_bind_group(1, &self.display.bind_group, &[]);
        display_render_pass.draw(0..6, 0..1);
        drop(display_render_pass);
        self.queue.submit(std::iter::once(display_encoder.finish()));

        let (texture_a, texture_b) = (
            self.texture_a.as_mut().unwrap(),
            self.texture_b.as_mut().unwrap(),
        );
        const ITERATIONS: usize = 8;
        for i in 1..=ITERATIONS {
            let mut effect_encoder =
                self.device
                    .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some(&format!("Display Render Encoder {i}")),
                    });

            texture_a.update(&self.device);
            texture_b.update(&self.device);

            // Choose texture view, bind group and blur direction based on iteration count
            let radius = (ITERATIONS - i) as f32;
            let (view, mut bind_group, direction) = if i.is_multiple_of(2) {
                (
                    &texture_a.texture_view,
                    &texture_b.bind_group,
                    [0.0, radius],
                )
            } else {
                (
                    &texture_b.texture_view,
                    &texture_a.bind_group,
                    [radius, 0.0],
                )
            };
            // On the first pass, read from the initial display texture
            if i == 1 {
                bind_group = &self.display_texture.as_ref().unwrap().bind_group;
            }
            // Update direction buffer
            self.effect_options.uniform.direction = direction;
            self.effect_options.update(&self.queue);

            // Execute render pass
            let mut effect_render_pass =
                effect_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some(&format!("Effect Render Pass {i}")),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        depth_slice: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });
            effect_render_pass.set_pipeline(&self.effect_render_pipeline);
            effect_render_pass.set_bind_group(0, Some(bind_group), &[]);
            effect_render_pass.set_bind_group(1, &self.effect_options.bind_group, &[]);
            effect_render_pass.draw(0..6, 0..1);
            drop(effect_render_pass);
            self.queue.submit(std::iter::once(effect_encoder.finish()));
        }

        let mut final_encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Final Render Encoder"),
                });
        let output_texture = self.surface.get_current_texture()?;
        let output_view = output_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut final_render_pass = final_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Final Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &output_view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        final_render_pass.set_pipeline(&self.final_render_pipeline);
        final_render_pass.set_bind_group(
            0,
            &self.display_texture.as_ref().unwrap().bind_group,
            &[],
        );
        let effect_bind_group = if ITERATIONS.is_multiple_of(2) {
            &texture_b.bind_group
        } else {
            &texture_a.bind_group
        };
        final_render_pass.set_bind_group(1, effect_bind_group, &[]);
        final_render_pass.set_bind_group(2, &self.final_options.bind_group, &[]);
        final_render_pass.draw(0..6, 0..1);
        drop(final_render_pass);
        self.queue.submit(std::iter::once(final_encoder.finish()));
        output_texture.present();

        self.window.request_redraw();
        Ok(())
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
            self.is_surface_configured = true;

            // Update frame textures
            let texture_size = wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            };
            self.display_texture = Some(FrameTexture::new(
                &self.device,
                &self.texture_bind_group_layout,
                &texture_size,
                "Display Texture"
            ));
            self.texture_a = Some(FrameTexture::new(
                &self.device,
                &self.texture_bind_group_layout,
                &texture_size,
                "Texture A",
            ));
            self.texture_b = Some(FrameTexture::new(
                &self.device,
                &self.texture_bind_group_layout,
                &texture_size,
                "Texture B",
            ));

            // Update option uniforms
            self.options.uniform.canvas_width = width;
            self.options.uniform.canvas_height = height;
            self.options.update(&self.queue);
            self.effect_options.uniform.resolution[0] = width as f32;
            self.effect_options.uniform.resolution[1] = height as f32;
            self.effect_options.update(&self.queue);
        }
    }

    pub fn update_options(&mut self, options: &Options) {
        self.options.uniform.palette = options.palette;
        self.final_options.uniform.glow_strength = options.glow_strength;
        self.options.update(&self.queue);
        self.final_options.update(&self.queue);
    }

    pub fn update_display(&mut self, display: &DisplayBuffer) {
        self.display.uniform.pixels = *display;
        self.display.update(&self.queue);
    }
}
