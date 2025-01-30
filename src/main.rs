mod audio;
mod engine;
mod game;

use anyhow::Result;
use audio::sound::{MusicPlayer, SoundManager};
use engine::{Camera, Raycaster};
use game::{maps::MapFile, world::World, Enemy, Game, Particle, Weapon};
use glam::Vec2;
use log::{error, info};
use pixels::{Pixels, SurfaceTexture};
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{DeviceEvent, ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{CursorGrabMode, WindowBuilder};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const MOVE_SPEED: f32 = 2.5; // Units per second
const MOUSE_SENSITIVITY: f32 = 0.002; // Slightly reduced for smoother control
const FOOTSTEP_INTERVAL: f32 = 0.5; // Time between footstep sounds

struct GameState {
    camera: Camera,
    raycaster: Raycaster,
    last_update: Instant,
    last_footstep: Instant,
    move_forward: bool,
    move_backward: bool,
    move_left: bool,
    move_right: bool,
    game_focused: bool,
    world: World,
    sound_manager: SoundManager,
    music_player: MusicPlayer,
    game: Game,
    enemies: Vec<Enemy>,
}

impl GameState {
    fn new(stream_handle: rodio::OutputStreamHandle) -> Result<Self> {
        info!("Loading map from assets/maps/test.toml");
        let map_file = MapFile::load("assets/maps/test.toml")?;
        let (world, enemy_spawns) = World::load_from_map(&map_file)?;

        let mut raycaster = Raycaster::new(WIDTH, HEIGHT);
        raycaster.set_map(world.map.clone());

        // Load wall textures
        let texture_paths = [
            "assets/textures/walls/brick.png",
            "assets/textures/walls/greystone.png",
            "assets/textures/walls/redbrick.png",
            "assets/textures/walls/stone.png",
        ];

        for path in texture_paths.iter() {
            if let Err(e) = raycaster.load_texture(path) {
                error!("Failed to load texture {}: {}", path, e);
            }
        }

        // Initialize audio
        let mut sound_manager = SoundManager::new(stream_handle.clone());
        let mut music_player = MusicPlayer::new(stream_handle);

        // Register sound effects
        sound_manager.register_sound("step", "assets/audio/effects/step.wav");
        sound_manager.register_sound("gun1", "assets/audio/weapons/gun1.wav");
        sound_manager.register_sound("gun2", "assets/audio/weapons/gun2.wav");

        // Start background music
        if let Err(e) = music_player.play_music("assets/audio/music/track0.wav") {
            error!("Failed to play background music: {}", e);
        }

        // Initialize game state
        let mut game = Game::new(WIDTH, HEIGHT);

        // Load weapon
        if let Ok(idle_texture) =
            engine::texture::Texture::load("assets/textures/weapons/gun1/idle.png")
        {
            if let Ok(fire_texture) =
                engine::texture::Texture::load("assets/textures/weapons/gun1/fire.png")
            {
                game.weapon = Some(Weapon::new(idle_texture, fire_texture));
            }
        }

        // Create enemies from map data
        let mut enemies = Vec::new();
        info!("Creating {} enemies from map data", enemy_spawns.len());

        for (spawn_pos, patrol_points) in enemy_spawns {
            if let Ok(idle_texture) =
                engine::texture::Texture::load("assets/textures/weapons/gun2/idle.png")
            {
                if let Ok(fire_texture) =
                    engine::texture::Texture::load("assets/textures/weapons/gun2/fire.png")
                {
                    info!("Creating ranged enemy at {:?}", spawn_pos);
                    let mut enemy = Enemy::new_ranged(spawn_pos, idle_texture, fire_texture);
                    info!("Setting patrol points: {:?}", patrol_points);
                    enemy.set_patrol_points(patrol_points);
                    enemies.push(enemy);
                } else {
                    error!("Failed to load enemy fire texture");
                }
            } else {
                error!("Failed to load enemy idle texture");
            }
        }

        Ok(Self {
            camera: Camera::new(world.spawn_point.x, world.spawn_point.y),
            raycaster,
            last_update: Instant::now(),
            last_footstep: Instant::now(),
            move_forward: false,
            move_backward: false,
            move_left: false,
            move_right: false,
            game_focused: true,
            world,
            sound_manager,
            music_player,
            game,
            enemies,
        })
    }

    fn update(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        if self.game_focused {
            let mut is_moving = false;

            // Update camera position based on movement flags
            if self.move_forward {
                self.camera.move_forward(MOVE_SPEED * dt, &self.world.map);
                is_moving = true;
            }
            if self.move_backward {
                self.camera.move_forward(-MOVE_SPEED * dt, &self.world.map);
                is_moving = true;
            }
            if self.move_left {
                self.camera.move_right(-MOVE_SPEED * dt, &self.world.map);
                is_moving = true;
            }
            if self.move_right {
                self.camera.move_right(MOVE_SPEED * dt, &self.world.map);
                is_moving = true;
            }

            // Play footstep sound if moving
            if is_moving
                && now.duration_since(self.last_footstep).as_secs_f32() >= FOOTSTEP_INTERVAL
            {
                if let Err(e) = self.sound_manager.play_sound("step") {
                    error!("Failed to play footstep sound: {}", e);
                }
                self.last_footstep = now;
            }

            // Update weapon
            if let Some(weapon) = &mut self.game.weapon {
                weapon.update(dt, is_moving);
            }

            // Update enemies and handle their projectiles
            let mut i = 0;
            while i < self.enemies.len() {
                if let Some((pos, vel, damage)) =
                    self.enemies[i].update(self.camera.position, dt, &self.world.map)
                {
                    // Enemy wants to shoot
                    if let Ok(projectile_texture) =
                        engine::texture::Texture::load("assets/textures/particles/purple.png")
                    {
                        self.game.particles.add_particle(Particle::new(
                            pos,
                            vel,
                            projectile_texture,
                            damage,
                            true,
                        ));
                        if let Err(e) = self.sound_manager.play_sound("gun2") {
                            error!("Failed to play enemy gun sound: {}", e);
                        }
                    }
                }

                // Remove dead enemies
                if !self.enemies[i].is_alive() {
                    self.enemies.swap_remove(i);
                } else {
                    i += 1;
                }
            }

            // Update particles and check collisions
            self.game.particles.update(dt, &self.world.map);

            // Collect particle effects
            let mut player_damage = 0;
            let mut enemy_damages = Vec::new();

            // Check particle collisions
            for particle in self.game.particles.get_particles() {
                if particle.from_enemy {
                    // Check if particle hits player
                    let to_player = self.camera.position - particle.position;
                    if to_player.length() < 0.5 {
                        player_damage += particle.damage;
                    }
                } else {
                    // Check if particle hits enemies
                    for (i, enemy) in self.enemies.iter().enumerate() {
                        let to_enemy = enemy.position - particle.position;
                        if to_enemy.length() < 0.5 {
                            enemy_damages.push((i, particle.damage));
                        }
                    }
                }
            }

            // Apply collected damages
            if player_damage > 0 {
                self.game.take_damage(player_damage);
            }

            for (enemy_idx, damage) in enemy_damages {
                if let Some(enemy) = self.enemies.get_mut(enemy_idx) {
                    enemy.take_damage(damage);
                }
            }

            // Update game state
            self.game.update(dt, &self.world.map);
        }
    }

    fn render(&mut self, frame: &mut [u8]) {
        // Clear frame
        for pixel in frame.chunks_exact_mut(4) {
            pixel[0] = 0x40; // R
            pixel[1] = 0x40; // G
            pixel[2] = 0x40; // B
            pixel[3] = 0xFF; // A
        }

        // Get particles for rendering
        let particles = self.game.particles.get_particles();

        // Render world, enemies, and particles
        self.raycaster
            .render(&self.camera, &self.enemies, particles, frame);

        // Render weapon on top
        if let Some(weapon) = &self.game.weapon {
            weapon.render(frame, WIDTH, HEIGHT);
        }

        // Render health bar
        self.game.render(frame);
    }

    fn handle_key_event(&mut self, key_code: VirtualKeyCode, pressed: bool) -> Option<bool> {
        match key_code {
            VirtualKeyCode::W => {
                if self.game_focused {
                    self.move_forward = pressed;
                }
                None
            }
            VirtualKeyCode::S => {
                if self.game_focused {
                    self.move_backward = pressed;
                }
                None
            }
            VirtualKeyCode::A => {
                if self.game_focused {
                    self.move_left = pressed;
                }
                None
            }
            VirtualKeyCode::D => {
                if self.game_focused {
                    self.move_right = pressed;
                }
                None
            }
            VirtualKeyCode::Escape if pressed => {
                // Toggle game focus
                self.game_focused = !self.game_focused;
                // Reset movement flags when unfocusing
                if !self.game_focused {
                    self.move_forward = false;
                    self.move_backward = false;
                    self.move_left = false;
                    self.move_right = false;
                }
                Some(self.game_focused)
            }
            _ => None,
        }
    }

    fn handle_mouse_motion(&mut self, delta_x: f64) {
        if self.game_focused {
            self.camera.rotate(-delta_x as f32 * MOUSE_SENSITIVITY);
        }
    }

    fn handle_mouse_input(&mut self, button: MouseButton, pressed: bool) {
        if self.game_focused && button == MouseButton::Left && pressed {
            if let Some(weapon) = &mut self.game.weapon {
                if weapon.fire() {
                    if let Err(e) = self.sound_manager.play_sound("gun1") {
                        error!("Failed to play gun sound: {}", e);
                    }

                    // Create player projectile
                    if let Ok(projectile_texture) =
                        engine::texture::Texture::load("assets/textures/particles/purple.png")
                    {
                        self.game.particles.add_particle(Particle::new(
                            self.camera.position,
                            self.camera.direction * 10.0,
                            projectile_texture,
                            20,
                            false,
                        ));
                    }
                }
            }
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Lucid Raycaster")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    // Initialize audio
    let (_stream, stream_handle) = rodio::OutputStream::try_default()
        .map_err(|e| anyhow::anyhow!("Failed to initialize audio: {}", e))?;

    // Start with cursor hidden and captured
    window.set_cursor_visible(false);
    let _ = window.set_cursor_grab(CursorGrabMode::Confined);

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;
    let mut game = GameState::new(stream_handle)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::DeviceEvent {
                event: DeviceEvent::Key(input),
                ..
            } => {
                if let Some(keycode) = input.virtual_keycode {
                    let pressed = input.state == ElementState::Pressed;
                    if let Some(focused) = game.handle_key_event(keycode, pressed) {
                        window.set_cursor_visible(!focused);
                        let _ = window.set_cursor_grab(if focused {
                            CursorGrabMode::Confined
                        } else {
                            CursorGrabMode::None
                        });
                    }
                }
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                game.handle_mouse_input(button, state == ElementState::Pressed);
            }
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta: (x, _) },
                ..
            } => {
                game.handle_mouse_motion(x);
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    error!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::MainEventsCleared => {
                // Update game state
                game.update();

                // Draw frame
                game.render(pixels.frame_mut());

                if let Err(err) = pixels.render() {
                    error!("pixels.render() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                }

                window.request_redraw();
            }
            _ => (),
        }
    });

    Ok(())
}
