use super::entity::{Direction, Entity, Mob, Player};
use super::graphics::{Screen, PLAYER};
use super::input::{Key, KeyBoard};
use super::level::Level;
use piston_window::{PistonWindow, WindowSettings};
use piston_window::generic_event::GenericEvent;
use piston_window::{clear, image as draw_image};
use piston_window::{Filter, G2dTexture, Texture, TextureSettings, Transformed};
use std::boxed::Box;
use std::path::PathBuf;

static EXIT_KEY: &'static Key = &Key::Escape;

pub struct Game {
    width: u32,
    height: u32,
    x_offset: i32,
    y_offset: i32,
    scale: u32,
    running: bool,
    pub keyboard: KeyBoard,
    window: PistonWindow,
    screen: Screen,
    texture: G2dTexture,
    level: Level,
    player: Box<dyn Mob>,
}

impl Game {
    pub fn new(width: u32, height: u32, scale: u32) -> Game {
        use piston_window::OpenGL;

        let mut window: PistonWindow =
            WindowSettings::new("Speartacus", (width * scale, height * scale))
                .exit_on_esc(true)
                .opengl(OpenGL::V3_2)
                .build()
                .unwrap();

        let screen = Screen::new(width, height);

        let texture: G2dTexture = Texture::from_image(
            &mut window.factory,
            &screen.canvas,
            &TextureSettings::new().mag(Filter::Nearest),
        ).unwrap();

        let lvl_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res/sprites/map.png");
        let level = Level::load_level(&lvl_path);

        Game {
            width,
            height,
            x_offset: 0,
            y_offset: 0,
            scale,
            running: false,
            keyboard: KeyBoard::new(),
            window,
            screen,
            texture,
            level,
            player: Box::new(Player::new((width / 2) as f32, (height / 2) as f32, 0.5, &PLAYER))
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn run(&mut self) {
        use piston_window::AdvancedWindow;
        use std::time::{Duration, Instant};
        use std::ops::Add;

        let mut last_time = Instant::now();
        let mut timer = Instant::now();
        let ns = 1_000_000_000.0_f64 / 60.0_f64;
        let mut delta = 0.0_f64;
        let mut frames = 0_u32;
        let mut updates= 0_u32;
        while let Some(e) = self.window.next() {
            if self.running {
                self.keyboard.update(&e);
                delta += last_time.elapsed().subsec_nanos() as f64 / ns;
                last_time = Instant::now();
                while delta >= 1.0 {
                    self.update(&e);
                   updates += 1;
                   delta -= 1.0;
                }
                self.render(&e);
                frames += 1;

                if (timer.elapsed().as_secs() * 1000 + timer.elapsed().subsec_millis() as u64) > 1000 {
                    timer = timer.add(Duration::from_millis(1000));
                    self.window.set_title(format!("Speartacus | {} ups, {} frames", updates, frames));
                    updates = 0;
                    frames = 0;
                }
            } else {
                break;
            }
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    fn update_offsets(&mut self) {
        if self.keyboard.up { self.y_offset -= 1; }
        if self.keyboard.down { self.y_offset += 1; }
        if self.keyboard.left { self.x_offset -= 1; }
        if self.keyboard.right { self.x_offset += 1; }
    }

    fn update<E: GenericEvent>(&mut self, _event: &E) {
        if self.keyboard.contains_key(&EXIT_KEY) {
            self.stop();
        }
        //self.update_offsets();

        self.level.update();
        /*println!("Player pos {:?}", self.player.get_pos());
        let (x, y) = self.player.get_pos();
        if self.screen.width as i32 / 4 * 3 - x < 5 || self.screen.width as i32 / 4 * 3 - x > -5 && self.player.direction() == Direction::Right
            || self.screen.width as i32 / 4 - x < 5 || self.screen.width as i32 / 4 - x > -5 && self.player.direction() == Direction::Left
            || self.screen.height as i32 / 5 * 4 - y < 5 || self.screen.height as i32 / 5 * 4 - y > -5  && self.player.direction() == Direction::Down
            || self.screen.height as i32 / 5 - y < 5 || self.screen.height as i32 / 5 - y > -5 && self.player.direction() == Direction::Up
        {
            Mob::update(self.player.as_mut(), &self.keyboard, true);
            self.update_offsets();
        } else {*/
            Mob::update(self.player.as_mut(), &self.keyboard, &self.level);
        //}

    }

    fn render<E: GenericEvent>(&mut self, event: &E)
    {
        self.screen.clear();
        self.level.render(self.x_offset, self.y_offset, &mut self.screen);
        self.player.render(&mut self.screen);
        self.texture.update(&mut self.window.encoder, &self.screen.canvas).unwrap();
        let ref texture = self.texture;
        let scale = self.scale as f64;
        self.window.draw_2d::<E,_, _>(&event, |c, g| {
                clear([1.0; 4], g);
                draw_image(texture, c.scale(scale, scale).transform, g);
        });
    }
}
