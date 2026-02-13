//! Rendering backend for the Fish Dating Simulator.
//!
//! Wraps sable-gpu text rendering into a simple API for drawing
//! ASCII art and text at grid positions in a GPU-accelerated window.

use sable_gpu::prelude::*;
use wgpu::util::DeviceExt;

/// Grid-based text renderer for ASCII art games.
pub struct GameRenderer {
    pub sprite_pipeline: wgpu::RenderPipeline,
    pub text_renderer: TextRenderer,
    pub font: BitmapFont,
    pub font_texture: Texture,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: wgpu::BindGroup,
    pub font_bind_group: wgpu::BindGroup,
    pub camera: Camera2D,
}

/// Color presets for the game.
pub struct Colors;

impl Colors {
    pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
    pub const CYAN: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
    pub const GREEN: [f32; 4] = [0.2, 1.0, 0.2, 1.0];
    pub const RED: [f32; 4] = [1.0, 0.3, 0.3, 1.0];
    pub const ORANGE: [f32; 4] = [1.0, 0.6, 0.1, 1.0];
    pub const BLUE: [f32; 4] = [0.3, 0.5, 1.0, 1.0];
    pub const PINK: [f32; 4] = [1.0, 0.5, 0.7, 1.0];
    pub const GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
    pub const DARK_GRAY: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
    pub const LIGHT_BLUE: [f32; 4] = [0.5, 0.7, 1.0, 1.0];
    pub const PURPLE: [f32; 4] = [0.7, 0.3, 1.0, 1.0];
}

impl GameRenderer {
    /// The scale for all text rendering (2x the 8x8 builtin font).
    pub const SCALE: f32 = 2.0;
    /// Character width in pixels at scale 1.0.
    pub const CHAR_W: f32 = 8.0;
    /// Character height in pixels at scale 1.0.
    pub const CHAR_H: f32 = 8.0;

    /// Scaled character width.
    pub fn char_width(&self) -> f32 {
        Self::CHAR_W * Self::SCALE
    }

    /// Scaled character height (line height).
    pub fn char_height(&self) -> f32 {
        Self::CHAR_H * Self::SCALE
    }

    /// Create the renderer from a GPU context.
    pub fn new(gpu: &GpuContext, width: u32, height: u32) -> Self {
        let device = gpu.device();
        let queue = gpu.queue();

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Fish Dating Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let font = BuiltinFont::create_font();
        let font_texture = BuiltinFont::create_texture(device, queue);
        let text_renderer = TextRenderer::new(device, 20000);

        let camera = Camera2D::new(width as f32, height as f32);
        let camera_uniform = Camera2DUniform::from_camera(&camera);
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera BGL"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture BGL"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera BG"),
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
        });

        let font_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Font BG"),
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(font_texture.view()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(font_texture.sampler()),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let sprite_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Sprite Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[SpriteVertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: gpu.surface_format(),
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            sprite_pipeline,
            text_renderer,
            font,
            font_texture,
            camera_buffer,
            camera_bind_group,
            font_bind_group,
            camera,
        }
    }

    /// Resize viewport.
    pub fn resize(&mut self, width: u32, height: u32) {
        self.camera.set_viewport(width as f32, height as f32);
    }

    /// Update camera uniform buffer.
    pub fn update_camera(&self, queue: &wgpu::Queue) {
        let camera_uniform = Camera2DUniform::from_camera(&self.camera);
        queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );
    }

    /// Begin a new frame of text drawing.
    pub fn begin(&mut self) {
        self.text_renderer.begin();
    }

    /// Draw text at a pixel position with a given style.
    pub fn draw_text(&mut self, text: &str, pos: [f32; 2], style: &TextStyle) {
        self.text_renderer
            .draw_text(text, pos, &self.font, style);
    }

    /// Draw text at a grid position (column, row) from top-left of screen.
    pub fn draw_at_grid(&mut self, text: &str, col: f32, row: f32, color: [f32; 4]) {
        let (left, _, top, _) = self.camera.visible_bounds();
        let x = left + col * self.char_width();
        let y = top + row * self.char_height();
        let style = TextStyle::new()
            .with_scale(Self::SCALE)
            .with_color(color[0], color[1], color[2], color[3]);
        self.text_renderer
            .draw_text(text, [x, y], &self.font, &style);
    }

    /// Draw multi-line text at a grid position.
    pub fn draw_multiline_at_grid(
        &mut self,
        text: &str,
        col: f32,
        row: f32,
        color: [f32; 4],
    ) {
        for (i, line) in text.lines().enumerate() {
            self.draw_at_grid(line, col, row + i as f32, color);
        }
    }

    /// Draw centered text at a given row.
    pub fn draw_centered(&mut self, text: &str, row: f32, color: [f32; 4]) {
        let (_, _, top, _) = self.camera.visible_bounds();
        let y = top + row * self.char_height();
        let style = TextStyle::new()
            .with_scale(Self::SCALE)
            .with_color(color[0], color[1], color[2], color[3])
            .with_align(TextAlign::Center);
        self.text_renderer
            .draw_text(text, [0.0, y], &self.font, &style);
    }

    /// Draw multi-line centered text.
    pub fn draw_multiline_centered(&mut self, text: &str, start_row: f32, color: [f32; 4]) {
        for (i, line) in text.lines().enumerate() {
            self.draw_centered(line, start_row + i as f32, color);
        }
    }

    /// End text drawing and return vertex count.
    pub fn end(&mut self, queue: &wgpu::Queue) -> u32 {
        self.text_renderer.end(queue)
    }

    /// Get the number of columns visible on screen.
    pub fn screen_cols(&self) -> f32 {
        let (left, right, _, _) = self.camera.visible_bounds();
        (right - left) / self.char_width()
    }

    /// Get the number of rows visible on screen.
    pub fn screen_rows(&self) -> f32 {
        let (_, _, top, bottom) = self.camera.visible_bounds();
        (bottom - top) / self.char_height()
    }
}
