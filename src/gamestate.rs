pub use crate::character::*;
pub use crate::item::*;
pub use crate::effect::*;
pub use crate::inventory::*;
pub use crate::battlefield::*;
pub use crate::hexgrid::*;
pub use crate::ui::Assets;
pub use crate::battlestate::{BattleState, UnitRef};
pub use std::collections::{HashSet};


#[derive(PartialEq)]
pub enum Screen {
    Menu,
    Battle,
}


impl Default for Screen {
    fn default() -> Self {
        Screen::Menu
    }
}

#[derive(Default)]
pub struct GameState {
    pub battle: Option<BattleState>,
    pub player_party: Vec<Hero>,
    pub storage: Storage,
    pub current_screen: Screen,
    pub assets: Option<Assets>,
}

// impl GameState {
//     pub fn get_unit(&self, uref: UnitRef) -> &impl UnitTrait {
//         match uref {
//             UnitRef::Hero(i) => &self.heroes[i],
//             UnitRef::Enemy(i) => &self.enemies[i],
//         }
//     }

//     pub fn get_unit_mut(&mut self, uref: UnitRef) -> &impl UnitTrait {
//         match uref {
//             UnitRef::Hero(i) => UnitViewMut::Hero(&mut self.heroes[i]),
//             UnitRef::Enemy(i) => UnitViewMut::Enemy(&mut self.enemies[i]),
//         }
//     }
// }



impl GameState {
    pub fn new() -> Self {
        Self {
            current_screen: Screen::Menu,
            ..Default::default()
        }
    }
}