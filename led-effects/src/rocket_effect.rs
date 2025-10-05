use crate::effect::LedEffect;

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use libm::{floorf, fmodf};
use smart_leds::RGB8;

/// A simple pseudo-random generator since we don't have std::rand
fn random_f32(seed: &mut u32) -> f32 {
    *seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
    (*seed as f32) / (u32::MAX as f32)
}

struct Spark {
    energy: f32,
    pos: f32,
    hue: f32,
}

pub struct RocketEffect {
    // Parameters
    flight_time: f32,
    rocket_size: usize,
    boost_delay: f32,
    boost_multiplier: f32,
    exhaust_hue: f32,
    exhaust_sat: f32,
    exhaust_val: f32,
    rocket_hue: f32,
    rocket_sat: f32,
    rocket_val: f32,
    multi_color: bool,

    // State
    rocket_pos: f32,
    rocket_vel: f32,
    elapsed_time: f32,

    // Spark simulation
    sparks: Vec<Spark>,
    max_sparks: usize,
    friction: f32,

    // Pixel Buffers for additive blending
    pixels_r: Vec<f32>,
    pixels_g: Vec<f32>,
    pixels_b: Vec<f32>,

    num_leds: usize,
    rng_seed: u32,
}

impl RocketEffect {
    pub fn new(num_leds: usize) -> Self {
        let max_sparks = (num_leds / 6).max(1);
        let mut sparks = Vec::with_capacity(max_sparks);
        let mut seed = 12345; // Initial seed

        for _ in 0..max_sparks {
            let pos = random_f32(&mut seed) * num_leds as f32;
            sparks.push(Spark {
                pos,
                energy: 1.0 * (1.0 - pos / num_leds as f32) + random_f32(&mut seed) * 0.4,
                hue: random_f32(&mut seed),
            });
        }

        Self {
            flight_time: 5.0,
            rocket_size: 5,
            boost_delay: 1.0,
            boost_multiplier: 50.0, // A bit more boost for fun
            exhaust_hue: 0.02,
            exhaust_sat: 1.0,
            exhaust_val: 1.0,
            rocket_hue: 0.0,
            rocket_sat: 0.0,
            rocket_val: 1.0,
            multi_color: false,

            rocket_pos: 0.0,
            rocket_vel: 0.0,
            elapsed_time: 0.0,

            sparks,
            max_sparks,
            friction: 0.9 / num_leds as f32,

            pixels_r: vec![0.0; num_leds],
            pixels_g: vec![0.0; num_leds],
            pixels_b: vec![0.0; num_leds],

            num_leds,
            rng_seed: seed,
        }
    }

    /// Converts HSV to an (r, g, b) tuple of f32s in [0.0, 1.0]
    fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
        let h = fmodf(h, 1.0) * 6.0;
        let i = floorf(h) as i32;
        let f = h - i as f32;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);

        match i {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            _ => (v, p, q),
        }
    }
}

impl LedEffect for RocketEffect {
    fn name(&self) -> &str {
        "Rocket"
    }

    fn before_render(&mut self, delta: f32) {
        let spark_delta = delta * 10.0;

        // 1. Cool all pixels
        let cool_factor = (0.1 / spark_delta).min(0.99);
        for i in 0..self.num_leds {
            self.pixels_r[i] *= cool_factor;
            self.pixels_g[i] *= cool_factor;
            self.pixels_b[i] *= cool_factor;
        }

        // 2. Update rocket physics
        self.elapsed_time += delta;
        let base_accel = 2.0 * self.num_leds as f32 / (self.flight_time * self.flight_time);
        let current_accel = if self.elapsed_time > self.boost_delay {
            base_accel * self.boost_multiplier
        } else {
            base_accel
        };
        self.rocket_vel += current_accel * delta;
        self.rocket_pos += self.rocket_vel * delta;

        // Reset if it flies off the top
        if self.rocket_pos >= self.num_leds as f32 {
            self.rocket_pos = 0.0;
            self.rocket_vel = 0.0;
            self.elapsed_time = 0.0;
        }

        // 3. Update sparks
        let effective_sparks =
            floorf(self.max_sparks as f32 * (self.rocket_size as f32 / 20.0)) as usize;

        for i in 0..self.sparks.len().min(effective_sparks) {
            let spark = &mut self.sparks[i];

            if spark.energy <= 0.0 {
                // Respawn spark at rocket's current position
                spark.energy = 1.0 + random_f32(&mut self.rng_seed) * 0.4;
                spark.pos = self.rocket_pos;
                if self.multi_color {
                    spark.hue = random_f32(&mut self.rng_seed);
                }
            }

            spark.energy -= self.friction * spark_delta;
            spark.energy = spark.energy.max(0.0);
            spark.pos -= spark.energy * spark.energy * spark_delta; // Move downward

            if spark.pos < 0.0 || spark.pos >= self.num_leds as f32 {
                spark.pos = self.rocket_pos;
                spark.energy = 0.0; // Mark for respawn on next frame
                continue;
            }

            let spark_idx = floorf(spark.pos) as usize;
            if spark_idx < self.num_leds {
                let contrib_v = spark.energy * spark.energy; // Gamma
                let mut h = if self.multi_color {
                    spark.hue
                } else {
                    self.exhaust_hue
                };

                if contrib_v < 0.5 {
                    h += 0.1 + random_f32(&mut self.rng_seed) * 0.2; // Hue shift when fizzling
                }

                let s = (self.exhaust_sat * (1.1 - contrib_v)).clamp(0.0, 1.0);
                let v = (contrib_v * self.exhaust_val).clamp(0.0, 1.0);

                let (r, g, b) = Self::hsv_to_rgb(h, s, v);
                self.pixels_r[spark_idx] += r;
                self.pixels_g[spark_idx] += g;
                self.pixels_b[spark_idx] += b;
            }
        }

        // 4. Draw the rocket body
        let (r, g, b) = Self::hsv_to_rgb(self.rocket_hue, self.rocket_sat, self.rocket_val);
        for j in 0..self.rocket_size {
            let body_idx = floorf(self.rocket_pos + j as f32) as usize;
            if body_idx < self.num_leds {
                self.pixels_r[body_idx] = r; // Use direct assignment for solid body
                self.pixels_g[body_idx] = g;
                self.pixels_b[body_idx] = b;
            }
        }
    }

    fn render(&self, index: usize, _num_leds: usize) -> RGB8 {
        let r = (self.pixels_r[index].clamp(0.0, 1.0) * 255.0) as u8;
        let g = (self.pixels_g[index].clamp(0.0, 1.0) * 255.0) as u8;
        let b = (self.pixels_b[index].clamp(0.0, 1.0) * 255.0) as u8;
        RGB8 { r, g, b }
    }
}

