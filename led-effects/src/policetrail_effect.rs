use smart_leds::RGB8;
use crate::effect::LedEffect;
use libm::floorf;

pub struct PoliceTrail {
    pub speed: f32,
    pub size: usize,
    pub trail_length: usize,  // How many LEDs behind the dot should fade
    time: f32,
    num_leds: usize,

    // precomputed position for render
    red_pos: usize,
    blue_pos: usize,
}

impl PoliceTrail {
    pub fn new(speed: f32, size: usize, trail_length: usize, num_leds: usize) -> Self {
        Self {
            speed,
            size,
            trail_length,
            time: 0.0,
            num_leds,
            red_pos: 0,
            blue_pos: 0,
        }
    }

    /// Calculate the distance behind the head position (wrapping around)
    fn distance_behind(index: usize, head: usize, num_leds: usize) -> usize {
        if index <= head {
            head - index
        } else {
            num_leds - index + head
        }
    }

    /// Calculate brightness based on distance from head (255 at head, fading behind)
    fn trail_brightness(distance: usize, trail_length: usize, size: usize) -> u8 {
        if distance < size {
            // Full brightness for the dot itself
            255
        } else if distance < size + trail_length {
            // Fade from 255 to 0 over trail_length
            let fade_distance = distance - size;
            let fade_ratio = 1.0 - (fade_distance as f32 / trail_length as f32);
            (255.0 * fade_ratio) as u8
        } else {
            0
        }
    }
}

impl LedEffect for PoliceTrail {
    fn before_render(&mut self, delta: f32) {
        self.time += delta * self.speed;
        
        let pos = self.time * self.num_leds as f32;
        self.red_pos = floorf(pos) as usize % self.num_leds;
        let half = self.num_leds / 2;
        self.blue_pos = (self.red_pos + half) % self.num_leds;
    }

    fn render(&self, index: usize, _num_leds: usize) -> RGB8 {
        // Calculate distance behind each dot head
        let red_distance = Self::distance_behind(index, self.red_pos, self.num_leds);
        let blue_distance = Self::distance_behind(index, self.blue_pos, self.num_leds);

        // Get brightness for each color channel
        let r = Self::trail_brightness(red_distance, self.trail_length, self.size);
        let b = Self::trail_brightness(blue_distance, self.trail_length, self.size);

        RGB8 { r, g: 0, b }
    }

    fn name(&self) -> &str {
        "PoliceTrail"
    }
}