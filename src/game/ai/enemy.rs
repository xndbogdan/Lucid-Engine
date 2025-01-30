use crate::engine::texture::Texture;
use glam::Vec2;
use std::time::Instant;

#[derive(Clone, Debug)]
pub enum AIState {
    Idle,
    Patrol,
    Chase,
    Attack,
    Retreat,
}

pub enum EnemyType {
    Melee {
        damage: i32,
        attack_range: f32,
    },
    Ranged {
        damage: i32,
        attack_range: f32,
        projectile_speed: f32,
        last_shot: Instant,
        shoot_interval: f32,
        idle_texture: Texture,
        fire_texture: Texture,
        firing: bool,
    },
}

pub struct Enemy {
    pub position: Vec2,
    pub direction: Vec2,
    pub health: i32,
    pub state: AIState,
    pub texture: Texture,
    pub speed: f32,
    pub chase_range: f32,
    enemy_type: EnemyType,
    pub patrol_points: Vec<Vec2>,
    current_patrol_point: usize,
}

impl Enemy {
    pub fn new_melee(position: Vec2, texture: Texture) -> Self {
        Self {
            position,
            direction: Vec2::new(1.0, 0.0),
            health: 100,
            state: AIState::Idle,
            texture,
            speed: 2.0,
            chase_range: 5.0,
            enemy_type: EnemyType::Melee {
                damage: 10,
                attack_range: 1.0,
            },
            patrol_points: vec![],
            current_patrol_point: 0,
        }
    }

    pub fn new_ranged(position: Vec2, idle_texture: Texture, fire_texture: Texture) -> Self {
        Self {
            position,
            direction: Vec2::new(1.0, 0.0),
            health: 50,
            state: AIState::Idle,
            texture: idle_texture.clone(),
            speed: 2.0,
            chase_range: 10.0,
            enemy_type: EnemyType::Ranged {
                damage: 5,
                attack_range: 8.0,
                projectile_speed: 8.0,
                last_shot: Instant::now(),
                shoot_interval: 1.0,
                idle_texture,
                fire_texture,
                firing: false,
            },
            patrol_points: vec![],
            current_patrol_point: 0,
        }
    }

    pub fn set_patrol_points(&mut self, points: Vec<Vec2>) {
        self.patrol_points = points;
        self.state = AIState::Patrol;
    }

    fn can_see_player(&self, player_pos: Vec2, map: &[Vec<i32>]) -> bool {
        let to_player = player_pos - self.position;
        let distance = to_player.length();

        // Check if player is within range
        if distance > self.chase_range {
            return false;
        }

        // Ray cast to check line of sight
        let direction = to_player.normalize();
        let steps = (distance * 2.0) as usize;
        let step_size = distance / steps as f32;

        for i in 0..=steps {
            let t = i as f32 * step_size;
            let point = self.position + direction * t;

            let x = point.x.floor() as usize;
            let y = point.y.floor() as usize;

            // Check if we hit a wall before reaching the player
            if x >= map[0].len() || y >= map.len() || map[y][x] != 0 {
                return false;
            }

            // Check if we reached the player
            if (point - player_pos).length() < 0.5 {
                return true;
            }
        }

        false
    }

