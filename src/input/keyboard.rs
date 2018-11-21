use piston::input::{Button, Key};
use piston_window::GenericEvent;
use std::collections::HashSet;

pub struct KeyBoard {
    keys: HashSet<Key>,
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
            println!("pressed {:?}", button);
            self.keys.insert(button);
        };

        if let Some(Button::Keyboard(button)) = event.release_args() {
            println!("released {:?}", button);
            self.keys.remove(&button);
        };

        self.up = self.keys.contains(&Key::Up) ||
            self.keys.contains(&Key::W);

        self.down = self.keys.contains(&Key::Down) ||
            self.keys.contains(&Key::S);

        self.left = self.keys.contains(&Key::Left) ||
            self.keys.contains(&Key::A);

        self.right = self.keys.contains(&Key::Right) ||
            self.keys.contains(&Key::D);

        //println!("up {}, down {}, left {}, right {}", self.up, self.down, self.left, self.right);
    }
}
