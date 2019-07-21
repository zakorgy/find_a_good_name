use crate::entity::{Collider, CollisionKind, Entity, EntityId, MessageDispatcher, Telegram};
use crate::graphics::{
    screen::Screen,
    sprite::SPRITE_SIZE_SHIFT_VALUE,
};
use crate::level::room::Room;
use cgmath::Vector2;
use image::Rgba;

pub struct Projectile {
    position: Vector2<f32>,
    heading: Vector2<f32>,
    speed: f32,
    removed: bool,
    id: EntityId,
    collides: bool,
    //sprite: Sprite,
}

impl Projectile {
    pub fn new(
        position: Vector2<f32>,
        heading: Vector2<f32>,
        speed: f32,
        id: EntityId,
    ) -> Projectile {
        Projectile {
            position,
            heading,
            speed,
            removed: false,
            id,
            collides: false,
        }
    }

    fn collision(&self, _room: &Room, _offset: Vector2<f32>) -> bool {
        //        let xy = (self.position + offset).cast::<i32>().unwrap();
        //        let xy0 = right_shift_vec(xy, SPRITE_SIZE_SHIFT_VALUE);
        false
    }
}

impl Entity for Projectile {
    fn move_entity(&mut self, forces: &[Vector2<f32>], room: &Room) {
        if !self.collision(&room, forces[0]) {
            self.position += forces[0];
        }
    }

    fn update(&mut self, room: &Room, _dispatcher: &mut MessageDispatcher) {
        if self.position.x < 0.
            || self.position.x > (room.dimensions.x << SPRITE_SIZE_SHIFT_VALUE) as f32
            || self.position.y < 0.
            || self.position.y > (room.dimensions.y << SPRITE_SIZE_SHIFT_VALUE) as f32
        {
            self.remove();
        }
        let dist = self.heading * self.speed;
        self.move_entity(&[dist], room);
    }

    fn render(&self, screen: &mut Screen, offset: Vector2<f32>) {
        if let Some(Vector2 { x: ax, y: ay }) = self.relative_pos(offset).cast() {
            screen.put_pixel(
                ax,
                ay,
                Rgba {
                    data: [0, 0, 0, 255],
                },
            );
        }
    }

    fn remove(&mut self) {
        self.removed = true;
    }

    fn is_removed(&self) -> bool {
        self.removed
    }

    fn relative_pos(&self, offset: Vector2<f32>) -> Vector2<i32> {
        (self.position - offset).cast().unwrap()
    }

    fn absolute_pos(&self) -> Vector2<i32> {
        self.position.cast::<i32>().unwrap()
    }

    fn collider(&self) -> Option<Collider> {
        let sprite_size = 1.; //self.sprite.size() as f32;
        Some(Collider::new(
            self.position,
            (sprite_size, sprite_size).into(),
            CollisionKind::Friendly,
        ))
    }

    fn collides_with(&mut self, other: &Option<Collider>) -> bool {
        if let Some(ref collider) = other {
            let collides = self.collider().unwrap().intersects(collider);
            self.collides |= collides;
            return collides;
        }
        false
    }

    fn collides(&self) -> bool {
        self.collides
    }

    fn id(&self) -> EntityId {
        self.id
    }

    fn handle_message(&mut self, _message: Telegram, _dispatcher: &mut MessageDispatcher) {}

    fn set_pos(&mut self, pos: Vector2<f32>) {
        self.position = pos;
    }
}
