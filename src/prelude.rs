pub use crate::character::*;
pub use crate::item::*;
pub use crate::effect::*;
pub use crate::inventory::*;
pub use crate::battlefield::*;
pub use crate::hexgrid::*;

#[derive(Default)]
pub struct GameState {
    pub battlefield: Battlefield,
    pub player_party: Vec<Hero>,
    pub enemies: Vec<Enemy>,
    pub storage: Storage,
}

impl GameState {
    pub fn new() -> Self {
        Self::default()
    }
}