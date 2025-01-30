use crate::engine::texture::Texture;

pub mod ai;
pub mod maps;
pub mod particle;
pub mod state;
pub mod weapon;
pub mod world;

pub use ai::enemy::Enemy;
pub use particle::{Particle, ParticleSystem};
pub use weapon::Weapon;

pub struct Game {
    pub width: u32,
    pub height: u32,
    pub weapon: Option<Weapon>,
    pub particles: ParticleSystem,
    pub player_health: i32,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            weapon: None,
            particles: ParticleSystem::new(),
            player_health: 100,
        }
    }

    pub fn load_weapon(&mut self, idle_path: &str, fire_path: &str) -> Result<(), String> {
        let idle_texture = Texture::load(idle_path)?;
        let fire_texture = Texture::load(fire_path)?;
        self.weapon = Some(Weapon::new(idle_texture, fire_texture));
        Ok(())
    }

    pub fn update(&mut self, dt: f32, map: &[Vec<i32>]) {
        // Update particles
        self.particles.update(dt, map);
    }

    pub fn render(&mut self, frame: &mut [u8]) {
        // Render health bar
        self.render_health_bar(frame);
    }

    fn render_health_bar(&self, frame: &mut [u8]) {
        const BAR_WIDTH: u32 = 200;
        const BAR_HEIGHT: u32 = 20;
        const BAR_PADDING: u32 = 10;

        let health_width = (BAR_WIDTH as f32 * (self.player_health as f32 / 100.0)) as u32;

        // Draw health bar background
        for y in 0..BAR_HEIGHT {
            for x in 0..BAR_WIDTH {
                let idx = ((y + BAR_PADDING) * self.width + (x + BAR_PADDING)) * 4;
                frame[idx as usize] = 0x40; // R
                frame[idx as usize + 1] = 0x40; // G
                frame[idx as usize + 2] = 0x40; // B
                frame[idx as usize + 3] = 0xFF; // A
            }
        }

        // Draw health bar fill
        for y in 0..BAR_HEIGHT {
            for x in 0..health_width {
                let idx = ((y + BAR_PADDING) * self.width + (x + BAR_PADDING)) * 4;
                frame[idx as usize] = 0xFF; // R
                frame[idx as usize + 1] = 0x40; // G
                frame[idx as usize + 2] = 0x40; // B
                frame[idx as usize + 3] = 0xFF; // A
            }
        }
    }

    pub fn handle_input(&mut self) {
        // Will be implemented later
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.player_health = (self.player_health - amount).max(0);
    }

    pub fn is_alive(&self) -> bool {
        self.player_health > 0
    }

    pub fn heal(&mut self, amount: i32) {
        self.player_health = (self.player_health + amount).min(100);
    }
}
