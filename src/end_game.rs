use macroquad::prelude::*;

const ANIMATION_SPEED: i32 = 20;
const SPEED: f32 = 70.0;

fn side(x: f32) -> f32 {
    if x < 400.0 {
        1.0
    } else {
        -1.0
    }
}

pub struct EndAnimation {
    pub ax: f32,
    pub ay: f32,
    pub bx: f32,
    pub by: f32,
    pub cx: f32,
    pub cy: f32,
    pub dx: f32,
    pub dy: f32,
    texture: Vec<Texture2D>,
    pub animation_a_completed: bool,
    pub animation_b_completed: bool,
    pub animation_c_completed: bool,
    pub animation_d_completed: bool,
    update_interval: i32,
    cur_frame_a: usize,
    cur_frame_b: usize,
    cur_frame_c: usize,
    cur_frame_d: usize,
}

impl EndAnimation {
    pub async fn new(
            ax: f32, ay: f32,
            bx: f32, by: f32,
            cx: f32, cy: f32,
            dx: f32, dy: f32) -> Self {

        let sprites = vec![
            Texture2D::from_file_with_format(include_bytes!("../assets/divs/div_1.png"), None),
            Texture2D::from_file_with_format(include_bytes!("../assets/divs/div_2.png"), None),
        ];

        Self {
            ax,
            ay,
            bx,
            by,
            cx,
            cy,
            dx,
            dy,
            texture: sprites,
            animation_a_completed: false,
            animation_b_completed: false,
            animation_c_completed: false,
            animation_d_completed: false,
            update_interval: 0,
            cur_frame_a: 0,
            cur_frame_b: 0,
            cur_frame_c: 0,
            cur_frame_d: 0,
        }
    }

    pub fn update_a(&mut self) {
        self.ay = 547.0;
        if !self.animation_a_completed {
            self.ax += get_frame_time() * SPEED * side(self.ax);
            self.update_interval += 1;
            if self.update_interval > ANIMATION_SPEED {
                self.update_interval = 0;
                self.cur_frame_a += 1;
                if self.cur_frame_a == self.texture.len() {
                    self.cur_frame_a = 0;
                }
            }
            if (side(self.ax) == 1.0 && self.ax > 348.0) || (side(self.ax) == -1.0 && self.ax < 440.0) {
                self.animation_a_completed = true;
            }
        }
    }

    pub fn update_b(&mut self) {
        self.by = 547.0;
        if !self.animation_b_completed {
            self.bx += get_frame_time() * SPEED * side(self.bx);
            self.update_interval += 1;
            if self.update_interval > ANIMATION_SPEED {
                self.update_interval = 0;
                self.cur_frame_b += 1;
                if self.cur_frame_b == self.texture.len() {
                    self.cur_frame_b = 0;
                }
            }
            if (side(self.bx) == 1.0 && self.bx > 336.0) || (side(self.bx) == -1.0 && self.bx < 452.0) {
                self.animation_b_completed = true;
            }
        }
    }

    pub fn update_c(&mut self) {
        if !self.animation_c_completed {
            self.cy = 547.0;
            self.cx += get_frame_time() * SPEED * side(self.cx);
            self.update_interval += 1;
            if self.update_interval > ANIMATION_SPEED {
                self.update_interval = 0;
                self.cur_frame_c += 1;
                if self.cur_frame_c == self.texture.len() {
                    self.cur_frame_c = 0;
                }
            }
            if (side(self.cx) == 1.0 && self.cx > 324.0) || (side(self.cx) == -1.0 && self.cx < 464.0) {
                self.animation_c_completed = true;
                if side(self.cx) == 1.0 {
                    self.cx = 348.0;
                } else {
                    self.cx = 440.0;
                }
            }
        } else {
            self.cy = 524.0;
        }
    }

    pub fn update_d(&mut self) {
        if !self.animation_d_completed {
            self.dy = 547.0;
            self.dx += get_frame_time() * SPEED * side(self.dx);
            self.update_interval += 1;
            if self.update_interval > ANIMATION_SPEED {
                self.update_interval = 0;
                self.cur_frame_d += 1;
                if self.cur_frame_d == self.texture.len() {
                    self.cur_frame_d = 0;
                }
            }
            if (side(self.dx) == 1.0 && self.dx > 324.0) || (side(self.dx) == -1.0 && self.dx < 464.0) {
                self.animation_d_completed = true;
                if side(self.dx) == 1.0 {
                    self.dx = 348.0;
                } else {
                    self.dx = 440.0;
                }
            }
        } else {
            self.dy = 501.0;
        }
    }

    pub fn draw(&mut self) {
        self.update_a();
        if self.animation_a_completed {
            self.update_b();
        }
        if self.animation_b_completed {
            self.update_c();
        }
        if self.animation_c_completed {
            self.update_d();
        }

        draw_texture(self.texture[self.cur_frame_a], self.ax, self.ay, WHITE);
        draw_texture(self.texture[self.cur_frame_b], self.bx, self.by, WHITE);
        draw_texture(self.texture[self.cur_frame_c], self.cx, self.cy, WHITE);
        draw_texture(self.texture[self.cur_frame_d], self.dx, self.dy, WHITE);
    }
}
