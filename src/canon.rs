use macroquad::prelude::*;

const CANON_HEAD_COLOR: Color = MAGENTA;
const CANON_COLOR: Color = Color::new(0.0, 1.00, 1.00, 1.00);
const STATUS_LINE_HEIGHT: f32 = 30.0;

enum Status {
    MovingRight,
    MovingLeft,
    Idle,
}

pub struct Canon {
    pub angle: f32,
    cx: f32, // canon center x
    cy: f32, // canon center y
    pub ex: f32, // canon end x
    pub ey: f32, // canon end y
    r: f32, // radius
    status: Status,
}

impl Canon {
    pub async fn new() -> Self {
        Self {
            angle: -1.57,
            cx: screen_width() / 2.0,
            cy: screen_height() - 80.0 - STATUS_LINE_HEIGHT,
            ex: screen_width() / 2.0,
            ey: screen_height() - 80.0 - STATUS_LINE_HEIGHT,
            r: 30.0,
            status: Status::Idle,
        }
    }

    pub fn draw(&mut self) {
        // Status line
        draw_line(0.0,screen_height() - STATUS_LINE_HEIGHT, screen_width(), screen_height() - STATUS_LINE_HEIGHT,2.0, CANON_COLOR);
        // Canon stand
        draw_rectangle(screen_width() / 2.0 - 40.0, screen_height() - 60.0 - STATUS_LINE_HEIGHT, 80.0, 60.0, WHITE);
        // Canon base
        draw_rectangle(screen_width() / 2.0 - 15.0, screen_height() - 80.0 - STATUS_LINE_HEIGHT, 30.0, 20.0, CANON_HEAD_COLOR);
        // Canon
        draw_line(self.cx,self.cy,self.ex + screen_width() / 2.0, self.ey + screen_height() - 80.0 - STATUS_LINE_HEIGHT,8.0, CANON_COLOR);
        // Canon base
        draw_circle(screen_width() / 2.0, screen_height() - 80.0 - STATUS_LINE_HEIGHT, 15.0, CANON_HEAD_COLOR);
        // Canon center
        draw_circle(screen_width() / 2.0, screen_height() - 80.0 - STATUS_LINE_HEIGHT, 4.0, CANON_COLOR);
        // Canon end
        draw_circle(self.ex + screen_width() / 2.0, self.ey + screen_height() - 80.0 - STATUS_LINE_HEIGHT, 4.0, CANON_COLOR);
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
                if self.angle > -0.4 {
                    self.angle = -0.4;
                    self.status = Status::Idle;
                }
            },
            Status::MovingLeft => {
                self.angle -= 0.04;
                if self.angle < -2.7 {
                    self.angle = -2.7;
                    self.status = Status::Idle;
                }
            },
            Status::Idle => {},
        }

        self.ex = self.r*self.angle.cos();
        self.ey = self.r*self.angle.sin();
    }
}