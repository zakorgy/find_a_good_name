use super::super::graphics::Screen;
use super::super::level::{Room, RoomId};

use std::boxed::Box;
use std::collections::{HashMap, VecDeque};

pub type EntityId = u32;


pub const INVALID_ID: EntityId = 0;
pub const GAME_ID: EntityId = 1;
pub const PLAYER_ID: EntityId = 2;
pub const ENTITY_MANAGER_ID: EntityId = 3;
const FIRST_FREE_ID: EntityId = 10;
const EPSILON: f32 = 0.5;



pub trait Entity {
    fn update(&mut self, room: &Room);
    fn move_entity(&mut self, x: f32, y: f32, room: &Room) {}
    fn render(&self, screen: &mut Screen, x_offset: f32, y_offset: f32);
    fn remove(&mut self);
    fn is_removed(&self) -> bool;
    fn set_pos(&mut self, x: f32, y: f32) {}
    fn relative_pos(&self, x_offset: f32, y_offset: f32) -> (i32, i32);
    fn absolute_pos(&self) -> (i32, i32);
    fn collider(&self) -> Option<Collider> {
        None
    }
    fn collides_with(&mut self, other: &Option<Collider>) -> bool {
        false
    }
    fn collides(&self) -> bool {
        false
    }
    fn id(&self) -> EntityId;
    fn set_id(&mut self, id: EntityId) {}
    fn handle_message(&mut self, message: Telegram, dispatcher: &mut MessageDispatcher) {}
    fn send_message(&self, message: Message, receiver: EntityId, dispatcher: &mut MessageDispatcher) {}
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub struct Collider {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Collider {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Collider {
        Collider {
            x,
            y,
            width,
            height,
        }
    }

    pub fn intersects(&self, other: &Collider) -> bool {
        let l1 = (self.x + EPSILON, self.y + EPSILON);
        let r1 = (self.x + self.width - EPSILON , self.y + self.height - EPSILON);
        let l2 = (other.x + EPSILON, other.y + EPSILON);
        let r2 = (other.x + other.width - EPSILON, other.y + other.height - EPSILON);

        // If one rectangle is on left side of other
        if (l1.0 >= r2.0 || l2.0 >= r1.0) {
            return false;
        }

        // If one rectangle is above other
        if (l1.1 >= r2.1 || l2.1 >= r1.1) {
            return false;
        }

        true
    }

    pub fn top_left(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Door {
    pub collider: Collider,
    id: EntityId,
    pub room: RoomId,
    pub removed: bool,
}

impl<'a> From<&'a ((u32, u32), RoomId)> for Door {
    fn from(info: &((u32, u32), RoomId)) -> Self {
        Door {
            collider: Collider::new(((info.0).0 * 8) as f32 + 4.0, ((info.0).1 * 8) as f32 + 4.0, 1.0, 1.0),
            id: INVALID_ID,
            room: info.1,
            removed: false,
        }
    }
}

impl Entity for Door {
    fn update(&mut self, room: &Room) {}

    fn move_entity(&mut self, x: f32, y: f32, room: &Room) {}

    fn render(&self, screen: &mut Screen, x_offset: f32, y_offset: f32) {}

    fn remove(&mut self) {
        self.removed = true;
    }

    fn is_removed(&self) -> bool {
        self.removed
    }

    fn relative_pos(&self, x_offset: f32, y_offset: f32) -> (i32, i32) {
        ((self.collider.x - x_offset) as i32, (self.collider.y - y_offset) as i32)
    }

    fn absolute_pos(&self) -> (i32, i32) {
        (self.collider.x as i32, self.collider.y as i32)
    }

    fn collider(&self) -> Option<Collider> {
        Some(self.collider)
    }

    fn id(&self) -> EntityId {
        self.id
    }

    fn set_id(&mut self, id: EntityId) {
        self.id = id;
    }

    fn handle_message(&mut self, message: Telegram, dispatcher: &mut MessageDispatcher) {
        let Telegram { sender, receiver, message } = message;
        match message {
            Message::Collides => {
                if sender == PLAYER_ID {
                    self.send_message(Message::LoadRoom(self.room), GAME_ID, dispatcher);
                }
            }
            _ => {}
        }
    }

    fn send_message(&self, message: Message, receiver: EntityId, dispatcher: &mut MessageDispatcher) {
        dispatcher.queue_message(self.id(), receiver, message);
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Telegram {
    pub sender: EntityId,
    pub receiver: EntityId,
    pub message: Message,
}

#[derive(Debug, Copy, Clone)]
pub enum Message {
    LoadRoom(RoomId),
    Collides,
}

pub struct EntityManager {
    entities: HashMap<EntityId, Box<dyn Entity>>,
    next_id: EntityId,
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            entities: HashMap::with_capacity(1),
            next_id: FIRST_FREE_ID,
        }
    }

    pub fn add_entity(&mut self, mut entity: Box<dyn Entity>) {
        let id = if entity.id() == INVALID_ID {
            let id = self.next_id();
            entity.set_id(id);
            id
        } else {
            entity.id()
        };
        self.entities.insert(id, entity);
    }

    // TODO: this is just a temporary solution
    pub fn clean_up(&mut self) {
        self.entities.retain(|&k, _| k == PLAYER_ID);
    }

    pub fn update(&mut self, room: &Room) {
        for entity in self.entities.values_mut() {
            entity.update(room);
        }
    }

    pub fn render(&self, screen: &mut Screen, x_offset: f32, y_offset: f32) {
        for entity in self.entities.values() {
            entity.render(screen, x_offset, y_offset);
        }
    }

    pub fn check_collisions(&mut self, dispatcher: &mut MessageDispatcher) {
        let mut colliding_entites = Vec::new();
        {
            let mut player = self.entities.remove(&PLAYER_ID).unwrap();
            for (k, ref e) in self.entities.iter() {
                if player.collides_with(&e.collider()) {
                    colliding_entites.push((*k, e.collider()));
                }
            }
            self.entities.insert(PLAYER_ID, player);
        }
        let mut player = self.entities.get_mut(&PLAYER_ID).unwrap();
        let epsilon = 0.0;
        for (id, collider) in colliding_entites {
            if let Some((enemy_x, enemy_y)) = collider.and_then(|c| Some(c.top_left())) {
                let (player_x, player_y) = player.collider().and_then(|c| Some(c.top_left())).unwrap();
                let x = (player_x - enemy_x);
                let y = (player_y - enemy_y);
                let x_dir = x.signum();
                let y_dir = y.signum();
                if x.abs() >= y.abs() {
                    let dist = 8.0 - x.abs() + epsilon;
                    player.set_pos(player_x + x_dir * dist, player_y);
                } else {
                    let dist = 8.0 - y.abs() + epsilon;
                    player.set_pos(player_x , player_y + y_dir * dist);
                }
            }
            dispatcher.queue_message(PLAYER_ID, id, Message::Collides);
            dispatcher.queue_message(id, PLAYER_ID, Message::Collides);
        }
    }

    pub fn get_entity_mut(&mut self, id: &EntityId) -> &mut Box<Entity> {
        self.entities.get_mut(id).unwrap()
    }

    fn next_id(&mut self) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

pub struct MessageDispatcher {
    messages_to_game: VecDeque<Telegram>,
    messages: VecDeque<Telegram>,
}

impl MessageDispatcher {
    pub fn new() -> Self {
        MessageDispatcher {
            messages_to_game : VecDeque::new(),
            messages : VecDeque::new(),
        }
    }

    pub fn poll_game_message(&mut self) -> Option<Telegram> {
        self.messages_to_game.pop_front()
    }

    fn discharge(&mut self, manager: &mut EntityManager, message: Telegram) {
        let ref mut entity = manager.get_entity_mut(&message.receiver);
        entity.handle_message(message, self);
    }

    pub fn queue_message(&mut self, sender: EntityId, receiver: EntityId, message: Message) {
        let telegram = Telegram {
            sender,
            receiver,
            message,
        };

        if receiver == GAME_ID {
            self.messages_to_game.push_back(telegram);
            return;
        } else {
            self.messages.push_back(telegram);
        }
    }

    pub fn dispatch_messages(&mut self, manager: &mut EntityManager) {
        while let Some(message) = self.messages.pop_front() {
            self.discharge(manager, message)
        }
    }
}
