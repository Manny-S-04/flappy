use std::collections::VecDeque;

pub const WIDTH: f32 = 480.0;
pub const HEIGHT: f32 = 640.0;
pub const GROUND_TOP: f32 = HEIGHT - 80.0;

pub const BIRD_X: f32 = 140.0;
pub const BIRD_R: f32 = 15.0;

pub const PIPE_W: f32 = 72.0;
pub const GAP: f32 = 170.0;
pub const SPEED: f32 = 170.0;

pub const STEP: f32 = 1.0 / 120.0;

const GRAVITY: f32 = 1500.0;
const FLAP_VEL: f32 = -480.0;
const MAX_FALL: f32 = 700.0;
const SPACING: f32 = 230.0;
const GAP_MARGIN: f32 = 60.0;

#[derive(Copy, Clone, Debug)]
pub struct Pipe {
    pub x: f32,
    pub gap_y: f32,
    scored: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Ready,
    Playing,
    Paused,
    Dead,
}

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

// xorshift64
#[derive(Copy, Clone, Debug)]
pub struct Rng(u64);

impl Rng {
    pub const fn new(seed: u64) -> Rng {
        if seed == 0 {
            return Rng(0x9E37_79B9_7F4A_7C15);
        }
        Rng(seed)
    }
}

fn next_f32(rng: &mut Rng) -> f32 {
    let mut x: u64 = rng.0;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    rng.0 = x;
    (x >> 40) as f32 / (1u64 << 24) as f32
}

#[derive(Clone, Debug)]
pub struct Game {
    pub bird_y: f32,
    pub vel: f32,
    pub pipes: VecDeque<Pipe>,
    pub score: u32,
    pub best: u32,
    pub status: Status,
    pub time: f32,
    pub scroll: f32,
    rng: Rng,
}

impl Game {
    pub fn new(rng: Rng) -> Game {
        let mut game: Game = Game {
            bird_y: 0.0,
            vel: 0.0,
            pipes: VecDeque::new(),
            score: 0,
            best: 0,
            status: Status::Ready,
            time: 0.0,
            scroll: 0.0,
            rng,
        };
        restart(&mut game);
        game
    }
}

pub fn restart(game: &mut Game) {
    game.bird_y = HEIGHT * 0.45;
    game.vel = 0.0;
    game.pipes.clear();
    game.score = 0;
    game.status = Status::Ready;
}

pub fn flap(game: &mut Game) {
    match game.status {
        Status::Ready => {
            game.status = Status::Playing;
            spawn_pipe(game, WIDTH + 120.0);
            game.vel = FLAP_VEL;
        }
        Status::Playing => game.vel = FLAP_VEL,
        Status::Dead => {
            if is_grounded(game) {
                restart(game);
            }
        }
        Status::Paused => {}
    }
}

pub fn toggle_pause(game: &mut Game) {
    match game.status {
        Status::Playing => game.status = Status::Paused,
        Status::Paused => game.status = Status::Playing,
        _ => {}
    }
}

pub fn is_grounded(game: &Game) -> bool {
    game.bird_y + BIRD_R >= GROUND_TOP
}

pub fn tilt(game: &Game) -> f32 {
    if game.status == Status::Ready {
        return 0.0;
    }
    (game.vel / 600.0).clamp(-0.45, 1.3)
}

pub fn step(game: &mut Game, dt: f32) {
    match game.status {
        Status::Paused => {}
        Status::Ready => {
            game.time += dt;
            game.scroll += SPEED * dt;
            game.bird_y = HEIGHT * 0.45 + (game.time * 4.0).sin() * 8.0;
        }
        Status::Playing => {
            game.time += dt;
            game.scroll += SPEED * dt;
            fall(game, dt);
            move_pipes(game, dt);
            if is_grounded(game) || hits_pipe(game) {
                game.status = Status::Dead;
                game.best = game.best.max(game.score);
            }
        }
        // world freezes, bird drops
        Status::Dead => fall(game, dt),
    }
}

fn fall(game: &mut Game, dt: f32) {
    game.vel = (game.vel + GRAVITY * dt).min(MAX_FALL);
    game.bird_y += game.vel * dt;
    if game.bird_y < BIRD_R {
        game.bird_y = BIRD_R;
        game.vel = 0.0;
    }
    if is_grounded(game) {
        game.bird_y = GROUND_TOP - BIRD_R;
    }
}

fn move_pipes(game: &mut Game, dt: f32) {
    for i in 0..game.pipes.len() {
        game.pipes[i].x -= SPEED * dt;
        if game.pipes[i].scored || game.pipes[i].x + PIPE_W >= BIRD_X {
            continue;
        }
        game.pipes[i].scored = true;
        game.score += 1;
    }

    while !game.pipes.is_empty() && game.pipes[0].x + PIPE_W < 0.0 {
        game.pipes.pop_front();
    }

    if game.pipes.is_empty() {
        return;
    }
    let last_x: f32 = game.pipes[game.pipes.len() - 1].x;
    if last_x < WIDTH - SPACING {
        spawn_pipe(game, last_x + SPACING);
    }
}

fn spawn_pipe(game: &mut Game, x: f32) {
    let lo: f32 = GAP / 2.0 + GAP_MARGIN;
    let hi: f32 = GROUND_TOP - GAP / 2.0 - GAP_MARGIN;
    let gap_y: f32 = lo + next_f32(&mut game.rng) * (hi - lo);
    game.pipes.push_back(Pipe {
        x,
        gap_y,
        scored: false,
    });
}

fn hits_pipe(game: &Game) -> bool {
    for i in 0..game.pipes.len() {
        let (top, bottom): (Rect, Rect) = pipe_rects(&game.pipes[i]);
        if circle_hits_rect(BIRD_X, game.bird_y, BIRD_R, top) {
            return true;
        }
        if circle_hits_rect(BIRD_X, game.bird_y, BIRD_R, bottom) {
            return true;
        }
    }
    false
}

pub fn pipe_rects(pipe: &Pipe) -> (Rect, Rect) {
    let top: Rect = Rect {
        x0: pipe.x,
        y0: 0.0,
        x1: pipe.x + PIPE_W,
        y1: pipe.gap_y - GAP / 2.0,
    };
    let bottom: Rect = Rect {
        x0: pipe.x,
        y0: pipe.gap_y + GAP / 2.0,
        x1: pipe.x + PIPE_W,
        y1: GROUND_TOP,
    };
    (top, bottom)
}

fn circle_hits_rect(cx: f32, cy: f32, r: f32, rect: Rect) -> bool {
    let nx: f32 = cx.clamp(rect.x0, rect.x1);
    let ny: f32 = cy.clamp(rect.y0, rect.y1);
    let dx: f32 = cx - nx;
    let dy: f32 = cy - ny;
    dx * dx + dy * dy < r * r
}
