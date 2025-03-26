use macroquad::prelude::*;

const BULLET_SPEED: f32 = 300.0;

pub struct Bullet {
    x: f32,
    y: f32,
    angle: f32,
    texture: Texture2D,
    pub destroyed: bool,
    pub rect: Rect,
    radius: f32,
}

impl Bullet {
    pub async fn new(x:f32, y:f32, angle: f32) -> Self {
        Self { x,
            y,
            angle,
            texture: Texture2D::from_file_with_format(include_bytes!("../assets/bullet.png"), None),
            destroyed: false,
            rect: Rect::new(x, y, 3.0, 3.0),
            radius: 20.0,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        self.radius += dt * BULLET_SPEED;

        self.x = self.radius*self.angle.cos() + screen_width() / 2.0;
        self.y = self.radius*self.angle.sin() + screen_height() - 110.0;

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
