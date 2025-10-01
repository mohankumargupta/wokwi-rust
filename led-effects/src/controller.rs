use crate::effect::LedEffect; 
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;


pub struct EffectController<'a> {
    effects: Vec<Box<dyn LedEffect + 'a>>,
    current_effect_index: usize,
}

impl<'a> EffectController<'a> {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
            current_effect_index: 0,
        }
    }

    pub fn add_effect(&mut self, effect: Box<dyn LedEffect + 'a>) {
        self.effects.push(effect);
    }

    pub fn next_effect(&mut self) {
        self.current_effect_index = (self.current_effect_index + 1) % self.effects.len();
    }

    pub fn get_current_effect(&mut self) -> &mut dyn LedEffect {
        self.effects[self.current_effect_index].as_mut()
    }
}

