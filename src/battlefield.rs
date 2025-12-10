use crate::hexgrid::Hex;
use crate::character::{Hero, Enemy};


#[derive(Default, Debug, Clone)]
pub struct Battlefield {
    pub occupied: std::collections::HashMap<Hex, String>, // who occupies that hex
}


impl Battlefield {
    pub fn move_unit(&mut self, from: Hex, to: Hex) -> Result<(), &'static str> {
        if self.occupied.contains_key(&to) { return Err("Target occupied"); }
        if let Some(unit) = self.occupied.remove(&from) { self.occupied.insert(to, unit); Ok(()) } else { Err("No unit at source") }
    }
}