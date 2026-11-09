use renderer::canvas::*;
use renderer::color::{self, Color, rgba};
use renderer::math::Vec2;

use crate::font::{draw_number, number_height, number_width};
use crate::game::*;

pub const W: usize = WIDTH as usize;
pub const H: usize = HEIGHT as usize;

const SKY_TOP: Color = 0xFF2B3A55;
const SKY_BOTTOM: Color = 0xFF7FB3D5;
const PIPE: Color = 0xFF6FAE4C;
const DIRT: Color = 0xFFC9A66B;
const GRASS: Color = 0xFF7BC950;
const BODY: Color = 0xFFF2A65A;
const BELLY: Color = 0xFFF7D08A;
const BEAK: Color = 0xFFE4572E;
const WHITE: Color = color::WHITE;
const INK: Color = 0xFF101010;

// x, y, size
const CLOUDS: [(f32, f32, i32); 4] = [
    (60.0, 110.0, 22),
    (260.0, 70.0, 28),
    (420.0, 160.0, 18),
    (560.0, 220.0, 24),
];

const LIP_H: f32 = 26.0;
const LIP_OUT: f32 = 6.0;
const STRIPE: f32 = 28.0;

pub fn render(game: &Game, canvas: &mut Canvas) {
    draw_sky(canvas);
    draw_clouds(canvas, game.scroll);
    for i in 0..game.pipes.len() {
        draw_pipe(canvas, &game.pipes[i]);
    }
    draw_ground(canvas, game.scroll);
    draw_bird(canvas, game);

    match game.status {
        Status::Ready => draw_ready(canvas, game.time),
        Status::Playing => draw_score(canvas, game.score),
        Status::Paused => {
            draw_score(canvas, game.score);
            draw_paused(canvas);
        }
        Status::Dead => {
            if is_grounded(game) {
                draw_game_over(canvas, game.score, game.best);
                return;
            }
            draw_score(canvas, game.score);
        }
    }
}

