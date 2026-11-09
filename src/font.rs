use renderer::canvas::{Canvas, fill_rect};
use renderer::color::Color;

// 3x5, row-major, top-left pixel is bit 14
const DIGITS: [u16; 10] = [
    0b111_101_101_101_111,
    0b010_110_010_010_111,
    0b111_001_111_100_111,
    0b111_001_111_001_111,
    0b101_101_111_001_001,
    0b111_100_111_001_111,
    0b111_100_111_101_111,
    0b111_001_001_001_001,
    0b111_101_111_101_111,
    0b111_101_111_001_111,
];

const GLYPH_W: i32 = 3;
const GLYPH_H: i32 = 5;

fn digits_of(n: u32) -> Vec<usize> {
    let text: String = n.to_string();
    let bytes: &[u8] = text.as_bytes();
    let mut digits: Vec<usize> = Vec::with_capacity(bytes.len());
    for i in 0..bytes.len() {
        digits.push((bytes[i] - b'0') as usize);
    }
    digits
}

pub fn number_width(n: u32, scale: i32) -> i32 {
    let count: i32 = digits_of(n).len() as i32;
    count * (GLYPH_W + 1) * scale - scale
}

pub fn number_height(scale: i32) -> i32 {
    GLYPH_H * scale
}

pub fn draw_number(canvas: &mut Canvas, n: u32, x: i32, y: i32, scale: i32, c: Color) {
    let digits: Vec<usize> = digits_of(n);
    for i in 0..digits.len() {
        let gx: i32 = x + i as i32 * (GLYPH_W + 1) * scale;
        let bits: u16 = DIGITS[digits[i]];
        for row in 0..GLYPH_H {
            for col in 0..GLYPH_W {
                let bit: i32 = 14 - (row * GLYPH_W + col);
                if bits >> bit & 1 == 1 {
                    fill_rect(canvas, gx + col * scale, y + row * scale, scale, scale, c);
                }
            }
        }
    }
}
