use crate::engine::camera::Camera;
use crate::engine::raycaster::Raycaster;
use crate::game::ai::Enemy;

pub struct GameState {
    pub camera: Camera,
    pub raycaster: Raycaster,
    pub paused: bool,
}

impl GameState {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            camera: Camera::new(1.5, 1.5),
            raycaster: Raycaster::new(width, height),
            paused: false,
        }
    }

    pub fn update(&mut self, dt: f64) {
        if !self.paused {
            // Game logic updates will be added here
        }
    }

    pub fn render(
        &mut self,
        frame: &mut [u8],
        enemies: &[Enemy],
        particles: &[crate::game::Particle],
    ) {
        // Clear frame
        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = 0x40; // R
            pixel[1] = 0x40; // G
            pixel[2] = 0x40; // B
            pixel[3] = 0xFF; // A
        }

        // Render world, enemies, and particles
        self.raycaster
            .render(&self.camera, enemies, particles, frame);
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }
}

#[derive(Debug)]
pub enum GameScreen {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}
