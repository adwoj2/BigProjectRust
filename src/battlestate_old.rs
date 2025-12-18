use crate::ai::{enemy_ai, hex_distance};
use crate::character::{Ability, Enemy};
use crate::gamestate::{Effect, GameState, Stats};
use crate::hexgrid::Hex;
use crate::pathfinding::{movement_range};
use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

type AbilityId = usize;
type ObjectId = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Movement,
    AbilityTarget { hero_idx: usize, ability_idx: usize },
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

pub enum Occupant {
    Unit(UnitRef),
    Object(ObjectId),
}

#[derive(Debug)]
pub struct BattleState {
    pub heroes: HashMap<u32, HeroInstance>,
    pub enemies: HashMap<u32, EnemyInstance>,

    pub turn_order: Vec<UnitRef>,
    pub active_unit_idx: usize, // turn_order index

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
    pub fn unit(&self, u: UnitRef) -> Option<&dyn Unit> {
        match u {
            UnitRef::Hero(id) => self.heroes.get(&id).map(|h| h as &dyn Unit),
            UnitRef::Enemy(id) => self.enemies.get(&id).map(|e| e as &dyn Unit),
        }
    }

    pub fn unit_mut(&mut self, u: UnitRef) -> Option<&mut dyn Unit> {
        match u {
            UnitRef::Hero(id) => self.heroes.get_mut(&id).map(|h| h as &mut dyn Unit),
            UnitRef::Enemy(id) => self.enemies.get_mut(&id).map(|e| e as &mut dyn Unit),
        }
    }
}

// Unit info methods
impl BattleState {
    pub fn unit_hex(&self, u: UnitRef) -> Option<Hex> {
        match u {
            UnitRef::Hero(id) => self.heroes.get(&id).map(|h| h.hex),
            UnitRef::Enemy(id) => self.enemies.get(&id).map(|e| e.hex),
        }
    }

    pub fn unit_name(&self, u: UnitRef) -> Option<String> {
        match u {
            UnitRef::Hero(i) => self.heroes.get(&i).map(|h| h.name.clone()),
            UnitRef::Enemy(i) => self.enemies.get(&i).map(|e| e.name.clone()),
        }
    }

    // pub fn unit_stats(&self, u: UnitRef) -> Option<&Stats> {
    //     match u {
    //         UnitRef::Hero(i) => self.heroes.get(&i).map(|h| &h.stats),
    //         UnitRef::Enemy(i) => self.enemies.get(&i).map(|e| &e.stats),
    //     }
    // }

    pub fn unit_movement(&self, u: UnitRef) -> Option<i32> {
        match u {
            UnitRef::Hero(id) => self.heroes.get(&id).map(|h| h.stats.movement),
            UnitRef::Enemy(id) => self.enemies.get(&id).map(|e| e.stats.movement),
        }
    }



    pub fn unit_current_health(&self, u: UnitRef) -> i32 {
        match u {
            UnitRef::Hero(id) => self.heroes.get(&id).map(|h| h.stats.hp).unwrap_or(0),
            UnitRef::Enemy(id) => self.enemies.get(&id).map(|e| e.stats.hp).unwrap_or(0),
        }
    }

}


// Mutable unit info methods
impl BattleState {
    pub fn unit_hex_mut(&mut self, u: UnitRef) -> &mut Hex {
        match u {
            UnitRef::Hero(i) => &mut self.heroes[i].hex,
            UnitRef::Enemy(i) => &mut self.enemies[i].hex,
        }
    }

    pub fn unit_stats_mut(&mut self, u: UnitRef) -> &mut Stats {
        match u {
            UnitRef::Hero(i) => &mut self.heroes[i].stats,
            UnitRef::Enemy(i) => &mut self.enemies[i].stats,
        }
    }

    pub fn unit_current_movement_mut(&mut self, u: UnitRef) -> &mut i32 {
        match u {
            UnitRef::Hero(i) => &mut self.heroes[i].current_movement,
            UnitRef::Enemy(i) => &mut self.enemies[i].current_movement,
        }
    }

    pub fn unit_effects_mut(&mut self, u: UnitRef) -> &mut Vec<Effect> {
        match u {
            UnitRef::Hero(i) => &mut self.heroes[i].effects,
            UnitRef::Enemy(i) => &mut self.enemies[i].effects,
        }
    }  

    pub fn unit_current_health_mut(&mut self, u: UnitRef) -> &mut i32 {
        match u {
            UnitRef::Hero(i) => &mut self.heroes[i].stats.hp,
            UnitRef::Enemy(i) => &mut self.enemies[i].stats.hp,
        }
    }
}

