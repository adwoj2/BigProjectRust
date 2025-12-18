use std::collections::{HashMap, HashSet};
use macroquad::prelude::*;
use crate::hexgrid::Hex;
use crate::character::{Hero, Enemy, Stats};
use crate::effect::Effect;
use crate::battlefield::{HexInfo, Terrain};
use crate::battle_command::{BattleCommand};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnitRef {
    Hero(u32),
    Enemy(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeroInstance {
    pub id: u32,
    pub name: String,
    pub hex: Hex,

    pub stats: Stats,
    pub abilities: Vec<Ability>,

    pub current_movement: i32,
    pub action_available: bool,
    pub effects: Vec<Effect>,

    pub texture: Texture2D,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnemyInstance {
    pub id: u32,
    pub name: String,
    pub hex: Hex,

    pub stats: Stats,
    // pub abilities: Vec<Ability>,

    pub current_movement: i32,
    // pub action_available: bool,
    pub effects: Vec<Effect>,

    pub texture: Texture2D,
}

pub struct BattleState {
    pub heroes: HashMap<u32, HeroInstance>,
    pub enemies: HashMap<u32, EnemyInstance>,

    pub turn_order: Vec<UnitRef>,
    pub active_unit_idx: usize,

    pub selected_unit: Option<UnitRef>,
    pub selected_ability: Option<usize>,

    pub grid_width: i32,
    pub grid_height: i32,

     pub hex_map: HashMap<Hex, HexInfo>,
}

impl HeroInstance {
    pub fn from_hero(hero: &Hero, hex: Hex, texture: Texture2D) -> Self {
        Self {
            id: hero.id,
            name: hero.name.clone(),
            hex,
            stats: hero.base_stats.clone(),
            abilities: hero.abilities.clone(),
            current_movement: hero.base_stats.movement,
            action_available: true,
            effects: vec![],
            texture,
        }
    }
}

impl EnemyInstance {
    pub fn from_enemy(enemy: &Enemy, hex: Hex, texture: Texture2D) -> Self {
        Self {
            id: enemy.id,
            name: enemy.name.clone(),
            hex,
            stats: enemy.base_stats.clone(),
            // abilities: enemy.abilities.clone(),
            current_movement: enemy.base_stats.movement,
            // action_available: true,
            effects: vec![],
            texture,
        }
    }
}


// --------------------- Access helpers ---------------------
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

    pub fn hero(&self, id: u32) -> Option<&HeroInstance> {
        self.heroes.get(&id)
    }

    pub fn hero_mut(&mut self, id: u32) -> Option<&mut HeroInstance> {
        self.heroes.get_mut(&id)
    }

    pub fn enemy(&self, id: u32) -> Option<&EnemyInstance> {
        self.enemies.get(&id)
    }

    pub fn enemy_mut(&mut self, id: u32) -> Option<&mut EnemyInstance> {
        self.enemies.get_mut(&id)
    }
}

// --------------------- Unit attributes access ---------------------
impl BattleState {
    pub fn unit_current_health(&self, u: UnitRef) -> i32 {
        match u {
            UnitRef::Hero(id) => self.heroes.get(&id).map(|h| h.stats.hp).unwrap_or(0),
            UnitRef::Enemy(id) => self.enemies.get(&id).map(|e| e.stats.hp).unwrap_or(0),
        }
    }

    pub fn unit_current_health_mut(&mut self, u: UnitRef) -> Option<&mut i32> {
        match u {
            UnitRef::Hero(id) => self.heroes.get_mut(&id).map(|h| &mut h.stats.hp),
            UnitRef::Enemy(id) => self.enemies.get_mut(&id).map(|e| &mut e.stats.hp),
        }
    }

    pub fn unit_hex(&self, u: UnitRef) -> Option<Hex> {
        match u {
            UnitRef::Hero(id) => self.heroes.get(&id).map(|h| h.hex),
            UnitRef::Enemy(id) => self.enemies.get(&id).map(|e| e.hex),
        }
    }

    pub fn unit_name(&self, u: UnitRef) -> Option<String> {
        match u {
            UnitRef::Hero(id) => self.heroes.get(&id).map(|h| h.name.clone()),
            UnitRef::Enemy(id) => self.enemies.get(&id).map(|e| e.name.clone()),
        }
    }
}

// --------------------- Unit manipulation ---------------------
impl BattleState {
    pub fn move_unit(&mut self, u: UnitRef, dest: Hex) {
        if let Some(prev_hex) = self.unit_hex(u) {
            if let Some(info) = self.hex_map.get_mut(&prev_hex) {
                info.occupying_unit = None;
            }
        }

        if let Some(info) = self.hex_map.get_mut(&dest) {
            info.occupying_unit = Some(u);
        }

        // Update the unit itself
        match u {
            UnitRef::Hero(id) => if let Some(hero) = self.heroes.get_mut(&id) { hero.hex = dest; },
            UnitRef::Enemy(id) => if let Some(enemy) = self.enemies.get_mut(&id) { enemy.hex = dest; },
        }
    }

    pub fn kill_unit(&mut self, u: UnitRef) {
        match u {
            UnitRef::Hero(id) => { self.heroes.remove(&id); },
            UnitRef::Enemy(id) => { self.enemies.remove(&id); },
        }
        self.turn_order.retain(|&unit| unit != u);
    }
}

// --------------------- Hex map management ---------------------
impl BattleState {
    pub fn initialize_hex_map(&mut self) {
        self.hex_map.clear();
        for q in 0..self.grid_width {
            for r in 0..self.grid_height {
                let terrain = if (q + r) % 5 == 0 { Terrain::Forest } else { Terrain::Plain }; //Placeholder
                self.hex_map.insert(Hex { q, r }, HexInfo::new(terrain));
            }
        }
    }

    pub fn is_hex_passable(&self, hex: Hex) -> bool {
        self.hex_map.get(&hex).map_or(false, |info| info.passable && !info.is_occupied())
    }
}

// --------------------- Turn order management ---------------------
impl BattleState {
    pub fn generate_turn_order(&mut self) {
        let mut units: Vec<(i32, UnitRef)> = self.heroes.iter()
            .map(|(&id, h)| (h.stats.dexterity, UnitRef::Hero(id)))
            .chain(
                self.enemies.iter().map(|(&id, e)| (e.stats.dexterity, UnitRef::Enemy(id)))
            )
            .collect();

        units.sort_by_key(|&(dex, _)| std::cmp::Reverse(dex));

        self.turn_order = units.into_iter().map(|(_, u)| u).collect();
        self.active_unit_idx = 0;
    }

    pub fn active_unit(&self) -> Option<UnitRef> {
        self.turn_order.get(self.active_unit_idx).copied()
    }

    pub fn next_unit(&mut self) {
        self.active_unit_idx = (self.active_unit_idx + 1) % self.turn_order.len();
    }
}

impl BattleState {
    pub fn new(heroes: &[Hero], enemies: &[Enemy], assets: &Assets) -> Self {
        let heroes = heroes.iter().enumerate().map(|(i, h)| {
            HeroInstance::from_hero(
                    h,
                    Hex { q: 2 + i as i32, r: 3 },
                    assets.hero.clone(),
                )
        }).collect();
    
        let enemies = enemies.into_iter().enumerate().map(|(i, e)| {
            EnemyInstance::from_enemy(
                e,
                Hex { q: 7 + i as i32, r: 5 },
                assets.enemy.clone(),
            )
        }).collect();

        let mut battle = Self {
            heroes,
            enemies,
            turn_order: Vec::new(),
            active_unit: 0,
            phase: TurnPhase::Start,
            input_mode: InputMode::Normal,
            selected_unit: None,
            selected_unit_range: HashMap::new(),
            selected_ability: None,
            grid_width: 10,
            grid_height: 10,
            hex_map: HashMap::new(),
        };

        battle.initialize_hex_map();
        battle.update_occupied_hexes();
        battle.generate_turn_order();

        battle
    }
}

pub enum BattleCommand {
    SelectUnit(UnitRef),
    MoveSelectedUnit(Hex),
    UseAbility {
        caster: UnitRef,
        ability_idx: usize,
        target: Hex,
    },
    CancelAction,
    EndTurn,
}

impl BattleState {
    pub fn handle_command(&mut self, cmd: BattleCommand) {
        match cmd {
            BattleCommand::SelectUnit(unit) => {
                self.select_unit(unit);
            }

            BattleCommand::MoveSelectedUnit(hex) => {
                self.try_move_selected(hex);
            }

            BattleCommand::UseAbility { caster, ability_idx, target } => {
                self.try_use_ability(caster, ability_idx, target);
            }

            BattleCommand::CancelAction => {
                self.clear_selection();
            }

            BattleCommand::EndTurn => {
                self.end_turn();
            }
        }
    }
}

pub trait Unit {
    fn hex(&self) -> Hex;
    fn stats(&self) -> &Stats;
    fn stats_mut(&mut self) -> &mut Stats;
    fn effects(&self) -> &Vec<Effect>;
    fn effects_mut(&mut self) -> &mut Vec<Effect>;
}

impl Unit for HeroInstance {
    fn hex(&self) -> Hex { self.hex }
    fn stats(&self) -> &Stats { &self.stats }
    fn stats_mut(&mut self) -> &mut Stats { &mut self.stats }
    fn effects(&self) -> &Vec<Effect> { &self.effects }
    fn effects_mut(&mut self) -> &mut Vec<Effect> { &mut self.effects }
}

impl Unit for EnemyInstance {
    fn hex(&self) -> Hex { self.hex }
    fn stats(&self) -> &Stats { &self.stats }
    fn stats_mut(&mut self) -> &mut Stats { &mut self.stats }
    fn effects(&self) -> &Vec<Effect> { &self.effects }
    fn effects_mut(&mut self) -> &mut Vec<Effect> { &mut self.effects }
}