pub mod enemy;
pub mod player;
mod projectile;

use crate::graphics::{
    screen::Screen,
    sprite::{SPRITE_SIZE_F32, SPRITE_SIZE_U32}
};
use projectile::Projectile;
use crate::level::room::{Room, RoomId};
use cgmath::Vector2;
use image::{Rgba, GenericImageView, RgbaImage, SubImage};

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
    fn render(&self, screen: &mut Screen, offset: Vector2<f32>) {
        self.render_impl(screen, offset, false)
    }
    fn render_impl(&self, screen: &mut Screen, offset: Vector2<f32>, flip: bool) {
        let pixels = match self.sprite_view() {
            Some(view) => view,
            None => return,
        };
        let (width, height) = pixels.dimensions();
        let Vector2 { x: ax, y: ay } = self.relative_pos(offset);
        for y in 0..height {
            for x in 0..width {
                let xp = x as i32 + ax;
                let yp = y as i32 + ay;
                if xp < 0
                    || xp >= screen.dimensions.x as i32
                    || yp < 0
                    || yp >= screen.dimensions.y as i32
                {
                    continue;
                }
                #[cfg(feature = "debug_rect")]
                {
                    if y == 0
                        || y == height- 1
                        || x == 0
                        || x == width - 1
                    {
                        screen.put_pixel(
                            xp as u32,
                            yp as u32,
                            Rgba {
                                data: [255, 0, 255, 255],
                            },
                        );
                        continue;
                    }
                }
                let pixel = match pixels.get_pixel(
                    if flip {
                        width - 1 - x
                    } else {
                        x
                    },
                    y,
                ) {
                    Rgba {
                        data: [255, 0, 255, 255],
                    } => continue,
                    pixel => pixel,
                };
                screen.put_pixel(xp as u32, yp as u32, pixel);
            }
        }
        #[cfg(feature = "debug_rect")] {
            if let Some(collider) = self.collider() {
                let (width, height) = collider.dimensions.cast().unwrap().into();
                let Vector2 { x: ax, y: ay } = collider.relative_pos(offset);
                for y in 0..height {
                    for x in 0..width {
                        let xp = x as i32 + ax;
                        let yp = y as i32 + ay;
                        if y == 0
                            || y == height- 1
                            || x == 0
                            || x == width - 1
                        {
                            screen.put_pixel(
                                xp as u32,
                                yp as u32,
                                Rgba {
                                    data: [255, 255, 255, 255],
                                },
                            );
                        }
                    }
                }
            }
        }
    }
    fn sprite_view(&self) -> Option<SubImage<&RgbaImage>> {
        None
    }
    fn remove(&mut self);
    fn is_removed(&self) -> bool;
    fn set_pos(&mut self, _pos: Vector2<f32>) {}
    fn offset_pos(&mut self, pos: Vector2<f32>) {
        let abs_pos = self.absolute_pos().cast().unwrap();
        self.set_pos(pos + abs_pos);
    }
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
    fn send_message(
        &self,
        _message: Message,
        _receiver: EntityId,
        _dispatcher: &mut MessageDispatcher,
    ) {
    }
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

    pub fn hostile(&self) -> bool {
        self.kind == CollisionKind::Hostile
    }

    pub fn relative_pos(&self, offset: Vector2<f32>) -> Vector2<i32> {
        (self.origin - offset).cast().unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Door {
    pub collider: Collider,
    id: EntityId,
    pub room: RoomId,
    pub removed: bool,
}

static DOOR_COLLIDER_DIMS: Vector2<f32> = Vector2::new(1., 1.);
static DOOR_COLLIDER_OFFSET: Vector2<f32> =
    Vector2::new(SPRITE_SIZE_F32 / 2., SPRITE_SIZE_F32 / 2.);

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
        let Telegram {
            sender,
            receiver: _,
            message,
        } = message;
        match message {
            Message::Collides => {
                if sender == PLAYER_ID {
                    self.send_message(Message::LoadRoom(self.room), GAME_ID, dispatcher);
                }
            }
            _ => {}
        }
    }

    fn send_message(
        &self,
        message: Message,
        receiver: EntityId,
        dispatcher: &mut MessageDispatcher,
    ) {
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

    pub fn handle_message(&mut self, message: Telegram, _dispatcher: &mut MessageDispatcher) {
        let Telegram {
            sender: _,
            receiver: _,
            message,
        } = message;
        match message {
            Message::SpawnEntity(position, heading, speed) => {
                let projectile = Projectile::new(position, heading, speed, INVALID_ID);
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
        self.entities.retain(|_id, entity| {
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
                    colliding_entites.push((*k, e.collider().unwrap()));
                }
            }
            self.entities.insert(PLAYER_ID, player);
        }
        if colliding_entites.is_empty() {
            return
        }
        let player = self.entities.get_mut(&PLAYER_ID).unwrap();
        let player_collider = player.collider().unwrap();
        let epsilon = 0.5;
        for (id, collider) in colliding_entites {
            let Vector2 {
                x: enemy_x,
                y: enemy_y,
            } = collider.origin;
            let x = player_collider.origin.x - enemy_x;
            let y = player_collider.origin.y - enemy_y;
            let x_dir = x.signum();
            let y_dir = y.signum();
            if x.abs() >= y.abs() {
                // Picke the collider size of the top left entity
                let collider_size = if x_dir.is_sign_positive() {
                    collider.dimensions.x
                } else {
                    player_collider.dimensions.x
                };
                let dist = collider_size - x.abs() + epsilon;
                let offset = (x_dir * dist.ceil(), 0.).into();
                player.offset_pos(offset);
            } else {
                let collider_size = if y_dir.is_sign_positive() {
                    collider.dimensions.y
                } else {
                    player_collider.dimensions.y
                };
                let dist = collider_size - y.abs() + epsilon;
                let offset = (0. , y_dir * dist.ceil()).into();
                player.offset_pos(offset);
            }
            dispatcher.queue_message(PLAYER_ID, id, Message::Collides);
            dispatcher.queue_message(id, PLAYER_ID, Message::Collides);
        }
    }

    pub fn get_entity_mut(&mut self, id: &EntityId) -> &mut Box<dyn Entity> {
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
            messages: VecDeque::new(),
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
