use macroquad::{prelude::*, audio::{PlaySoundParams, play_sound, stop_sound}};
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
mod animation;
use animation::Animation;
mod end_game;
use end_game::EndAnimation;
mod functions;
use functions::*;
extern crate rand;
use rand::Rng;

const MAX_LANDED: usize = 4;

pub enum GameState {
    Intro,
    Instructions,
    Game,
    LevelFail,
}

pub enum GamePhase {
    Helicopters,
    Jets,
    Paratroopers,
}

fn window_conf() -> Conf {
    let mut title = String::from("Paratrooper v");
    title.push_str(env!("CARGO_PKG_VERSION"));
    Conf {
        window_title: title
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
    let mut animations: Vec<Animation> = Vec::new();
    let mut paratroopers: Vec<Paratrooper> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut game_state = GameState::Intro;
    let mut game_phase = GamePhase::Helicopters;
    let resources = Resources::new().await;
    let mut divs_loaded = false;
    let mut end_animation = EndAnimation::new(
        0.0, 0.0,
        0.0, 0.0,
        0.0, 0.0,
        0.0, 0.0
        ).await;

    play_sound(resources.intro, PlaySoundParams {
        looped: false,
        volume: 0.3,
    });
    
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
                    canon.destroyed = false;
                    game.spawned_enemy = 0;
                    game.enemy_amount_now = 0;
                    game.enemy_on_screen = 0;
                    game.level = 0;
                    enemies.clear();
                    paratroopers.clear();
                    bombs.clear();
                    ldivs.clear();
                    rdivs.clear();
                    bullets.clear();
                    end_animation.animation_a_completed = false;
                    end_animation.animation_b_completed = false;
                    end_animation.animation_c_completed = false;
                    end_animation.animation_d_completed = false;
                    divs_loaded = false;
                    game.destoyed_by_bomb = false;
                    game_state = GameState::Game;
                    game_phase = GamePhase::Helicopters;
                }
            },
            
            GameState::Game => {
                stop_sound(resources.intro);
                draw_score(&game.score.to_string());
                draw_hiscore(&game.hiscore.to_string());

                canon.draw();
                canon.update();

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
                        // 4 or more divs landed - game over
                        ////////////////////////////////////
                        
                        if !divs_loaded {
                            for div in &mut ldivs {
                                if div.y < 545.0 {
                                    div.x += 0.1
                                }
                                if div.y < 520.0 {
                                    div.x += 0.2
                                }
                            }

                            for div in &mut rdivs {
                                if div.y < 545.0 {
                                    div.x -= 0.1
                                }
                                if div.y < 520.0 {
                                    div.x -= 0.2
                                }
                            }
                            
                            ldivs.sort_by(|a, b| b.x.partial_cmp(&a.x).unwrap());
                            rdivs.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

                            if ldivs.len() >= 4 {
                                end_animation.ax = ldivs[0].x;
                                end_animation.ay = ldivs[0].y;
                                end_animation.bx = ldivs[1].x;
                                end_animation.by = ldivs[1].y;
                                end_animation.cx = ldivs[2].x;
                                end_animation.cy = ldivs[2].y;
                                end_animation.dx = ldivs[3].x;
                                end_animation.dy = ldivs[3].y;
                                
                                for div in &mut ldivs {
                                    div.destroyed = true;
                                }
                            } else {
                                end_animation.ax = rdivs[0].x;
                                end_animation.ay = rdivs[0].y;
                                end_animation.bx = rdivs[1].x;
                                end_animation.by = rdivs[1].y;
                                end_animation.cx = rdivs[2].x;
                                end_animation.cy = rdivs[2].y;
                                end_animation.dx = rdivs[3].x;
                                end_animation.dy = rdivs[3].y;
                                
                                for div in &mut rdivs {
                                    div.destroyed = true;
                                }
                            }
                            divs_loaded = true;
                        } else {
                            end_animation.draw();
                        }

                        if end_animation.animation_d_completed {
                            end_animation.draw();
                            canon.destroyed = true;
                            animations.push(
                                Animation::new(screen_width() / 2.0 - 24.0, screen_height() - 150.0, "enemy_explode").await,
                            );
                            game.fail_time = get_time();
                            play_sound(resources.outro, PlaySoundParams {
                                looped: false,
                                volume: 0.3,
                            });
                            game_state = GameState::LevelFail;
                        }
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
                    } else {
                        if paratrooper.trooper_y > screen_height() - 53.0 && !paratrooper.have_para {
                            animations.push(
                                Animation::new(paratrooper.trooper_x - 15.0, paratrooper.trooper_y - 25.0, "die").await,
                            );
                        }
                    }
                }

                for div in &mut rdivs {
                    div.draw();
                    for paratrooper in &mut paratroopers {
                        if !paratrooper.have_para {
                            if let Some(_) = div.rect.intersect(paratrooper.trooper_rect) {
                                paratrooper.destroyed = true;
                                div.destroyed = true;
                                animations.push(
                                    Animation::new(paratrooper.trooper_x - 15.0, paratrooper.trooper_y - 25.0, "die").await,
                                );
                            }
                        }
                    }
                }

                for div in &mut ldivs {
                    div.draw();
                    for paratrooper in &mut paratroopers {
                        if !paratrooper.have_para {
                            if let Some(_) = div.rect.intersect(paratrooper.trooper_rect) {
                                paratrooper.destroyed = true;
                                div.destroyed = true;
                                animations.push(
                                    Animation::new(paratrooper.trooper_x - 15.0, paratrooper.trooper_y - 25.0, "die").await,
                                );
                            }
                        }
                    }
                }

                if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::Kp8) {
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
                                animations.push(
                                    Animation::new(enemy.center_x() - 24.0, enemy.center_y() - 25.0, "enemy_explode").await,
                                );
                                break;
                            }
                        }
                    }
                    for paratrooper in &mut paratroopers {
                        if !bullet.destroyed {
                            if let Some(_) = bullet.rect.intersect(paratrooper.trooper_rect) {
                                paratrooper.destroyed = true;
                                bullet.destroyed = true;
                                game.score += 5;
                                play_sound(resources.crash, PlaySoundParams {
                                    looped: false,
                                    volume: 0.3,
                                });
                                animations.push(
                                    Animation::new(paratrooper.trooper_x - 18.0, paratrooper.trooper_y - 19.0, "enemy_explode").await,
                                );
                                break;
                            }
                        }
                    }
                    for paratrooper in &mut paratroopers {
                        if !bullet.destroyed {
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
                                animations.push(
                                    Animation::new(bomb.x, bomb.y, "bomb_explode").await,
                                );
                                break;
                            }
                        }
                    }
                }

                for animation in &mut animations {
                    animation.draw();
                }

                for bomb in &mut bombs {
                    bomb.draw();
                    if bomb.y > screen_height() - 80.0 {
                        canon.destroyed = true;
                        bomb.destroyed = true;
                        animations.push(
                            Animation::new(screen_width() / 2.0 - 24.0, screen_height() - 150.0, "enemy_explode").await,
                        );
                        game.fail_time = get_time();
                        game.destoyed_by_bomb = true;
                        play_sound(resources.outro, PlaySoundParams {
                            looped: false,
                            volume: 0.3,
                        });
                        game_state = GameState::LevelFail;
                    }
                }

                // Check how many divs were landed and switch game phase
                if rdivs.len() >= MAX_LANDED || ldivs.len() >= MAX_LANDED {
                    game_phase = GamePhase::Paratroopers;
                }


                // Store Hi-Score in var
                if game.score > game.hiscore {
                    game.hiscore = game.score;
                }
            },
            GameState::LevelFail => {
                draw_score(&game.score.to_string());
                draw_hiscore(&game.hiscore.to_string());

                if !game.destoyed_by_bomb {
                    end_animation.draw();
                }
                canon.draw();

                for animation in &mut animations {
                    animation.draw();
                }

                for enemy in &mut enemies {
                    enemy.draw();
                }
                if get_time() - game.fail_time >= 6.0 {
                    draw_play_again_text();

                    if is_key_pressed(KeyCode::Space) {
                        game.score = 0;
                        canon.destroyed = false;
                        game.spawned_enemy = 0;
                        game.enemy_amount_now = 0;
                        game.enemy_on_screen = 0;
                        game.level = 0;
                        enemies.clear();
                        paratroopers.clear();
                        bombs.clear();
                        ldivs.clear();
                        rdivs.clear();
                        bullets.clear();
                        end_animation.animation_a_completed = false;
                        end_animation.animation_b_completed = false;
                        end_animation.animation_c_completed = false;
                        end_animation.animation_d_completed = false;
                        divs_loaded = false;
                        game.destoyed_by_bomb = false;
                        game_state = GameState::Game;
                        game_phase = GamePhase::Helicopters;
                    }
                    if is_key_pressed(KeyCode::I) {
                        game_state = GameState::Instructions;
                    }
                }
            },
        }

        // Clear vectors
        match bombs.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                bombs.remove(idx);
            },
            None => {},
        };

        match enemies.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                enemies.remove(idx);
            },
            None => {},
        };

        match animations.iter().position(|x| x.animation_completed == true) {
            Some(idx) => {
                animations.remove(idx);
            },
            None => {},
        };
        
        match bullets.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                bullets.remove(idx);
            },
            None => {},
        };

        match paratroopers.iter().position(|x| x.destroyed == true) {
            Some(idx) => {
                paratroopers.remove(idx);
            },
            None => {},
        };

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

        next_frame().await
    }
}
