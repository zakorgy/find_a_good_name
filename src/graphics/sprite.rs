use image::{GenericImageView, RgbaImage, SubImage};
use std::path::PathBuf;

pub static SPRITE_SIZE_U32: u32 = 16;
pub static SPRITE_SIZE_SHIFT_VALUE: u32 = 4;
pub static HALF_SPRITE_SIZE_U32: u32 = 8;
pub static HALF_SPRITE_SIZE_SHIFT_VALUE: u32 = 3;
pub static SPRITE_SIZE_F32: f32 = 16.0;

lazy_static! {
    pub static ref SHEET: SpriteSheet = {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res/sprites/sheet.png");
        SpriteSheet::new(path.to_str().unwrap().to_owned())
    };
    static ref GRASS0: Sprite = Sprite::new(SPRITE_SIZE_U32, 9, 2, &SHEET);
    static ref GRASS1: Sprite = Sprite::new(SPRITE_SIZE_U32, 9, 1, &SHEET);
    static ref GRASS2: Sprite = Sprite::new(SPRITE_SIZE_U32, 9, 3, &SHEET);
    static ref GRASS3: Sprite = Sprite::new(SPRITE_SIZE_U32, 10, 1, &SHEET);
    static ref GRASS4: Sprite = Sprite::new(SPRITE_SIZE_U32, 10, 2, &SHEET);
    static ref GRASS5: Sprite = Sprite::new(SPRITE_SIZE_U32, 10, 3, &SHEET);
    pub static ref GRASSES: Vec<&'static Sprite> = vec![&GRASS0, &GRASS1, &GRASS2, &GRASS3, &GRASS4, &GRASS5];
    pub static ref VOID: Sprite = Sprite::new(SPRITE_SIZE_U32, 18, 0, &SHEET);
    static ref CORNER0: Sprite = Sprite::new(SPRITE_SIZE_U32, 3, 0, &SHEET);
    static ref CORNER1: Sprite = Sprite::new(SPRITE_SIZE_U32, 3, 1, &SHEET);
    static ref CORNER2: Sprite = Sprite::new(SPRITE_SIZE_U32, 4, 0, &SHEET);
    static ref CORNER3: Sprite = Sprite::new(SPRITE_SIZE_U32, 4, 1, &SHEET);
    pub static ref CORNERS: Vec<&'static Sprite> = vec![&CORNER0, &CORNER1, &CORNER2, &CORNER3];
    static ref WALL0: Sprite = Sprite::new(SPRITE_SIZE_U32, 0, 3, &SHEET);
    static ref WALL1: Sprite = Sprite::new(SPRITE_SIZE_U32, 1, 3, &SHEET);
    static ref WALL2: Sprite = Sprite::new(SPRITE_SIZE_U32, 2, 3, &SHEET);
    pub static ref WALLS: Vec<&'static Sprite> = vec![&WALL0, &WALL1, &WALL2,];
    pub static ref DOOR: Sprite = Sprite::new(SPRITE_SIZE_U32, 2, 8, &SHEET);
    pub static ref ROOM: Sprite = Sprite::new(HALF_SPRITE_SIZE_U32, 12, 28, &SHEET);
    pub static ref CURRENT_ROOM: Sprite = Sprite::new(HALF_SPRITE_SIZE_U32, 12, 29, &SHEET);
    pub static ref NO_ROOM: Sprite = Sprite::new(HALF_SPRITE_SIZE_U32, 13, 28, &SHEET);
}

lazy_static! {
    pub static ref GOBLIN: SpriteSheet = {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res/sprites/goblin.png");
        SpriteSheet::new(path.to_str().unwrap().to_owned())
    };
    static ref PLAYER0: Sprite = Sprite::new(SPRITE_SIZE_U32, 0, 0, &GOBLIN);
    static ref PLAYER1: Sprite = Sprite::new(SPRITE_SIZE_U32, 1, 0, &GOBLIN);
    static ref PLAYER2: Sprite = Sprite::new(SPRITE_SIZE_U32, 2, 0, &GOBLIN);
    pub static ref PLAYER_DOWN: Vec<&'static Sprite> = vec![&PLAYER0, &PLAYER1, &PLAYER2];
    static ref PLAYER3: Sprite = Sprite::new(SPRITE_SIZE_U32, 0, 1, &GOBLIN);
    static ref PLAYER4: Sprite = Sprite::new(SPRITE_SIZE_U32, 1, 1, &GOBLIN);
    static ref PLAYER5: Sprite = Sprite::new(SPRITE_SIZE_U32, 2, 1, &GOBLIN);
    pub static ref PLAYER_UP: Vec<&'static Sprite> = vec![&PLAYER3, &PLAYER4, &PLAYER5];
    static ref PLAYER6: Sprite = Sprite::new(SPRITE_SIZE_U32, 0, 2, &GOBLIN);
    static ref PLAYER7: Sprite = Sprite::new(SPRITE_SIZE_U32, 1, 2, &GOBLIN);
    static ref PLAYER8: Sprite = Sprite::new(SPRITE_SIZE_U32, 2, 2, &GOBLIN);
    pub static ref PLAYER_LEFT: Vec<&'static Sprite> = vec![&PLAYER6, &PLAYER7, &PLAYER8];
}

