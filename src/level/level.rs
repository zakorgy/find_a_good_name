use std::collections::HashMap;
use std::path::PathBuf;
use super::super::graphics::Screen;
use super::room::*;

pub struct Level {
    pub map_grid: [[bool; 8]; 8],
    rooms: HashMap<RoomId, Room>,
    current: RoomId,
}

impl Level {
    pub fn new(room_count: usize) ->Self {
        let path1 = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res/sprites/room1.png");
        let path2 = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res/sprites/room2.png");
        let mut rooms = HashMap::new();
        /*for i in 0 .. room_count {
            let room = Room::load_room(&path);
            let id = i as RoomId;
            rooms.insert(id, room);
        }*/
        let mut map_grid = [[false; 8]; 8];
        let room1 = RoomBuilder::new().with_path(&path1).with_neighbour(Neighbour::North(1)).build();
        rooms.insert(0, room1);
        map_grid[4][4] = true;
        let room2 = RoomBuilder::new().with_path(&path2).with_neighbour(Neighbour::South(0)).build();
        rooms.insert(1, room2);
        map_grid[4][3] = true;
        Level {
            map_grid,
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
        );
    }

    pub fn dimensions(&self) -> (i32, i32) {
        self.current_room().dimensions()
    }
}
