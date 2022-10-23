use macroquad::prelude::*;

const ANIMATION_SPEED: i32 = 8;

// enum Type {
//     Helicopter,
//     Ship,
// }

pub struct Enemy {
    x: f32,
    y: f32,
    texture: Vec<Texture2D>,
    destroyed: bool,
    update_interval: i32,
    cur_frame: usize,
    pub rect: Rect,
}

impl Enemy {
    pub async fn new() -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();

        for i in 1..4 {
            let path = format!("assets/enemy/jet_{}.png", i);
            sprites.push(load_texture(&path).await.unwrap());
        }

        Self {
            x: 110.0,
            y: 110.0,
            texture: sprites,
            destroyed: false,
            update_interval: 0,
            cur_frame: 0,
            rect: Rect::new(0.0, 0.0, 0.0,0.0),
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