use super::super::graphics::Screen;
use super::super::level::Room;

pub trait Entity {
    fn update(&mut self);
    fn render(&self, screen: &mut Screen, x_offset: f32, y_offset: f32);
    fn remove(&mut self);
    fn is_removed(&self) -> bool;
    fn relative_pos(&self, x_offset: f32, y_offset: f32) -> (i32, i32);
    fn absolute_pos(&self) -> (i32, i32);
    fn collider(&self) -> Option<Collider> {
        None
    }
    fn collides(&self, other: &Option<Collider>) -> bool {
        false
    }
}

pub trait Mob: Entity {
    fn move_entity(&mut self, x: f32, y: f32, room: &Room);
    fn update(&mut self, room: &Room);
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub struct Collider {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Collider {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Collider {
        Collider {
            x,
            y,
            width,
            height,
        }
    }

    pub fn intersects(&self, other: &Collider) -> bool {
        let l1 = (self.x, self.y);
        let r1 = (self.x + self.width, self.y + self.height);
        let l2 = (other.x, other.y);
        let r2 = (other.x + other.width, other.y + other.height);

        // If one rectangle is on left side of other
        if (l1.0 >= r2.0 || l2.0 >= r1.0) {
            return false;
        }

        // If one rectangle is above other
        if (l1.1 >= r2.1 || l2.1 >= r1.1) {
            return false;
        }

        true
    }
}
