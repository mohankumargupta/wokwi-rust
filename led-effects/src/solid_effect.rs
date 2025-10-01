use smart_leds::{
    RGB8
};
use crate::effect::LedEffect; 

pub struct SolidColor {
    pub color: RGB8,
}

impl LedEffect for SolidColor {
    fn before_render(&mut self, _delta: f32) {
        // No time-based logic needed for a solid color
    }

    fn render(&self, _index: usize, _num_leds: usize) -> RGB8 {
        self.color
    }

    fn name(&self) -> &str {
        "Solid Color"
    }
}

