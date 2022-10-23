use macroquad::prelude::*;

mod canon;
use canon::Canon;

mod enemy;
use enemy::Enemy;

mod bullet;
use bullet::Bullet;

mod game;
use game::Game;

mod functions;
use functions::*;

pub enum GameState {
    Intro,
    Instructions,
    Game,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Paratrooper"
        .to_owned(),
        fullscreen: false,
        sample_count: 16,
        window_width: 800,
        window_height: 600,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;
    let mut canon = Canon::new().await;
    let mut enemy = Enemy::new().await;
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut game_state = GameState::Intro;
    let resources = Resources::new().await;
    
    loop {
        clear_background(BLACK);

        match game_state {
            GameState::Intro => {
                draw_texture(resources.intro_texture, 0.0, 0.0, WHITE);
                
                if is_key_pressed(KeyCode::Space) {
                    game.score = 0;
                    game_state = GameState::Game;
                }
                if is_key_pressed(KeyCode::I) {
                    game_state = GameState::Instructions;
                }
            },

            GameState::Instructions => {
                draw_texture(resources.instructions_texture, 0.0, 0.0, WHITE);
                
                if is_key_pressed(KeyCode::Space) {
                    game.score = 0;
                    game_state = GameState::Game;
                }
            },
            
            GameState::Game => {

                draw_score(&game.score.to_string());
                
                canon.draw();
                canon.update();

                enemy.draw();
                enemy.update();

                if is_key_pressed(KeyCode::Up) {
                    bullets.push(Bullet::new(canon.ex + screen_width() / 2.0, canon.ey + screen_height() - 110.0, canon.angle).await);
                    game.score -= 1;
                    if game.score < 0 {
                        game.score = 0;
                    }
                }

                for bullet in &mut bullets {
                    bullet.draw();
                }
            }
        }

        next_frame().await
    }
}
