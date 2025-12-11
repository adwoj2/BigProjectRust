use serde::{Serialize, Deserialize};
use macroquad::prelude::*;
use crate::inventory::Inventory;
use crate::item::Item;
use crate::effect::Effect;
use crate::hexgrid::Hex;

#[derive(Debug, Clone)]
pub enum Unit {
    Hero(Hero),
    Enemy(Enemy),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stats {
    pub max_hp: i32,
    pub hp: i32,
    pub strength: i32,
    pub dexterity: i32,
    pub intelligence: i32,
    pub defense: i32,
    pub movement: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hero {
    pub id: u32,
    pub name: String,
    
    // #[serde(skip)]
    // pub texture: Option<Texture2D>,

    pub hex: Hex,
    pub stats: Stats,
    pub xp: u64,
    pub abilities: Vec<Ability>,
    pub inventory: Inventory,
    pub effects: Vec<Effect>,
}

impl Hero {
    pub fn take_damage(&mut self, amount: i32) {
        self.stats.hp = (self.stats.hp - amount).max(0);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Enemy {
    pub id: u32,
    pub name: String,
    
    // #[serde(skip)]
    // pub texture: Option<Texture2D>,

    pub hex: Hex,
    pub stats: Stats,
    pub effects: Vec<Effect>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ability {
    pub id: u32,
    pub name: String,
    pub description: String,
}