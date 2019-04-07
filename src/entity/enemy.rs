use super::super::graphics::image::{GenericImageView, Rgba};
use super::super::graphics::{AnimatedSprite, Screen};
use super::super::level::Room;
use super::entity::{Direction, Entity, Mob};

pub struct Enemy {
    x: f32,
    y: f32,
    speed: f32,
    direction: Direction,
    removed: bool,
    sprite: AnimatedSprite,
    collides: bool,
    flipped: bool,
}

impl Enemy {
    pub fn new(x: f32, y: f32, speed: f32, sprite: AnimatedSprite) -> Enemy {
        Enemy {
            x,
            y,
            speed,
            direction: Direction::Right,
            removed: false,
            sprite,
            collides: false,
            flipped: false,
        }
    }
}

impl Entity for Enemy {
    fn update(&mut self) {}

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
                let pixel = match pixels.get_pixel(if self.flipped { 7 - x } else { x }, y) {
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
}

impl Mob for Enemy {
    fn move_entity(&mut self, x: f32, y: f32, room: &Room) {}

    fn update(&mut self, room: &Room) {
        self.sprite.update()
    }
}
