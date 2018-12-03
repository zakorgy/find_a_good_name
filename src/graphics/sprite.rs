use image::{GenericImageView, RgbaImage, SubImage};
use std::path::PathBuf;

lazy_static! {
    pub static ref SHEET: SpriteSheet = {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("res/sprites/tileset.png");
        SpriteSheet::new(path.to_str().unwrap().to_owned(), 60, 120)
    };

    static ref GROUND1: Sprite = Sprite::new(8, 0, 11, &SHEET);
    static ref GROUND2: Sprite = Sprite::new(8, 1, 11, &SHEET);
    pub static ref VOID: Sprite = Sprite::new(8, 0, 0, &SHEET);
    pub static ref GROUNDS: Vec<&'static Sprite> = vec![&GROUND1, &GROUND2];
}

pub struct SpriteSheet {
    path: String,
    width: u32,
    height: u32,
    image: RgbaImage,
}

impl SpriteSheet {
    pub fn new(path: String, width: u32, height: u32) -> SpriteSheet {
        let image = match image::open(&path) {
            Ok(image) => image.to_rgba(),
            Err(err) => panic!("Error loading image: {:?}", err),
        };

        SpriteSheet {path, width, height, image}
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