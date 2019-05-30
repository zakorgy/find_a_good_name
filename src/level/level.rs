use rand::{thread_rng, Rng};
use cgmath::Vector2;

use super::super::graphics::Screen;
use super::room::*;
use std::collections::HashMap;
use std::path::PathBuf;

pub const MAP_GRID_SIZE: usize = 9;
pub const MAP_GRID_SIZE_MINUS_ONE: usize = 8;

struct LevelBuilder {
    rooms: [[Option<RoomBuilder>; MAP_GRID_SIZE]; MAP_GRID_SIZE],
    taken_positions: HashMap<Vector2<i32>, RoomId>,
    possible_positions: Vec<Vector2<i32>>,
    number_of_rooms: usize,
    next_id: u8,
}

impl LevelBuilder {
    //Fill a 2D array with zeros (type: integer, 0 means nothing to place there)
    //Set the place of the starting room (for example at [5,5])
    //Select a random array element in this 2 dimensional array.
    //If the selected element has a room next to it, make a room there! (fill the array element with not zero,
    //I think different rooms will have different numbers, 0 will be the starter room where the player will spawn,
    //1 will be one room type, 2 will be an other,...)
    //If there is no room next to the selected element, do step 3 again. We shouldn't make room
    //if the selected empty place has 2 or more neighbours! This will make the level a little bit different (figure 1, 2nd image)
    //Do it until we want: if we want 15 room we need to do step 3 until we have 15 rooms
    //(don't forget, we already have a starter room!)

    pub fn new() -> LevelBuilder {
        let start_pos = Vector2::new(MAP_GRID_SIZE as i32 / 2, MAP_GRID_SIZE as i32 / 2);
        let mut builder = LevelBuilder {
            rooms: Default::default(),
            taken_positions: std::iter::once((start_pos, 0)).collect(),
            possible_positions: vec![],
            number_of_rooms: 15,
            next_id: 1,
        };

        let start_room = RoomBuilder::new()
            .with_grid_pos(start_pos)
            .with_room_type(RoomType::Start)
            .with_path(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res/sprites/room1.png"))
            .with_id(0);

        builder.rooms[start_pos.x as usize][start_pos.y as usize] = Some(start_room);
        let ref free_positions = builder.free_neighbour_positions(start_pos);
        builder.possible_positions.extend_from_slice(free_positions);
        builder
    }

    pub fn with_number_of_rooms(mut self, number_of_rooms: usize) -> Self {
        self.number_of_rooms = number_of_rooms;
        self
    }

    fn free_neighbour_positions(&self, pos: Vector2<i32>) -> Vec<Vector2<i32>> {
        let mut free_positions = Vec::new();
        for x in [pos.x - 1, pos.x + 1].into_iter() {
            if *x < 0 || *x > MAP_GRID_SIZE_MINUS_ONE as i32 {
                continue;
            }
            let y = pos.y;
            if let None = self.rooms[*x as usize][y as usize] {
                free_positions.push(Vector2::new(*x, y));
            }
        }
        for y in [pos.y - 1, pos.y + 1].into_iter() {
            if *y < 0 || *y > MAP_GRID_SIZE_MINUS_ONE as i32 {
                continue;
            }
            let x = pos.x;
            if let None = self.rooms[x as usize][*y as usize] {
                free_positions.push(Vector2::new(x, *y));
            }
        }
        free_positions
    }

    pub fn build(mut self) -> Level {
        self.create_rooms();
        self.set_room_doors();

        let mut map_grid = [[false; MAP_GRID_SIZE]; MAP_GRID_SIZE];
        let mut rooms = HashMap::new();

        for x in 0..MAP_GRID_SIZE {
            for y in 0..MAP_GRID_SIZE {
                if let Some(room) = std::mem::replace(&mut self.rooms[x][y], None) {
                    map_grid[x][y] = true;
                    let (id, room) = room.build();
                    rooms.insert(id, room);
                }
            }
        }

        Level {
            map_grid,
            rooms,
            current: 0,
        }
    }

    fn create_rooms(&mut self) {
        let mut rng = thread_rng();
        let mut new_room_index = 0;
        let mut new_pos = Vector2::new(0, 0);
        for _ in 1..self.number_of_rooms {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("res/sprites/room{}.png", rng.gen_range(1, 4)));
            for _ in 0..400 {
                new_room_index = rng.gen_range(0, self.possible_positions.len());
                new_pos = self.possible_positions[new_room_index];
                if self.neighbour_count(new_pos) < 2 {
                    break;
                }
            }

            // add new room to rooms
            let id = self.next_id;
            let room = RoomBuilder::new()
                .with_path(&path)
                .with_grid_pos(new_pos.into())
                .with_id(id);
            self.next_id += 1;
            self.rooms[new_pos.x as usize][new_pos.y as usize] = Some(room);

            // update taken positions
            self.taken_positions.insert(new_pos, id);

            // update number of rooms
            self.number_of_rooms += 1;

            // update possible positions
            self.possible_positions.remove(new_room_index);
            let ref free_positions = self.free_neighbour_positions(new_pos);
            self.possible_positions.extend_from_slice(free_positions);

            if self.taken_positions.len() == self.number_of_rooms {
                break;
            }
        }
    }

