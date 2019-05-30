use super::super::graphics::image::{GenericImageView, Rgba};
use super::super::graphics::{AnimatedSprite, Screen};
use super::super::level::Room;
use super::entity::{Collider, Direction, Entity, EntityId, MessageDispatcher, INVALID_ID};

pub struct Enemy {
    x: f32,
    y: f32,
    _speed: f32,
    _direction: Direction,
    removed: bool,
    sprite: AnimatedSprite,
    _collides: bool,
    flipped: bool,
    id: EntityId,
}

impl Enemy {
    pub fn new(x: f32, y: f32, _speed: f32, sprite: AnimatedSprite) -> Enemy {
        Enemy {
            x,
            y,
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

    fn render(&self, screen: &mut Screen, x_offset: f32, y_offset: f32) {
        let pixels = self.sprite.view();
        let (ax, ay) = self.relative_pos(x_offset, y_offset);
        for y in 0..self.sprite.size() {
            for x in 0..self.sprite.size() {
                let xp = x as i32 + ax;
                let yp = y as i32 + ay;
                if xp < 0 || xp >= screen.width as i32 || yp < 0 || yp >= screen.height as i32 {
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
                let pixel = match pixels.get_pixel(if self.flipped { self.sprite.size() - 1 - x } else { x }, y) {
                    Rgba {
                        data: [255, 0, 255, 255],
                    } => continue,
                    pixel => pixel,
                };
                screen.canvas.put_pixel(xp as u32, yp as u32, pixel);
            }
        }
    }

    fn remove(&mut self) {
        self.removed = true;
    }

    fn is_removed(&self) -> bool {
        self.removed
    }

    fn relative_pos(&self, x_offset: f32, y_offset: f32) -> (i32, i32) {
        ((self.x - x_offset) as i32, (self.y - y_offset) as i32)
    }

    fn absolute_pos(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }

    fn collider(&self) -> Option<Collider> {
        let sprite_size = self.sprite.size() as f32;
        Some(Collider::new(self.x, self.y, sprite_size, sprite_size))
    }

    fn id(&self) -> EntityId {
        self.id
    }
}
