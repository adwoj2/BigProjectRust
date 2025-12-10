mod prelude;
mod character;
mod item;
mod effect;
mod inventory;
mod crafting;
mod battlefield;
mod hexgrid;
mod pathfinding;
mod ai;
mod ui;


use macroquad::prelude::*;
use prelude::GameState;


#[macroquad::main("BigTask")]
async fn main() {
    let mut state = GameState::new();
    ui::run(&mut state).await;
}