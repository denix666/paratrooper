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
        let die_sprites = vec![
            Texture2D::from_file_with_format(include_bytes!("../assets/die_animation/skull_1.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/die_animation/skull_2.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/die_animation/skull_3.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/die_animation/skull_4.png"), None),
        ];

        let bomb_explode_sprites = vec![
            Texture2D::from_file_with_format(include_bytes!("../assets/bomb_explosion/explode_1.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/bomb_explosion/explode_2.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/bomb_explosion/explode_3.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/bomb_explosion/explode_4.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/bomb_explosion/explode_5.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/bomb_explosion/explode_6.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/bomb_explosion/explode_7.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/bomb_explosion/explode_8.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/bomb_explosion/explode_9.png"), None),
        ];

        let enemy_explosion_sprites = vec![
            Texture2D::from_file_with_format(include_bytes!("../assets/enemy_explosion/enemy_explosion_1.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/enemy_explosion/enemy_explosion_2.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/enemy_explosion/enemy_explosion_3.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/enemy_explosion/enemy_explosion_4.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/enemy_explosion/enemy_explosion_5.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/enemy_explosion/enemy_explosion_6.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/enemy_explosion/enemy_explosion_7.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/enemy_explosion/enemy_explosion_8.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/enemy_explosion/enemy_explosion_9.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/enemy_explosion/enemy_explosion_10.png"), None),
        ];

        let mut animation_sprites:Vec<Texture2D> = Vec::new();

        match animation_type {
            "die" => animation_sprites = die_sprites,
            "bomb_explode" => animation_sprites = bomb_explode_sprites,
            "enemy_explode" => animation_sprites = enemy_explosion_sprites,
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
