

use crate::item::Item;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inventory {
    pub equipped: HashMap<String, Item>, // prosty mapping slot_name -> item
    pub backpack: Vec<Item>,
}

impl Inventory {
    pub fn new() -> Self { Self { equipped: HashMap::new(), backpack: Vec::new() } }

    pub fn equip(&mut self, slot: &str, item: Item) -> Option<Item> {
        // przekazanie wÅ‚asności itema do ekwipunku
        self.equipped.insert(slot.to_string(), item)
    }

    pub fn unequip(&mut self, slot: &str) -> Option<Item> {
        self.equipped.remove(slot)
    }

    pub fn add_to_backpack(&mut self, item: Item) {
        self.backpack.push(item);
    }

    pub fn remove_from_backpack(&mut self, idx: usize) -> Option<Item> {
        if idx < self.backpack.len() { Some(self.backpack.remove(idx)) } else { None }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Storage {
    pub items: Vec<Item>,
}

impl Storage {
    pub fn pull_item(&mut self, index: usize) -> Option<Item> { if index < self.items.len() { Some(self.items.remove(index)) } else { None } }
    pub fn push_item(&mut self, item: Item) { self.items.push(item); }
}