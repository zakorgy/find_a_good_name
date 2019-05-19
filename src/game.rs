use super::entity::{Door, Enemy, Entity, EntityId, EntityManager, MessageDispatcher, Player, Telegram, Message};
use super::entity::PLAYER_ID;
use super::graphics::{AnimatedSprite, Screen, ENEMIES, PLAYERS};
use super::input::{Key, KeyBoard};
use super::level::{Level, RoomId};
use piston_window::generic_event::GenericEvent;
use piston_window::{clear, image as draw_image};
use piston_window::{Filter, G2dTexture, Texture, TextureSettings, Transformed};
use piston_window::{PistonWindow, WindowSettings};
use std::boxed::Box;
use std::cell::RefCell;
use std::rc::Rc;

static EXIT_KEY: &'static Key = &Key::Escape;
static PAUSE_KEY: &'static Key = &Key::Space;

#[derive(Debug)]
enum GameState {
    Start,
    Running,
    Pause,
    LoadLevel,
    LoadRoom(RoomId, bool),
    End,
}

pub struct Game {
    width: u32,
    height: u32,
    x_offset: i32,
    y_offset: i32,
    scale: u32,
    state: GameState,
    pub keyboard: Rc<RefCell<KeyBoard>>,
    window: PistonWindow,
    screen: Screen,
    texture: G2dTexture,
    level: Level,
    entity_manager: EntityManager,
    dispatcher: MessageDispatcher,
}

impl Game {
    pub fn new(width: u32, height: u32, scale: u32) -> Game {
        use piston_window::OpenGL;

        let mut window: PistonWindow =
            WindowSettings::new("ATOMA", (width * scale, height * scale))
                .exit_on_esc(true)
                .opengl(OpenGL::V3_2)
                .build()
                .unwrap();

        let screen = Screen::new(width, height);

        let texture: G2dTexture = Texture::from_image(
            &mut window.factory,
            &screen.canvas,
            &TextureSettings::new().mag(Filter::Nearest),
        )
        .unwrap();

        Game {
            width,
            height,
            x_offset: 0,
            y_offset: 0,
            scale,
            state: GameState::Start,
            keyboard: Rc::new(RefCell::new(KeyBoard::new())),
            window,
            screen,
            texture,
            level: Level::new(27),
            entity_manager: EntityManager::new(),
            dispatcher: MessageDispatcher::new(),
        }
    }

    pub fn run(&mut self) {
        use piston_window::AdvancedWindow;
        use std::ops::Add;
        use std::time::{Duration, Instant};

        let mut last_time = Instant::now();
        let mut timer = Instant::now();
        let ns = 1_000_000_000.0_f64 / 60.0_f64;
        let mut delta = 0.0_f64;
        let mut frames = 0_u32;
        let mut updates = 0_u32;
        while let Some(e) = self.window.next() {
            match self.state {
                GameState::Start => {
                    let player = Box::new(Player::new(
                        0.0,
                        0.0,
                        0.7,
                        AnimatedSprite::new(PLAYERS.to_vec(), vec![5, 10]),
                        Rc::clone(&self.keyboard),
                        PLAYER_ID,
                    ));
                    self.entity_manager.add_entity(player);

                    self.state = GameState::LoadRoom(0, true);
                }
                GameState::LoadRoom(id, game_start) => {
                    let prev_id = self.level.current_room_id();
                    self.level.set_current_room(id);
                    self.entity_manager.clean_up();
                    let midle_point = self.level.current_room().spawn_point();
                    let spawn_point = if game_start {
                        midle_point
                    } else {
                        let pos = self.level.current_room().load_info.doors.iter().find(|i| {
                            if let Some(((pos), room_id)) = i {
                                if *room_id == prev_id {
                                    return true;
                                }
                            }
                            false
                        }).unwrap().unwrap().0;
                        let mut pos = ((pos.0 * 8) as f32, (pos.1 * 8 ) as f32);
                        if (midle_point.0 - pos.0) > 0.0 {
                            pos.0 += 6.0;
                        } else if (midle_point.0 - pos.0) < 0.0 {
                            pos.0 -= 6.0;
                        }

                        if (midle_point.1 - pos.1) > 0.0 {
                            pos.1 += 6.0;
                        } else if (midle_point.1 - pos.1) < 0.0 {
                            pos.1 -= 6.0;
                        }
                        pos
                    };

                    self.entity_manager.get_entity_mut(&PLAYER_ID).set_pos(spawn_point.0, spawn_point.1);
                    self.x_offset = 0;
                    self.y_offset = 0;
                    let enemy = Box::new(Enemy::new(
                        32f32,
                        32f32,
                        0.5,
                        AnimatedSprite::new(ENEMIES.to_vec(), vec![30, 45, 55, 60, 65]),
                    ));
                    self.entity_manager.add_entity(enemy);

                    let enemy = Box::new(Enemy::new(
                        64f32,
                        72f32,
                        0.5,
                        AnimatedSprite::new(ENEMIES.to_vec(), vec![30, 45, 55, 60, 65]),
                    ));
                    self.entity_manager.add_entity(enemy);
                    self.load_room();
                    self.state = GameState::Running;
                }
                GameState::Pause => {
                    self.keyboard.borrow_mut().update(&e);
                    if self.keyboard.borrow().contains_key(&PAUSE_KEY) {
                        self.resume();
                    }
                }
                GameState::Running => {
                    self.keyboard.borrow_mut().update(&e);
                    delta += last_time.elapsed().subsec_nanos() as f64 / ns;
                    last_time = Instant::now();
                    while delta >= 1.0 {
                        self.update(&e);
                        updates += 1;
                        delta -= 1.0;
                    }
                    self.render(&e);
                    frames += 1;

                    if (timer.elapsed().as_secs() * 1000 + timer.elapsed().subsec_millis() as u64)
                        > 1000
                    {
                        timer = timer.add(Duration::from_millis(1000));
                        self.window
                            .set_title(format!("ATOMA | {} ups, {} frames", updates, frames));
                        updates = 0;
                        frames = 0;
                    }
                },
                GameState::End => break,
                _ => {},
            }
        }
    }

