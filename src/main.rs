use macroquad::prelude::*;

mod canon;
use canon::Canon;

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
    let mut canon = Canon::new().await;
    
    loop {
        clear_background(BLACK);

        canon.draw();
        canon.update();

        next_frame().await
    }
}
