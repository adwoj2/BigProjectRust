use crate::hexgrid::Hex;
use crate::battlestate::{BattleState, UnitRef};
use crate::pathfinding::bfs_path;
use std::collections::HashSet;

/// Compute the hex distance between two axial coords.
pub fn hex_distance(a: Hex, b: Hex) -> i32 {
    ((a.q - b.q).abs()
        + (a.q + a.r - b.q - b.r).abs()
        + (a.r - b.r).abs()) / 2
}

pub fn enemy_ai(battle: &mut BattleState, enemy_index: usize) {
    let target_hex = enemy_ai_choose_move(battle, enemy_index); // for now only move
    battle.enemies[enemy_index].hex = target_hex;
    battle.update_occupied_hexes();
}

/// Enemy AI: move the enemy as close as possible to the closest hero.
/// Returns the new Hex the enemy *should move to*.
/// If no movement or path is possible, returns current position.
pub fn enemy_ai_choose_move(battle: &BattleState, enemy_index: usize) -> Hex {
    let enemy = &battle.enemies[enemy_index];
    let enemy_hex = enemy.hex;
    let move_points = enemy.current_movement;

    println!("Enemy");


    // No movement possible
    if move_points <= 0 {
        return enemy_hex;
    }

    // Gather hero positions
    let hero_positions: Vec<(usize, Hex)> = battle.heroes
        .iter()
        .enumerate()
        .map(|(i, h)| (i, h.hex))
        .collect();

    if hero_positions.is_empty() {
        return enemy_hex; // No targets
    }

    // --- 1) Pick closest hero by Hex distance ---
    let (closest_hero_index, closest_hero_hex) = hero_positions
        .into_iter()
        .min_by_key(|(_, h)| hex_distance(enemy_hex, *h))
        .unwrap();

    // --- 2) Pathfind toward hero ---
    let grid_boundary = Hex { q: battle.grid_width - 1, r: battle.grid_height - 1 };
    let full_path = bfs_path(enemy_hex, closest_hero_hex, grid_boundary, battle);

    if full_path.is_empty() {
        println!("Enemy {} at {:?} cannot find path to hero {} at {:?}", 
            enemy_index, enemy_hex, closest_hero_index, closest_hero_hex);
        return enemy_hex; // No path found
    }

    // The path includes enemy_hex as the first element; skip it
    let path_steps = &full_path[1..];

    // --- 3) Prevent walking through other units ---
    // Build an occupied set excluding the moving enemy
    let mut occupied: HashSet<Hex> = HashSet::new();
    for (i, h) in battle.heroes.iter().enumerate() {
        if h.hex != enemy_hex {
            occupied.insert(h.hex);
        }
    }
    for (i, e) in battle.enemies.iter().enumerate() {
        if i != enemy_index {
            occupied.insert(e.hex);
        }
    }

    // Filter path until we hit an occupied hex
    let mut free_path: Vec<Hex> = Vec::new();
    for hex in path_steps {
        if occupied.contains(hex) {
            break;
        }
        free_path.push(*hex);
    }

    println!("Enemy {} moving from {:?} towards hero {} at {:?}, free path: {:?}", 
        enemy_index, enemy_hex, closest_hero_index, closest_hero_hex, free_path);
    // Only target in path
    if free_path.is_empty() {
        return enemy_hex; 
    }

    // --- 4) Move as far along the path as allowed by movement ---
    let steps_to_move = free_path.len().min(move_points as usize);

    free_path[steps_to_move - 1]
}
