use image::{GenericImageView, RgbaImage, SubImage};
use std::path::PathBuf;

lazy_static! {
    pub static ref SHEET: SpriteSheet = {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res/sprites/tileset.png");
        SpriteSheet::new(path.to_str().unwrap().to_owned())
    };

    static ref GROUND1: Sprite = Sprite::new(8, 0, 11, &SHEET);
    static ref GROUND2: Sprite = Sprite::new(8, 1, 11, &SHEET);
    pub static ref VOID: Sprite = Sprite::new(8, 0, 0, &SHEET);
    pub static ref PLAYER: Sprite = Sprite::new(8, 0, 8, &SHEET);
    static ref PLAYER0: Sprite = Sprite::new(8, 7, 7, &SHEET);
    static ref PLAYER1: Sprite = Sprite::new(8, 0, 8, &SHEET);
    pub static ref PLAYERS: Vec<&'static Sprite> = vec![&PLAYER0, &PLAYER1];
    pub static ref GROUNDS: Vec<&'static Sprite> = vec![&GROUND1, &GROUND2];
    static ref WALL_TOP0: Sprite = Sprite::new(8, 1, 0, &SHEET);
    static ref WALL_TOP1: Sprite = Sprite::new(8, 2, 0, &SHEET);
    static ref WALL_TOP2: Sprite = Sprite::new(8, 3, 0, &SHEET);
    pub static ref WALL_TOPS: Vec<&'static Sprite> = vec![
        &WALL_TOP0,
        &WALL_TOP1,
        &WALL_TOP2,
    ];
    static ref WALL0: Sprite = Sprite::new(8, 4, 1, &SHEET);
    static ref WALL1: Sprite = Sprite::new(8, 5, 1, &SHEET);
    static ref WALL2: Sprite = Sprite::new(8, 6, 1, &SHEET);
    pub static ref WALLS: Vec<&'static Sprite> = vec![
        &WALL0,
        &WALL1,
        &WALL2,
    ];
    static ref GRASS0: Sprite = Sprite::new(8, 2, 2, &SHEET);
    static ref GRASS1: Sprite = Sprite::new(8, 6, 2, &SHEET);
    pub static ref GRASSES: Vec<&'static Sprite> = vec![
        &GRASS0,
        &GRASS1,
    ];
}

pub struct SpriteSheet {
    image: RgbaImage,
}

impl SpriteSheet {
    pub fn new(path: String) -> SpriteSheet {
        let image = match image::open(&path) {
            Ok(image) => image.to_rgba(),
            Err(err) => panic!("Error loading image: {:?}", err),
        };

        SpriteSheet {
            image
        }
    }
}

pub struct Sprite {
    pub size: u32,
    x: u32,
    y: u32,
    sheet: &'static SpriteSheet,
}

impl Sprite {
    pub fn new(size: u32, x: u32, y: u32, sheet: &'static SpriteSheet) -> Sprite {
        Sprite {
            size,
            x,
            y,
            sheet,
        }
    }

    pub fn view(&self) -> SubImage<&RgbaImage> {
        self.sheet.image.view(self.x * self.size, self.y * self.size, self.size, self.size)
    }
}

pub struct AnimatedSprite {
    sprites: Vec<&'static Sprite>,
    timing: Vec<u8>,
    current: usize,
    timer: u8,
}

impl AnimatedSprite {
    pub fn new(sprites: Vec<&'static Sprite>, timing: Vec<u8>) -> AnimatedSprite {
        AnimatedSprite {
            sprites,
            timing,
            current: 0,
            timer: 0,
        }
    }

    pub fn update(&mut self) {
        self.timer += 1;
        let current_timing = *self.timing.get(self.current).unwrap();
        if self.timer > current_timing {
            self.current += 1;
            if self.timing.len() <= self.current {
                self.reset();
            }
        }
    }

    pub fn view(&self) -> SubImage<&RgbaImage> {
        self.sprites[self.current].view()
    }

    pub fn reset(&mut self) {
        self.current = 0;
        self.timer = 0;
    }

    pub fn size(&self) -> u32 {
        self.sprites[0].size
    }
}
