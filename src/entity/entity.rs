use super::super::graphics::{Screen, Sprite};
use super::super::input::KeyBoard;
use super::super::level::Level;
use super::super::graphics::image::{GenericImageView, Rgba};

pub trait Entity {
    fn update(&mut self);
    fn render(&self, screen: &mut Screen);
    fn remove(&mut self);
    fn is_removed(&self) -> bool;
    fn get_pos(&self) -> (i32, i32);
}

pub trait Mob: Entity {
    fn move_entity(&mut self, x: f32, y: f32, level: &Level);
    fn collision(&self, level: &Level, x_offset: f32, y_offset: f32) -> bool { false }
    fn update(&mut self, keyboard: &KeyBoard, level: &Level);
    fn direction(&self) -> Direction;
}

pub struct Player {
    x: f32,
    y: f32,
    speed: f32,
    direction: Direction,
    removed: bool,
    sprite: &'static Sprite,
}


#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Player {
    pub fn new(x: f32, y: f32, speed: f32, sprite: &'static Sprite) -> Player {
        Player {
            x,
            y,
            speed,
            direction: Direction::Right,
            removed: false,
            sprite
        }
    }
}

impl Entity for Player {
    fn get_pos(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }
    fn update(&mut self) {}
    fn render(&self, screen: &mut Screen) {
        let pixels = self.sprite.view();
        for y in 0..self.sprite.size {
            for x in 0..self.sprite.size {
                let xp = x as i32 + self.x as i32;
                let yp = y as i32 + self.y as i32;
                if xp < 0 || xp >= screen.width as i32
                    || yp < 0 ||yp >= screen.height as i32 {
                    continue;
                }
                let pixel = match pixels.get_pixel(x, y) {
                    Rgba {data: [255, 0, 255, 255]} => continue,
                    pixel => pixel,
                };
                screen.canvas.put_pixel(xp as u32, yp as u32, pixel)
            }
        }
    }
    fn remove(&mut self) {
        self.removed = true;
    }
    fn is_removed(&self) -> bool {
        self.removed
    }
}

impl Mob for Player {
    fn move_entity(&mut self, x: f32, y: f32, level: &Level) {
        if y < 0.0 {self.direction = Direction::Up;}
        if y > 0.0 {self.direction = Direction::Down;}
        if x < 0.0 {self.direction = Direction::Left;}
        if x > 0.0 {self.direction = Direction::Right;}
        if !self.collision(&level, x, y) {
            self.x += x;
            self.y += y;
        }
    }

    fn update(&mut self, keyboard: &KeyBoard, level: &Level) {
        let (mut xa, mut ya) = (0.0, 0.0);
        if keyboard.up { ya -= self.speed ; }
        if keyboard.down { ya += self.speed; }
        if keyboard.left { xa -= self.speed }
        if keyboard.right { xa += self.speed }
        if xa != 0.0 || ya != 0.0 {
            self.move_entity(xa, 0.0, level);
            self.move_entity(0.0, ya, level);
        }
    }

    fn collision(&self, level: &Level, x_offset: f32, y_offset: f32) -> bool {
        let x = (self.x + x_offset) as i32;
        let y = (self.y + y_offset) as i32;
        let x0 = x + 1 >> 3;
        let y0 = y >> 3;
        let x7 = (x + 7) >> 3;
        let y7 = (y + 7) >> 3;
        match self.direction {
            Direction::Up => {
                level.get_tile(x0, y0).solid || level.get_tile(x7, y0).solid
            },
            Direction::Down => {
                level.get_tile(x0, y7).solid || level.get_tile(x7, y7).solid
            },
            Direction::Right => {
                level.get_tile(x7, y0).solid || level.get_tile(x7, y7).solid
            },
            Direction::Left => {
                level.get_tile(x0, y0).solid || level.get_tile(x0, y7).solid
            },
        }
    }

    fn direction(&self) -> Direction { self.direction }
}
