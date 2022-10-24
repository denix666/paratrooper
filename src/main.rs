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

extern crate rand;
use rand::Rng;

pub enum GameState {
    Intro,
    Instructions,
    Game,
}

pub enum GamePhase {
    Helicopters,
    Jets,
    Paratroopers,
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
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut game_state = GameState::Intro;
    let mut game_phase = GamePhase::Helicopters;
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

                match game_phase {
                    GamePhase::Helicopters => {
                        // Spawn helicopters
                        if game.spawned_enemy < 10 + game.level { // Amount of helicopters in this phase
                            if game.enemy_amount_now < 6 {
                                if get_time() - game.last_spawn_time >= 2.0 { // TODO need to add random delay
                                    game.last_spawn_time = get_time();
                                    match rand::thread_rng().gen_range(0..=1) { 
                                        0 => {
                                            enemies.push(
                                                Enemy::new("helicopter", "right").await,
                                            );
                                        },
                                        _ => {
                                            enemies.push(
                                                Enemy::new("helicopter", "left").await,
                                            );
                                        },
                                    };
                                    game.spawned_enemy += 1;
                                }
                            }
                        } else {
                            game.enemy_on_screen = 0;
                            for enemy in &mut enemies {
                                if !enemy.destroyed {
                                    game.enemy_on_screen += 1;
                                }
                            }
                            if game.enemy_on_screen == 0 {
                                if get_time() - game.last_spawn_time >= 11.0 { // Spawn delay of the nex phase
                                    game_phase = GamePhase::Jets;
                                    game.spawned_enemy = 0;
                                    enemies.clear();
                                }
                            }
                        }
                    },
                    GamePhase::Jets => {
                        // Spawn jets
                        if game.spawned_enemy < 3 { // TODO how match jets will spawn in this phase (may be random amount?)
                            if get_time() - game.last_spawn_time >= 2.0 { // TODO need to add random delay
                                game.last_spawn_time = get_time();
                                enemies.push(Enemy::new("jet", "left").await);
                                game.spawned_enemy += 1;
                            }
                        } else {
                            game.enemy_on_screen = 0;
                            for enemy in &mut enemies {
                                if !enemy.destroyed {
                                    game.enemy_on_screen += 1;
                                }
                            }
                            if game.enemy_on_screen == 0 {
                                if get_time() - game.last_spawn_time >= 12.0 { // Spawn delay of the nex phase
                                    game_phase = GamePhase::Helicopters;
                                    game.spawned_enemy = 0;
                                    enemies.clear();
                                    game.level += 1;
                                }
                            }
                        }
                    },
                    GamePhase::Paratroopers => {},
                }

                // Draw enemies
                for enemy in &mut enemies {
                    enemy.draw();
                    enemy.update();
                }
                
                if is_key_pressed(KeyCode::Up) {
                    bullets.push(Bullet::new(canon.ex + screen_width() / 2.0, canon.ey + screen_height() - 110.0, canon.angle).await);
                    game.score -= 1;
                    if game.score < 0 {
                        game.score = 0;
                    }
                }

                // Draw bullets
                for bullet in &mut bullets {
                    bullet.draw();
                }
            }
        }

        next_frame().await
    }
}
