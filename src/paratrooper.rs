use macroquad::prelude::*;

const SPEED: f32 = 100.0;

pub struct Paratrooper {
    trooper_x: f32,
    trooper_y: f32,
    para_texture: Texture2D,
    trooper_texture: Texture2D,
    pub para_rect: Rect,
    pub trooper_rect: Rect,
    destroyed: bool,
    have_para: bool,
}

impl Paratrooper {
    pub async fn new(trooper_x: f32, trooper_y: f32) -> Self {
        Self {
            trooper_x,
            trooper_y,
            para_texture: load_texture("assets/para.png").await.unwrap(),
            trooper_texture: load_texture("assets/trooper.png").await.unwrap(),
            para_rect: Rect::new(0.0, 0.0, 32.0, 37.0),
            trooper_rect: Rect::new(0.0, 0.0, 12.0, 23.0),
            destroyed: false,
            have_para: false,
        }
    }

    pub fn update(&mut self) {
        self.trooper_y += get_frame_time() * SPEED;
        
        self.para_rect.w = self.para_texture.width();
        self.para_rect.h = self.para_texture.height();
        self.para_rect.x = self.trooper_x - 10.0;
        self.para_rect.y = self.trooper_y - self.para_texture.height();

        if self.trooper_y > screen_height() - 30.0 - self.trooper_texture.height() && !self.have_para {
            self.destroyed = true;
        }
    }

    pub fn draw(&mut self) {
        if !self.destroyed {
            self.update();
            draw_texture(self.trooper_texture, self.trooper_x, self.trooper_y, WHITE);
        }
    }
}