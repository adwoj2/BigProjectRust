use crate::gamestate::{GameState, Stats, Effect};
use crate::hexgrid::Hex;
use crate::character::{Hero, Enemy};
use crate::pathfinding::{movement_range, bfs_path};
use macroquad::prelude::*;
use std::collections::{HashMap, HashSet};
use std::cmp::Reverse;

pub struct BattleState {
    pub heroes: Vec<HeroInstance>,
    pub enemies: Vec<EnemyInstance>,

    pub turn_order: Vec<UnitRef>,
    pub active_unit: usize, // index into turn_order

    pub selected_unit: Option<UnitRef>,
    pub selected_unit_range: HashMap<Hex, (i32, Vec<Hex>)>,

    pub grid_width: i32,
    pub grid_height: i32,
}

impl BattleState {
    pub fn unit_hex(&self, u: &UnitRef) -> Hex {
        match *u {
            UnitRef::Hero(i) => self.heroes[i].hex,
            UnitRef::Enemy(i) => self.enemies[i].hex,
        }
    }

    pub fn unit_name(&self, u: &UnitRef) -> String {
        match *u {
            UnitRef::Hero(i) => self.heroes[i].name.clone(),
            UnitRef::Enemy(i) => self.enemies[i].name.clone(),
        }
    }

    pub fn unit_movement(&self, u: &UnitRef) -> i32 {
        match *u {
            UnitRef::Hero(i) => self.heroes[i].stats.movement,
            UnitRef::Enemy(i) => self.enemies[i].stats.movement,
        }
    }
}

pub enum UnitRef {
    Hero(usize),   // index into battle.heroes
    Enemy(usize),  // index into battle.enemies
}

pub struct HeroInstance {
    pub id: u32,
    pub name: String,

    // POS ON BATTLEFIELD
    pub hex: Hex,

    // Reference to global stats
    pub stats: Stats,

    // TEMPORARY battle modifications
    // pub current_hp: i32,
    pub effects: Vec<Effect>,
    pub current_movement: i32,

    // Texture for rendering
    pub texture: Texture2D,
}

pub struct EnemyInstance {
    pub id: u32,
    pub name: String,
    pub hex: Hex,
    pub stats: Stats,
    // pub current_hp: i32,
    pub effects: Vec<Effect>,
    pub current_movement: i32,
    pub texture: Texture2D,
}

pub fn start_battle(state: &mut GameState) {
    let assets = state.assets.as_ref().unwrap();

    let heroes_instance = state.player_party.iter().map(|hero| HeroInstance {
        id: hero.id,
        name: hero.name.clone(),
        hex: Hex { q: 2, r: 3 }, // default start pos (later: spawn rules)
        stats: hero.stats.clone(),
        // current_hp: hero.stats.hp,
        current_movement: hero.stats.movement,
        effects: Vec::new(),
        texture: assets.hero.clone(),
    }).collect();

    let goblin_stats = Stats {
        max_hp: 30,
        hp: 20,
        strength: 12,
        dexterity: 15,
        intelligence: 2,
        defense: 7,
        movement: 3,
    };

    let enemies = vec![Enemy {
        id: 0,
        name: "Goblin".to_string(),
        hex: Hex { q: 7, r: 5 },
        stats: goblin_stats,
        effects: Vec::new(),
    }];

    let enemies_instance = enemies.into_iter().map(|enemy| EnemyInstance {
        id: enemy.id,
        name: enemy.name,
        hex: enemy.hex,
        stats: enemy.stats.clone(),
        // current_hp: enemy.stats.hp,
        current_movement: enemy.stats.movement,
        effects: Vec::new(),
        texture: assets.enemy.clone(),
    }).collect();

    let mut battle = BattleState {
        heroes: heroes_instance,
        enemies: enemies_instance,
        turn_order: Vec::new(),
        active_unit: 0,
        selected_unit: None,
        selected_unit_range: HashMap::new(),
        grid_width: 10,
        grid_height: 10,
    };

    // Assign battle state
    state.battle = Some(battle);
}

pub fn generate_turn_order(battle: &mut BattleState) {
    let mut units: Vec<(i32, UnitRef)> = battle.heroes.iter().enumerate()
        .map(|(i, hero)| (hero.stats.dexterity, UnitRef::Hero(i)))
        .chain(
            battle.enemies.iter().enumerate()
                .map(|(i, enemy)| (enemy.stats.dexterity, UnitRef::Enemy(i)))
        )
        .collect();

    // Sort by dexterity descending
    units.sort_by_key(|&(dex, _)| Reverse(dex));

    // Store only the UnitRef
    battle.turn_order = units.into_iter().map(|(_, unit)| unit).collect();
    battle.active_unit = 0;
}