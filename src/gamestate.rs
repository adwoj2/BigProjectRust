pub use crate::assets::Assets;
pub use crate::battlestate::BattleState;
pub use crate::character::*;
pub use crate::hexgrid::Hex;
pub use crate::inventory::*;
use std::collections::HashMap;

#[derive(PartialEq, Default)]
pub enum Screen {
    #[default]
    Menu,
    Battle,
    Victory,
    Defeat,
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

impl GameState {
    pub fn start_battle(&mut self) {
        let goblin_stats = Stats {
            max_hp: 30,
            hp: 30,
            damage: (5, 10),
            attack: 12,
            defense: 7,
            initiative: 15,
            movement: 3,
        };
        let orc_stats = Stats {
            max_hp: 60,
            hp: 45,
            damage: (15, 20),
            attack: 15,
            defense: 2,
            initiative: 5,
            movement: 2,
        };

        let enemies: HashMap<i32, Enemy> = HashMap::from([
            (
                0,
                Enemy {
                    id: 0,
                    name: "Goblin".to_string(),
                    hex: Hex { q: 7, r: 5 }, // for testing
                    stats: goblin_stats,
                    effects: Vec::new(),
                },
            ),
            (
                1,
                Enemy {
                    id: 1,
                    name: "Orc".to_string(),
                    hex: Hex { q: 1, r: 6 }, // for testing
                    stats: orc_stats,
                    effects: Vec::new(),
                },
            ),
        ]);

        let heroes = &self.player_party;
        let enemy_vec: &Vec<Enemy> = &enemies.into_values().collect();
        let assets = self.assets.clone();

        self.battle = Some(BattleState::new(heroes, enemy_vec, assets.unwrap()));
    }
}
