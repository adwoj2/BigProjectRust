mod battlestate;
mod gamestate;
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
use gamestate::GameState;
use crate::inventory::Inventory;
use crate::ui::Assets;
use crate::character::*;
use crate::hexgrid::Hex;


#[macroquad::main("BigTask")]
async fn main() {
    let mut state = GameState::new();

    let assets = Assets::load().await;

    let fighter_stats = Stats{
        max_hp: 100,
        hp: 75,
        strength: 16,
        dexterity: 12,
        intelligence: 4,
        defense: 11,
        movement: 2,
    };
    
    state.player_party.push(Hero {
        id: 0,
        name: "Fighter".to_string(),
        hex: Hex { q: 2, r: 3 },
        stats: fighter_stats,
        xp: 0,
        abilities: Vec::new(),
        inventory: Inventory::new(),
        effects: Vec::new(),
    });

    state.assets = Some(assets);

    ui::run(&mut state).await;
}