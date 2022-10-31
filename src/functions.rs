use macroquad::{prelude::*, audio::{Sound, load_sound}};

pub struct Resources {
    pub intro_texture: Texture2D,
    pub instructions_texture: Texture2D,
    pub shot: Sound,
    pub bomb: Sound,
    pub crash: Sound,
    pub outro: Sound,
}

impl Resources {
    pub async fn new() -> Self {
        Self {
            intro_texture: load_texture("assets/intro.png").await.unwrap(),
            instructions_texture: load_texture("assets/instructions.png").await.unwrap(),
            shot: load_sound("assets/audio/shot.ogg").await.unwrap(),
            bomb: load_sound("assets/audio/bomb.ogg").await.unwrap(),
            crash: load_sound("assets/audio/crash.ogg").await.unwrap(),
            outro: load_sound("assets/audio/outro.ogg").await.unwrap(),
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

pub fn draw_play_again_text() {
    draw_text_ex("PRESS `I' FOR INSTRUCTIONS", 220.0, screen_height() / 2.0 - 15.0, 
        TextParams {
            font_size: 30,
            color: WHITE,
            ..Default::default()
        },
    );

    draw_text_ex("PRESS space FOR PLAY", 260.0, screen_height() / 2.0 + 20.0, 
        TextParams {
            font_size: 30,
            color: WHITE,
            ..Default::default()
        },
    );
}