use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AIState {
    Idle,
    Patrol,
    Chase,
    Attack,
    Retreat,
}

pub struct Enemy {
    pub position: Vec2,
    pub direction: Vec2,
    pub state: AIState,
    pub health: i32,
    pub speed: f32,
}

impl Enemy {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            direction: Vec2::new(1.0, 0.0),
            state: AIState::Idle,
            health: 100,
            speed: 2.0,
        }
    }

    // Placeholder methods to be implemented in Phase 6
    pub fn update(&mut self, _player_pos: Vec2, _dt: f32) {
        // Will implement AI behavior here
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.health = (self.health - amount).max(0);
        if self.health < 30 {
            self.state = AIState::Retreat;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }
}

// Will implement pathfinding in Phase 6
pub mod path {
    use glam::Vec2;

    pub fn find_path(_start: Vec2, _end: Vec2, _map: &[Vec<i32>]) -> Vec<Vec2> {
        Vec::new() // Placeholder
    }
}
