use macroquad::prelude::*;

const ANIMATION_SPEED: i32 = 8;
const ENEMY_SPEED: f32 = 220.0;

fn load_paratroopers(enemy_type: String) -> bool {
    if enemy_type == "helicopter" {
        match macroquad::rand::gen_range(0, 3) {
            1 => true,
            2 => true,
            3 => true,
            _ => false,
        }
    } else {
        false
    }
}

fn load_bombs(enemy_type: String) -> bool {
    if enemy_type == "jet" {
        match macroquad::rand::gen_range(0, 2) {
            1 => true,
            2 => true,
            _ => false,
        }
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
    pub have_bomb: bool,
    pub will_jump_at: f32,
    pub will_bomb_at: f32,
    pub paratrooper_jumped: bool,
    pub bomb_released: bool,
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

        let jump_point: i32 = match from_side {
            "left" => macroquad::rand::gen_range(50, 300)/50*50,
            _ => macroquad::rand::gen_range(500, 750)/50*50,
        };

        let bomb_push_point: f32 = match from_side {
            "left" => 100.0,
            _ => 700.0,
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
            have_bomb: load_bombs(enemy_type.to_string()),
            will_jump_at: jump_point as f32,
            paratrooper_jumped: false,
            will_bomb_at: bomb_push_point,
            bomb_released: false,
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

    pub fn center_x(&mut self) -> f32 {
        self.texture[self.cur_frame].width() / 2.0 + self.x
    }

    pub fn center_y(&mut self) -> f32 {
        self.texture[self.cur_frame].height() / 2.0 + self.y
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
        self.update_animation();
        self.update();
        draw_texture(self.texture[self.cur_frame], self.x, self.y, WHITE);
    }
}