pub struct Game {
    pub score: i32,
}

impl Game {
    pub async fn new()  -> Self {
        Self {
            score: 0,
        }
    }
}