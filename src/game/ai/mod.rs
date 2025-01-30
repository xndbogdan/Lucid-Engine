pub mod enemy;

pub use enemy::{AIState, Enemy, EnemyType};

use glam::Vec2;

/// Find a path between two points using simple line of sight
pub fn find_path(start: Vec2, end: Vec2, map: &[Vec<i32>]) -> Vec<Vec2> {
    // Simple direct path - will be improved later with proper pathfinding
    let direction = (end - start).normalize();
    let distance = (end - start).length();
    let steps = (distance * 2.0) as usize;
    let step_size = distance / steps as f32;

    let mut path = Vec::with_capacity(steps);
    for i in 0..=steps {
        let t = i as f32 * step_size;
        let point = start + direction * t;

        // Check if point is valid (not in a wall)
        let x = point.x.floor() as usize;
        let y = point.y.floor() as usize;
        if x < map[0].len() && y < map.len() && map[y][x] == 0 {
            path.push(point);
        } else {
            break; // Stop at first wall
        }
    }

    path
}
