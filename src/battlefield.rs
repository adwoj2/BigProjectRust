use std::collections::{HashMap, HashSet};
use crate::hexgrid::Hex;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Terrain {
    Plain,
    Forest,
    Mountain,
    Water,
}

// Placeholder for later potential objects
#[derive(Clone, Debug)]
pub enum Object {
    HealingShrine, 
    Trap, 
}

#[derive(Clone, Debug)]
pub struct HexInfo {
    pub terrain: Terrain,
    pub passable: bool,
    pub occupying_unit: Option<UnitRef>,
    pub objects: Vec<Object>,
}

impl HexInfo {
    pub fn new(terrain: Terrain) -> Self {
        let passable = matches!(terrain, Terrain::Plain | Terrain::Forest);
        Self {
            terrain,
            passable,
            occupying_unit: None,
            objects: vec![],
        }
    }

    pub fn is_occupied(&self) -> bool {
        self.occupying_unit.is_some()
    }
}
