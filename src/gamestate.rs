pub use crate::battlestate::{BattleState};
pub use crate::character::*;
pub use crate::effect::*;
pub use crate::inventory::*;
pub use crate::ui::Assets;

#[derive(PartialEq)]
#[derive(Default)]
pub enum Screen {
    #[default]
    Menu,
    Battle,
}

#[derive(Default)]
pub struct GameState {
    pub battle: Option<BattleState>,
    pub player_party: Vec<Hero>,
    pub storage: Storage,
    pub current_screen: Screen,
    pub assets: Option<Assets>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            current_screen: Screen::Menu,
            ..Default::default()
        }
    }
}
