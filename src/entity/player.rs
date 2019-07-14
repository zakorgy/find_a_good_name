use crate::entity::{Collider, CollisionKind, Direction, Entity, EntityId, Message, MessageDispatcher, Telegram, ENTITY_MANAGER_ID, PLAYER_ID};
use crate::graphics::{
    screen::Screen,
    sprite::{AnimatedSprite, SPRITE_SIZE_F32, SPRITE_SIZE_U32},
};
use crate::input::keyboard::KeyBoard;
use crate::level::room::Room;
use cgmath::Vector2;
use image::{RgbaImage, SubImage};
use piston::input::Key;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Player {
    position: Vector2<f32>,
    speed: f32,
    direction: Direction,
    shoot_direction: Option<Direction>,
    removed: bool,
    sprites: std::collections::HashMap<Direction, AnimatedSprite>,
    collides: bool,
    id: EntityId,
    keyboard: Rc<RefCell<KeyBoard>>,
}

impl Player {
    pub fn new(
        speed: f32,
        sprites: Vec<(Direction, AnimatedSprite)>,
        keyboard: Rc<RefCell<KeyBoard>>,
        id: EntityId,
    ) -> Player {
        Player {
            position: (0., 0.).into(),
            speed,
            direction: Direction::Right,
            shoot_direction: None,
            removed: false,
            sprites: sprites.into_iter().collect(),
            collides: false,
            id,
            keyboard,
        }
    }

    fn collision(&self, room: &Room, offset: Vector2<f32>) -> bool {
        let collider = self.collider().unwrap();
        let xy = (collider.origin + offset).cast::<i32>().unwrap();
        let xy0 = xy / SPRITE_SIZE_U32 as i32;
        let size_minus_one = collider.dimensions.x as i32 - 1;
        let xy7 = (xy + Vector2::new(size_minus_one, size_minus_one)) / SPRITE_SIZE_U32 as i32;
        match self.direction {
            Direction::Up => room.get_tile(xy0.x, xy0.y).solid || room.get_tile(xy7.x, xy0.y).solid,
            Direction::Down => {
                room.get_tile(xy0.x, xy7.y).solid || room.get_tile(xy7.x, xy7.y).solid
            }
            Direction::Right => {
                room.get_tile(xy7.x, xy0.y).solid || room.get_tile(xy7.x, xy7.y).solid
            }
            Direction::Left => {
                room.get_tile(xy0.x, xy0.y).solid || room.get_tile(xy0.x, xy7.y).solid
            }
        }
    }

    fn middle_point(&self) -> Vector2<f32> {
        self.position + Vector2::new(SPRITE_SIZE_F32 / 2., SPRITE_SIZE_F32 / 2.)
    }

    fn sprite(&self) -> &AnimatedSprite {
        &self.sprites[&self.shoot_direction.unwrap_or(self.direction)]
    }

    fn sprite_mut(&mut self) -> &mut AnimatedSprite {
        self.sprites
            .get_mut(&self.shoot_direction.unwrap_or(self.direction))
            .unwrap()
    }
}

impl Entity for Player {
    fn move_entity(&mut self, distance: Vector2<f32>, room: &Room) {
        if distance.y < 0.0 {
            self.direction = Direction::Up;
        }
        if distance.y > 0.0 {
            self.direction = Direction::Down;
        }
        if distance.x < 0.0 {
            self.direction = Direction::Left;
        }
        if distance.x > 0.0 {
            self.direction = Direction::Right;
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

        self.shoot_direction = None;
        let mut proj_heading = None;
        if self.keyboard.borrow().keys.contains(&Key::W) {
            self.shoot_direction = Some(Direction::Up);
            proj_heading = Some((0., -1.).into());
        }
        if self.keyboard.borrow().keys.contains(&Key::S) {
            self.shoot_direction = Some(Direction::Down);
            proj_heading = Some((0., 1.).into());
        }
        if self.keyboard.borrow().keys.contains(&Key::A) {
            self.shoot_direction = Some(Direction::Left);
            proj_heading = Some((-1., 0.).into());
        }
        if self.keyboard.borrow().keys.contains(&Key::D) {
            self.shoot_direction = Some(Direction::Right);
            proj_heading = Some((1., 0.).into());
        }

        if let Some(heading) = proj_heading {
            dispatcher.queue_message(
                PLAYER_ID,
                ENTITY_MANAGER_ID,
                Message::SpawnEntity(self.middle_point(), heading, 4.0),
            );
        }
        let mut update_sprite = false;

        if ya != 0.0 {
            self.move_entity((0.0, ya).into(), room);
            update_sprite = true;
        }

        if xa != 0.0 {
            self.move_entity((xa, 0.0).into(), room);
            update_sprite = true;
        }

        if update_sprite {
            self.sprite_mut().update();
        } else {
            self.sprite_mut().reset();
        }
    }

    fn sprite_view(&self) -> Option<SubImage<&RgbaImage>> {
        Some(self.sprite().view())
    }

    fn render(&self, screen: &mut Screen, offset: Vector2<f32>) {
        let flip = match self.shoot_direction.unwrap_or(self.direction) {
            Direction::Left => true,
            _ => false,
        };
        self.render_impl(screen, offset, flip);
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
        let _sprite_size = self.sprite().size() as f32;
        Some(Collider::new(
            self.position + Vector2::new(4.0, 8.0),
            (8.0, 8.0).into(),
            CollisionKind::Friendly,
        ))
    }

    fn collides_with(&mut self, other: &Option<Collider>) -> bool {
        if let Some(ref collider) = other {
            if collider.hostile() {
                let collides = self.collider().unwrap().intersects(collider);
                self.collides |= collides;
                return collides;
            }
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
