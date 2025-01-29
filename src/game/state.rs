use crate::engine::{Camera, Raycaster};

pub struct GameState {
    pub camera: Camera,
    pub raycaster: Raycaster,
    pub paused: bool,
    pub fps: f64,
    pub frame_time: f64,
    pub map: Vec<Vec<i32>>,
}

impl GameState {
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
            camera: Camera::new(2.5, 2.5),
            raycaster,
            paused: false,
            fps: 0.0,
            frame_time: 0.0,
            map,
        }
    }

    pub fn update(&mut self, dt: f64) {
        if !self.paused {
            // Will implement game logic updates
        }

        // Update FPS counter
        self.frame_time = dt;
        self.fps = 1.0 / dt;
    }

    pub fn render(&mut self, frame: &mut [u8]) {
        self.raycaster.render(&self.camera, frame);
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }
}

pub enum GameScreen {
    MainMenu,
    Playing,
    Paused,
    GameOver,
}

impl Default for GameScreen {
    fn default() -> Self {
        Self::MainMenu
    }
}
