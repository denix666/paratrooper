pub struct Game {
    pub score: i32,
    pub hiscore: i32,
    pub enemy_amount_now: i32,
    pub last_spawn_time: f64,
    pub spawned_enemy: i32,
    pub enemy_on_screen: i32,
    pub level: i32,
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            score: 0,
            hiscore: 0,
            enemy_amount_now: 0,
            last_spawn_time: 0.0,
            spawned_enemy: 0,
            enemy_on_screen: 0,
            level: 0,
        }
    }
}