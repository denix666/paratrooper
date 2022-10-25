use macroquad::prelude::*;
extern crate rand;
use rand::Rng;

const ANIMATION_SPEED: i32 = 8;
const ENEMY_SPEED: f32 = 200.0;

fn load_paratroopers(enemy_type: String) -> bool {
    if enemy_type == "helicopter" {
        rand::thread_rng().gen_bool(1.0 / 3.0)
    } else {
        false
    }
}

pub struct Enemy {
    pub x: f32,
    pub y: f32,
    texture: Vec<Texture2D>,
    pub destroyed: bool,
    update_interval: i32,
    cur_frame: usize,
    pub rect: Rect,
    side: String,
    pub have_paratrooper: bool,
    pub will_jump_at: f32,
    pub paratrooper_jumped: bool,
}

impl Enemy {
    pub async fn new(enemy_type: &str, from_side: &str) -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();

        for i in 1..4 {
            let path = format!("assets/enemy/{}_{}_{}.png", enemy_type, from_side, i);
            sprites.push(load_texture(&path).await.unwrap());
        }

        let start_x = match from_side {
            "right" => screen_width(),
            _ => 0.0,
        };

        let start_y = match from_side {
            "right" => 40.0,
            _ => 5.0,
        };

        let jump_point = match from_side {
            "left" => rand::thread_rng().gen_range(100.0..=350.0)/50.0*50.0,
            _ => rand::thread_rng().gen_range(500.0..=700.0)/50.0*50.0,
        };

        Self {
            x: start_x,
            y: start_y,
            texture: sprites,
            destroyed: false,
            update_interval: 0,
            cur_frame: 0,
            rect: Rect::new(0.0, 0.0, 0.0,0.0),
            side: from_side.to_string(),
            have_paratrooper: load_paratroopers(enemy_type.to_string()),
            will_jump_at: jump_point,
            paratrooper_jumped: false,
        }
    }

    pub fn update_animation(&mut self) {
        self.update_interval += 1;
        if self.update_interval > ANIMATION_SPEED {
            self.update_interval = 0;
            self.cur_frame += 1;
            if self.cur_frame == self.texture.len() {
                self.cur_frame = 0;
            }
        }
    }

    pub fn update(&mut self) {
        if self.side == "left" {
            self.x += get_frame_time() * ENEMY_SPEED;
            if self.x > screen_width() {
                self.destroyed = true;
            }
        } else {
            self.x -= get_frame_time() * ENEMY_SPEED;
            if self.x < 0.0 - self.texture[self.cur_frame].width() {
                self.destroyed = true;
            }
        }
        
        self.rect.w = self.texture[self.cur_frame].width();
        self.rect.h = self.texture[self.cur_frame].height();
        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn draw(&mut self) {
        if !self.destroyed {
            self.update_animation();
            self.update();
            draw_texture(self.texture[self.cur_frame], self.x, self.y, WHITE);
        }
    }
}