use macroquad::prelude::*;

const CANON_HEAD_COLOR: Color = GRAY;
const CANON_COLOR: Color = BLUE;

enum Status {
    MovingRight,
    MovingLeft,
    Idle,
}

pub struct Canon {
    angle: f32,
    cx: f32, // canon center x
    cy: f32, // canon center y
    ex: f32, // canon end x
    ey: f32, // canon end y
    r: f32, // radius
    status: Status,
}

impl Canon {
    pub async fn new() -> Self {
        Self {
            angle: -1.57,
            cx: screen_width() / 2.0,
            cy: screen_height() - 80.0,
            ex: screen_width() / 2.0,
            ey: screen_height() - 80.0,
            r: 30.0,
            status: Status::Idle,
        }
    }

    pub fn draw(&mut self) {
        // Canon stand
        draw_rectangle(screen_width() / 2.0 - 40.0, screen_height() - 60.0, 80.0, 60.0, WHITE);
        // Canon base
        draw_rectangle(screen_width() / 2.0 - 15.0, screen_height() - 80.0, 30.0, 20.0, CANON_HEAD_COLOR);
        // Canon
        draw_line(self.cx,self.cy,self.ex + screen_width() / 2.0, self.ey + screen_height() - 80.0,8.0, CANON_COLOR);
        // Canon base
        draw_circle(screen_width() / 2.0, screen_height() - 80.0, 15.0, CANON_HEAD_COLOR);
        // Canon center
        draw_circle(screen_width() / 2.0, screen_height() - 80.0, 4.0, CANON_COLOR);
        // Canon end
        draw_circle(self.ex + screen_width() / 2.0, self.ey + screen_height() - 80.0, 4.0, CANON_COLOR);
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::Left) {
            self.status = Status::MovingLeft;
        }

        if is_key_down(KeyCode::Right) {
            self.status = Status::MovingRight;
        }

        if is_key_down(KeyCode::Up) {
            self.status = Status::Idle;
        }

        match self.status {
            Status::MovingRight => {
                self.angle += 0.04;
                if self.angle > 0.0 {
                    self.angle = 0.0;
                    self.status = Status::Idle;
                }
            },
            Status::MovingLeft => {
                self.angle -= 0.04;
                if self.angle < -3.14 {
                    self.angle = -3.14;
                    self.status = Status::Idle;
                }
            },
            Status::Idle => {},
        }

        self.ex = self.r*self.angle.cos();
        self.ey = self.r*self.angle.sin();
    }
}