    fn neighbour_count(&self, pos: Vector2<i32>) -> usize {
        let mut neighbours = 0;
        if self.taken_positions.contains_key(&(pos.x + 1, pos.y).into()) {
            neighbours += 1;
        }
        if self.taken_positions.contains_key(&(pos.x - 1, pos.y).into()) {
            neighbours += 1;
        }
        if self.taken_positions.contains_key(&(pos.x, pos.y + 1).into()) {
            neighbours += 1;
        }
        if self.taken_positions.contains_key(&(pos.x, pos.y - 1).into()) {
            neighbours += 1;
        }
        neighbours
    }

    fn set_room_doors(&mut self) {
        for x in 0..MAP_GRID_SIZE {
            for y in 0..MAP_GRID_SIZE {
                if let Some(ref mut room) = self.rooms[x][y] {
                    let xi = x as i32;
                    let yi = y as i32;
                    if let Some(id) = self.taken_positions.get(&(xi - 1, yi).into()) {
                        room.add_neighbour(Neighbour::West(*id));
                    }
                    if let Some(id) = self.taken_positions.get(&(xi + 1, yi).into()) {
                        room.add_neighbour(Neighbour::East(*id));
                    }
                    if let Some(id) = self.taken_positions.get(&(xi, yi - 1).into()) {
                        room.add_neighbour(Neighbour::North(*id));
                    }
                    if let Some(id) = self.taken_positions.get(&(xi, yi + 1).into()) {
                        room.add_neighbour(Neighbour::South(*id));
                    }
                }
            }
        }
    }
}

pub struct Level {
    pub map_grid: [[bool; MAP_GRID_SIZE]; MAP_GRID_SIZE],
    rooms: HashMap<RoomId, Room>,
    current: RoomId,
}

impl Level {
    pub fn new(room_count: usize) -> Self {
        LevelBuilder::new().with_number_of_rooms(room_count).build()
    }

    pub fn current_room(&self) -> &Room {
        &self.rooms[&self.current]
    }

    pub fn current_room_id(&self) -> RoomId {
        self.current
    }

    pub fn set_current_room(&mut self, id: RoomId) {
        self.current = id;
    }

    pub fn update(&mut self) {}

    pub fn render(&self, scroll: Vector2<i32>, screen: &mut Screen) {
        self.current_room().render(scroll, screen);
    }

    pub fn dimensions(&self) -> Vector2<i32> {
        self.current_room().dimensions()
    }

    pub fn map_info(&self) -> MapInfo {
        MapInfo {
            map_grid: &self.map_grid,
            current_grid_pos: self.rooms[&self.current].grid_pos,
        }
    }
}

pub struct MapInfo<'a> {
    pub map_grid: &'a [[bool; MAP_GRID_SIZE]; MAP_GRID_SIZE],
    pub current_grid_pos: Vector2<i32>,
}