lazy_static! {
    pub static ref BLOB: SpriteSheet = {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res/sprites/blob.png");
        SpriteSheet::new(path.to_str().unwrap().to_owned())
    };
    static ref ENEMY0: Sprite = Sprite::new(SPRITE_SIZE_U32, 0, 0, &BLOB);
    static ref ENEMY1: Sprite = Sprite::new(SPRITE_SIZE_U32, 1, 0, &BLOB);
    static ref ENEMY2: Sprite = Sprite::new(SPRITE_SIZE_U32, 2, 0, &BLOB);
    pub static ref ENEMIES: Vec<&'static Sprite> = vec![&ENEMY0, &ENEMY1, &ENEMY2];
}

/*pub static SPRITE_SIZE_U32: u32 = 8;
pub static SPRITE_SIZE_SHIFT_VALUE: u32 = 3;
pub static HALF_SPRITE_SIZE_U32: u32 = 4;
pub static HALF_SPRITE_SIZE_SHIFT_VALUE: u32 = 2;
pub static SPRITE_SIZE_F32: f32 = 8.0;

lazy_static! {
    pub static ref SHEET: SpriteSheet = {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res/sprites/tileset.png");
        SpriteSheet::new(path.to_str().unwrap().to_owned())
    };
    static ref GROUND1: Sprite = Sprite::new(SPRITE_SIZE_U32, 0, 11, &SHEET);
    static ref GROUND2: Sprite = Sprite::new(SPRITE_SIZE_U32, 1, 11, &SHEET);
    pub static ref VOID: Sprite = Sprite::new(SPRITE_SIZE_U32, 0, 0, &SHEET);
    pub static ref PLAYER: Sprite = Sprite::new(SPRITE_SIZE_U32, 0, 8, &SHEET);
    static ref PLAYER0: Sprite = Sprite::new(SPRITE_SIZE_U32, 7, 7, &SHEET);
    static ref PLAYER1: Sprite = Sprite::new(SPRITE_SIZE_U32, 0, 8, &SHEET);
    pub static ref PLAYER_SOUTH: Vec<&'static Sprite> = vec![&PLAYER0, &PLAYER1];
    static ref ENEMY0: Sprite = Sprite::new(SPRITE_SIZE_U32, 2, 9, &SHEET);
    static ref ENEMY1: Sprite = Sprite::new(SPRITE_SIZE_U32, 3, 9, &SHEET);
    pub static ref ENEMIES: Vec<&'static Sprite> = vec![&ENEMY0, &ENEMY1];
    pub static ref GROUNDS: Vec<&'static Sprite> = vec![&GROUND1, &GROUND2];
    static ref WALL_TOP0: Sprite = Sprite::new(SPRITE_SIZE_U32, 1, 0, &SHEET);
    static ref WALL_TOP1: Sprite = Sprite::new(SPRITE_SIZE_U32, 2, 0, &SHEET);
    static ref WALL_TOP2: Sprite = Sprite::new(SPRITE_SIZE_U32, 3, 0, &SHEET);
    pub static ref WALL_TOPS: Vec<&'static Sprite> = vec![&WALL_TOP0, &WALL_TOP1, &WALL_TOP2,];
    static ref WALL0: Sprite = Sprite::new(SPRITE_SIZE_U32, 4, 1, &SHEET);
    static ref WALL1: Sprite = Sprite::new(SPRITE_SIZE_U32, 5, 1, &SHEET);
    static ref WALL2: Sprite = Sprite::new(SPRITE_SIZE_U32, 6, 1, &SHEET);
    pub static ref WALLS: Vec<&'static Sprite> = vec![&WALL0, &WALL1, &WALL2,];
    static ref GRASS0: Sprite = Sprite::new(SPRITE_SIZE_U32, 2, 2, &SHEET);
    static ref GRASS1: Sprite = Sprite::new(SPRITE_SIZE_U32, 6, 2, &SHEET);
    pub static ref GRASSES: Vec<&'static Sprite> = vec![&GRASS0, &GRASS1,];
    pub static ref DOOR: Sprite = Sprite::new(SPRITE_SIZE_U32, 2, 1, &SHEET);
    pub static ref ROOM: Sprite = Sprite::new(HALF_SPRITE_SIZE_U32, 12, 28, &SHEET);
    pub static ref CURRENT_ROOM: Sprite = Sprite::new(HALF_SPRITE_SIZE_U32, 12, 29, &SHEET);
    pub static ref NO_ROOM: Sprite = Sprite::new(HALF_SPRITE_SIZE_U32, 13, 28, &SHEET);
}*/

pub struct SpriteSheet {
    image: RgbaImage,
}

impl SpriteSheet {
    pub fn new(path: String) -> SpriteSheet {
        let image = match image::open(&path) {
            Ok(image) => image.to_rgba(),
            Err(err) => panic!("Error loading image: {:?}", err),
        };

        SpriteSheet { image }
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
        Sprite { size, x, y, sheet }
    }

    pub fn view(&self) -> SubImage<&RgbaImage> {
        self.sheet
            .image
            .view(self.x * self.size, self.y * self.size, self.size, self.size)
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
        self.sprites[self.current % self.sprites.len()].view()
    }

    pub fn reset(&mut self) {
        self.current = 0;
        self.timer = 0;
    }

    pub fn size(&self) -> u32 {
        self.sprites[0].size
    }
}
