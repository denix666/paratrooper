use macroquad::prelude::*;

const BULLET_SPEED: f32 = 300.0;

pub struct Bullet {
    x: f32,
    y: f32,
    angle: f32,
    texture: Texture2D,
    pub destroyed: bool,
    pub rect: Rect,
    r: f32,
}

impl Bullet {
    pub async fn new(x:f32, y:f32, angle: f32) -> Self {
        Self {
            x,
            y,
            angle,
            texture: load_texture("assets/bullet.png").await.unwrap(),
            destroyed: false,
            rect: Rect::new(x, y, 3.0, 3.0),
            r: 10.0,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        self.r += dt * BULLET_SPEED;

        self.x = self.r*self.angle.cos() + screen_width() / 2.0;
        self.y = self.r*self.angle.sin() + screen_height() - 110.0;
        
        if self.y < 0.0  || self.x < 0.0 || self.x > screen_width() {
            self.destroyed = true;
        }
    }

    pub fn draw(&mut self) {
        if !self.destroyed {
            self.rect.x = self.x;
            self.rect.y = self.y;
            self.update_position(get_frame_time());
            draw_texture(self.texture, self.x, self.y, WHITE);
        }
    }
}