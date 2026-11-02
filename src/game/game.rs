use std::time::Instant;

pub struct Player {
    y: f32, // y position of the player
    vy: f32, // how fast y changes
}

pub struct Obstacle {
    x: f32,
    width: i32,
    height: i32,
}

pub enum State {
    Running,
    Dead { died_at: Instant },
}

pub struct Game {
    player: Player,
    obstacles: Vec<Obstacle>,
    state: State,
    speed: f32,
    spawn_timer: f32,
    score: f32,
}