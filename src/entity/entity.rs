use super::super::graphics::Screen;
use super::super::graphics::{SPRITE_SIZE_U32, SPRITE_SIZE_F32};
use super::super::level::{Room, RoomId};
use cgmath::Vector2;
use super::Projectile;

use std::boxed::Box;
use std::collections::{HashMap, VecDeque};

pub type EntityId = u32;


pub const INVALID_ID: EntityId = 0;
pub const GAME_ID: EntityId = 1;
pub const PLAYER_ID: EntityId = 2;
pub const ENTITY_MANAGER_ID: EntityId = 3;
const FIRST_FREE_ID: EntityId = 10;
const EPSILON: Vector2<f32> = Vector2::new(0.005, 0.005);



pub trait Entity {
    fn update(&mut self, room: &Room, dispatcher: &mut MessageDispatcher);
    fn move_entity(&mut self, _distance: Vector2<f32>, _room: &Room) {}
    fn render(&self, screen: &mut Screen, _offset: Vector2<f32>);
    fn remove(&mut self);
    fn is_removed(&self) -> bool;
    fn set_pos(&mut self, _pos: Vector2<f32>) {}
    fn relative_pos(&self, _offset: Vector2<f32>) -> Vector2<i32>;
    fn absolute_pos(&self) -> Vector2<i32>;
    fn collider(&self) -> Option<Collider> {
        None
    }
    fn collides_with(&mut self, _other: &Option<Collider>) -> bool {
        false
    }
    fn collides(&self) -> bool {
        false
    }
    fn id(&self) -> EntityId;
    fn set_id(&mut self, _id: EntityId) {}
    fn handle_message(&mut self, _message: Telegram, _dispatcher: &mut MessageDispatcher) {}
    fn send_message(&self, _message: Message, _receiver: EntityId, _dispatcher: &mut MessageDispatcher) {}
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum CollisionKind {
    Friendly,
    Hostile,
}

#[derive(Debug, Copy, Clone)]
pub struct Collider {
    pub origin: Vector2<f32>,
    pub dimensions: Vector2<f32>,
    pub kind: CollisionKind,
}

impl Collider {
    pub fn new(origin: Vector2<f32>, dimensions: Vector2<f32>, kind: CollisionKind) -> Collider {
        Collider {
            origin,
            dimensions,
            kind,
        }
    }

    pub fn intersects(&self, other: &Collider) -> bool {
        let l1 = self.origin + EPSILON;
        let r1 = self.origin + self.dimensions - EPSILON;
        let l2 = other.origin + EPSILON;
        let r2 = other.origin + other.dimensions - EPSILON;

        // If one rectangle is on left side of other
        if l1.x >= r2.x || l2.x >= r1.x {
            return false;
        }

        // If one rectangle is above other
        if l1.y >= r2.y || l2.y >= r1.y {
            return false;
        }

        true
    }

    pub fn origin(&self) -> Vector2<f32> {
        self.origin
    }

    pub fn hostile(&self) -> bool { self.kind == CollisionKind::Hostile }
}

#[derive(Debug, Copy, Clone)]
pub struct Door {
    pub collider: Collider,
    id: EntityId,
    pub room: RoomId,
    pub removed: bool,
}

static DOOR_COLLIDER_DIMS: Vector2<f32> = Vector2::new(1., 1.);
static DOOR_COLLIDER_OFFSET: Vector2<f32> = Vector2::new(SPRITE_SIZE_F32 / 2., SPRITE_SIZE_F32 / 2.);

impl<'a> From<&'a (Vector2<u32>, RoomId)> for Door {
    fn from(info: &(Vector2<u32>, RoomId)) -> Self {
        Door {
            collider: Collider::new(
                (info.0 * SPRITE_SIZE_U32).cast().unwrap() + DOOR_COLLIDER_OFFSET,
                DOOR_COLLIDER_DIMS,
                CollisionKind::Hostile,
            ),
            id: INVALID_ID,
            room: info.1,
            removed: false,
        }
    }
}

impl Entity for Door {
    fn update(&mut self, _room: &Room, _dispatcher: &mut MessageDispatcher) {}

    fn render(&self, _screen: &mut Screen, _offset: Vector2<f32>) {}

    fn remove(&mut self) {
        self.removed = true;
    }

    fn is_removed(&self) -> bool {
        self.removed
    }

    fn relative_pos(&self, offset: Vector2<f32>) -> Vector2<i32> {
        (self.collider.origin() - offset).cast().unwrap()
    }

    fn absolute_pos(&self) -> Vector2<i32> {
        self.collider.origin().cast().unwrap()
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
        let Telegram { sender, receiver: _, message } = message;
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
    SpawnEntity(Vector2<f32>, Vector2<f32>, f32),
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

    pub fn handle_message(&mut self, message: Telegram, dispatcher: &mut MessageDispatcher) {
        let Telegram { sender, receiver: _, message } = message;
        match message {
            Message::SpawnEntity(position, heading, speed) => {
                let mut projectile = Projectile::new(
                    position,
                    heading,
                    speed,
                    INVALID_ID,
                );
                self.add_entity(Box::new(projectile));
            }
            _ => {}
        }
    }

    // TODO: this is just a temporary solution
    pub fn clean_up(&mut self) {
        self.entities.retain(|&k, _| k == PLAYER_ID);
    }

    pub fn update(&mut self, room: &Room, dispatcher: &mut MessageDispatcher) {
        self.entities.retain(|id, entity| {
            entity.update(room, dispatcher);
            !entity.is_removed()
        });
    }

    pub fn render(&self, screen: &mut Screen, offset: Vector2<f32>) {
        for entity in self.entities.values() {
            entity.render(screen, offset);
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
        let player = self.entities.get_mut(&PLAYER_ID).unwrap();
        let epsilon = 0.0;
        for (id, collider) in colliding_entites {
            if let Some(Vector2 {x: enemy_x, y: enemy_y}) = collider.and_then(|c| Some(c.origin())) {
                let Vector2 {x: collider_x, y: collider_y} = player.collider().and_then(|c| Some(c.origin())).unwrap();
                let x = collider_x - enemy_x;
                let y = collider_y - enemy_y;
                let x_dir = x.signum();
                let y_dir = y.signum();
                let Vector2 {x: player_x, y: player_y} = player.absolute_pos().cast().unwrap();
                if x.abs() >= y.abs() {
                    let dist = SPRITE_SIZE_F32 + 4.0 * x_dir  - x.abs() + epsilon;
                    player.set_pos((player_x + x_dir * dist, player_y).into());
                } else {
                    let offset = if y_dir < 0.0 {
                        8.0 * y_dir
                    } else {
                        0.0
                    };
                    let dist = SPRITE_SIZE_F32 + offset - y.abs() + epsilon;
                    player.set_pos((player_x, player_y + y_dir * dist).into());
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
    game_messages: VecDeque<Telegram>,
    entity_messages: VecDeque<Telegram>,
    messages: VecDeque<Telegram>,
}

impl MessageDispatcher {
    pub fn new() -> Self {
        MessageDispatcher {
            game_messages: VecDeque::new(),
            entity_messages: VecDeque::new(),
            messages : VecDeque::new(),
        }
    }

    pub fn poll_game_message(&mut self) -> Option<Telegram> {
        self.game_messages.pop_front()
    }

    pub fn poll_entity_message(&mut self) -> Option<Telegram> {
        self.entity_messages.pop_front()
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
            self.game_messages.push_back(telegram);
            return;
        } else if receiver == ENTITY_MANAGER_ID {
            self.entity_messages.push_back(telegram);
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
