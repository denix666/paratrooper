use macroquad::{prelude::*, audio::{PlaySoundParams, play_sound}};
mod canon;
mod paratrooper;
use paratrooper::Paratrooper;
use canon::Canon;
mod enemy;
use enemy::Enemy;
mod divs;
use divs::Divs;
mod bullet;
use bullet::Bullet;
mod bomb;
use bomb::Bomb;
mod game;
use game::Game;
mod functions;
use functions::*;
extern crate rand;
use rand::Rng;

const MAX_LANDED: usize = 4;

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
    let mut rdivs: Vec<Divs> = Vec::new();
    let mut ldivs: Vec<Divs> = Vec::new();
    let mut bombs: Vec<Bomb> = Vec::new();
    let mut paratroopers: Vec<Paratrooper> = Vec::new();
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
                draw_hiscore(&game.hiscore.to_string());

                canon.draw();
                canon.update();

                if is_key_pressed(KeyCode::Space) {
                    bombs.push(
                        Bomb::new(100.0, 50.0 + 30.0, "left".to_string()).await,
                    );
                }

                match game_phase {
                    GamePhase::Helicopters => {
                        // Spawn helicopters
                        if game.spawned_enemy < 10 + game.level { // Amount of helicopters in this phase
                            if game.enemy_amount_now < 6 {
                                if get_time() - game.last_spawn_time >= rand::thread_rng().gen_range(0.4..=6.0) {
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
                                if get_time() - game.last_spawn_time >= 4.0 { // Spawn delay of the next phase
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
                            if get_time() - game.last_spawn_time >= rand::thread_rng().gen_range(0.4..=6.0) {
                                game.last_spawn_time = get_time();
                                match rand::thread_rng().gen_range(0..=1) { 
                                    0 => {
                                        enemies.push(
                                            Enemy::new("jet", "right").await,
                                        );
                                    },
                                    _ => {
                                        enemies.push(
                                            Enemy::new("jet", "left").await,
                                        );
                                    },
                                };
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
                                if get_time() - game.last_spawn_time >= 4.0 { // Spawn delay of the next phase
                                    game_phase = GamePhase::Helicopters;
                                    game.spawned_enemy = 0;
                                    enemies.clear();
                                    game.level += 1;
                                }
                            }
                        }
                    },
                    GamePhase::Paratroopers => {
                        println!("divs phase");
                    },
                }

                // Draw enemies
                for enemy in &mut enemies {
                    enemy.draw();
                    if enemy.have_paratrooper && !enemy.paratrooper_jumped {
                        if enemy.center_x() + 20.0 > enemy.will_jump_at && enemy.center_x() - 20.0 < enemy.will_jump_at {
                            paratroopers.push(
                                Paratrooper::new(enemy.will_jump_at, enemy.y).await,
                            );
                            enemy.paratrooper_jumped = true;
                        }
                    }
                    if enemy.have_bomb  && !enemy.bomb_released {
                        if enemy.center_x() + 20.0 > enemy.will_bomb_at && enemy.center_x() - 20.0 < enemy.will_bomb_at {
                            let side: String;
                            if enemy.will_bomb_at == 100.0 {
                                side = "left".to_string()
                            } else {
                                side = "right".to_string()
                            }
                            bombs.push(
                                Bomb::new(enemy.will_bomb_at, enemy.y + 30.0, side).await,
                            );
                            enemy.bomb_released = true;
                            play_sound(resources.bomb, PlaySoundParams {
                                looped: false,
                                volume: 1.0,
                            });
                        }
                    }
                }

                for paratrooper in &mut paratroopers {
                    for div in &mut ldivs {
                        if let Some(_) = div.rect.intersect(paratrooper.trooper_rect) {
                            paratrooper.landed = true;
                        }
                    }
                    for div in &mut rdivs {
                        if let Some(_) = div.rect.intersect(paratrooper.trooper_rect) {
                            paratrooper.landed = true;
                        }
                    }

                    paratrooper.draw();

                    if !paratrooper.destroyed {
                        if paratrooper.landed {
                            paratrooper.destroyed = true;
                            if paratrooper.trooper_x > 400.0 {
                                rdivs.push(
                                    Divs::new(paratrooper.trooper_x, paratrooper.trooper_y).await,
                                );
                            } else {
                                ldivs.push(
                                    Divs::new(paratrooper.trooper_x, paratrooper.trooper_y).await,
                                );
                            }
                        }
                    }
                }

                for div in &mut rdivs {
                    div.draw();
                    for paratrooper in &mut paratroopers {
                        if !paratrooper.destroyed && !paratrooper.have_para {
                            if let Some(_) = div.rect.intersect(paratrooper.trooper_rect) {
                                paratrooper.destroyed = true;
                                div.destroyed = true;
                            }
                        }
                    }
                }

                for div in &mut ldivs {
                    div.draw();
                    for paratrooper in &mut paratroopers {
                        if !paratrooper.destroyed && !paratrooper.have_para {
                            if let Some(_) = div.rect.intersect(paratrooper.trooper_rect) {
                                paratrooper.destroyed = true;
                                div.destroyed = true;
                            }
                        }
                    }
                }

                if is_key_pressed(KeyCode::Up) {
                    bullets.push(Bullet::new(canon.ex + screen_width() / 2.0, canon.ey + screen_height() - 110.0, canon.angle).await);
                    game.score -= 1;
                    play_sound(resources.shot, PlaySoundParams {
                        looped: false,
                        volume: 0.3,
                    });
                    if game.score < 0 {
                        game.score = 0;
                    }
                }

                // Draw bullets and check if they are intersect with any object
                for bullet in &mut bullets {
                    bullet.draw();
                    for enemy in &mut enemies {
                        if !enemy.destroyed && !bullet.destroyed {
                            if let Some(_) = bullet.rect.intersect(enemy.rect) {
                                enemy.destroyed = true;
                                bullet.destroyed = true;
                                game.score += 10;
                                play_sound(resources.crash, PlaySoundParams {
                                    looped: false,
                                    volume: 0.3,
                                });
                                break;
                            }
                        }
                    }
                    for paratrooper in &mut paratroopers {
                        if !paratrooper.destroyed && !bullet.destroyed {
                            if let Some(_) = bullet.rect.intersect(paratrooper.trooper_rect) {
                                paratrooper.destroyed = true;
                                bullet.destroyed = true;
                                game.score += 5;
                                play_sound(resources.crash, PlaySoundParams {
                                    looped: false,
                                    volume: 0.3,
                                });
                                break;
                            }
                        }
                    }
                    for paratrooper in &mut paratroopers {
                        if !paratrooper.destroyed && !bullet.destroyed {
                            if let Some(_) = bullet.rect.intersect(paratrooper.para_rect) {
                                bullet.destroyed = true;
                                paratrooper.have_para = false;
                                paratrooper.para_destroyed = true;
                                play_sound(resources.crash, PlaySoundParams {
                                    looped: false,
                                    volume: 0.3,
                                });
                                break;
                            }
                        }
                    }
                    for bomb in &mut bombs {
                        if !bomb.destroyed && !bullet.destroyed {
                            if let Some(_) = bullet.rect.intersect(bomb.rect) {
                                bullet.destroyed = true;
                                bomb.destroyed = true;
                                game.score += 30;
                                play_sound(resources.crash, PlaySoundParams {
                                    looped: false,
                                    volume: 0.3,
                                });
                                break;
                            }
                        }
                    }
                }

                for bomb in &mut bombs {
                    bomb.draw();
                }

                // Clear vectors
                match bombs.iter().position(|x| x.destroyed == true) {
                    Some(idx) => {
                        bombs.remove(idx);
                    },
                    None => {},
                };
                
                match bullets.iter().position(|x| x.destroyed == true) {
                    Some(idx) => {
                        bullets.remove(idx);
                    },
                    None => {},
                };

                // match paratroopers.iter().position(|x| x.destroyed == true) {
                //     Some(idx) => {
                //         paratroopers.remove(idx);
                //     },
                //     None => {},
                // };

                match ldivs.iter().position(|x| x.destroyed == true) {
                    Some(idx) => {
                        ldivs.remove(idx);
                    },
                    None => {},
                };

                match rdivs.iter().position(|x| x.destroyed == true) {
                    Some(idx) => {
                        rdivs.remove(idx);
                    },
                    None => {},
                };

                // Check how many divs were landed and switch game phase
                if rdivs.len() >= MAX_LANDED || ldivs.len() >= MAX_LANDED {
                    game_phase = GamePhase::Paratroopers;
                }


                // Store Hi-Score in var
                if game.score > game.hiscore {
                    game.hiscore = game.score;
                }
            }
        }

        next_frame().await
    }
}
