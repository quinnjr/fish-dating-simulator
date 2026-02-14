//! Rendering backend for the cult_papa Fish Dating Simulator.
//!
//! Wraps sable-gpu text rendering into a simple API for drawing
//! ASCII art and text at grid positions in a GPU-accelerated window.

use std::path::Path;

use sable_gpu::prelude::*;
use wgpu::util::DeviceExt;

/// An image sprite that can be drawn at a grid position.
pub struct ImageSprite {
    pub texture: Texture,
    pub bind_group: wgpu::BindGroup,
    pub batch: SpriteBatch,
}

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
    /// Bind group layout for textures (reused for image sprites).
    texture_bind_group_layout: wgpu::BindGroupLayout,
    /// Loaded image sprites (easter egg faces, etc.)
    pub cult_papa_face: Option<ImageSprite>,
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

        let mut renderer = Self {
            sprite_pipeline,
            text_renderer,
            font,
            font_texture,
            camera_buffer,
            camera_bind_group,
            font_bind_group,
            camera,
            texture_bind_group_layout,
            cult_papa_face: None,
        };

        // Try to load cult_papa face image for the easter egg
        renderer.try_load_cult_papa_face(device, queue);

        renderer
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
    ///
    /// Centers the *block* as a whole based on the widest line, then draws
    /// every line from the same starting column so internal ASCII-art
    /// alignment is preserved.
    pub fn draw_multiline_centered(&mut self, text: &str, start_row: f32, color: [f32; 4]) {
        let max_width = text.lines().map(|l| l.len()).max().unwrap_or(0) as f32;
        let cols = self.screen_cols();
        let start_col = (cols - max_width) / 2.0;
        for (i, line) in text.lines().enumerate() {
            self.draw_at_grid(line, start_col, start_row + i as f32, color);
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

    // ─── Image Sprite Rendering ─────────────────────────────────────────────

    /// Attempt to load the cult_papa face image.
    fn try_load_cult_papa_face(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        // Try several possible paths for the image
        let paths = [
            "images.jpeg",
            "assets/images.jpeg",
        ];

        for path in &paths {
            if Path::new(path).exists() {
                match Texture::from_file(device, queue, path, &TextureConfig::new().with_mipmaps(false)) {
                    Ok(texture) => {
                        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                            label: Some("cult_papa Face BG"),
                            layout: &self.texture_bind_group_layout,
                            entries: &[
                                wgpu::BindGroupEntry {
                                    binding: 0,
                                    resource: wgpu::BindingResource::TextureView(texture.view()),
                                },
                                wgpu::BindGroupEntry {
                                    binding: 1,
                                    resource: wgpu::BindingResource::Sampler(texture.sampler()),
                                },
                            ],
                        });

                        let batch = SpriteBatch::new(device, 16);

                        tracing::info!("Loaded cult_papa face from: {}", path);
                        self.cult_papa_face = Some(ImageSprite {
                            texture,
                            bind_group,
                            batch,
                        });
                        return;
                    }
                    Err(e) => {
                        tracing::warn!("Failed to load cult_papa face from {}: {:?}", path, e);
                    }
                }
            }
        }

        tracing::info!("cult_papa face image not found (easter egg face will use ASCII fallback)");
    }

    /// Begin image sprite drawing. Call before `draw_image_*` methods.
    pub fn begin_images(&mut self) {
        if let Some(ref mut face) = self.cult_papa_face {
            face.batch.begin();
        }
    }

    /// Draw the cult_papa face at a grid position with a given size in grid cells.
    /// `size_cells` is how many grid cells wide/tall the image should be.
    pub fn draw_cult_papa_face(
        &mut self,
        col: f32,
        row: f32,
        size_cells: f32,
        tint: [f32; 4],
    ) {
        let (left, _, top, _) = self.camera.visible_bounds();
        let pixel_size = size_cells * self.char_width();
        // Position is the center of the sprite
        let x = left + col * self.char_width() + pixel_size * 0.5;
        let y = top + row * self.char_height() + pixel_size * 0.5;

        if let Some(ref mut face) = self.cult_papa_face {
            let params = SpriteParams::new()
                .with_color(tint[0], tint[1], tint[2], tint[3]);
            face.batch.draw([x, y], [pixel_size, pixel_size], &params);
        }
    }

    /// Draw the cult_papa face centered at a grid row, with a given size in cells.
    pub fn draw_cult_papa_face_centered(
        &mut self,
        row: f32,
        size_cells: f32,
        tint: [f32; 4],
    ) {
        let cols = self.screen_cols();
        let col = (cols - size_cells) / 2.0;
        self.draw_cult_papa_face(col, row, size_cells, tint);
    }

    /// End image sprite drawing. Returns the sprite count for rendering.
    pub fn end_images(&mut self, queue: &wgpu::Queue) -> u32 {
        if let Some(ref mut face) = self.cult_papa_face {
            face.batch.end(queue)
        } else {
            0
        }
    }

    /// Render image sprites in the given render pass.
    /// Must be called after setting the pipeline and camera bind group.
    pub fn render_images<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, count: u32) {
        if count == 0 {
            return;
        }
        if let Some(ref face) = self.cult_papa_face {
            render_pass.set_bind_group(1, &face.bind_group, &[]);
            face.batch.render(render_pass, count);
            // Restore font bind group for any subsequent text rendering
            render_pass.set_bind_group(1, &self.font_bind_group, &[]);
        }
    }

    /// Returns true if the cult_papa face image is loaded.
    pub fn has_cult_papa_face(&self) -> bool {
        self.cult_papa_face.is_some()
    }
}
