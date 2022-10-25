use macroquad::prelude::*;
extern crate rand;
use rand::Rng;

const SPEED: f32 = 150.0;

pub struct Paratrooper {
    trooper_x: f32,
    trooper_y: f32,
    para_texture: Texture2D,
    trooper_texture: Texture2D,
    pub para_rect: Rect,
    pub trooper_rect: Rect,
    pub destroyed: bool,
    pub para_destroyed: bool,
    pub have_para: bool,
    open_para_at: f32,
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
            para_destroyed: false,
            have_para: false,
            open_para_at: rand::thread_rng().gen_range(150.0..=400.0),
        }
    }

    pub fn update(&mut self) {
        let land_speed = match self.have_para {
            true => get_frame_time() * (SPEED - 100.0),
            false => get_frame_time() * SPEED,
        };

        self.trooper_y += land_speed;
        
        self.para_rect.w = self.para_texture.width();
        self.para_rect.h = self.para_texture.height();
        self.para_rect.x = self.trooper_x - 10.0;
        self.para_rect.y = self.trooper_y - self.para_texture.height();

        self.trooper_rect.w = self.trooper_texture.width();
        self.trooper_rect.h = self.trooper_texture.height();
        self.trooper_rect.x = self.trooper_x;
        self.trooper_rect.y = self.trooper_y;

        if self.trooper_y > self.open_para_at && !self.have_para && !self.para_destroyed {
            self.have_para = true;
        }
        
        if self.trooper_y > screen_height() - 30.0 - self.trooper_texture.height() && !self.have_para {
            self.destroyed = true;
        }
    }

    pub fn draw(&mut self) {
        if !self.destroyed {
            self.update();
            draw_texture(self.trooper_texture, self.trooper_x, self.trooper_y, WHITE);
            if self.have_para {
                draw_texture(self.para_texture, self.trooper_x - 10.0, self.trooper_y - self.para_texture.height(), WHITE);
            }
        }
    }
}