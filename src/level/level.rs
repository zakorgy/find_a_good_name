use std::collections::HashMap;
use std::path::PathBuf;
use super::super::graphics::Screen;
use super::room::*;

pub struct Level {
    rooms: HashMap<RoomId, Room>,
    current: RoomId,
}

impl Level {
    pub fn new(room_count: usize) ->Self {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res/sprites/room1.png");
        let mut rooms = HashMap::new();
        for i in 0 .. room_count {
            let room = Room::load_room(&path);
            let id = i as RoomId;
            rooms.insert(id, room);
        }
        Level {
            rooms,
            current: 0,
        }
    }

    pub fn current_room(&self) -> &Room {
        &self.rooms[&self.current]
    }

    pub fn update(&mut self) {}

    pub fn render(&self, x_scroll: i32, y_scroll: i32, screen: &mut Screen) {
        self.current_room().render(
            x_scroll,
            y_scroll,
            screen,
        )
    }

    pub fn dimensions(&self) -> (i32, i32) {
        self.current_room().dimensions()
    }
}