fn v(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

fn disc(canvas: &mut Canvas, p: Vec2, r: i32, c: Color) {
    fill_circle(canvas, p.x.round() as i32, p.y.round() as i32, r, c);
}

// bird space offset, rotated into place
fn at(center: Vec2, angle: f32, dx: f32, dy: f32) -> Vec2 {
    (center + v(dx, dy)).rotate(center, angle)
}

fn draw_sky(canvas: &mut Canvas) {
    fill_triangle_gradient(
        canvas,
        [v(0.0, 0.0), v(WIDTH, 0.0), v(0.0, GROUND_TOP)],
        [SKY_TOP, SKY_TOP, SKY_BOTTOM],
    );
    fill_triangle_gradient(
        canvas,
        [v(WIDTH, 0.0), v(WIDTH, GROUND_TOP), v(0.0, GROUND_TOP)],
        [SKY_TOP, SKY_BOTTOM, SKY_BOTTOM],
    );
}

fn draw_clouds(canvas: &mut Canvas, scroll: f32) {
    let wrap: f32 = WIDTH + 200.0;
    let puff: Color = rgba(255, 255, 255, 0x38);
    for i in 0..CLOUDS.len() {
        let (x, y, s): (f32, f32, i32) = CLOUDS[i];
        let cx: i32 = ((x - scroll * 0.2).rem_euclid(wrap) - 100.0) as i32;
        let cy: i32 = y as i32;
        fill_circle(canvas, cx, cy, s, puff);
        fill_circle(canvas, cx + s, cy + s / 3, s * 3 / 4, puff);
        fill_circle(canvas, cx - s, cy + s / 3, s * 2 / 3, puff);
    }
}

fn draw_pipe(canvas: &mut Canvas, pipe: &Pipe) {
    let (top, bottom): (Rect, Rect) = pipe_rects(pipe);
    let dark: Color = color::shade(PIPE, 0.7);
    let light: Color = color::shade(PIPE, 1.25);

    let halves: [Rect; 2] = [top, bottom];
    for i in 0..halves.len() {
        let r: Rect = halves[i];
        let x: i32 = r.x0 as i32;
        let y: i32 = r.y0 as i32;
        let w: i32 = (r.x1 - r.x0) as i32;
        let h: i32 = (r.y1 - r.y0) as i32;
        fill_rect(canvas, x, y, w, h, PIPE);
        fill_rect(canvas, x + 8, y, 8, h, light);
        fill_rect(canvas, x + w - 12, y, 12, h, dark);
    }

    let lip_x: i32 = (pipe.x - LIP_OUT) as i32;
    let lip_w: i32 = (PIPE_W + 2.0 * LIP_OUT) as i32;
    let lip_h: i32 = LIP_H as i32;
    let lips: [f32; 2] = [top.y1 - LIP_H, bottom.y0];
    for i in 0..lips.len() {
        let y: i32 = lips[i] as i32;
        fill_rect(canvas, lip_x, y, lip_w, lip_h, PIPE);
        fill_rect(canvas, lip_x + 8, y, 8, lip_h, light);
        fill_rect(canvas, lip_x + lip_w - 12, y, 12, lip_h, dark);
        fill_rect(canvas, lip_x, y, lip_w, 3, dark);
        fill_rect(canvas, lip_x, y + lip_h - 3, lip_w, 3, dark);
    }
}

fn draw_ground(canvas: &mut Canvas, scroll: f32) {
    let top: i32 = GROUND_TOP as i32;
    fill_rect(canvas, 0, top, W as i32, H as i32 - top, DIRT);

    let stripe: Color = color::shade(DIRT, 0.85);
    let offset: f32 = scroll.rem_euclid(STRIPE);
    let y0: f32 = GROUND_TOP + 14.0;
    let y1: f32 = HEIGHT;
    let slant: f32 = (y1 - y0) * 0.6;
    let mut x: f32 = -offset - slant;
    while x < WIDTH + STRIPE {
        let a: Vec2 = v(x, y1);
        let b: Vec2 = v(x + slant, y0);
        let c: Vec2 = v(x + slant + STRIPE / 2.0, y0);
        let d: Vec2 = v(x + STRIPE / 2.0, y1);
        fill_triangle(canvas, [a, b, c], stripe);
        fill_triangle(canvas, [a, c, d], stripe);
        x += STRIPE;
    }

    fill_rect(canvas, 0, top, W as i32, 14, GRASS);
    fill_rect(canvas, 0, top + 14, W as i32, 3, color::shade(GRASS, 0.6));
}

fn draw_bird(canvas: &mut Canvas, game: &Game) {
    let center: Vec2 = v(BIRD_X, game.bird_y);
    let angle: f32 = tilt(game);

    disc(canvas, center, BIRD_R as i32, BODY);
    disc(canvas, at(center, angle, -2.0, 5.0), 9, BELLY);

    let mut flap: f32 = 0.0;
    if game.status != Status::Dead {
        flap = (game.time * 18.0).sin();
    }
    fill_triangle(
        canvas,
        [
            at(center, angle, -12.0, 0.0),
            at(center, angle, 2.0, 0.0),
            at(center, angle, -6.0, 10.0 * flap),
        ],
        color::shade(BODY, 0.75),
    );

    disc(canvas, at(center, angle, 6.0, -6.0), 6, WHITE);
    disc(canvas, at(center, angle, 8.0, -6.0), 3, INK);

    fill_triangle(
        canvas,
        [
            at(center, angle, 11.0, -2.0),
            at(center, angle, 24.0, 2.0),
            at(center, angle, 11.0, 6.0),
        ],
        BEAK,
    );
}

fn draw_score(canvas: &mut Canvas, score: u32) {
    let scale: i32 = 7;
    let x: i32 = W as i32 / 2 - number_width(score, scale) / 2;
    draw_number(canvas, score, x + 3, 43, scale, rgba(0, 0, 0, 0x60));
    draw_number(canvas, score, x, 40, scale, WHITE);
}

fn dim(canvas: &mut Canvas) {
    fill_rect(canvas, 0, 0, W as i32, H as i32, rgba(0, 0, 0, 0x90));
}

fn play_button(canvas: &mut Canvas, cx: f32, top: f32, size: f32) {
    fill_triangle_gradient(
        canvas,
        [
            v(cx - size * 0.4, top),
            v(cx + size * 0.5, top + size / 2.0),
            v(cx - size * 0.4, top + size),
        ],
        [GRASS, WHITE, GRASS],
    );
}

fn draw_ready(canvas: &mut Canvas, time: f32) {
    let size: f32 = 52.0 + (time * 5.0).sin() * 4.0;
    play_button(canvas, WIDTH / 2.0, HEIGHT * 0.6, size);
}

fn draw_paused(canvas: &mut Canvas) {
    dim(canvas);
    let cx: i32 = W as i32 / 2;
    let cy: i32 = H as i32 / 2;
    let white: Color = rgba(255, 255, 255, 0xE0);
    fill_rect(canvas, cx - 30, cy - 40, 20, 80, white);
    fill_rect(canvas, cx + 10, cy - 40, 20, 80, white);
}

fn draw_game_over(canvas: &mut Canvas, score: u32, best: u32) {
    dim(canvas);
    let scale: i32 = 16;
    let x: i32 = W as i32 / 2 - number_width(score, scale) / 2;
    let y: i32 = H as i32 / 2 - number_height(scale) - 30;
    draw_number(canvas, score, x, y, scale, BODY);

    let small: i32 = 5;
    let bx: i32 = W as i32 / 2 - number_width(best, small) / 2;
    let by: i32 = H as i32 / 2 - 10;
    draw_number(canvas, best, bx, by, small, rgba(255, 255, 255, 0x90));

    play_button(canvas, WIDTH / 2.0, HEIGHT / 2.0 + 50.0, 56.0);
}
