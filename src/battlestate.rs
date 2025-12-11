use crate::gamestate::{GameState, Stats, Effect};
use crate::hexgrid::Hex;
use crate::character::{Hero, Enemy};
use crate::pathfinding::{movement_range, bfs_path};
use crate::ai::{hex_distance, enemy_ai};
use macroquad::prelude::*;
use std::collections::{HashMap, HashSet};
use std::cmp::Reverse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnPhase {
    Start,
    Action,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainType {
    Rocks,
    Water,
}

#[derive(Debug)]
pub struct BattleState {
    pub heroes: Vec<HeroInstance>,
    pub enemies: Vec<EnemyInstance>,

    pub turn_order: Vec<UnitRef>,
    pub active_unit: usize, // index into turn_order

    pub phase: TurnPhase,

    pub terrain: HashMap<Hex, TerrainType>,
    pub occupied_hexes: HashSet<Hex>,

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


impl BattleState {
    pub fn tick(&mut self) {
        match self.phase {
            TurnPhase::Start => self.start_phase(),
            TurnPhase::Action => self.action_phase(),
            TurnPhase::End => self.end_phase(),
        }
    }

    fn start_phase(&mut self) {
        let unit = self.turn_order[self.active_unit];
        let movement = self.unit_movement(&unit);

        match unit {
            UnitRef::Hero(idx) => {
                let hero = &mut self.heroes[idx];

                hero.current_movement = movement;
                let hex = self.heroes[idx].hex;
                self.selected_unit = Some(unit);
                self.selected_unit_range =
                    movement_range(hex, movement, self.grid_bounds(), &self);
            }

            UnitRef::Enemy(idx) => {
                // Enemies get their movement reset but no selection
                let enemy = &mut self.enemies[idx];
                enemy.current_movement = movement;
            }
        }

        self.phase = TurnPhase::Action;
    }

    fn action_phase(&mut self) {
        match self.turn_order[self.active_unit] {
            UnitRef::Hero(_) => {
                // Wait for UI to resolve hero action.
                // When done, the UI should call:
                // self.phase = TurnPhase::End;
            }

            UnitRef::Enemy(i) => {
                self.execute_enemy_ai(i);
                self.phase = TurnPhase::End;
            }
        }
    }

    fn end_phase(&mut self) {
        self.selected_unit = None;
        self.selected_unit_range.clear();

        self.active_unit = (self.active_unit + 1) % self.turn_order.len();
        self.phase = TurnPhase::Start;
    }

    fn grid_bounds(&self) -> Hex {
        Hex { q: self.grid_width - 1, r: self.grid_height - 1 }
    }

    pub fn update_occupied_hexes(&mut self) {
        self.occupied_hexes.clear();
        for hero in &self.heroes {
            self.occupied_hexes.insert(hero.hex);
        }
        for enemy in &self.enemies {
            self.occupied_hexes.insert(enemy.hex);
        }
        println!("Updated occupied hexes: {:?}", self.occupied_hexes);
    }

    pub fn is_passable(&self, hex: Hex) -> bool {
        !self.terrain.contains_key(&hex) &&
        !self.occupied_hexes.contains(&hex)
    }

    pub fn is_passable_for_unit(&self, start: Hex, hex: Hex) -> bool {
        if hex == start {
            true 
        } else {
            self.is_passable(hex)
        }
    }
}

impl BattleState {
    pub fn is_player_turn(&self) -> bool {
        match self.turn_order[self.active_unit] {
            UnitRef::Hero(_) => true,
            UnitRef::Enemy(_) => false,
        }
    }

    pub fn end_turn(&mut self) {
        self.phase = TurnPhase::End;
    }

}

impl BattleState {
    fn execute_enemy_ai(&mut self, enemy_idx: usize) {
        enemy_ai(self, enemy_idx);
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitRef {
    Hero(usize),   // index into battle.heroes
    Enemy(usize),  // index into battle.enemies
}

#[derive(Debug, Clone)]
pub struct HeroInstance {
    pub id: u32,
    pub name: String,
    pub hex: Hex,
    pub stats: Stats,

    pub effects: Vec<Effect>,
    pub current_movement: i32,

    pub texture: Texture2D,
}

#[derive(Debug, Clone)]
pub struct EnemyInstance {
    pub id: u32,
    pub name: String,
    pub hex: Hex,
    pub stats: Stats,
    pub effects: Vec<Effect>,
    pub current_movement: i32,
    pub texture: Texture2D,
}

pub fn start_battle(state: &mut GameState) {
    let assets = state.assets.as_ref().expect("Assets must be loaded before starting a battle");

    let heroes_instance: Vec<HeroInstance> = state.player_party.iter().map(|hero| HeroInstance {
        id: hero.id,
        name: hero.name.clone(),
        hex: Hex { q: 2, r: 3 }, // default start pos for testing
        stats: hero.stats.clone(),
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

    let enemies_instance: Vec<EnemyInstance> = enemies.into_iter().map(|enemy| EnemyInstance {
        id: enemy.id,
        name: enemy.name,
        hex: enemy.hex,
        stats: enemy.stats.clone(),
        current_movement: enemy.stats.movement,
        effects: Vec::new(),
        texture: assets.enemy.clone(),
    }).collect();

    let mut battle = BattleState {
        heroes: heroes_instance,
        enemies: enemies_instance,
        turn_order: Vec::new(),
        active_unit: 0,

        phase: TurnPhase::Start,
        terrain: HashMap::new(),

        occupied_hexes: HashSet::new(),
        selected_unit: None,
        selected_unit_range: HashMap::new(),
        grid_width: 10,
        grid_height: 10,
    };

    generate_turn_order(&mut battle);


    battle.terrain.insert(Hex { q: 3, r: 4 }, TerrainType::Rocks);
    battle.terrain.insert(Hex { q: 4, r: 4 }, TerrainType::Rocks);
    battle.terrain.insert(Hex { q: 5, r: 2 }, TerrainType::Rocks);

    state.battle = Some(battle);

}


pub fn generate_turn_order(battle: &mut BattleState) {
    let mut units: Vec<(i32, UnitRef)> = battle.heroes.iter().enumerate()
        .map(|(i, hero)| (hero.stats.dexterity, UnitRef::Hero(i)))
        .chain(
            battle.enemies.iter().enumerate().map(|(i, enemy)| (enemy.stats.dexterity, UnitRef::Enemy(i)))
        )
        .collect();

    units.sort_by_key(|&(dex, _)| Reverse(dex));

    battle.turn_order = units.into_iter().map(|(_, unit)| unit).collect();
    battle.active_unit = 0;
}
