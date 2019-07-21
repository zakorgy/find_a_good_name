use crate::entity::{Collider, CollisionKind, Direction, Entity, EntityId, MessageDispatcher, INVALID_ID};
use crate::graphics::sprite::AnimatedSprite;
use crate::level::room::Room;
use cgmath::Vector2;
use image::{RgbaImage, SubImage};

pub struct Enemy {
    position: Vector2<f32>,
    _speed: f32,
    _direction: Direction,
    removed: bool,
    sprite: AnimatedSprite,
    _collides: bool,
    _flipped: bool,
    id: EntityId,
}

impl Enemy {
    pub fn new(position: Vector2<f32>, _speed: f32, sprite: AnimatedSprite) -> Enemy {
        Enemy {
            position,
            _speed,
            _direction: Direction::RIGHT,
            removed: false,
            sprite,
            _collides: false,
            _flipped: false,
            id: INVALID_ID,
        }
    }
}

impl Entity for Enemy {
    fn update(&mut self, _room: &Room, _dispatcher: &mut MessageDispatcher) {
        self.sprite.update()
    }

    fn sprite_view(&self) -> Option<SubImage<&RgbaImage>> {
        Some(self.sprite.view())
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
        Some(Collider::new(
            self.position + Vector2::new(1., 6.0),
            (14.0, 10.0).into(),
            CollisionKind::Hostile,
        ))
    }

    fn id(&self) -> EntityId {
        self.id
    }
}
