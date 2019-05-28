use super::super::graphics::image::{GenericImageView, Rgba};
use super::super::graphics::{AnimatedSprite, Screen};
use super::super::input::KeyBoard;
use super::super::level::Room;
use super::entity::{Collider, Direction, Entity, EntityId, Message, MessageDispatcher, Telegram};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Player {
    x: f32,
    y: f32,
    speed: f32,
    direction: Direction,
    removed: bool,
    sprite: AnimatedSprite,
    collides: bool,
    flipped: bool,
    id: EntityId,
    keyboard: Rc<RefCell<KeyBoard>>,
}

impl Player {
    pub fn new(
        x: f32,
        y: f32,
        speed: f32,
        sprite: AnimatedSprite,
        keyboard: Rc<RefCell<KeyBoard>>,
        id: EntityId,
    ) -> Player {
        Player {
            x,
            y,
            speed,
            direction: Direction::Right,
            removed: false,
            sprite,
            collides: false,
            flipped: false,
            id,
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
            Direction::Up => room.get_tile(x0, y0).solid || room.get_tile(x7, y0).solid,
            Direction::Down => room.get_tile(x0, y7).solid || room.get_tile(x7, y7).solid,
            Direction::Right => room.get_tile(x7, y0).solid || room.get_tile(x7, y7).solid,
            Direction::Left => room.get_tile(x0, y0).solid || room.get_tile(x0, y7).solid,
        }
    }
}

impl Entity for Player {
    fn move_entity(&mut self, x: f32, y: f32, room: &Room) {
        if x < 0.0 {
            self.direction = Direction::Left;
            self.flipped = true;
        }
        if x > 0.0 {
            self.direction = Direction::Right;
            self.flipped = false;
        }
        if y < 0.0 {
            self.direction = Direction::Up;
        }
        if y > 0.0 {
            self.direction = Direction::Down;
        }
        if !self.collision(&room, x, y) {
            self.x += x;
            self.y += y;
        }
    }

    fn update(&mut self, room: &Room) {
        self.collides = false;
        let (mut xa, mut ya) = (0.0, 0.0);
        if self.keyboard.borrow().up {
            ya -= self.speed;
        }
        if self.keyboard.borrow().down {
            ya += self.speed;
        }
        if self.keyboard.borrow().left {
            xa -= self.speed
        }
        if self.keyboard.borrow().right {
            xa += self.speed
        }
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

    fn collider(&self) -> Option<Collider> {
        let sprite_size = self.sprite.size() as f32;
        Some(Collider::new(self.x, self.y, sprite_size, sprite_size))
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

    fn handle_message(&mut self, message: Telegram, dispatcher: &mut MessageDispatcher) {
        let Telegram { sender, receiver, message } = message;
    }

    fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
}
