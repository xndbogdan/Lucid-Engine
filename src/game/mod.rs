pub mod ai;
pub mod state;
pub mod world;

use crate::engine::{Camera, Raycaster};

/// Represents the game state and logic
pub struct Game {
    pub camera: Camera,
    pub raycaster: Raycaster,
    pub running: bool,
    pub map: Vec<Vec<i32>>,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        // Create a simple test map
        let map = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1],
        ];

        let mut raycaster = Raycaster::new(width, height);
        raycaster.set_map(map.clone());

        Self {
            camera: Camera::new(2.5, 2.5), // Start in the middle of the test map
            raycaster,
            running: true,
            map,
        }
    }

    pub fn update(&mut self) {
        // Will implement game logic updates
    }

    pub fn render(&mut self, frame: &mut [u8]) {
        self.raycaster.render(&self.camera, frame);
    }

    pub fn handle_input(&mut self) {
        // Will implement input handling
    }
}

// Game configuration
pub struct GameConfig {
    pub display: DisplayConfig,
    pub controls: ControlConfig,
}

pub struct DisplayConfig {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
    pub fov: f32,
}

pub struct ControlConfig {
    pub mouse_sensitivity: f32,
    pub invert_mouse: bool,
    pub movement_speed: f32,
}
