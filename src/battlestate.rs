use crate::ai::enemy_ai;
use crate::assets::Assets;
use crate::battlefield::{HexInfo, Terrain};
use crate::character::{Ability, Enemy, Hero, Stats};
use crate::effect::Effect;
use crate::hexgrid::Hex;
use crate::pathfinding::movement_range;
use ::rand::{thread_rng, Rng};
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub enum BattleResult {
    Victory,
    Defeat,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnitRef {
    Hero(u32),
    Enemy(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Movement,
    AbilityTarget(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TurnPhase {
    Start,
    Action,
    End,
}

#[derive(Clone, Debug)]
pub enum BattleCommand {
    SelectUnit(UnitRef),
    SelectUnitAtHex(Hex),
    MoveSelectedUnit(Hex),
    SelectAbility(usize),
    UseAbility(Hex),
    CancelAction,
    RequestEndTurn,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

pub trait UnitRender {
    fn hex(&self) -> Hex;
    fn texture(&self) -> &Texture2D;
    fn health_percent(&self) -> f32;
}

impl UnitRender for HeroInstance {
    fn hex(&self) -> Hex {
        self.hex
    }

    fn texture(&self) -> &Texture2D {
        &self.texture
    }

    fn health_percent(&self) -> f32 {
        self.stats.hp as f32 / self.stats.max_hp as f32
    }
}

impl UnitRender for EnemyInstance {
    fn hex(&self) -> Hex {
        self.hex
    }

    fn texture(&self) -> &Texture2D {
        &self.texture
    }

    fn health_percent(&self) -> f32 {
        (self.stats.hp as f32 / self.stats.max_hp as f32).clamp(0.0, 1.0)
    }
}

pub struct BattleState {
    pub heroes: HashMap<u32, HeroInstance>,
    pub enemies: HashMap<u32, EnemyInstance>,

    pub turn_order: Vec<UnitRef>,
    pub active_unit_idx: usize,

    pub selected_unit: Option<UnitRef>,
    pub selected_unit_range: HashMap<Hex, (i32, Vec<Hex>)>,
    pub selected_ability: Option<usize>,
    pub selected_ability_range: Vec<Hex>,

    pub grid_width: i32,
    pub grid_height: i32,
    pub hex_map: HashMap<Hex, HexInfo>,

    pub phase: TurnPhase,
    pub input_mode: InputMode,

    pub result: Option<BattleResult>,
}

impl HeroInstance {
    pub fn from_hero(hero: &Hero, hex: Hex, texture: Texture2D) -> Self {
        Self {
            id: hero.id,
            name: hero.name.clone(),
            hex,
            stats: hero.stats.clone(),
            abilities: hero.abilities.clone(),
            current_movement: hero.stats.movement,
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
            stats: enemy.stats.clone(),
            // abilities: enemy.abilities.clone(),
            current_movement: enemy.stats.movement,
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

    pub fn unit_current_movement(&self, u: UnitRef) -> Option<i32> {
        match u {
            UnitRef::Hero(id) => self.heroes.get(&id).map(|h| h.current_movement),
            UnitRef::Enemy(id) => self.enemies.get(&id).map(|e| e.current_movement),
        }
    }

    pub fn unit_current_movement_mut(&mut self, u: UnitRef) -> Option<&mut i32> {
        match u {
            UnitRef::Hero(id) => self.heroes.get_mut(&id).map(|h| &mut h.current_movement),
            UnitRef::Enemy(id) => self.enemies.get_mut(&id).map(|e| &mut e.current_movement),
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

    pub fn unit_movement(&self, u: UnitRef) -> Option<i32> {
        match u {
            UnitRef::Hero(id) => self.heroes.get(&id).map(|h| h.stats.movement),
            UnitRef::Enemy(id) => self.enemies.get(&id).map(|e| e.stats.movement),
        }
    }

    pub fn unit_action_available(&self, u: UnitRef) -> Option<bool> {
        match u {
            UnitRef::Hero(id) => self.heroes.get(&id).map(|h| h.action_available),
            UnitRef::Enemy(id) => Some(false),
        }
    }

    pub fn unit_abilities(&self, u: UnitRef) -> Option<&[Ability]> {
        match u {
            UnitRef::Hero(id) => self.heroes.get(&id).map(|h| h.abilities.as_slice()),
            UnitRef::Enemy(id) => None,
        }
    }

    pub fn selected_unit_hex(&self) -> Option<Hex> {
        match self.selected_unit {
            Some(unit) => self.unit_hex(unit),
            None => None,
        }
    }

    pub fn active_unit_hex(&self) -> Option<Hex> {
        let unit = self.active_unit();
        self.unit_hex(unit)
    }

    fn unit_stats(&self, unit: UnitRef) -> Option<&Stats> {
        match unit {
            UnitRef::Hero(id) => self.heroes.get(&id).map(|h| &h.stats),
            UnitRef::Enemy(id) => self.enemies.get(&id).map(|e| &e.stats),
        }
    }

    fn unit_stats_mut(&mut self, unit: UnitRef) -> Option<&mut Stats> {
        match unit {
            UnitRef::Hero(id) => self.heroes.get_mut(&id).map(|h| &mut h.stats),
            UnitRef::Enemy(id) => self.enemies.get_mut(&id).map(|e| &mut e.stats),
        }
    }
}

// --------------------- Unit manipulation ---------------------
impl BattleState {
    pub fn move_unit(&mut self, unit: UnitRef, target: Hex, cost: i32) {
        match unit {
            UnitRef::Hero(id) => {
                let hero = self.heroes.get_mut(&id).unwrap();
                hero.current_movement -= cost;
                hero.hex = target;
            }
            UnitRef::Enemy(id) => {
                let enemy = self.enemies.get_mut(&id).unwrap();
                enemy.current_movement -= cost;
                enemy.hex = target;
            }
        }

        self.update_occupied_hexes();
        self.update_selected_unit_range();
    }

    pub fn attack_unit(&mut self, attacker: UnitRef, target: UnitRef, damage_multiplier: f32) {
        let attacker_stats = match self.unit_stats(attacker) {
            Some(s) => s.clone(),
            None => return,
        };

        let target_stats = match self.unit_stats_mut(target) {
            Some(s) => s,
            None => return,
        };

        let attack_vs_defense = attacker_stats.attack - target_stats.defense;

        let attack_modifier = if attack_vs_defense >= 0 {
            1.0 + ((attack_vs_defense as f32) * 0.05).min(2.0)
        } else {
            (1.0 + (attack_vs_defense as f32) * 0.05).max(0.3)
        };

        let mut rng = thread_rng();
        let base_damage = rng.gen_range(attacker_stats.damage.0..=attacker_stats.damage.1) as f32;

        let damage = (base_damage * attack_modifier * damage_multiplier)
            .round()
            .max(0.0) as i32;

        target_stats.hp = (target_stats.hp - damage).max(0);

        // println!(
        //     "{} attacks {} for {} damage!",
        //     self.unit_name(attacker).unwrap(),
        //     self.unit_name(target).unwrap(),
        //     damage
        // );

        if target_stats.hp == 0 {
            self.kill_unit(target);
        }
    }

    fn kill_unit(&mut self, unit: UnitRef) {
        println!("{} has been defeated!", self.unit_name(unit).unwrap());

        if self.active_unit() == unit {
            self.active_unit_idx -= 1;
        }

        match unit {
            UnitRef::Hero(id) => {
                self.heroes.remove(&id);
            }
            UnitRef::Enemy(id) => {
                self.enemies.remove(&id);
            }
        }

        if self.selected_unit == Some(unit) {
            self.selected_unit = None;
            self.selected_unit_range.clear();
        }

        self.turn_order.retain(|u| *u != unit);

        self.update_occupied_hexes();
        self.update_selected_unit_range();
    }
}

// --------------------- Hex map management ---------------------
impl BattleState {
    pub fn initialize_hex_map(&mut self) {
        self.hex_map.clear();
        for q in 0..self.grid_width {
            for r in 0..self.grid_height {
                let terrain = if (q + r) % 5 == 0 {
                    Terrain::Forest
                } else {
                    Terrain::Plain
                }; //Placeholder
                self.hex_map.insert(Hex { q, r }, HexInfo::new(terrain));
            }
        }
        let mountain_hexes = Vec::from([
            Hex { q: 3, r: 4 },
            Hex { q: 4, r: 4 },
            Hex { q: 5, r: 2 },
            Hex { q: 8, r: 2 },
            Hex { q: 9, r: 4 },
        ]);

        for mountain_hex in mountain_hexes {
            self.hex_map
                .insert(mountain_hex, HexInfo::new(Terrain::Mountain));
        }
    }

    pub fn update_occupied_hexes(&mut self) {
        for info in self.hex_map.values_mut() {
            info.occupying_unit = None;
        }

        for (id, hero) in &self.heroes {
            if let Some(info) = self.hex_map.get_mut(&hero.hex) {
                info.occupying_unit = Some(UnitRef::Hero(*id));
            } else {
                eprintln!("Warning: Hero {:?} on invalid hex {:?}", id, hero.hex);
            }
        }

        for (id, enemy) in &self.enemies {
            if let Some(info) = self.hex_map.get_mut(&enemy.hex) {
                info.occupying_unit = Some(UnitRef::Enemy(*id));
            } else {
                eprintln!("Warning: Enemy {:?} on invalid hex {:?}", id, enemy.hex);
            }
        }
    }

    pub fn is_hex_passable(&self, hex: Hex) -> bool {
        self.hex_map
            .get(&hex)
            .map_or(false, |info| info.passable && !info.is_occupied())
    }

    pub fn units_in_area(&self, unit: UnitRef, area: Vec<Hex>) -> (Vec<UnitRef>, Vec<UnitRef>) {
        let in_range = |hex: &Hex| area.iter().any(|h| h == hex);

        let heroes: Vec<UnitRef> = self
            .heroes
            .iter()
            .filter(|(i, hero)| in_range(&hero.hex))
            .map(|(i, _)| UnitRef::Hero(*i))
            .collect();

        let enemies: Vec<UnitRef> = self
            .enemies
            .iter()
            .filter(|(i, enemy)| in_range(&enemy.hex))
            .map(|(i, _)| UnitRef::Enemy(*i))
            .collect();

        match unit {
            UnitRef::Hero(_) => (heroes, enemies),
            UnitRef::Enemy(_) => (enemies, heroes),
        }
    }

    pub fn is_unit_in_area(&self, unit: UnitRef, area: &[Hex]) -> bool {
        let unit_hex = self.unit_hex(unit).unwrap();
        area.iter().any(|&h| h == unit_hex)
    }
}

// --------------------- Turn order management ---------------------
impl BattleState {
    pub fn generate_turn_order(&mut self) {
        let mut units: Vec<(i32, UnitRef)> = self
            .heroes
            .iter()
            .map(|(&id, h)| (h.stats.initiative, UnitRef::Hero(id)))
            .chain(
                self.enemies
                    .iter()
                    .map(|(&id, e)| (e.stats.initiative, UnitRef::Enemy(id))),
            )
            .collect();

        units.sort_by_key(|&(dex, _)| std::cmp::Reverse(dex));

        self.turn_order = units.into_iter().map(|(_, u)| u).collect();
        self.active_unit_idx = 0;
    }

    pub fn active_unit(&self) -> UnitRef {
        if self.turn_order.len() <= self.active_unit_idx {
            return self.turn_order[0]; //Temporary bugfix
        }
        self.turn_order[self.active_unit_idx]
    }

    pub fn next_unit(&mut self) {
        self.active_unit_idx = (self.active_unit_idx + 1) % self.turn_order.len();
    }

    pub fn is_player_turn(&self) -> bool {
        match self.active_unit() {
            UnitRef::Hero(_) => true,
            UnitRef::Enemy(_) => false,
        }
    }
}

// --------------------- Tick and Phase management ---------------------

impl BattleState {
    pub fn tick(&mut self) {
        match self.phase {
            TurnPhase::Start => self.start_phase(),
            TurnPhase::Action => self.action_phase(),
            TurnPhase::End => self.end_phase(),
        }
    }

    fn start_phase(&mut self) {
        let unit = self.active_unit();
        let movement = self.unit_movement(unit).unwrap();

        match unit {
            UnitRef::Hero(id) => {
                if let Some(hero) = self.heroes.get_mut(&id) {
                    hero.current_movement = movement;
                    hero.action_available = true;

                    self.select_unit(unit);

                    self.input_mode = InputMode::Movement;
                } else {
                    println!("Hero ID {} not found!", id);
                }
            }
            UnitRef::Enemy(id) => {
                if let Some(enemy) = self.enemies.get_mut(&id) {
                    enemy.current_movement = movement;
                } else {
                    println!("Enemy ID {} not found!", id);
                }
            }
        }

        self.phase = TurnPhase::Action;
    }

    fn action_phase(&mut self) {
        match self.active_unit() {
            UnitRef::Hero(_) => {
                // Needs player input to proceed
            }

            UnitRef::Enemy(i) => {
                print!("ACTIVE ENEMY {}", self.active_unit_idx);

                enemy_ai(self, UnitRef::Enemy(i));
                self.phase = TurnPhase::End;
            }
        }
    }

    fn end_phase(&mut self) {
        if self.enemies.is_empty() {
            self.result = Some(BattleResult::Victory);
        } else if self.heroes.is_empty() {
            self.result = Some(BattleResult::Defeat);
        }

        self.selected_unit = None;
        self.selected_unit_range.clear();

        self.next_unit();
        self.phase = TurnPhase::Start;
    }
}

// --------------------- Send to UI ---------------------

impl BattleState {
    pub fn units_for_render(&self) -> Vec<&dyn UnitRender> {
        self.heroes
            .values()
            .map(|h| h as &dyn UnitRender)
            .chain(self.enemies.values().map(|e| e as &dyn UnitRender))
            .collect()
    }
}

impl BattleState {
    pub fn new(heroes: &[Hero], enemies: &Vec<Enemy>, assets: Assets) -> Self {
        let heroes = heroes
            .iter()
            .enumerate()
            .map(|(i, h)| {
                (
                    i as u32,
                    HeroInstance::from_hero(
                        h,
                        Hex {
                            q: 2 + i as i32,
                            r: 3,
                        },
                        assets.hero.clone(),
                    ),
                )
            })
            .collect();

        let enemies = enemies
            .into_iter()
            .enumerate()
            .map(|(i, e)| {
                (
                    i as u32,
                    EnemyInstance::from_enemy(
                        e,
                        Hex {
                            q: 7 + i as i32,
                            r: 5,
                        },
                        assets.enemy[&e.name].clone(),
                    ),
                )
            })
            .collect();

        let mut battle = Self {
            heroes,
            enemies,
            turn_order: Vec::new(),
            active_unit_idx: 0,
            phase: TurnPhase::Start,
            input_mode: InputMode::Normal,
            selected_unit: None,
            selected_unit_range: HashMap::new(),
            selected_ability: None,
            selected_ability_range: Vec::new(),
            grid_width: 10,
            grid_height: 10,
            hex_map: HashMap::new(),
            result: None,
        };

        battle.initialize_hex_map();
        battle.update_occupied_hexes();
        battle.generate_turn_order();

        battle
    }
}

impl BattleState {
    pub fn handle_command(&mut self, cmd: BattleCommand) {
        match cmd {
            BattleCommand::SelectUnit(unit) => {
                self.select_unit(unit);
            }

            BattleCommand::SelectUnitAtHex(hex) => {
                self.try_select_unit_at_hex(hex);
            }

            BattleCommand::MoveSelectedUnit(hex) => {
                self.try_move_selected(hex);
            }

            BattleCommand::SelectAbility(ability_idx) => {
                self.select_ability(ability_idx);
            }

            BattleCommand::UseAbility(target) => {
                self.try_use_ability(target);
            }

            BattleCommand::CancelAction => {
                self.clear_ability_selection();
            }

            BattleCommand::RequestEndTurn => {
                self.request_end_turn();
            }
        }
    }

    fn select_unit(&mut self, unit: UnitRef) {
        self.selected_unit = Some(unit);
        self.update_selected_unit_range();
    }

    fn update_selected_unit_range(&mut self) {
        if let Some(unit) = self.selected_unit {
            let hex = self.unit_hex(unit).unwrap();
            let movement = self.unit_current_movement(unit).unwrap();
            self.selected_unit_range = movement_range(hex, movement, self);
        }
    }

    pub fn try_select_unit_at_hex(&mut self, hex: Hex) {
        match self.hex_map[&hex].occupying_unit {
            Some(unit) => {
                self.select_unit(unit);
                self.input_mode = InputMode::Movement;
            }
            None => {}
        }
    }

    fn try_move_selected(&mut self, target_hex: Hex) {
        let active_unit = self.active_unit();

        if self.selected_unit != Some(active_unit) {
            return;
        }

        let starting_hex = self.unit_hex(active_unit).unwrap();
        let active_unit_movement = self.unit_current_movement(active_unit).unwrap();
        let reachable: HashMap<Hex, (i32, Vec<Hex>)> =
            movement_range(starting_hex, active_unit_movement, self);

        if let Some((cost, _path)) = reachable.get(&target_hex) {
            self.move_unit(active_unit, target_hex, *cost);
        }
    }

    fn select_ability(&mut self, ability_idx: usize) {
        self.selected_ability = Some(ability_idx);
        let caster = self.selected_unit.unwrap();
        let ability = &self.unit_abilities(caster).unwrap()[ability_idx];
        let caster_hex = self.unit_hex(caster).unwrap();

        self.selected_ability_range =
            caster_hex.range_to_area(ability.range, self.grid_width, self.grid_height);
        self.input_mode = InputMode::AbilityTarget(ability_idx);
    }

    fn try_use_ability(&mut self, target: Hex) {
        let occupying_unit = self.hex_map[&target].occupying_unit;
        let caster_ref = if let UnitRef::Hero(caster_ref) = self.active_unit() {
            caster_ref
        } else {
            eprintln!("WARNING: ABILITY USED DURING ENEMY TURN");
            return;
        };

        if !self.selected_ability_range.contains(&target) {
            println!("TARGET TOO FAR");
            return;
        }

        if let Some(target_unit) = occupying_unit {
            if let UnitRef::Hero(_) = target_unit {
                // Later dependent on ability
                println!("Cannot use ability on an ally");
                return;
            }

            let ability_index = self.selected_ability.unwrap();
            let damage_modifier;
            let effect;

            {
                let caster = self.heroes.get(&caster_ref).unwrap();
                let ability = &caster.abilities[ability_index];
                damage_modifier = ability.damage_modifier;
                effect = ability.effect.clone();
            }

            self.attack_unit(self.active_unit(), target_unit, damage_modifier);

            if let Some(e) = effect {
                if let Some(targetted_enemy_instance) = self.unit_mut(target_unit) {
                    targetted_enemy_instance.effects_mut().push(e);
                } else {
                    println!("Target died, effect skipped");
                }
            }

            if let Some(caster_mut) = self.heroes.get_mut(&caster_ref) {
                caster_mut.action_available = false;
            } else {
                println!("Caster died, cannot mark action as used");
            }

            self.clear_ability_selection();
        }
    }

    fn clear_ability_selection(&mut self) {
        self.selected_ability = None;
        self.selected_ability_range.clear();
        self.input_mode = InputMode::Movement;
    }

    fn request_end_turn(&mut self) {
        if self.phase == TurnPhase::Action && self.is_player_turn() {
            self.phase = TurnPhase::End;
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
    fn hex(&self) -> Hex {
        self.hex
    }
    fn stats(&self) -> &Stats {
        &self.stats
    }
    fn stats_mut(&mut self) -> &mut Stats {
        &mut self.stats
    }
    fn effects(&self) -> &Vec<Effect> {
        &self.effects
    }
    fn effects_mut(&mut self) -> &mut Vec<Effect> {
        &mut self.effects
    }
}

impl Unit for EnemyInstance {
    fn hex(&self) -> Hex {
        self.hex
    }
    fn stats(&self) -> &Stats {
        &self.stats
    }
    fn stats_mut(&mut self) -> &mut Stats {
        &mut self.stats
    }
    fn effects(&self) -> &Vec<Effect> {
        &self.effects
    }
    fn effects_mut(&mut self) -> &mut Vec<Effect> {
        &mut self.effects
    }
}
