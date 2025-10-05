use smart_leds::RGB8;
use crate::effect::LedEffect;
use libm::{fabsf, sinf};
use core::f32::consts::PI;
use num_traits::float::FloatCore;

/// Converts HSV to RGB8.
/// h, s, v all in [0,1]
fn hsv2rgb(h: f32, s: f32, v: f32) -> RGB8 {
    let h = h.fract();
    let s = s.clamp(0.0, 1.0);
    let v = v.clamp(0.0, 1.0);

    let i = (h * 6.0).floor();
    let f = h * 6.0 - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);

    let (r, g, b) = match i as i32 % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
        _ => (0.0, 0.0, 0.0),
    };

    RGB8 {
        r: (r * 255.0) as u8,
        g: (g * 255.0) as u8,
        b: (b * 255.0) as u8,
    }
}

/// Pixelblaze-like wave function: 0.5 + 0.5 * sin(PI * x)
fn wave(x: f32) -> f32 {
    0.5 + 0.5 * sinf(PI * x)
}

pub struct DrogenEffect {
    t1: f32,
    num_leds: usize,
}

impl DrogenEffect {
    pub fn new(num_leds: usize) -> Self {
        Self {
            t1: 0.0,
            num_leds,
        }
    }
}

impl LedEffect for DrogenEffect {
    fn before_render(&mut self, delta: f32) {
        // Pixelblaze time(0.2) increments at 0.2 Hz (period = 5s)
        // So t1 += delta * 0.2, wrap at 1.0
        self.t1 += delta * 0.4;
        if self.t1 > 1.0 {
            self.t1 -= 1.0;
        }
    }

    fn render(&self, index: usize, _num_leds: usize) -> RGB8 {
        let hl = self.num_leds as f32 / 2.0;
        let i = index as f32;
        let mut c = 0.1 - fabsf(i - hl) / hl;
        c = wave(c);
        c = wave(c + self.t1);
        hsv2rgb(c, 1.0, 1.0)
    }

    fn name(&self) -> &str {
        "Drogen"
    }
}