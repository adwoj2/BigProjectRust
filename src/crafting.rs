use crate::item::Item;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Blueprint {
    pub id: u32,
    pub result: Item,
    pub cost: HashMap<String, u32>,
}

impl Blueprint {
    pub fn craft(&self, materials: &mut HashMap<String, u32>) -> Option<Item> {
        // sprawdzamy materiały
        for (k, v) in &self.cost {
            if materials.get(k).unwrap_or(&0) < v {
                return None;
            }
        }
        for (k, v) in &self.cost {
            *materials.get_mut(k).unwrap() -= v;
        }
        Some(self.result.clone())
    }
}
