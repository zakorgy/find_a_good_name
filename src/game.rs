extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings};
use piston_window::generic_event::GenericEvent;
use piston_window::{clear, rectangle};

pub struct Game {
    pub width: u32,
    pub height: u32,
    pub scale: u32,
    pub running: bool,
    window: PistonWindow,
}

impl Game {
    pub fn new(width: u32, height: u32, scale: u32) -> Game {
        use piston_window::OpenGL;

        let window: PistonWindow =
            WindowSettings::new("Speartacus", (width * scale, height * scale))
                .exit_on_esc(true)
                .opengl(OpenGL::V3_2)
                .build()
                .unwrap();

        Game {
            width,
            height,
            scale,
            running: false,
            window,
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
                delta += last_time.elapsed().subsec_nanos() as f64 / ns;
                last_time = Instant::now();
                while delta >= 1.0 {
                    self.update();
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

    pub fn update(&mut self) {}

    pub fn render<E: GenericEvent>(&mut self, event: &E) where
    {
        self.window.draw_2d::<E,_, _>(&event, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            rectangle([0.0, 1.0, 0.0, 1.0], // red
                      [0.0, 0.0, 100.0, 100.0], // rectangle
                      c.transform, g);
        });
    }
}
