mod audio;
mod engine;
mod game;
// We'll add modding later in Phase 8
// mod modding;

use engine::camera::Camera;
use engine::raycaster::Raycaster;
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{DeviceEvent, ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{CursorGrabMode, WindowBuilder};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const MOVE_SPEED: f32 = 2.5; // Units per second
const MOUSE_SENSITIVITY: f32 = 0.002; // Slightly reduced for smoother control

// Test map: 1 represents walls, 0 represents empty space
const MAP: [[i32; 8]; 8] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 1, 0, 1],
    [1, 0, 1, 0, 0, 1, 0, 1],
    [1, 0, 0, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];

struct GameState {
    camera: Camera,
    raycaster: Raycaster,
    last_update: Instant,
    move_forward: bool,
    move_backward: bool,
    move_left: bool,
    move_right: bool,
    game_focused: bool,
    map: Vec<Vec<i32>>,
}

impl GameState {
    fn new() -> Self {
        // Convert static map to owned Vec<Vec<i32>>
        let map: Vec<Vec<i32>> = MAP.iter().map(|row| row.to_vec()).collect();
        let mut raycaster = Raycaster::new(WIDTH, HEIGHT);
        raycaster.set_map(map.clone());

        Self {
            camera: Camera::new(1.5, 1.5), // Start near the corner, looking into the room
            raycaster,
            last_update: Instant::now(),
            move_forward: false,
            move_backward: false,
            move_left: false,
            move_right: false,
            game_focused: true,
            map,
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        if self.game_focused {
            // Update camera position based on movement flags
            if self.move_forward {
                self.camera.move_forward(MOVE_SPEED * dt, &self.map);
            }
            if self.move_backward {
                self.camera.move_forward(-MOVE_SPEED * dt, &self.map);
            }
            if self.move_left {
                self.camera.move_right(-MOVE_SPEED * dt, &self.map);
            }
            if self.move_right {
                self.camera.move_right(MOVE_SPEED * dt, &self.map);
            }
        }
    }

    fn render(&mut self, frame: &mut [u8]) {
        self.raycaster.render(&self.camera, frame);
    }

    fn handle_key_event(&mut self, key_code: VirtualKeyCode, pressed: bool) -> Option<bool> {
        match key_code {
            VirtualKeyCode::W => {
                if self.game_focused {
                    self.move_forward = pressed;
                }
                None
            }
            VirtualKeyCode::S => {
                if self.game_focused {
                    self.move_backward = pressed;
                }
                None
            }
            VirtualKeyCode::A => {
                if self.game_focused {
                    self.move_left = pressed;
                }
                None
            }
            VirtualKeyCode::D => {
                if self.game_focused {
                    self.move_right = pressed;
                }
                None
            }
            VirtualKeyCode::Escape if pressed => {
                // Toggle game focus
                self.game_focused = !self.game_focused;
                // Reset movement flags when unfocusing
                if !self.game_focused {
                    self.move_forward = false;
                    self.move_backward = false;
                    self.move_left = false;
                    self.move_right = false;
                }
                Some(self.game_focused)
            }
            _ => None,
        }
    }

    fn handle_mouse_motion(&mut self, delta_x: f64) {
        if self.game_focused {
            // Negate delta_x to reverse rotation direction
            self.camera.rotate(-delta_x as f32 * MOUSE_SENSITIVITY);
        }
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Lucid Raycaster")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    // Start with cursor hidden and captured
    window.set_cursor_visible(false);
    let _ = window.set_cursor_grab(CursorGrabMode::Confined);

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;
    let mut game = GameState::new();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::DeviceEvent {
                event: DeviceEvent::Key(input),
                ..
            } => {
                if let Some(keycode) = input.virtual_keycode {
                    let pressed = input.state == ElementState::Pressed;
                    if let Some(focused) = game.handle_key_event(keycode, pressed) {
                        window.set_cursor_visible(!focused);
                        let _ = window.set_cursor_grab(if focused {
                            CursorGrabMode::Confined
                        } else {
                            CursorGrabMode::None
                        });
                    }
                }
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta: (x, _) },
                ..
            } => {
                game.handle_mouse_motion(x);
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    error!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::MainEventsCleared => {
                // Update game state
                game.update();

                // Draw frame
                game.render(pixels.frame_mut());

                if let Err(err) = pixels.render() {
                    error!("pixels.render() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                }

                window.request_redraw();
            }
            _ => (),
        }
    });

    Ok(())
}
