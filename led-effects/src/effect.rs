use smart_leds::{
    RGB8
};

pub trait LedEffect {
    /// Called once per frame to update time-based animation logic.
    /// `delta` is the time in milliseconds since the last frame.
    fn before_render(&mut self, delta: f32);

    /// Called for each pixel in the strip to determine its color.
    /// `index` is the position of the pixel.
    /// `num_leds` is the total number of LEDs in the strip.
    fn render(&self, index: usize, num_leds: usize) -> RGB8;

    /// Returns the name of the effect.
    fn name(&self) -> &str;
}

