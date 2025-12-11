use crate::effect::Effect;
use crate::hexgrid::Hex;
use crate::inventory::Inventory;
use crate::item::Item;
use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum Unit {
    Hero(Hero),
    Enemy(Enemy),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stats {
    pub max_hp: i32,
    pub hp: i32,
    pub damage: (i32, i32),
    pub attack: i32,
    pub defense: i32,
    pub initiative: i32,
    pub movement: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hero {
    pub id: u32,
    pub name: String,

    pub hex: Hex,
    pub stats: Stats,
    pub xp: u64,
    pub abilities: Vec<Ability>,
    pub inventory: Inventory,
    pub effects: Vec<Effect>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Enemy {
    pub id: u32,
    pub name: String,

    pub hex: Hex,
    pub stats: Stats,
    pub effects: Vec<Effect>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ability {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub damage_modifier: f32,
    pub effect: Option<Effect>,
    pub range: i32,
}
