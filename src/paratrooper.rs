use macroquad::prelude::*;
extern crate rand;
use rand::Rng;

const SPEED: f32 = 200.0;
//const ANIMATION_SPEED: i32 = 8;

pub struct Paratrooper {
    pub trooper_x: f32,
    pub trooper_y: f32,
    para_texture: Texture2D,
    trooper_texture: Texture2D,
    // die_animation_texture: Vec<Texture2D>,
    // explode_animation_texture: Vec<Texture2D>,
    // update_interval: i32,
    pub para_rect: Rect,
    pub trooper_rect: Rect,
    pub destroyed: bool,
    pub para_destroyed: bool,
    pub have_para: bool,
    open_para_at: f32,
    pub landed: bool,
    // cur_frame: usize,
    // die_animation_completed: bool,
    // explode_animation_completed: bool,
}

impl Paratrooper {
    pub async fn new(trooper_x: f32, trooper_y: f32) -> Self {
        let mut die_animation_sprites:Vec<Texture2D> = Vec::new();
        for i in 1..=4 { // Number of sprites in animation
            let path = format!("assets/die_animation/skull_{}.png", i);
            die_animation_sprites.push(load_texture(&path).await.unwrap());
        }

        let mut explode_animation_sprites:Vec<Texture2D> = Vec::new();
        for i in 1..=9 { // Number of sprites in animation
            let path = format!("assets/explode/explode_{}.png", i);
            explode_animation_sprites.push(load_texture(&path).await.unwrap());
        }
        
        Self {
            trooper_x,
            trooper_y,
            // update_interval: 0,
            // die_animation_texture: die_animation_sprites,
            // explode_animation_texture: explode_animation_sprites,
            para_texture: load_texture("assets/para.png").await.unwrap(),
            trooper_texture: load_texture("assets/trooper.png").await.unwrap(),
            para_rect: Rect::new(0.0, 0.0, 32.0, 37.0),
            trooper_rect: Rect::new(0.0, 0.0, 12.0, 23.0),
            destroyed: false,
            para_destroyed: false,
            have_para: false,
            open_para_at: rand::thread_rng().gen_range(150.0..=400.0),
            landed: false,
            // die_animation_completed: false,
            // explode_animation_completed: false,
            // cur_frame: 0,
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

        if self.trooper_y > screen_height() - 30.0 - self.trooper_texture.height() && self.have_para {
            self.landed = true;
        }
    }

    // pub fn die_animation(&mut self) {
    //     if self.destroyed && !self.have_para {
    //         if !self.die_animation_completed {
    //             draw_texture(self.die_animation_texture[self.cur_frame], self.trooper_x - 10.0, self.trooper_y - 15.0, WHITE);
    //             self.update_interval += 1;
    //             if self.update_interval > ANIMATION_SPEED {
    //                 self.update_interval = 0;
    //                 self.cur_frame += 1;
    //                 if self.cur_frame == self.die_animation_texture.len() {
    //                     self.cur_frame = 0;
    //                     self.die_animation_completed = true;
    //                 }
    //             }
    //         }
    //     }
    // }

    // pub fn explode_animation(&mut self) {
    //     if self.destroyed && self.have_para {
    //         if !self.explode_animation_completed {
    //             draw_texture(self.explode_animation_texture[self.cur_frame], self.trooper_x - 10.0, self.trooper_y - 15.0, WHITE);
    //             self.update_interval += 1;
    //             if self.update_interval > 2 {
    //                 self.update_interval = 0;
    //                 self.cur_frame += 1;
    //                 if self.cur_frame == self.explode_animation_texture.len() {
    //                     self.cur_frame = 0;
    //                     self.explode_animation_completed = true;
    //                 }
    //             }
    //         }
    //     }
    // }

    pub fn draw(&mut self) {
        if !self.destroyed && !self.landed {
            self.update();
            draw_texture(self.trooper_texture, self.trooper_x, self.trooper_y, WHITE);
            if self.have_para {
                draw_texture(self.para_texture, self.trooper_x - 10.0, self.trooper_y - self.para_texture.height(), WHITE);
            }
        }
        // self.die_animation();
        // self.explode_animation();
    }
}