// Turn phase methods
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
                self.selected_unit_range = movement_range(hex, movement, self.grid_bounds(), self);

                self.input_mode = InputMode::Movement;
            }

            UnitRef::Enemy(idx) => {
                let enemy = &mut self.enemies[idx];
                enemy.current_movement = movement;
            }
        }

        self.phase = TurnPhase::Action;
    }

    fn action_phase(&mut self) {
        match self.turn_order[self.active_unit] {
            UnitRef::Hero(_) => {
                print!("ACTIVE HERO {}", self.active_unit);

                // Needs player input to proceed
            }

            UnitRef::Enemy(i) => {
                print!("ACTIVE ENEMY {}", self.active_unit);

                self.execute_enemy_ai(UnitRef::Enemy(i));
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
}

// Battlefield grid methods
impl BattleState {
    fn grid_bounds(&self) -> Hex {
        Hex {
            q: self.grid_width - 1,
            r: self.grid_height - 1,
        }
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
        !self.terrain.contains_key(&hex) && !self.occupied_hexes.contains(&hex)
    }

    pub fn is_passable_for_unit(&self, start: Hex, hex: Hex) -> bool {
        if hex == start {
            true
        } else {
            self.is_passable(hex)
        }
    }

    pub fn grid_boundary(&self) -> Hex {
        Hex {
            q: self.grid_width - 1,
            r: self.grid_height - 1,
        }
    }
}

// Unit action methods
impl BattleState {
    pub fn kill_unit(&mut self, u: UnitRef) {
        match u {
            UnitRef::Hero(id) => { self.heroes.remove(&id); },
            UnitRef::Enemy(id) => { self.enemies.remove(&id); },
        }
        self.turn_order.retain(|&unit| unit != u);
    }

    pub fn range_to_area(&self, center_hex: Hex, range: i32) -> Vec<Hex> {
        let mut area = Vec::new();

        for dq in -range..=range {
            for dr in -range..=range {
                let ds = -dq - dr;
                if ds.abs() <= range {
                    let hex = Hex {
                        q: center_hex.q + dq,
                        r: center_hex.r + dr,
                    };
                    if hex.q >= 0
                        && hex.q < self.grid_width
                        && hex.r >= 0
                        && hex.r < self.grid_height
                    {
                        area.push(hex);
                    }
                }
            }
        }

        area
    }

    pub fn units_in_range(&self, unit: UnitRef, area: Vec<Hex>) -> (Vec<UnitRef>, Vec<UnitRef>) {
        let in_range = |hex: &Hex| area.iter().any(|h| h == hex);

        let heroes: Vec<UnitRef> = self.heroes
            .iter()
            .enumerate()
            .filter(|(i, hero)| {
                in_range(&hero.hex) && UnitRef::Hero(*i)
            })
            .map(|(i, _)| UnitRef::Hero(i))
            .collect();

        let enemies: Vec<UnitRef> = self.enemies
            .iter()
            .enumerate()
            .filter(|(i, enemy)| {
                in_range(&enemy.hex) && UnitRef::Enemy(*i)
            })
            .map(|(i, _)| UnitRef::Enemy(i))
            .collect();

        match unit {
            UnitRef::Hero(_) => (heroes, enemies),
            UnitRef::Enemy(_) => (enemies, heroes),
        }
    }

    pub fn is_unit_in_area(&self, unit: UnitRef, area: &[Hex]) -> bool {
        let unit_hex = self.unit_hex(unit);
        area.iter().any(|&h| h == unit_hex)
    }

    pub fn attack_unit(&mut self, attacker: UnitRef, target: UnitRef, damage_multiplier: f32) {
        let (attacker_stats, target_stats) = match (attacker, target) {
            
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

        println!(
            "Attack vs Defense: {}, Modifier: {:.2}, base damage: {:.2}, final damage: {}",
            attack_vs_defense, attack_modifier, base_damage, damage
        );

        target_stats.hp = (target_stats.hp - damage).max(0);
        // Needs bugfix so that the game won't crash
        if target_stats.hp == 0 {
            println!("{} has been defeated!", self.unit_name(target));
            match target {
                UnitRef::Hero(idx) => {
                    self.heroes.remove(*idx);
                    if self.selected_unit == Some(UnitRef::Hero(*idx)) {
                        self.selected_unit = None;
                        self.selected_unit_range.clear();
                    }
                    self.turn_order.retain(|u| match u {
                        UnitRef::Hero(i) => *i != *idx,
                        _ => true,
                    });
                }
                UnitRef::Enemy(idx) => {
                    self.enemies.remove(*idx);
                    if self.selected_unit == Some(UnitRef::Enemy(*idx)) {
                        self.selected_unit = None;
                        self.selected_unit_range.clear();
                    }
                    self.turn_order.retain(|u| match u {
                        UnitRef::Enemy(i) => *i != *idx,
                        _ => true,
                    });
                }
            }
            self.update_occupied_hexes();
        }

        println!(
            "{} attacks {} for {} damage!",
            self.unit_name(attacker),
            self.unit_name(target),
            damage
        );
    }

    pub fn move_unit_to(&mut self, unit: UnitRef, target_hex: Hex) {
        let unit_hex = self.unit_hex(unit);
        let movement = self.unit_movement(unit);

        let path_map = movement_range(unit_hex, movement, self.grid_bounds(), self);

        // TODO animation
        if let Some((dist, path)) = path_map.get(&target_hex) {
            if *dist > 0 {
                *self.unit_hex_mut(unit) = target_hex;
                let current_movement = self.unit_current_movement_mut(unit);
                *current_movement -= *dist;

                println!(
                    "{} moved from {:?} to {:?}, remaining movement: {}",
                    self.unit_name(unit),
                    unit_hex,
                    target_hex,
                    *current_movement
                );

                self.update_occupied_hexes();
            }
        }
    }

    pub fn hero_use_ability(&mut self, hero_idx: usize, ability_idx: usize, target_hex: Hex) {
        let ability_range = self.heroes[hero_idx].abilities[ability_idx].range;
        let damage = self.heroes[hero_idx].abilities[ability_idx].damage_modifier;
        let effect = self.heroes[hero_idx].abilities[ability_idx].effect.clone();
        let hero_hex = self.heroes[hero_idx].hex;

        if hex_distance(hero_hex, target_hex) > ability_range {
            return;
        }

        if let Some(enemy_idx) = self.enemies.iter().position(|e| e.hex == target_hex) {
            self.attack_unit(UnitRef::Hero(hero_idx), UnitRef::Enemy(enemy_idx), damage);

            if let Some(eff) = effect {
                self.enemies[enemy_idx].effects.push(eff);
            }

            self.heroes[hero_idx].action_available = false;
            self.selected_ability = None;
        }
    }

}

impl BattleState {
    pub fn generate_turn_order(&mut self) {
        let mut units: Vec<(i32, UnitRef)> = self.heroes.iter()
            .map(|(&id, h)| (h.stats.initiative, UnitRef::Hero(id)))
            .chain(
                self.enemies.iter().map(|(&id, e)| (e.stats.initiative, UnitRef::Enemy(id)))
            )
            .collect();

        units.sort_by_key(|&(initiative, _)| std::cmp::Reverse(initiative));

        self.turn_order = units.into_iter().map(|(_, u)| u).collect();
        self.active_unit_idx = 0;
    }

    pub fn active_unit(&self) -> Option<UnitRef> {
        self.turn_order.get(self.active_unit_idx).copied()
    }

    pub fn next_unit(&mut self) {
        self.active_unit_idx = (self.active_unit_idx + 1) % self.turn_order.len();
    }

    pub fn is_player_turn(&self) -> bool {
        match self.turn_order[self.active_unit_idx] {
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
    fn execute_enemy_ai(&mut self, enemy: UnitRef) {
        enemy_ai(self, enemy);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitRef {
    Hero(u32),
    Enemy(u32),
}

#[derive(Debug, Clone)]
pub struct HeroInstance {
    pub id: u32,
    pub name: String,
    pub hex: Hex,
    pub stats: Stats,

    pub abilities: Vec<Ability>,
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
    let assets = state
        .assets
        .as_ref()
        .expect("Assets must be loaded before starting a battle");

    let heroes_instance: Vec<HeroInstance> = state
        .player_party
        .iter()
        .map(|hero| HeroInstance {
            id: hero.id,
            name: hero.name.clone(),
            hex: Hex { q: 2, r: 3 }, // for testing
            stats: hero.stats.clone(),
            action_available: true,
            abilities: hero.abilities.clone(),
            current_movement: hero.stats.movement,
            effects: Vec::new(),
            texture: assets.hero.clone(),
        })
        .collect();

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

    let enemies = vec![
        Enemy {
            id: 0,
            name: "Goblin".to_string(),
            hex: Hex { q: 7, r: 5 }, // for testing
            stats: goblin_stats,
            effects: Vec::new(),
        },
        Enemy {
            id: 1,
            name: "Orc".to_string(),
            hex: Hex { q: 8, r: 6 }, // for testing
            stats: orc_stats,
            effects: Vec::new(),
        },
    ];

    let enemies_instance: Vec<EnemyInstance> = enemies
        .into_iter()
        .map(|enemy| EnemyInstance {
            texture: assets.enemy.get(&enemy.name).unwrap().clone(),
            id: enemy.id,
            name: enemy.name,
            hex: enemy.hex,
            stats: enemy.stats.clone(),
            current_movement: enemy.stats.movement,
            effects: Vec::new(),
        })
        .collect();

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

    // for testing terrain
    battle
        .terrain
        .insert(Hex { q: 3, r: 4 }, TerrainType::Rocks);
    battle
        .terrain
        .insert(Hex { q: 4, r: 4 }, TerrainType::Rocks);
    battle
        .terrain
        .insert(Hex { q: 5, r: 2 }, TerrainType::Rocks);

    state.battle = Some(battle);
}

