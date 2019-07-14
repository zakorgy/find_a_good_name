use crate::entity::{Collider, CollisionKind, Direction, Entity, EntityId, MessageDispatcher, INVALID_ID};
use crate::graphics::{screen::Screen, sprite::AnimatedSprite};
use crate::level::room::Room;
use cgmath::Vector2;
use image::{GenericImageView, Rgba};

pub struct Enemy {
    position: Vector2<f32>,
    _speed: f32,
    _direction: Direction,
    removed: bool,
    sprite: AnimatedSprite,
    _collides: bool,
    flipped: bool,
    id: EntityId,
}

impl Enemy {
    pub fn new(position: Vector2<f32>, _speed: f32, sprite: AnimatedSprite) -> Enemy {
        Enemy {
            position,
            _speed,
            _direction: Direction::Right,
            removed: false,
            sprite,
            _collides: false,
            flipped: false,
            id: INVALID_ID,
        }
    }
}

impl Entity for Enemy {
    fn update(&mut self, _room: &Room, _dispatcher: &mut MessageDispatcher) {
        self.sprite.update()
    }

    fn render(&self, screen: &mut Screen, offset: Vector2<f32>) {
        let pixels = self.sprite.view();
        let Vector2 { x: ax, y: ay } = self.relative_pos(offset);
        for y in 0..self.sprite.size() {
            for x in 0..self.sprite.size() {
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
                        || y == self.sprite.size() - 1
                        || x == 0
                        || x == self.sprite.size() - 1
                    {
                        screen.canvas.put_pixel(
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
                    if self.flipped {
                        self.sprite.size() - 1 - x
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
        let sprite_size = self.sprite.size() as f32;
        Some(Collider::new(
            self.position,
            (sprite_size, sprite_size).into(),
            CollisionKind::Hostile,
        ))
    }

    fn id(&self) -> EntityId {
        self.id
    }
}
