use macroquad::prelude::*;

pub struct Resources {
    pub intro_texture: Texture2D,
    pub instructions_texture: Texture2D,
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            intro_texture: load_texture("assets/intro.png").await.unwrap(),
            instructions_texture: load_texture("assets/instructions.png").await.unwrap(),
        }
    }
}

pub fn draw_score(score: &str) {
    draw_text_ex("SCORE: ", 7.0, screen_height() - 7.0, 
        TextParams {
            font_size: 30,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(score, 100.0, screen_height() - 7.0, 
        TextParams {
            font_size: 30,
            color: MAGENTA,
            ..Default::default()
        },
    );
}

pub fn draw_hiscore(hiscore: &str) {
    draw_text_ex("HI-SCORE: ", 600.0, screen_height() - 7.0, 
        TextParams {
            font_size: 30,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex(hiscore, 730.0, screen_height() - 7.0, 
        TextParams {
            font_size: 30,
            color: MAGENTA,
            ..Default::default()
        },
    );
}