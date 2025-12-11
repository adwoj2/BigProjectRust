mod ai;
mod battlestate;
mod character;
mod crafting;
mod effect;
mod gamestate;
mod hexgrid;
mod inventory;
mod item;
mod pathfinding;
mod ui;

use crate::character::*;
use crate::effect::Effect;
use crate::hexgrid::Hex;
use crate::inventory::Inventory;
use crate::ui::Assets;
use gamestate::GameState;
use macroquad::prelude::*;

#[macroquad::main("BigTask")]
async fn main() {
    let mut state = GameState::new();

    let assets = Assets::load().await;

    let fighter_stats = Stats {
        max_hp: 100,
        hp: 75,
        damage: (10, 16),
        attack: 16,
        defense: 11,
        initiative: 12,
        movement: 4,
    };

    let melee = Ability {
        id: 0,
        name: "Slash".to_string(),
        description: "A quick melee attack.".to_string(),
        damage_modifier: 1.5,
        effect: None,
        range: 1,
    };

    let ranged = Ability {
        id: 1,
        name: "Bola Throw".to_string(),
        description:
            "Throw a bola to entangle the target, reducing their movement. Range: 3 hexes."
                .to_string(),
        damage_modifier: 0.6,
        effect: Some(Effect::ReduceMovement(1, 2)), // reduce 1 movement for 2 turns
        range: 3,
    };

    let abilities = vec![melee, ranged];

    state.player_party.push(Hero {
        id: 0,
        name: "Fighter".to_string(),
        hex: Hex { q: 2, r: 3 },
        stats: fighter_stats,
        xp: 0,
        abilities: abilities,
        inventory: Inventory::new(),
        effects: Vec::new(),
    });

    state.assets = Some(assets);

    ui::run(&mut state).await;
}
