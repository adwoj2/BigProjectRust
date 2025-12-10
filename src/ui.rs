use macroquad::prelude::*;
use crate::prelude::GameState;


pub async fn run(state: &mut GameState) {
    loop {
        clear_background(WHITE);
        // rysowanie siatki, jednostek i UI
        if is_key_pressed(KeyCode::Escape) { break; }
        next_frame().await
    }
}