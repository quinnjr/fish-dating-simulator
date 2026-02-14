//! cult_papa Fish Dating Simulator
//!
//! A 2D ASCII-art dating simulator where you catch fish and take them on dates.
//! Built with the Sable engine.

use std::time::Instant;

use pollster::FutureExt;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowId;

use sable_gpu::prelude::*;
use sable_platform::prelude::*;

mod achievements;
#[allow(dead_code)]
mod ascii_art;
mod data;
mod dating;
mod easter_egg;
mod fishing;
mod game;
mod plugins;
#[allow(dead_code)]
mod render;
#[allow(dead_code)]
mod ui;

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;

/// Create the event loop with platform-specific settings.
fn create_event_loop() -> std::result::Result<EventLoop<()>, winit::error::EventLoopError> {
    #[cfg(target_os = "linux")]
    {
        use winit::platform::wayland::EventLoopBuilderExtWayland;
        use winit::platform::x11::EventLoopBuilderExtX11;

        if let Ok(event_loop) = EventLoop::builder().with_wayland().build() {
            tracing::info!("Using Wayland backend");
            return Ok(event_loop);
        }

        if let Ok(event_loop) = EventLoop::builder().with_x11().build() {
            tracing::info!("Using X11 backend");
            return Ok(event_loop);
        }
    }

    EventLoop::builder().build()
}

struct App {
    window: Option<Window>,
    gpu: Option<GpuContext>,
    renderer: Option<render::GameRenderer>,
    game: game::Game,
    last_frame: Instant,
    pending_key: Option<KeyCode>,
}

impl App {
    fn new() -> Self {
        // Load plugin fish from the plugins/ directory
        let registry = plugins::load_all_plugins();

        Self {
            window: None,
            gpu: None,
            renderer: None,
            game: game::Game::new(registry),
            last_frame: Instant::now(),
            pending_key: None,
        }
    }

    fn render_frame(&mut self) {
        let Some(gpu) = &self.gpu else { return };
        let Some(renderer) = &mut self.renderer else {
            return;
        };

        let output = match gpu.get_current_texture() {
            Ok(output) => output,
            Err(GpuError::SwapChainAcquire(
                wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated,
            )) => {
                let (width, height) = gpu.surface_size();
                if let Some(gpu) = &mut self.gpu {
                    gpu.resize(width, height);
                }
                return;
            }
            Err(e) => {
                tracing::error!("Surface error: {:?}", e);
                return;
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Update camera
        renderer.update_camera(gpu.queue());

        // Begin text rendering
        renderer.begin();
        renderer.begin_images();

        // Draw the current game screen
        self.game.render(renderer);

        // End text rendering
        let text_count = renderer.end(gpu.queue());
        let image_count = renderer.end_images(gpu.queue());

        // Submit render pass
        let mut encoder = gpu.create_command_encoder();
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.05,
                            b: 0.1,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&renderer.sprite_pipeline);
            render_pass.set_bind_group(0, &renderer.camera_bind_group, &[]);
            render_pass.set_bind_group(1, &renderer.font_bind_group, &[]);
            renderer.text_renderer.render(&mut render_pass, text_count);

            // Render image sprites (cult_papa face, etc.) on top
            renderer.render_images(&mut render_pass, image_count);
        }

        gpu.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let config = WindowConfig::new("cult_papa Fish Dating Simulator")
            .with_size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .with_resizable(true)
            .with_vsync(true);

        let window = Window::new(event_loop, &config).expect("Failed to create window");
        let gpu = GpuContext::new(&window)
            .block_on()
            .expect("Failed to create GPU context");

        tracing::info!(
            "GPU: {} ({:?})",
            gpu.adapter_info().name,
            gpu.adapter_info().backend
        );

        let renderer = render::GameRenderer::new(&gpu, WINDOW_WIDTH, WINDOW_HEIGHT);

        self.window = Some(window);
        self.renderer = Some(renderer);
        self.gpu = Some(gpu);
        self.last_frame = Instant::now();

        tracing::info!("cult_papa Fish Dating Simulator initialized!");
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                let _ = data::save::save_game(&self.game.player);
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if size.width > 0 && size.height > 0 {
                    if let Some(gpu) = &mut self.gpu {
                        gpu.resize(size.width, size.height);
                    }
                    if let Some(renderer) = &mut self.renderer {
                        renderer.resize(size.width, size.height);
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key),
                        state: ElementState::Pressed,
                        repeat: false,
                        ..
                    },
                ..
            } => {
                self.pending_key = Some(key);
            }
            WindowEvent::RedrawRequested => {
                let now = Instant::now();
                let dt = (now - self.last_frame).as_secs_f32().min(0.1);
                self.last_frame = now;

                // Process game logic
                let key = self.pending_key.take();
                self.game.update(dt, key);

                // Render
                self.render_frame();
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    tracing::info!("Starting cult_papa Fish Dating Simulator");
    tracing::info!("Catch fish. Date fish. Find love.");

    let event_loop = create_event_loop().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::new();
    event_loop.run_app(&mut app).expect("Event loop error");
}
