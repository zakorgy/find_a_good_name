use crate::entity::{
    Collider, CollisionKind, Direction, Entity, EntityId, Message, MessageDispatcher, Telegram, ENTITY_MANAGER_ID, PLAYER_ID,
    state::{State, StateMachine},
    moving_component::{MovingComponent, Force},
};
use crate::graphics::{
    screen::Screen,
    sprite::{AnimatedSprite, SPRITE_SIZE_F32, SPRITE_SIZE_U32},
};
use crate::input::keyboard::KeyBoard;
use crate::level::room::Room;
use cgmath::{InnerSpace, Vector2};
use image::{SubImage, RgbaImage};
use piston::input::Key;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Copy, Eq, PartialEq)]
enum PlayerState {
    Walking,
    Flying,
    Damaged,
    Dying,
}

impl State<Player> for PlayerState {
    fn enter(&self, owner: &mut Player, dispatcher: &mut MessageDispatcher) {}
    fn execute(&self, owner: &mut Player, dispatcher: &mut MessageDispatcher) {
        match *self {
            PlayerState::Walking => {
                owner.collides = false;
                let mut normalize = false;
                owner.direction = Direction::empty();
                let mut force: Vector2<f32> = (0., 0.).into();
                if owner.keyboard.borrow().up {
                    force.y -= 1.;
                    owner.direction |= Direction::UP;
                    owner.sprite_direction = Direction::UP;
                    normalize = true;
                }
                if owner.keyboard.borrow().down {
                    force.y += 1.;
                    owner.direction |= Direction::DOWN;
                    owner.sprite_direction = Direction::DOWN;
                    normalize = true;
                }
                if owner.keyboard.borrow().left {
                    force.x -= 1.;
                    owner.direction |= Direction::LEFT;
                    owner.sprite_direction = Direction::LEFT;
                    normalize = true;
                }
                if owner.keyboard.borrow().right {
                    force.x += 1.;
                    owner.direction |= Direction::RIGHT;
                    owner.sprite_direction = Direction::RIGHT;
                    normalize = true;
                }
                if normalize {
                    owner.moving.set_thrust(Force::new(force.normalize(), 2));
                }

                owner.shoot_direction = None;
                let mut proj_heading = None;
                if owner.keyboard.borrow().keys.contains(&Key::W) {
                    owner.shoot_direction = Some(Direction::UP);
                    proj_heading = Some((0., -1.).into());
                }
                if owner.keyboard.borrow().keys.contains(&Key::S) {
                    owner.shoot_direction = Some(Direction::DOWN);
                    proj_heading = Some((0., 1.).into());
                }
                if owner.keyboard.borrow().keys.contains(&Key::A) {
                    owner.shoot_direction = Some(Direction::LEFT);
                    proj_heading = Some((-1., 0.).into());
                }
                if owner.keyboard.borrow().keys.contains(&Key::D) {
                    owner.shoot_direction = Some(Direction::RIGHT);
                    proj_heading = Some((1., 0.).into());
                }

                if let Some(heading) = proj_heading {
                    dispatcher.queue_message(
                        PLAYER_ID,
                        ENTITY_MANAGER_ID,
                        Message::SpawnEntity(owner.middle_point(), heading, 4.0),
                    );
                }
            }
            _ => {}
        }
    }
    fn exit(&self, owner: &mut Player, dispatcher: &mut MessageDispatcher) {}
}

pub struct Player {
    moving: MovingComponent,
    direction: Direction,
    sprite_direction: Direction,
    shoot_direction: Option<Direction>,
    removed: bool,
    sprites: std::collections::HashMap<Direction, AnimatedSprite>,
    collides: bool,
    id: EntityId,
    keyboard: Rc<RefCell<KeyBoard>>,
    states: Option<StateMachine<Self, PlayerState>>,
}

