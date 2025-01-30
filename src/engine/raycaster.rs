use crate::engine::camera::Camera;
use crate::engine::texture::TextureCache;
use crate::game::ai::Enemy;
use crate::game::Particle;
use glam::Vec2;

pub struct Raycaster {
    width: u32,
    height: u32,
    z_buffer: Vec<f32>,
    map: Vec<Vec<i32>>,
    texture_cache: TextureCache,
}

impl Raycaster {
    pub fn new(width: u32, height: u32) -> Self {
        // Initialize with a default map
        let map = vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1],
        ];

        Self {
            width,
            height,
            z_buffer: vec![0.0; width as usize],
            map,
            texture_cache: TextureCache::new(),
        }
    }

    pub fn set_map(&mut self, map: Vec<Vec<i32>>) {
        self.map = map;
    }

    pub fn load_texture<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<usize, String> {
        self.texture_cache.load_texture(path)
    }

    pub fn render(
        &mut self,
        camera: &Camera,
        enemies: &[Enemy],
        particles: &[Particle],
        frame: &mut [u8],
    ) {
        // Clear frame with ceiling and floor colors
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = ((y * self.width + x) * 4) as usize;
                if y < self.height / 2 {
                    // Ceiling (dark gray)
                    frame[idx] = 0x40; // R
                    frame[idx + 1] = 0x40; // G
                    frame[idx + 2] = 0x40; // B
                    frame[idx + 3] = 0xff; // A
                } else {
                    // Floor (light gray)
                    frame[idx] = 0x80; // R
                    frame[idx + 1] = 0x80; // G
                    frame[idx + 2] = 0x80; // B
                    frame[idx + 3] = 0xff; // A
                }
            }
        }

        // Cast rays for walls
        for x in 0..self.width {
            let camera_x = 2.0 * x as f32 / self.width as f32 - 1.0;
            let ray_dir = Vec2::new(
                camera.direction.x + camera.plane.x * camera_x,
                camera.direction.y + camera.plane.y * camera_x,
            );

            let mut map_pos = Vec2::new(camera.position.x.floor(), camera.position.y.floor());
            let delta_dist = Vec2::new((1.0 / ray_dir.x).abs(), (1.0 / ray_dir.y).abs());

            let step = Vec2::new(
                if ray_dir.x < 0.0 { -1.0 } else { 1.0 },
                if ray_dir.y < 0.0 { -1.0 } else { 1.0 },
            );

            let mut side_dist = Vec2::new(
                if ray_dir.x < 0.0 {
                    (camera.position.x - map_pos.x) * delta_dist.x
                } else {
                    (map_pos.x + 1.0 - camera.position.x) * delta_dist.x
                },
                if ray_dir.y < 0.0 {
                    (camera.position.y - map_pos.y) * delta_dist.y
                } else {
                    (map_pos.y + 1.0 - camera.position.y) * delta_dist.y
                },
            );

            // DDA Algorithm
            let mut hit = false;
            let mut side = 0; // 0 for x-side, 1 for y-side

            while !hit {
                // Jump to next square
                if side_dist.x < side_dist.y {
                    side_dist.x += delta_dist.x;
                    map_pos.x += step.x;
                    side = 0;
                } else {
                    side_dist.y += delta_dist.y;
                    map_pos.y += step.y;
                    side = 1;
                }

                // Check if ray has hit a wall
                if map_pos.x >= 0.0
                    && map_pos.x < self.map[0].len() as f32
                    && map_pos.y >= 0.0
                    && map_pos.y < self.map.len() as f32
                {
                    if self.map[map_pos.y as usize][map_pos.x as usize] > 0 {
                        hit = true;
                    }
                }
            }

            // Calculate distance to wall
            let perp_wall_dist = if side == 0 {
                side_dist.x - delta_dist.x
            } else {
                side_dist.y - delta_dist.y
            };

            self.z_buffer[x as usize] = perp_wall_dist;

            // Calculate wall height
            let line_height = (self.height as f32 / perp_wall_dist) as i32;

            // Calculate drawing bounds
            let mut draw_start = -line_height / 2 + self.height as i32 / 2;
            if draw_start < 0 {
                draw_start = 0;
            }
            let mut draw_end = line_height / 2 + self.height as i32 / 2;
            if draw_end >= self.height as i32 {
                draw_end = self.height as i32 - 1;
            }

            // Calculate texture coordinates
            let wall_x = if side == 0 {
                camera.position.y + perp_wall_dist * ray_dir.y
            } else {
                camera.position.x + perp_wall_dist * ray_dir.x
            };
            let wall_x = wall_x - wall_x.floor();

            // Get the texture for this wall
            let tex_num = (self.map[map_pos.y as usize][map_pos.x as usize] - 1) as usize;
            if let Some(texture) = self.texture_cache.get_texture(tex_num) {
                // Draw the textured wall
                for y in draw_start..draw_end {
                    let d = y * 256 - self.height as i32 * 128 + line_height * 128;
                    let tex_y = ((d * texture.height as i32) / line_height) / 256;

                    let color =
                        texture.get_pixel((wall_x * texture.width as f32) as u32, tex_y as u32);

                    // Convert color from u32 RGBA to bytes
                    let r = ((color >> 24) & 0xFF) as u8;
                    let g = ((color >> 16) & 0xFF) as u8;
                    let b = ((color >> 8) & 0xFF) as u8;
                    let a = (color & 0xFF) as u8;

                    // Apply shading based on distance and side
                    let shade = if side == 1 { 0.7 } else { 1.0 };
                    let distance_shade = (1.0 / (1.0 + perp_wall_dist * 0.1)).min(1.0);
                    let final_shade = shade * distance_shade;

                    let idx = ((y * self.width as i32 + x as i32) * 4) as usize;
                    frame[idx] = (r as f32 * final_shade) as u8; // R
                    frame[idx + 1] = (g as f32 * final_shade) as u8; // G
                    frame[idx + 2] = (b as f32 * final_shade) as u8; // B
                    frame[idx + 3] = a; // A
                }
            }
        }

        // Sort sprites by distance
        let mut sprite_distances: Vec<(usize, f32)> = enemies
            .iter()
            .enumerate()
            .map(|(i, enemy)| {
                let dx = enemy.position.x - camera.position.x;
                let dy = enemy.position.y - camera.position.y;
                (i, dx * dx + dy * dy)
            })
            .collect();
        sprite_distances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Draw sprites
        for (i, _) in sprite_distances {
            let enemy = &enemies[i];

            // Translate sprite position relative to camera
            let sprite_pos = enemy.position - camera.position;

            // Transform sprite with the inverse camera matrix
            let inv_det =
                1.0 / (camera.plane.x * camera.direction.y - camera.direction.x * camera.plane.y);
            let transform_x =
                inv_det * (camera.direction.y * sprite_pos.x - camera.direction.x * sprite_pos.y);
            let transform_y =
                inv_det * (-camera.plane.y * sprite_pos.x + camera.plane.x * sprite_pos.y);

            let sprite_screen_x =
                ((self.width as f32 / 2.0) * (1.0 + transform_x / transform_y)) as i32;

            // Calculate sprite dimensions on screen
            let sprite_height = (self.height as f32 / transform_y).abs() as i32;
            let sprite_width = sprite_height; // Assuming square sprites

            // Calculate drawing bounds
            let draw_start_y = -sprite_height / 2 + self.height as i32 / 2;
            let draw_end_y = sprite_height / 2 + self.height as i32 / 2;
            let draw_start_x = -sprite_width / 2 + sprite_screen_x;
            let draw_end_x = sprite_width / 2 + sprite_screen_x;

            // Draw the sprite
            for stripe in draw_start_x..draw_end_x {
                if stripe < 0 || stripe >= self.width as i32 {
                    continue;
                }

                if transform_y > 0.0 && transform_y < self.z_buffer[stripe as usize] {
                    for y in draw_start_y..draw_end_y {
                        if y < 0 || y >= self.height as i32 {
                            continue;
                        }

                        let tex_x = ((stripe - (-sprite_width / 2 + sprite_screen_x))
                            * enemy.texture.width as i32
                            / sprite_width) as u32;
                        let tex_y = ((y - draw_start_y) * enemy.texture.height as i32
                            / sprite_height) as u32;

                        let color = enemy.texture.get_pixel(tex_x, tex_y);
                        let alpha = (color & 0xFF) as u8;

                        if alpha > 0 {
                            let idx = ((y * self.width as i32 + stripe) * 4) as usize;
                            if idx + 3 < frame.len() {
                                let r = ((color >> 24) & 0xFF) as u8;
                                let g = ((color >> 16) & 0xFF) as u8;
                                let b = ((color >> 8) & 0xFF) as u8;

                                // Apply distance-based shading
                                let distance_shade = (1.0 / (1.0 + transform_y * 0.1)).min(1.0);
                                frame[idx] = (r as f32 * distance_shade) as u8; // R
                                frame[idx + 1] = (g as f32 * distance_shade) as u8; // G
                                frame[idx + 2] = (b as f32 * distance_shade) as u8; // B
                                frame[idx + 3] = alpha; // A
                            }
                        }
                    }
                }
            }
        }

        // Draw particles
        for particle in particles {
            // Translate particle position relative to camera
            let particle_pos = particle.position - camera.position;

            // Transform particle with the inverse camera matrix
            let inv_det =
                1.0 / (camera.plane.x * camera.direction.y - camera.direction.x * camera.plane.y);
            let transform_x = inv_det
                * (camera.direction.y * particle_pos.x - camera.direction.x * particle_pos.y);
            let transform_y =
                inv_det * (-camera.plane.y * particle_pos.x + camera.plane.x * particle_pos.y);

            let particle_screen_x =
                ((self.width as f32 / 2.0) * (1.0 + transform_x / transform_y)) as i32;

            // Calculate particle dimensions on screen
            let particle_size = (self.height as f32 / transform_y * 0.1).abs() as i32; // Smaller than sprites

            // Calculate drawing bounds
            let draw_start_y = -particle_size / 2 + self.height as i32 / 2;
            let draw_end_y = particle_size / 2 + self.height as i32 / 2;
            let draw_start_x = -particle_size / 2 + particle_screen_x;
            let draw_end_x = particle_size / 2 + particle_screen_x;

            // Draw the particle
            if transform_y > 0.0 {
                for stripe in draw_start_x..draw_end_x {
                    if stripe < 0 || stripe >= self.width as i32 {
                        continue;
                    }

                    if transform_y < self.z_buffer[stripe as usize] {
                        for y in draw_start_y..draw_end_y {
                            if y < 0 || y >= self.height as i32 {
                                continue;
                            }

                            let tex_x = ((stripe - (-particle_size / 2 + particle_screen_x))
                                * particle.texture.width as i32
                                / particle_size) as u32;
                            let tex_y = ((y - draw_start_y) * particle.texture.height as i32
                                / particle_size) as u32;

                            let color = particle.texture.get_pixel(tex_x, tex_y);
                            let alpha = (color & 0xFF) as u8;

                            if alpha > 0 {
                                let idx = ((y * self.width as i32 + stripe) * 4) as usize;
                                if idx + 3 < frame.len() {
                                    let r = ((color >> 24) & 0xFF) as u8;
                                    let g = ((color >> 16) & 0xFF) as u8;
                                    let b = ((color >> 8) & 0xFF) as u8;

                                    // Apply distance-based shading and glow effect
                                    let distance_shade =
                                        (1.0 / (1.0 + transform_y * 0.05)).min(1.0);
                                    frame[idx] = (r as f32 * distance_shade) as u8; // R
                                    frame[idx + 1] = (g as f32 * distance_shade) as u8; // G
                                    frame[idx + 2] = (b as f32 * distance_shade) as u8; // B
                                    frame[idx + 3] = alpha; // A
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
