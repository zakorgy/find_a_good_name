use image::{DynamicImage, GenericImageView, RgbaImage, SubImage};

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

/*pub struct Sprite {
    size: u32,
    x: u32,
    y: u32,
    image: RgbaImage,
}

impl Sprite {
    pub fn new(size: u32, x: u32, y: u32, sheet: &SpriteSheet) -> Sprite {
        Sprite {
            size,
            x,
            y,
            image: sheet.image
                        .view((x - 1) * size, (y - 1) * size, size, size)
                        .to_image(),
        }
    }
}*/

pub struct SpriteView {
    pub size: u32,
    x: u32,
    y: u32,
}

impl SpriteView {
    pub fn new(size: u32, x: u32, y: u32) -> SpriteView {
        SpriteView {
            size,
            x,
            y
        }
    }

    pub fn view<'a>(&self, sheet: &'a SpriteSheet) -> SubImage<&'a RgbaImage> {
        sheet.image.view(self.x * self.size, self.y * self.size, self.size, self.size)
    }
}
