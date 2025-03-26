use macroquad::prelude::*;

pub struct Divs {
    pub x: f32,
    pub y: f32,
    texture: Texture2D,
    pub destroyed: bool,
    pub rect: Rect,
}

impl Divs {
    pub async fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            texture: Texture2D::from_file_with_format(include_bytes!("../assets/trooper.png"), None),
            rect: Rect::new(0.0, 0.0, 12.0, 23.0),
            destroyed: false,
        }
    }

    pub fn update(&mut self) {
        self.rect.w = self.texture.width();
        self.rect.h = self.texture.height();
        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn draw(&mut self) {
        if !self.destroyed {
            self.update();
            draw_texture(self.texture, self.x, self.y, WHITE);
        }
    }
}
