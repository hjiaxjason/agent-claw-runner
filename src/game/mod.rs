use std::time::Instant;
use rand::RngExt;

const MIN_GAP: f32 = 0.9;
const MAX_GAP: f32 = 2.0;

mod draw;

const GRAVITY: f32 = 160.0;
const START_SPEED: f32 = 30.0;
const FIRST_SPAWN: f32 = 1.5;
const JUMP_VELOCITY: f32 = 55.0;

const PLAYER_X: f32 = 5.0;      // fixed column the crab sits at
const PLAYER_W: f32 = 6.0;      // hitbox width (a bit narrower than the art)
const PLAYER_H: f32 = 2.0;      // hitbox height
pub struct Player {
    y: f32, // y position of the player
    vy: f32, // how fast y changes
}

#[derive(Clone, Copy)]
pub enum Kind { Kelp, Coral, Urchin, Clam }

pub struct Obstacle {
    x: f32,
    kind: Kind,
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
    width: f32,
    ground_offset: f32,
}

impl Kind {
    fn size(self) -> (i32, i32) {
        match self {
            Kind::Kelp | Kind::Coral => (3, 2),
            Kind::Urchin => (4, 2),
            Kind::Clam => (4, 1),
        }
    }
}

impl Game {
    pub fn new(width: u16) -> Self {
        Self {
            player: Player { y: 0.0, vy: 0.0 },
            obstacles: Vec::new(),
            state: State::Running,
            speed: START_SPEED,
            spawn_timer: FIRST_SPAWN,
            score: 0.0,
            width: width as f32,
            ground_offset: 0.0,
        }
        
    }

    pub fn jump(&mut self) {
        if self.player.y <= 0.0 {
            self.player.vy = JUMP_VELOCITY;
        }
    }

    pub fn update(&mut self, dt: f32) {
        let p = &mut self.player;
        p.vy -= GRAVITY * dt;
        p.y += p.vy * dt;
        if p.y <= 0.0 {
            p.y = 0.0;
            p.vy = 0.0;
        }
        self.ground_offset += self.speed * dt;

        self.spawn_timer -= dt;
        if self.spawn_timer <= 0.0 {
            let mut rng = rand::rng();
            let kind = match rng.random_range(0..4) {
                0 => Kind::Kelp,
                1 => Kind::Coral,
                2 => Kind::Urchin,
                _ => Kind::Clam,
            };
            self.obstacles.push(Obstacle {x : self.width, kind });
            self.spawn_timer = rng.random_range(MIN_GAP..MAX_GAP);

        }

        for o in &mut self.obstacles {
            o.x -= self.speed * dt;
        }
        self.obstacles.retain(|o| o.x + o.kind.size().0 as f32 > 0.0);
    }
}