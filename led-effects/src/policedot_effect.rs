use smart_leds::RGB8;
use crate::effect::LedEffect;
use libm::floorf; // <-- use libm for floor

pub struct PoliceDot {
    pub speed: f32,
    pub size: usize,
    time: f32,
    num_leds: usize,

    // precomputed positions for render
    red_start: usize,
    red_end: usize,
    blue_start: usize,
    blue_end: usize,
}

impl PoliceDot {
    pub fn new(speed: f32, size: usize, num_leds: usize) -> Self {
        Self {
            speed,
            size,
            time: 0.0,
            num_leds,
            red_start: 0,
            red_end: 0,
            blue_start: 0,
            blue_end: 0,
        }
    }

    fn in_range(index: usize, start: usize, end: usize, _num_leds: usize) -> bool {
        if start <= end {
            index >= start && index <= end
        } else {
            index >= start || index <= end
        }
    }
}

impl LedEffect for PoliceDot {
    fn before_render(&mut self, delta: f32) {
        self.time += delta * self.speed;
        if self.time >= 1.0 {
            self.time -= 1.0;
        }

        // compute indices using libm::floorf
        let idex_r = floorf(self.time * self.num_leds as f32) as usize % self.num_leds;
        let half = self.num_leds / 2;
        let idex_b = (idex_r + half) % self.num_leds;

        self.red_start = idex_r;
        self.red_end = (idex_r + self.size) % self.num_leds;
        self.blue_start = idex_b;
        self.blue_end = (idex_b + self.size) % self.num_leds;
    }

    fn render(&self, index: usize, _num_leds: usize) -> RGB8 {
        let r = if Self::in_range(index, self.red_start, self.red_end, self.num_leds) {
            255
        } else {
            0
        };
        let b = if Self::in_range(index, self.blue_start, self.blue_end, self.num_leds) {
            255
        } else {
            0
        };
        RGB8 { r, g: 0, b }
    }

    fn name(&self) -> &str {
        "PoliceDot"
    }
}


