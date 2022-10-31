use macroquad::prelude::*;

const ANIMATION_SPEED: i32 = 8;

pub struct Animation {
    pub x: f32,
    pub y: f32,
    animation_texture: Vec<Texture2D>,
    pub animation_completed: bool,
    update_interval: i32,
    cur_frame: usize,
}

impl Animation {
    pub async fn new(x: f32, y: f32, animation_type: &str) -> Self {
        let mut animation_sprites:Vec<Texture2D> = Vec::new();
        
        match animation_type {
            "die" => {
                for i in 1..=4 { // Number of sprites in animation
                    let path = format!("assets/die_animation/skull_{}.png", i);
                    animation_sprites.push(load_texture(&path).await.unwrap());
                }
            },
            "bomb_explode" => {
                for i in 1..=9 { // Number of sprites in animation
                    let path = format!("assets/bomb_explosion/explode_{}.png", i);
                    animation_sprites.push(load_texture(&path).await.unwrap());
                }
            },
            "enemy_explode" => {
                for i in 1..=10 { // Number of sprites in animation
                    let path = format!("assets/enemy_explosion/enemy_explosion_{}.png", i);
                    animation_sprites.push(load_texture(&path).await.unwrap());
                }
            },
            _ => {},
        };

        Self {
            x,
            y,
            animation_texture: animation_sprites,
            animation_completed: false,
            update_interval: 0,
            cur_frame: 0,
        }
    }

    pub fn die_animation(&mut self) {
        if !self.animation_completed {
            draw_texture(self.animation_texture[self.cur_frame], self.x, self.y, WHITE);
            self.update_interval += 1;
            if self.update_interval > ANIMATION_SPEED {
                self.update_interval = 0;
                self.cur_frame += 1;
                if self.cur_frame == self.animation_texture.len() {
                    self.cur_frame = 0;
                    self.animation_completed = true;
                }
            }
        }
    }

    pub fn draw(&mut self) {
        if !self.animation_completed {
            self.die_animation();
        }
    }
}