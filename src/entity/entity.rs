use super::super::graphics::Screen;
use super::super::level::Room;

use std::boxed::Box;
use std::collections::HashMap;

pub type EntityId = u32;


pub const GAME_ID: EntityId = 0;
pub const PLAYER_ID: EntityId = 1;
pub const ENTITY_MANAGER_ID: EntityId = 2;
pub const FIRST_FREE_ID: EntityId = 10;



pub trait Entity {
    fn update(&mut self, room: &Room);
    fn move_entity(&mut self, x: f32, y: f32, room: &Room) {}
    fn render(&self, screen: &mut Screen, x_offset: f32, y_offset: f32);
    fn remove(&mut self);
    fn is_removed(&self) -> bool;
    fn relative_pos(&self, x_offset: f32, y_offset: f32) -> (i32, i32);
    fn absolute_pos(&self) -> (i32, i32);
    fn collider(&self) -> Option<Collider> {
        None
    }
    fn collides(&self, other: &Option<Collider>) -> bool {
        false
    }
    fn id(&self) -> EntityId;
    fn handle_message(&mut self, message: Telegram, dispatcher: &mut MessageDispatcher) {}
    fn send_message(&self, message: Telegram, receiver: EntityId, dispatcher: &mut MessageDispatcher) {}
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
        let l1 = (self.x, self.y);
        let r1 = (self.x + self.width, self.y + self.height);
        let l2 = (other.x, other.y);
        let r2 = (other.x + other.width, other.y + other.height);

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
}

#[derive(Debug, Copy, Clone)]
pub struct Telegram {
    sender: EntityId,
    receiver: EntityId,
    message: Message,
}

#[derive(Debug, Copy, Clone)]
pub enum Message {
    LoadRoom,
}

pub struct EntityManager {
    entities: HashMap<EntityId, Box<dyn Entity>>,    
}

impl EntityManager {
    pub fn new() -> Self {
        EntityManager {
            entities: HashMap::with_capacity(1),
        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn Entity>) {
        self.entities.insert(entity.id(), entity);
    }

    pub fn remove_entity(&mut self, id: &EntityId) {
        self.entities.remove(id);
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

    pub fn check_collisions(&mut self) {
        let ref player = self.entities[&PLAYER_ID];
        for (k, ref e) in self.entities.iter() {
            if k != &PLAYER_ID && player.collides(&e.collider()) {
                println!("#### Colliding with {}", k);
            }
        }
    }

    pub fn get_entity_from_id(&mut self, id: &EntityId) -> &mut Box<Entity> {
        self.entities.get_mut(id).unwrap()
    }
}

pub struct MessageDispatcher {
    //messages: Vec<Telegram>,
}

impl MessageDispatcher {
    fn discharge(&mut self, entity: &mut Box<Entity>, message: Telegram) {
        entity.handle_message(message, self);
    }

    pub fn dispatch_message(&mut self, entity_manager: &mut EntityManager, sender: EntityId, receiver: EntityId, message: Message) {
        let message = Telegram {
            sender,
            receiver,
            message,
        };
        let ref mut entity = entity_manager.get_entity_from_id(&receiver);
        self.discharge(entity, message);

    }
}
