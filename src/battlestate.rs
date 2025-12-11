use crate::gamestate::{GameState, Stats, Effect};
use crate::hexgrid::Hex;
use crate::character::{Hero, Enemy, Ability};
use crate::pathfinding::{movement_range, bfs_path};
use crate::ai::{hex_distance, enemy_ai};
use macroquad::prelude::*;
use std::collections::{HashMap, HashSet};
use std::cmp::Reverse;
use ::rand::{Rng, thread_rng};
use crate::ui::hex_to_screen;

type AbilityId = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Movement,
    AbilityTarget {  
        hero_idx: usize,
        ability_idx: usize,
    },
}

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
    pub selected_ability: Option<AbilityId>,
    pub input_mode: InputMode,

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
                hero.action_available = true;
                let hex = self.heroes[idx].hex;
                self.selected_unit = Some(unit);
                self.selected_unit_range = movement_range(hex, movement, self.grid_bounds(), &self);
                
                self.input_mode = InputMode::Movement;
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
                // Needs player input to proceed
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
}

impl BattleState {
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
    pub fn attack_unit(&mut self, attacker: &UnitRef, target: &UnitRef, damage_multiplier: f32) {
        let (attacker_stats, target_stats) = match (attacker, target) {
            (UnitRef::Hero(a_idx), UnitRef::Enemy(t_idx)) => {
                (&self.heroes[*a_idx].stats, &mut self.enemies[*t_idx].stats)
            }
            (UnitRef::Enemy(a_idx), UnitRef::Hero(t_idx)) => {
                (&self.enemies[*a_idx].stats, &mut self.heroes[*t_idx].stats)
            }
            _ => {
                println!("Invalid attack: both units are of the same type.");
                return;
            }
        };

        let attack_vs_defense = attacker_stats.attack - target_stats.defense;
        let attack_modifier = if attack_vs_defense >= 0 {
            1.0 + ((attack_vs_defense as f32) * 0.05).min(2.0)
        } else {
            (1.0 - (attack_vs_defense as f32) * 0.05).max(0.3)
        };
        
        
        let mut rng = thread_rng();
        let base_damage = rng.gen_range(attacker_stats.damage.0..=attacker_stats.damage.1) as f32;
        let damage = (base_damage * attack_modifier * damage_multiplier).round() as i32;

        println!("Attack vs Defense: {}, Modifier: {:.2}, base damage: {:.2}, final damage: {}", attack_vs_defense, attack_modifier, base_damage, damage);

        target_stats.hp = (target_stats.hp - damage).max(0);
        //Needs bugfix so that the game won't crash
        // if target_stats.hp == 0 {
        //     println!("{} has been defeated!", self.unit_name(target));
        //     match target {
        //         UnitRef::Hero(idx) => {
        //             self.heroes.remove(*idx);
        //             self.turn_order.retain(|u| match u {
        //                 UnitRef::Hero(i) => *i != *idx,
        //                 _ => true,
        //             });
        //         }
        //         UnitRef::Enemy(idx) => {
        //             self.enemies.remove(*idx);
        //             self.turn_order.retain(|u| match u {
        //                 UnitRef::Enemy(i) => *i != *idx,
        //                 _ => true,
        //             });
        //         }
        //     }
        //     self.update_occupied_hexes();
        // }

        println!("{} attacks {} for {} damage!", 
            self.unit_name(attacker), 
            self.unit_name(target), 
            damage);
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
        self.input_mode = InputMode::Normal;
        self.selected_ability = None;
        self.selected_unit = None;
        self.selected_unit_range.clear();
    }
}

impl BattleState {
    fn execute_enemy_ai(&mut self, enemy_idx: usize) {
        enemy_ai(self, enemy_idx);
    }
}

impl BattleState {
    pub fn hero_use_ability(&mut self, hero_idx: usize, ability_idx: usize, target_hex: Hex) {
    let ability_range = self.heroes[hero_idx].abilities[ability_idx].range;
    let damage = self.heroes[hero_idx].abilities[ability_idx].damage_modifier;
    let effect = self.heroes[hero_idx].abilities[ability_idx].effect.clone();
    let hero_hex = self.heroes[hero_idx].hex;

    if hex_distance(hero_hex, target_hex) > ability_range {
        return;
    }

    if let Some(enemy_idx) = self.enemies.iter().position(|e| e.hex == target_hex) {
        self.attack_unit(&UnitRef::Hero(hero_idx), &UnitRef::Enemy(enemy_idx), damage);

        if let Some(eff) = effect {
            self.enemies[enemy_idx].effects.push(eff);
        }

        self.heroes[hero_idx].action_available = false;
        self.selected_ability = None;
    }
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

    pub abilities : Vec<Ability>,
    pub effects: Vec<Effect>,
    pub current_movement: i32,
    pub action_available: bool,
    

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
        action_available: true,
        abilities: hero.abilities.clone(),
        current_movement: hero.stats.movement,
        effects: Vec::new(),
        texture: assets.hero.clone(),
    }).collect();

    let goblin_stats = Stats {
        max_hp: 30,
        hp: 30,
        damage: (5, 10),
        attack: 12,
        defense: 7,
        initiative: 15,
        movement: 3,
    };
    let orc_stats = Stats {
        max_hp: 60,
        hp: 45,
        damage: (15, 20),
        attack: 15,
        defense: 2,
        initiative: 5,
        movement: 2,
    };

    let enemies = vec![Enemy {
        id: 0,
        name: "Goblin".to_string(),
        hex: Hex { q: 7, r: 5 },
        stats: goblin_stats,
        effects: Vec::new(),
    }, Enemy {
        id: 1,
        name: "Orc".to_string(),
        hex: Hex { q: 8, r: 6 },
        stats: orc_stats,
        effects: Vec::new(),
    }
    ];

    let enemies_instance: Vec<EnemyInstance> = enemies.into_iter().map(|enemy| EnemyInstance {
        texture: assets.enemy.get(&enemy.name).unwrap().clone(),
        id: enemy.id,
        name: enemy.name,
        hex: enemy.hex,
        stats: enemy.stats.clone(),
        current_movement: enemy.stats.movement,
        effects: Vec::new(),
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
        selected_ability: None,
        input_mode: InputMode::Normal,

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
        .map(|(i, hero)| (hero.stats.initiative, UnitRef::Hero(i)))
        .chain(
            battle.enemies.iter().enumerate().map(|(i, enemy)| (enemy.stats.initiative, UnitRef::Enemy(i)))
        )
        .collect();

    units.sort_by_key(|&(dex, _)| Reverse(dex));

    battle.turn_order = units.into_iter().map(|(_, unit)| unit).collect();
    battle.active_unit = 0;
}