    fn load_room(&mut self) {
        let load_info = self.level.current_room().load_info;
        for door_info in load_info.doors.iter() {
            if let Some(info) = door_info {
                let door: Door = info.into();
                self.entity_manager.add_entity(Box::new(door));
            }
        }
    }

    fn stop(&mut self) {
        self.state = GameState::End;
    }

    fn pause(&mut self) {
        self.state = GameState::Pause;
        self.keyboard.borrow_mut().clear();
    }

    fn resume(&mut self) {
        self.state = GameState::Running;
        self.keyboard.borrow_mut().clear();
    }

    fn update<E: GenericEvent>(&mut self, _event: &E) {
        if self.keyboard.borrow().contains_key(&EXIT_KEY) {
            self.stop();
        }
        if self.keyboard.borrow().contains_key(&PAUSE_KEY) {
            self.pause();
        }

        while let Some(Telegram {sender, receiver, message}) = self.dispatcher.poll_game_message() {
            match message {
                Message::LoadRoom(id) => {
                    self.state = GameState::LoadRoom(id, false);
                    return;
                }
                _ => {}
            }
        }

        self.entity_manager.check_collisions(&mut self.dispatcher);
        self.dispatcher.dispatch_messages(&mut self.entity_manager);
        self.level.update();
        self.entity_manager.update(&self.level.current_room());
        self.update_offsets();
    }

    fn update_offsets(&mut self) {
        let (x, y) = self.entity_manager.get_entity_mut(&PLAYER_ID).absolute_pos();
        let (lvl_width, lvl_height) = self.level.dimensions();

        self.x_offset = {
            let half_width = (self.width / 2) as i32;
            if x < half_width || lvl_width <= self.width as i32 {
                0
            } else if x > lvl_width - half_width {
                lvl_width - self.width as i32
            } else {
                x - half_width
            }
        };

        self.y_offset = {
            let half_height = (self.height / 2) as i32;
            if y < half_height || lvl_height <= self.height as i32 {
                0
            } else if y > lvl_height - half_height {
                lvl_height - self.height as i32
            } else {
                y - half_height
            }
        };
    }

    fn render<E: GenericEvent>(&mut self, event: &E) {
        self.screen.clear();
        self.level
            .render(self.x_offset, self.y_offset, &mut self.screen);
        self.entity_manager.render(&mut self.screen, self.x_offset as f32, self.y_offset as f32);
        self.screen.render_map(self.level.map_info());
        self.texture
            .update(&mut self.window.encoder, &self.screen.canvas)
            .unwrap();
        let ref texture = self.texture;
        let scale = self.scale as f64;
        self.window.draw_2d::<E, _, _>(&event, |c, g| {
            clear([1.0; 4], g);
            draw_image(texture, c.scale(scale, scale).transform, g);
        });
    }
}
