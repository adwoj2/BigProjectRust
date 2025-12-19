use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Assets {
    pub hero: Texture2D,
    pub enemy: HashMap<String, Texture2D>,
    pub rocks: Texture2D,
}

impl Assets {
    pub async fn load() -> Self {
        let hero = load_texture("assets/fighter.png").await.unwrap();
        let goblin = load_texture("assets/goblin.png").await.unwrap();
        let orc = load_texture("assets/orc.png").await.unwrap();
        let rocks = load_texture("assets/rocks.png").await.unwrap();
        hero.set_filter(FilterMode::Nearest);
        goblin.set_filter(FilterMode::Nearest);
        orc.set_filter(FilterMode::Nearest);
        rocks.set_filter(FilterMode::Nearest);
        let enemy = HashMap::from([("Goblin".to_string(), goblin), ("Orc".to_string(), orc)]);
        Self { hero, enemy, rocks }
    }
}
