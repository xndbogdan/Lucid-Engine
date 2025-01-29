use glam::Vec2;

pub struct Camera {
    pub position: Vec2,
    pub direction: Vec2,
    pub plane: Vec2,
}

impl Camera {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: Vec2::new(x, y),
            direction: Vec2::new(1.0, 0.0), // Looking along positive x-axis
            plane: Vec2::new(0.0, -0.66),   // FOV of about 66 degrees
        }
    }

    pub fn rotate(&mut self, angle: f32) {
        // Rotate direction vector
        let old_dir_x = self.direction.x;
        self.direction.x = self.direction.x * angle.cos() - self.direction.y * angle.sin();
        self.direction.y = old_dir_x * angle.sin() + self.direction.y * angle.cos();

        // Rotate camera plane
        let old_plane_x = self.plane.x;
        self.plane.x = self.plane.x * angle.cos() - self.plane.y * angle.sin();
        self.plane.y = old_plane_x * angle.sin() + self.plane.y * angle.cos();
    }

    pub fn move_forward(&mut self, speed: f32, map: &[Vec<i32>]) {
        let move_vec = self.direction * speed;
        self.move_with_collision(move_vec, map);
    }

    pub fn move_right(&mut self, speed: f32, map: &[Vec<i32>]) {
        // For right movement, rotate direction vector 90 degrees clockwise
        let right = Vec2::new(self.direction.y, -self.direction.x);
        let move_vec = right * speed;
        self.move_with_collision(move_vec, map);
    }

    fn move_with_collision(&mut self, move_vec: Vec2, map: &[Vec<i32>]) {
        const COLLISION_BUFFER: f32 = 0.3;
        let new_pos = self.position + move_vec;

        // Check if we're moving away from walls
        let moving_away_x = if move_vec.x > 0.0 {
            self.position.x < new_pos.x && self.is_wall_left(COLLISION_BUFFER, map)
        } else {
            self.position.x > new_pos.x && self.is_wall_right(COLLISION_BUFFER, map)
        };

        let moving_away_y = if move_vec.y > 0.0 {
            self.position.y < new_pos.y && self.is_wall_up(COLLISION_BUFFER, map)
        } else {
            self.position.y > new_pos.y && self.is_wall_down(COLLISION_BUFFER, map)
        };

        // Try to move in each direction independently
        let mut next_pos = self.position;

        // Update X position if we can move in that direction
        if !self.check_collision(Vec2::new(new_pos.x, self.position.y), map) || moving_away_x {
            next_pos.x = new_pos.x;
        }

        // Update Y position if we can move in that direction
        if !self.check_collision(Vec2::new(next_pos.x, new_pos.y), map) || moving_away_y {
            next_pos.y = new_pos.y;
        }

        self.position = next_pos;
    }

    fn check_collision(&self, pos: Vec2, map: &[Vec<i32>]) -> bool {
        const COLLISION_BUFFER: f32 = 0.3;

        // Check map boundaries
        if pos.x < COLLISION_BUFFER
            || pos.y < COLLISION_BUFFER
            || pos.x >= (map[0].len() as f32 - COLLISION_BUFFER)
            || pos.y >= (map.len() as f32 - COLLISION_BUFFER)
        {
            return true;
        }

        // Check collision points
        let check_positions = [
            Vec2::new(pos.x - COLLISION_BUFFER, pos.y - COLLISION_BUFFER),
            Vec2::new(pos.x - COLLISION_BUFFER, pos.y + COLLISION_BUFFER),
            Vec2::new(pos.x + COLLISION_BUFFER, pos.y - COLLISION_BUFFER),
            Vec2::new(pos.x + COLLISION_BUFFER, pos.y + COLLISION_BUFFER),
            Vec2::new(pos.x, pos.y), // Center point
        ];

        for check_pos in check_positions.iter() {
            let map_x = check_pos.x.floor() as usize;
            let map_y = check_pos.y.floor() as usize;

            if map[map_y][map_x] != 0 {
                return true;
            }
        }

        false
    }

    fn is_wall_left(&self, distance: f32, map: &[Vec<i32>]) -> bool {
        let check_pos = Vec2::new(self.position.x - distance, self.position.y);
        self.check_collision(check_pos, map)
    }

    fn is_wall_right(&self, distance: f32, map: &[Vec<i32>]) -> bool {
        let check_pos = Vec2::new(self.position.x + distance, self.position.y);
        self.check_collision(check_pos, map)
    }

    fn is_wall_up(&self, distance: f32, map: &[Vec<i32>]) -> bool {
        let check_pos = Vec2::new(self.position.x, self.position.y - distance);
        self.check_collision(check_pos, map)
    }

    fn is_wall_down(&self, distance: f32, map: &[Vec<i32>]) -> bool {
        let check_pos = Vec2::new(self.position.x, self.position.y + distance);
        self.check_collision(check_pos, map)
    }
}