    pub fn update(
        &mut self,
        player_pos: Vec2,
        dt: f32,
        map: &[Vec<i32>],
    ) -> Option<(Vec2, Vec2, i32)> {
        let mut should_shoot = None;

        // Update direction to face player or patrol point
        let target_pos = match self.state {
            AIState::Chase | AIState::Attack => player_pos,
            AIState::Patrol if !self.patrol_points.is_empty() => {
                self.patrol_points[self.current_patrol_point]
            }
            _ => self.position + self.direction,
        };
        self.direction = (target_pos - self.position).normalize();

        match self.state {
            AIState::Idle => {
                // Check if player is visible
                if self.can_see_player(player_pos, map) {
                    self.state = AIState::Chase;
                }
            }
            AIState::Patrol => {
                if !self.patrol_points.is_empty() {
                    let target = self.patrol_points[self.current_patrol_point];
                    let to_target = target - self.position;

                    if to_target.length() < 0.1 {
                        // Move to next patrol point
                        self.current_patrol_point =
                            (self.current_patrol_point + 1) % self.patrol_points.len();
                    } else {
                        // Move towards patrol point
                        self.move_towards(target, dt, map);
                    }
                }

                // Check if player is visible
                if self.can_see_player(player_pos, map) {
                    self.state = AIState::Chase;
                }
            }
            AIState::Chase => {
                let to_player = player_pos - self.position;
                let distance = to_player.length();

                if !self.can_see_player(player_pos, map) {
                    self.state = AIState::Patrol;
                } else {
                    match &mut self.enemy_type {
                        EnemyType::Melee { attack_range, .. } => {
                            if distance < *attack_range {
                                self.state = AIState::Attack;
                            } else {
                                // Move towards player
                                self.move_towards(player_pos, dt, map);
                            }
                        }
                        EnemyType::Ranged { attack_range, .. } => {
                            if distance < *attack_range {
                                self.state = AIState::Attack;
                            } else {
                                // Move to maintain optimal range
                                let optimal_range = *attack_range * 0.8;
                                if distance < optimal_range {
                                    // Move away from player
                                    let away =
                                        self.position + to_player.normalize() * -self.speed * dt;
                                    if !self.check_collision(away, map) {
                                        self.position = away;
                                    }
                                } else {
                                    // Move towards player
                                    self.move_towards(player_pos, dt, map);
                                }
                            }
                        }
                    }
                }
            }
            AIState::Attack => {
                let to_player = player_pos - self.position;
                let distance = to_player.length();

                if !self.can_see_player(player_pos, map) {
                    self.state = AIState::Chase;
                } else {
                    match &mut self.enemy_type {
                        EnemyType::Melee { attack_range, .. } => {
                            if distance > *attack_range * 1.2 {
                                self.state = AIState::Chase;
                            }
                        }
                        EnemyType::Ranged {
                            attack_range,
                            damage,
                            projectile_speed,
                            last_shot,
                            shoot_interval,
                            idle_texture,
                            fire_texture,
                            firing,
                        } => {
                            if distance > *attack_range * 1.2 {
                                self.state = AIState::Chase;
                                *firing = false;
                                self.texture = idle_texture.clone();
                            } else {
                                // Try to shoot
                                let now = Instant::now();
                                if now.duration_since(*last_shot).as_secs_f32() >= *shoot_interval {
                                    *last_shot = now;
                                    *firing = true;
                                    self.texture = fire_texture.clone();

                                    // Calculate projectile velocity
                                    let direction = to_player.normalize();
                                    should_shoot = Some((
                                        self.position,
                                        direction * *projectile_speed,
                                        *damage,
                                    ));
                                } else if *firing
                                    && now.duration_since(*last_shot).as_secs_f32() >= 0.1
                                {
                                    // Reset firing animation
                                    *firing = false;
                                    self.texture = idle_texture.clone();
                                }
                            }
                        }
                    }
                }
            }
            AIState::Retreat => {
                if self.health > 50 {
                    self.state = AIState::Chase;
                } else {
                    // Move away from player
                    let away_from_player = self.position - player_pos;
                    if away_from_player.length() > 0.0 {
                        let retreat_pos = self.position + away_from_player.normalize() * 5.0;
                        self.move_towards(retreat_pos, dt, map);
                    }
                }
            }
        }

        should_shoot
    }

    fn move_towards(&mut self, target: Vec2, dt: f32, map: &[Vec<i32>]) {
        let to_target = target - self.position;
        if to_target.length() > 0.0 {
            self.direction = to_target.normalize();
            let new_pos = self.position + self.direction * self.speed * dt;

            if !self.check_collision(new_pos, map) {
                self.position = new_pos;
            }
        }
    }

    fn check_collision(&self, pos: Vec2, map: &[Vec<i32>]) -> bool {
        let map_x = pos.x.floor() as usize;
        let map_y = pos.y.floor() as usize;

        map_x >= map[0].len() || map_y >= map.len() || map[map_y][map_x] != 0
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.health -= amount;
        if self.health < 40 {
            self.state = AIState::Retreat;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn get_damage(&self) -> i32 {
        match &self.enemy_type {
            EnemyType::Melee { damage, .. } => *damage,
            EnemyType::Ranged { damage, .. } => *damage,
        }
    }

    pub fn get_attack_range(&self) -> f32 {
        match &self.enemy_type {
            EnemyType::Melee { attack_range, .. } => *attack_range,
            EnemyType::Ranged { attack_range, .. } => *attack_range,
        }
    }
}
