use piston::input::{Button, Key};
use piston_window::GenericEvent;
use std::collections::HashSet;

pub struct KeyBoard {
    pub keys: HashSet<Key>,
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl KeyBoard {
    pub fn new() -> KeyBoard {
        KeyBoard {
            keys: HashSet::new(),
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    pub fn update<E: GenericEvent>(&mut self, event: &E) {
        if let Some(Button::Keyboard(button)) = event.press_args() {
            self.keys.insert(button);
        };

        if let Some(Button::Keyboard(button)) = event.release_args() {
            self.keys.remove(&button);
        };

        self.up = self.keys.contains(&Key::Up);
        self.down = self.keys.contains(&Key::Down);
        self.left = self.keys.contains(&Key::Left);
        self.right = self.keys.contains(&Key::Right);
    }

    pub fn contains_key(&self, key: &Key) -> bool {
        self.keys.contains(&key)
    }

    pub fn clear(&mut self) {
        self.keys.clear();
    }
}
