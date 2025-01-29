use glam::Vec2;

pub struct World {
    pub map: Vec<Vec<i32>>,
    pub width: usize,
    pub height: usize,
}

impl World {
    pub fn new(width: usize, height: usize) -> Self {
        // Create an empty map filled with walls around the edges
        let mut map = vec![vec![0; width]; height];

        // Add walls around the edges
        for x in 0..width {
            map[0][x] = 1;
            map[height - 1][x] = 1;
        }
        for y in 0..height {
            map[y][0] = 1;
            map[y][width - 1] = 1;
        }

        Self { map, width, height }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<i32> {
        self.map.get(y).and_then(|row| row.get(x)).copied()
    }

    pub fn set_tile(&mut self, x: usize, y: usize, value: i32) -> bool {
        if x < self.width && y < self.height {
            self.map[y][x] = value;
            true
        } else {
            false
        }
    }

    pub fn is_solid(&self, x: usize, y: usize) -> bool {
        self.get_tile(x, y).map_or(true, |tile| tile > 0)
    }

    pub fn check_collision(&self, pos: Vec2) -> bool {
        let x = pos.x as usize;
        let y = pos.y as usize;
        self.is_solid(x, y)
    }

    // Create a simple test map
    pub fn create_test_map() -> Self {
        let mut world = Self::new(10, 10);

        // Add some walls
        world.set_tile(3, 3, 1);
        world.set_tile(3, 4, 1);
        world.set_tile(3, 5, 1);
        world.set_tile(6, 4, 1);
        world.set_tile(6, 5, 1);
        world.set_tile(6, 6, 1);

        world
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_creation() {
        let world = World::new(5, 5);
        assert_eq!(world.width, 5);
        assert_eq!(world.height, 5);
        assert!(world.is_solid(0, 0));
        assert!(!world.is_solid(2, 2));
    }

    #[test]
    fn test_collision_detection() {
        let world = World::create_test_map();
        assert!(world.check_collision(Vec2::new(0.0, 0.0)));
        assert!(!world.check_collision(Vec2::new(1.0, 1.0)));
        assert!(world.check_collision(Vec2::new(3.0, 3.0)));
    }
}
