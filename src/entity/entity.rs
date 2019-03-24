use std::cell::RefCell;
use std::rc::Rc;
use super::super::graphics::{AnimatedSprite, Screen};
use super::super::input::KeyBoard;
use super::super::level::Room;
use super::super::graphics::image::{GenericImageView, Rgba};

pub trait Entity {
    fn update(&mut self);
    fn render(&self, screen: &mut Screen, x_offset: f32, y_offset: f32);
    fn remove(&mut self);
    fn is_removed(&self) -> bool;
    fn relative_pos(&self, x_offset: f32, y_offset: f32) -> (i32, i32);
    fn absolute_pos(&self) -> (i32, i32);
}

pub trait Mob: Entity {
    fn move_entity(&mut self, x: f32, y: f32, room: &Room);
    fn update(&mut self, room: &Room);
}

pub struct Player {
    x: f32,
    y: f32,
    speed: f32,
    direction: Direction,
    removed: bool,
    sprite: AnimatedSprite,
    collides: bool,
    flipped: bool,
    keyboard: Rc<RefCell<KeyBoard>>,
}


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Player {
    pub fn new(x: f32, y: f32, speed: f32, sprite: AnimatedSprite, keyboard: Rc<RefCell<KeyBoard>>) -> Player {
        Player {
            x,
            y,
            speed,
            direction: Direction::Right,
            removed: false,
            sprite,
            collides: false,
            flipped: false,
            keyboard,
        }
    }

    fn collision(&self, room: &Room, x_offset: f32, y_offset: f32) -> bool {
        let x = (self.x + x_offset) as i32;
        let y = (self.y + y_offset) as i32;
        let x0 = x /*+ 1*/ >> 3;
        let y0 = y >> 3;
        let x7 = (x + 7) >> 3;
        let y7 = (y + 7) >> 3;
        match self.direction {
            Direction::Up => {
                room.get_tile(x0, y0).solid || room.get_tile(x7, y0).solid
            },
            Direction::Down => {
                room.get_tile(x0, y7).solid || room.get_tile(x7, y7).solid
            },
            Direction::Right => {
                room.get_tile(x7, y0).solid || room.get_tile(x7, y7).solid
            },
            Direction::Left => {
                room.get_tile(x0, y0).solid || room.get_tile(x0, y7).solid
            },
        }
    }
}

impl Entity for Player {
    fn update(&mut self) {}

    fn render(&self, screen: &mut Screen, x_offset: f32, y_offset: f32) {
        let pixels = self.sprite.view();
        let (ax, ay) = self.relative_pos(x_offset, y_offset);
        for y in 0 .. self.sprite.size() {
            for x in 0 .. self.sprite.size() {
                let xp = x as i32 + ax;
                let yp = y as i32 + ay;
                if xp < 0 || xp >= screen.width as i32
                    || yp < 0 ||yp >= screen.height as i32 {
                    continue;
                }
                #[cfg(feature = "debug_rect")]
                {
                    if y == 0 || y == self.sprite.size() - 1 || x == 0 || x == self.sprite.size() - 1 {
                        screen.canvas.put_pixel(xp as u32, yp as u32, Rgba {data: [255, 0, 255, 255]});
                        continue;
                    }
                }
                let pixel = match pixels.get_pixel(if self.flipped { 7 - x } else { x }, y) {
                    Rgba {data: [255, 0, 255, 255]} => continue,
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

impl Mob for Player {
    fn move_entity(&mut self, x: f32, y: f32, room: &Room) {
        if x < 0.0 {
            self.direction = Direction::Left;
            self.flipped = true;
        }
        if x > 0.0 {
            self.direction = Direction::Right;
            self.flipped = false;
        }
        if y < 0.0 {self.direction = Direction::Up;}
        if y > 0.0 {self.direction = Direction::Down;}
        if self.collision(&room, x, y) {
            self.collides = true;
        } else {
            self.x += x;
            self.y += y;
            self.collides = false;
        }
    }

    fn update(&mut self, room: &Room) {
        let (mut xa, mut ya) = (0.0, 0.0);
        if self.keyboard.borrow().up { ya -= self.speed ; }
        if self.keyboard.borrow().down { ya += self.speed; }
        if self.keyboard.borrow().left { xa -= self.speed }
        if self.keyboard.borrow().right { xa += self.speed }
        let mut update_sprite = false;
        if xa != 0.0 {
            self.move_entity(xa, 0.0, room);
            update_sprite = true;
        }

        if ya != 0.0 {
            self.move_entity(0.0, ya, room);
            update_sprite = true;
        }

        if update_sprite {
            self.sprite.update();
        } else {
            self.sprite.reset();
        }
    }
}