impl Player {
    pub fn new(
        max_speed: f32,
        sprites: Vec<(Direction, AnimatedSprite)>,
        keyboard: Rc<RefCell<KeyBoard>>,
        id: EntityId,
    ) -> Player {
        Player {
            moving: MovingComponent::new(
                5.0, // mass
                max_speed,
                5.0, // max_force
            ),
            direction: Direction::RIGHT,
            sprite_direction: Direction::RIGHT,
            shoot_direction: None,
            removed: false,
            sprites: sprites.into_iter().collect(),
            collides: false,
            id,
            keyboard,
            states: Some(StateMachine::new(None, PlayerState::Walking)),
        }
    }

    fn collision(&mut self, room: &Room) -> bool {
        let collider = self.collider().unwrap();
        let xy = collider.origin.cast::<i32>().unwrap();
        let xy0 = xy / SPRITE_SIZE_U32 as i32;
        let size_minus_one = collider.dimensions.x as i32 - 1;
        let xy7 = (xy + Vector2::new(size_minus_one, size_minus_one)) / SPRITE_SIZE_U32 as i32;

        if /*self.direction.contains(Direction::UP)  &&*/
            (room.get_tile(xy0.x, xy0.y).solid || room.get_tile(xy7.x, xy0.y).solid) {
                return true
            }
        
        if /*self.direction.contains(Direction::DOWN)  &&*/
            (room.get_tile(xy0.x, xy7.y).solid || room.get_tile(xy7.x, xy7.y).solid) {
                return true
            }
        
        if /*self.direction.contains(Direction::RIGHT)  &&*/
            (room.get_tile(xy7.x, xy0.y).solid || room.get_tile(xy7.x, xy7.y).solid) {
                return true
            }
        
        if /*self.direction.contains(Direction::LEFT)  &&*/
            (room.get_tile(xy0.x, xy0.y).solid || room.get_tile(xy0.x, xy7.y).solid) {
                return true
            }
        
        false
    }

    fn middle_point(&self) -> Vector2<f32> {
        self.moving.pos() + Vector2::new(SPRITE_SIZE_F32 / 2., SPRITE_SIZE_F32 / 2.)
    }

    fn sprite(&self) -> &AnimatedSprite {
        &self.sprites[&self.shoot_direction.unwrap_or(self.sprite_direction)]
    }

    fn sprite_mut(&mut self) -> &mut AnimatedSprite {
        self.sprites
            .get_mut(&self.shoot_direction.unwrap_or(self.sprite_direction))
            .unwrap()
    }
}

impl Entity for Player {
    fn move_entity(&mut self, forces: &[Force], room: &Room) -> bool {
        let old_pos = self.moving.pos();
        let moved = self.moving.update(forces, true);
        let new_pos  = self.moving.pos();
        self.moving.set_pos((new_pos.x, old_pos.y).into());
        if self.collision(&room) {
            println!("Collision1");
            self.moving.set_pos(old_pos);
        }
        let old_pos = self.moving.pos();
        self.moving.set_pos((old_pos.x, new_pos.y).into());
        if self.collision(&room) {
            println!("Collision2");
            self.moving.set_pos(old_pos);
        }
        println!("-----");
        moved
    }

    fn update(&mut self, room: &Room, dispatcher: &mut MessageDispatcher) {
        let states = self.states.take().unwrap();
        states.update(self, dispatcher);
        self.states = Some(states);
        let update_sprite = self.moving.thrust().has_magnitude();
        self.move_entity(&[], room);
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
        let flip = self.shoot_direction.unwrap_or(self.direction).contains(Direction::LEFT);
        self.render_impl(screen, offset, flip);
    }

    fn reset_pos(&mut self) {
        self.moving.reset_pos();
    }

    fn remove(&mut self) {
        self.removed = true;
    }

    fn is_removed(&self) -> bool {
        self.removed
    }

    fn relative_pos(&self, offset: Vector2<f32>) -> Vector2<i32> {
        (self.moving.pos() - offset).cast().unwrap()
    }

    fn absolute_pos(&self) -> Vector2<i32> {
        self.moving.pos().cast::<i32>().unwrap()
    }

    fn collider(&self) -> Option<Collider> {
        let _sprite_size = self.sprite().size() as f32;
        Some(Collider::new(
            self.moving.pos() + Vector2::new(3.0, 6.0),
            (10.0, 10.0).into(),
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
        self.moving.set_pos(pos);
    }
}
