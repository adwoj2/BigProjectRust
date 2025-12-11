use crate::effect::Effect;
use serde::{Deserialize, Serialize};

// Unused yet
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemSlot {
    Head,
    Body,
    Boots,
    MainHand,
    OffHand,
    Accessory,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub slot: Option<ItemSlot>,
    pub stat_changes: Vec<(String, i32)>,
    pub effects: Vec<Effect>,
    // pub salvaged_materials: Vec<Material>,
}

impl Item {
    pub fn decompose(&self) -> Vec<(String, u32)> {
        vec![("iron_ore".to_string(), 2)]
    }
}
