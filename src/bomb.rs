use macroquad::prelude::*;

const SPEED: f32 = 100.0;

pub struct Bomb {
    pub x: f32,
    pub y: f32,
    texture: Texture2D,
    pub destroyed: bool,
    pub rect: Rect,
    pub gravi: f32,
    pub side: String,
}

impl Bomb {
    pub async fn new(x: f32, y: f32, side: String) -> Self {
        Self {
            x,
            y,
            texture: load_texture("assets/bomb.png").await.unwrap(),
            rect: Rect::new(0.0, 0.0, 10.0, 10.0),
            destroyed: false,
            gravi: 1.0,
            side,
        }
    }

    pub fn update(&mut self) {
        if self.side == "left" {
            if self.x < 400.0 {
                self.x += get_frame_time() * SPEED;
            }
        } else {
            if self.x > 400.0 {
                self.x -= get_frame_time() * SPEED;
            }
        }
        if self.y < 550.0 {
            self.gravi += 0.015;
            self.y += self.gravi;
        }

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