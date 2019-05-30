use super::super::graphics::image::{GenericImageView, Rgba};
use super::super::graphics::{SPRITE_SIZE_SHIFT_VALUE};
use super::super::graphics::{AnimatedSprite, Screen};
use super::super::input::KeyBoard;
use super::super::level::Room;
use super::entity::{Collider, Direction, Entity, EntityId, Message, MessageDispatcher, Telegram};
use super::entity::{PLAYER_ID, ENTITY_MANAGER_ID};
use std::cell::RefCell;
use std::rc::Rc;
use piston::input::Key;
use cgmath::Vector2;

fn right_shift_vec(vec: Vector2<i32>, value: u32) -> Vector2<i32> {
    [vec.x>> value, vec.y>> value].into()
}

pub struct Player {
    position: Vector2<f32>,
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
        speed: f32,
        sprite: AnimatedSprite,
        keyboard: Rc<RefCell<KeyBoard>>,
        id: EntityId,
    ) -> Player {
        Player {
            position: (0., 0.).into(),
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

    fn collision(&self, room: &Room, offset: Vector2<f32>) -> bool {
        let xy = (self.position + offset).cast::<i32>().unwrap();
        let xy0 = right_shift_vec(xy, SPRITE_SIZE_SHIFT_VALUE);
        let size_minus_one = self.sprite.size() as i32 - 1;
        let xy7 = right_shift_vec(xy + Vector2::new(size_minus_one, size_minus_one), SPRITE_SIZE_SHIFT_VALUE);
        match self.direction {
            Direction::Up => room.get_tile(xy0.x, xy0.y).solid || room.get_tile(xy7.x, xy0.y).solid,
            Direction::Down => room.get_tile(xy0.x, xy7.y).solid || room.get_tile(xy7.x, xy7.y).solid,
            Direction::Right => room.get_tile(xy7.x, xy0.y).solid || room.get_tile(xy7.x, xy7.y).solid,
            Direction::Left => room.get_tile(xy0.x, xy0.y).solid || room.get_tile(xy0.x, xy7.y).solid,
        }
    }
}

impl Entity for Player {
    fn move_entity(&mut self, distance: Vector2<f32>, room: &Room) {
        if distance.x < 0.0 {
            self.direction = Direction::Left;
            self.flipped = true;
        }
        if distance.x > 0.0 {
            self.direction = Direction::Right;
            self.flipped = false;
        }
        if distance.y < 0.0 {
            self.direction = Direction::Up;
        }
        if distance.y > 0.0 {
            self.direction = Direction::Down;
        }
        if !self.collision(&room, distance) {
            self.position += distance;
        }
    }

    fn update(&mut self, room: &Room, dispatcher: &mut MessageDispatcher) {
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

        if self.keyboard.borrow().keys.contains(&Key::W) {
            println!("W pressed, create new projectile");
            dispatcher.queue_message(PLAYER_ID, ENTITY_MANAGER_ID, Message::SpawnEntity((self.position.x, self.position.x)));
        }
        let mut update_sprite = false;
        if xa != 0.0 {
            self.move_entity((xa, 0.0).into(), room);
            update_sprite = true;
        }

        if ya != 0.0 {
            self.move_entity((0.0, ya).into(), room);
            update_sprite = true;
        }

        if update_sprite {
            self.sprite.update();
        } else {
            self.sprite.reset();
        }
    }

    fn render(&self, screen: &mut Screen, offset: Vector2<f32>) {
        let pixels = self.sprite.view();
        let Vector2 {x: ax, y: ay} = self.relative_pos(offset);
        for y in 0..self.sprite.size() {
            for x in 0..self.sprite.size() {
                let xp = x as i32 + ax;
                let yp = y as i32 + ay;
                if xp < 0 || xp >= screen.dimensions.x as i32 || yp < 0 || yp >= screen.dimensions.y as i32 {
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

    fn relative_pos(&self, offset: Vector2<f32>) -> Vector2<i32> {
        (self.position - offset).cast().unwrap()
    }

    fn absolute_pos(&self) -> Vector2<i32> {
        self.position.cast::<i32>().unwrap()
    }

    fn collider(&self) -> Option<Collider> {
        let sprite_size = self.sprite.size() as f32;
        Some(Collider::new(self.position, (sprite_size, sprite_size).into()))
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

    fn handle_message(&mut self, _message: Telegram, _dispatcher: &mut MessageDispatcher) {
    }

    fn set_pos(&mut self, pos: Vector2<f32>) {
        self.position = pos;
    }
}
