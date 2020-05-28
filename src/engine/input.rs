/*!
Module for processing the raw input into easy for the game to reason about information.
Currently handles only keyboard.

To be truly an 'engine' module requires the Button enum to be user-defined from external source.
*/
use enum_map::{Enum, EnumMap};
use quicksilver::lifecycle::{EventCache, Key};

/// Treat as if the game had dedicated controller with these buttons.
#[derive(Debug, Enum, Clone, Copy)]
pub enum Button {
    Left,
    Right,
    Up,
    Down,
    Jump,
}

// Reads the edge-based input and turn it into level-based.
pub struct ButtonsState {
    bindings: EnumMap<Button, (Option<Key>, u8)>,
}

impl Default for ButtonsState {
    fn default() -> Self {
        let mut bindings = EnumMap::<Button, (Option<Key>, u8)>::default();
        bindings[Button::Up] = (Some(Key::W), 0);
        bindings[Button::Left] = (Some(Key::A), 0);
        bindings[Button::Down] = (Some(Key::S), 0);
        bindings[Button::Right] = (Some(Key::D), 0);
        bindings[Button::Jump] = (Some(Key::Space), 0);
        Self { bindings }
    }
}

impl ButtonsState {
    pub fn update(&mut self, cache: &EventCache) {
        for (maybe_key, ref mut history) in self.bindings.values_mut() {
            if let Some(key) = maybe_key {
                *history <<= 1;
                *history |= cache.key(*key) as u8;
            }
        }
    }
    pub fn is_pressed(&self, button: Button) -> bool {
        (self.bindings[button].1 & 0b1) == 0b1
    }
    pub fn pressed(&self, button: Button) -> bool {
        (self.bindings[button].1 & 0b11) == 0b01
    }
    pub fn released(&self, button: Button) -> bool {
        (self.bindings[button].1 & 0b11) == 0b10
    }
}
