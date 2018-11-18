extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings};

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
            WindowSettings::new("piston: paint", (width * scale, height * scale))
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

    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            println!("Game running");
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}
