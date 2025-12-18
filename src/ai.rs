use crate::battlestate::{BattleState, UnitRef};
use crate::hexgrid::Hex;
use crate::pathfinding::bfs_path;

pub fn hex_distance(a: Hex, b: Hex) -> i32 {
    ((a.q - b.q).abs() + (a.q + a.r - b.q - b.r).abs() + (a.r - b.r).abs()) / 2
}

pub fn enemy_ai(battle: &mut BattleState, enemy: UnitRef) {
    let target_hex = enemy_ai_move(battle, enemy);
    battle.move_unit(enemy, target_hex);

    let attack_area = battle.range_to_area(battle.unit_hex(enemy), 1); // Placeholder for enemy attack range
    let (attackable_heroes, _) = battle.units_in_range(enemy, attack_area);

     let target = attackable_heroes
        .iter()
        .copied()
        .filter(|u| matches!(u, UnitRef::Hero(_)))
        .min_by_key(|&u| battle.unit_current_health(u));

    if let Some(hero) = target {
        enemy_ai_attack_hero(battle, enemy, hero);
    }
}

/// Return hex to move to for enemy AI. (closest hero)
/// If no movement or path is possible, returns current position.
pub fn enemy_ai_move(battle: &BattleState, enemy: UnitRef) -> Hex {
    let start = battle.unit_hex(enemy);
    let movement = battle.unit_movement(enemy);

    if movement <= 0 {
        return start;
    }

    let closest_hero_hex = battle.heroes
        .iter()
        .map(|h| h.hex)
        .min_by_key(|&h| hex_distance(start, h));

    let target = match closest_hero_hex {
        Some(h) => h,
        None => return start,
    };

    let boundary = Hex {
        q: battle.grid_width - 1,
        r: battle.grid_height - 1,
    };

    let path = bfs_path(start, target, boundary, battle);

    path.iter()
        .skip(1) // ignore starting hex
        .take(movement as usize)
        .take_while(|&&hex| battle.is_passable(hex))
        .last()
        .copied()
        .unwrap_or(start)
}

fn enemy_ai_can_attack_hero(battle: &BattleState, enemy: UnitRef, hero: UnitRef) -> bool {
    let attack_range = 1; //enemy.attack_range;// Placeholder for enemy attack range
    let attack_area = battle.range_to_area(battle.unit_hex(enemy), attack_range);
    battle.is_unit_in_area(hero, &attack_area)
}

fn enemy_ai_attack_hero(battle: &mut BattleState, enemy: UnitRef, hero: UnitRef) {
    battle.attack_unit(
        enemy,
        hero,
        1.0,
    );
}
