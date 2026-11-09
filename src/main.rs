#![allow(clippy::needless_range_loop)]

mod draw;
mod font;
mod game;

use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use renderer::canvas::Canvas;
use renderer::event::{Event, Key, is_quit};
use renderer::window;

use game::{Game, Rng, STEP};

const FRAME: Duration = Duration::from_micros(1_000_000 / 60);
// cap catch-up after a stall
const MAX_FRAME_DT: f32 = 0.25;

fn seed() -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(d) => d.as_nanos() as u64,
        Err(_) => 1,
    }
}

fn main() {
    let mut canvas: Canvas = Canvas::new(draw::W, draw::H);
    let mut game: Game = Game::new(Rng::new(seed()));

    unsafe {
        let mut win = window::open_window("flappy", canvas.width as u32, canvas.height as u32);
        let mut last: Instant = Instant::now();
        let mut acc: f32 = 0.0;

        'main: loop {
            let frame_start: Instant = Instant::now();

            while let Some(event) = window::poll_event(&mut win) {
                if is_quit(event) {
                    break 'main;
                }
                match event {
                    Event::KeyDown(Key::Space | Key::Up | Key::Char(b'w' | b'k')) => {
                        game::flap(&mut game)
                    }
                    Event::MouseDown(_, _) => game::flap(&mut game),
                    Event::KeyDown(Key::Char(b'p')) => game::toggle_pause(&mut game),
                    Event::KeyDown(Key::Char(b'r')) => game::restart(&mut game),
                    _ => {}
                }
            }

            // fixed step
            acc += last.elapsed().as_secs_f32().min(MAX_FRAME_DT);
            last = Instant::now();
            while acc >= STEP {
                game::step(&mut game, STEP);
                acc -= STEP;
            }

            draw::render(&game, &mut canvas);
            window::present(&win, &canvas);

            sleep(FRAME.saturating_sub(frame_start.elapsed()));
        }

        window::close_window(win);
    }
}
