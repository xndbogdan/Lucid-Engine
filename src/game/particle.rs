use crate::engine::texture::Texture;
use glam::Vec2;

pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub lifetime: f32,
    pub texture: Texture,
    pub damage: i32,
    pub from_enemy: bool,
}

impl Particle {
    pub fn new(
        position: Vec2,
        velocity: Vec2,
        texture: Texture,
        damage: i32,
        from_enemy: bool,
    ) -> Self {
        Self {
            position,
            velocity,
            lifetime: 2.0, // 2 seconds lifetime
            texture,
            damage,
            from_enemy,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position += self.velocity * dt;
        self.lifetime -= dt;
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 0.0
    }
}

pub struct ParticleSystem {
    particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn add_particle(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn update(&mut self, dt: f32, map: &[Vec<i32>]) {
        self.particles.retain_mut(|particle| {
            particle.update(dt);

            // Check collision with walls
            let map_x = particle.position.x.floor() as usize;
            let map_y = particle.position.y.floor() as usize;

            if map_x >= map[0].len() || map_y >= map.len() || map[map_y][map_x] != 0 {
                return false; // Remove particle if it hits a wall
            }

            particle.is_alive()
        });
    }

    pub fn get_particles(&self) -> &[Particle] {
        &self.particles
    }

    pub fn clear(&mut self) {
        self.particles.clear();
    }
}
