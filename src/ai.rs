use crate::battlestate::{BattleState, UnitRef};
use crate::hexgrid::Hex;
use crate::pathfinding::bfs_path;
use std::collections::HashSet;

pub fn hex_distance(a: Hex, b: Hex) -> i32 {
    ((a.q - b.q).abs() + (a.q + a.r - b.q - b.r).abs() + (a.r - b.r).abs()) / 2
}

pub fn enemy_ai(battle: &mut BattleState, enemy_index: usize) {
    let target_hex = enemy_ai_move(battle, enemy_index);
    battle.enemies[enemy_index].hex = target_hex;
    battle.update_occupied_hexes();

    let mut attackable_heroes: Vec<usize> = battle
        .heroes
        .iter()
        .enumerate()
        .map(|(i, _)| i)
        .filter(|i| enemy_ai_can_attack_hero(battle, enemy_index, *i))
        .collect();

    if let Some(&hero_index) = attackable_heroes.first() {
        enemy_ai_attack_hero(battle, enemy_index, hero_index);
    }
}

/// Move as close as possible to the closest hero.
/// Returns the new Hex the enemy *should move to*.
/// If no movement or path is possible, returns current position.
pub fn enemy_ai_move(battle: &BattleState, enemy_index: usize) -> Hex {
    let enemy = &battle.enemies[enemy_index];
    let enemy_hex = enemy.hex;
    let move_points = enemy.current_movement;

    if move_points <= 0 {
        return enemy_hex;
    }

    let hero_positions: Vec<(usize, Hex)> = battle
        .heroes
        .iter()
        .enumerate()
        .map(|(i, h)| (i, h.hex))
        .collect();

    if hero_positions.is_empty() {
        return enemy_hex;
    }

    let (closest_hero_index, closest_hero_hex) = hero_positions
        .into_iter()
        .min_by_key(|(_, h)| hex_distance(enemy_hex, *h))
        .unwrap();

    let grid_boundary = Hex {
        q: battle.grid_width - 1,
        r: battle.grid_height - 1,
    };
    let full_path = bfs_path(enemy_hex, closest_hero_hex, grid_boundary, battle);

    if full_path.is_empty() {
        println!(
            "Enemy {} at {:?} cannot find path to hero {} at {:?}",
            enemy_index, enemy_hex, closest_hero_index, closest_hero_hex
        );
        return enemy_hex;
    }

    let path_steps = &full_path[1..];

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

    let mut free_path: Vec<Hex> = Vec::new();
    for hex in path_steps {
        if occupied.contains(hex) {
            break;
        }
        free_path.push(*hex);
    }

    // Only target in path
    if free_path.is_empty() {
        return enemy_hex;
    }

    let steps_to_move = free_path.len().min(move_points as usize);

    free_path[steps_to_move - 1]
}

fn enemy_ai_can_attack_hero(battle: &BattleState, enemy_index: usize, hero_index: usize) -> bool {
    let enemy_hex = battle.enemies[enemy_index].hex;
    let hero_hex = battle.heroes[hero_index].hex;

    enemy_hex.is_adjacent(hero_hex)
}

fn enemy_ai_attack_hero(battle: &mut BattleState, enemy_index: usize, hero_index: usize) {
    battle.attack_unit(
        &UnitRef::Enemy(enemy_index),
        &UnitRef::Hero(hero_index),
        1.0,
    );
    println!("Enemy {} attacks Hero {}!", enemy_index, hero_index);
}
