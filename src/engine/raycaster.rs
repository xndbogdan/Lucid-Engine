use crate::engine::camera::Camera;
use glam::Vec2;

pub struct Raycaster {
    width: u32,
    height: u32,
    z_buffer: Vec<f32>,
    map: Vec<Vec<i32>>,
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
        }
    }

    pub fn set_map(&mut self, map: Vec<Vec<i32>>) {
        self.map = map;
    }

    pub fn render(&mut self, camera: &Camera, frame: &mut [u8]) {
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

        // Cast rays
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
                    // Use map coordinates directly without swapping
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

            // Draw the walls
            let wall_color = if side == 1 { 0xCC } else { 0xFF };
            for y in draw_start..draw_end {
                let idx = ((y * self.width as i32 + x as i32) * 4) as usize;
                frame[idx] = wall_color; // R
                frame[idx + 1] = wall_color; // G
                frame[idx + 2] = wall_color; // B
                frame[idx + 3] = 0xFF; // A
            }
        }
    }
}
