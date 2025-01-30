use crate::engine::texture::Texture;
use glam::Vec2;
use std::time::Instant;

pub struct Weapon {
    idle_texture: Texture,
    fire_texture: Texture,
    firing: bool,
    last_shot: Instant,
    bob_offset: f32,
    bob_time: f32,
}

impl Weapon {
    pub fn new(idle_texture: Texture, fire_texture: Texture) -> Self {
        Self {
            idle_texture,
            fire_texture,
            firing: false,
            last_shot: Instant::now(),
            bob_offset: 0.0,
            bob_time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, moving: bool) {
        // Update weapon bob when moving
        if moving {
            self.bob_time += dt * 5.0;
            self.bob_offset = (self.bob_time.sin() * 5.0).abs();
        } else {
            self.bob_time = 0.0;
            self.bob_offset = 0.0;
        }

        // Reset firing state if enough time has passed
        if self.firing && self.last_shot.elapsed().as_secs_f32() > 0.1 {
            self.firing = false;
        }
    }

    pub fn render(&self, frame: &mut [u8], width: u32, height: u32) {
        let texture = if self.firing {
            &self.fire_texture
        } else {
            &self.idle_texture
        };

        // Calculate weapon position (centered horizontally, bottom of screen)
        let weapon_width = width / 3; // Make weapon 1/3 of screen width
        let weapon_height = (weapon_width * texture.height / texture.width) as u32;
        let x_offset = (width - weapon_width) / 2;
        let y_offset = height - weapon_height + self.bob_offset as u32;

        // Scale texture to weapon size and blend with background
        for y in 0..weapon_height {
            let frame_y = y_offset + y;
            if frame_y >= height {
                continue;
            }

            for x in 0..weapon_width {
                let frame_x = x_offset + x;
                if frame_x >= width {
                    continue;
                }

                let tex_x = (x * texture.width) / weapon_width;
                let tex_y = (y * texture.height) / weapon_height;
                let color = texture.get_pixel(tex_x, tex_y);

                // Only draw non-transparent pixels (alpha > 0)
                let alpha = (color & 0xFF) as u8;
                if alpha > 0 {
                    let idx = ((frame_y * width + frame_x) * 4) as usize;
                    if idx + 3 < frame.len() {
                        // Extract color components
                        let r = ((color >> 24) & 0xFF) as u8;
                        let g = ((color >> 16) & 0xFF) as u8;
                        let b = ((color >> 8) & 0xFF) as u8;

                        // Alpha blending
                        if alpha == 255 {
                            // Fully opaque
                            frame[idx] = r;
                            frame[idx + 1] = g;
                            frame[idx + 2] = b;
                            frame[idx + 3] = alpha;
                        } else {
                            // Semi-transparent
                            let alpha_f = alpha as f32 / 255.0;
                            let bg_r = frame[idx];
                            let bg_g = frame[idx + 1];
                            let bg_b = frame[idx + 2];

                            frame[idx] = (r as f32 * alpha_f + bg_r as f32 * (1.0 - alpha_f)) as u8;
                            frame[idx + 1] =
                                (g as f32 * alpha_f + bg_g as f32 * (1.0 - alpha_f)) as u8;
                            frame[idx + 2] =
                                (b as f32 * alpha_f + bg_b as f32 * (1.0 - alpha_f)) as u8;
                            frame[idx + 3] = 255; // Keep background fully opaque
                        }
                    }
                }
            }
        }
    }

    pub fn fire(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_shot).as_secs_f32() >= 0.5 {
            self.firing = true;
            self.last_shot = now;
            true
        } else {
            false
        }
    }
}
