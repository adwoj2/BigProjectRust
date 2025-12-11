use crate::battlestate::{start_battle, BattleState, InputMode, TerrainType, UnitRef};
use crate::gamestate::{GameState, Screen};
use crate::hexgrid::Hex;
use crate::pathfinding::movement_range;
use macroquad::prelude::*;
use std::collections::HashMap;

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
        hero.set_filter(FilterMode::Nearest); // optional: prevents blurry scaling
        goblin.set_filter(FilterMode::Nearest);
        orc.set_filter(FilterMode::Nearest);
        rocks.set_filter(FilterMode::Nearest);
        let enemy = HashMap::from([("Goblin".to_string(), goblin), ("Orc".to_string(), orc)]);
        Self { hero, enemy, rocks }
    }
}

const HEX_RADIUS: f32 = 40.0;
const GRID_WIDTH: i32 = 10;
const GRID_HEIGHT: i32 = 10;
const GRID_OFFSET_X: f32 = 100.0;
const GRID_OFFSET_Y: f32 = 100.0;
const UNIT_SCALE: f32 = 0.8;
const CLICK_AREA_FACTOR: f32 = 0.8;

pub async fn run(state: &mut GameState) {
    loop {
        update(state);
        draw(state).await;

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

fn update(state: &mut GameState) {
    match state.current_screen {
        Screen::Menu => {
            // update_menu(state);
        }
        Screen::Battle => {
            update_battle(state);
        }
    }
}

fn update_menu(_state: &mut GameState) {
    todo!(); // for later implementation if needed
}

fn update_battle(state: &mut GameState) {
    if let Some(battle) = &mut state.battle {
        battle.tick();
    }

    handle_battle_input(state);
}

async fn draw(state: &mut GameState) {
    clear_background(LIGHTGRAY);

    match state.current_screen {
        Screen::Menu => draw_menu(state).await,
        Screen::Battle => draw_battlefield(state).await,
    }
}

// ----------------- Menu -----------------
async fn draw_menu(state: &mut GameState) {
    draw_text("MAIN MENU", 200.0, 100.0, 50.0, BLACK);

    let button_rect = Rect::new(250.0, 200.0, 200.0, 60.0);
    draw_rectangle(
        button_rect.x,
        button_rect.y,
        button_rect.w,
        button_rect.h,
        BLUE,
    );
    draw_text(
        "Start Game",
        button_rect.x + 20.0,
        button_rect.y + 40.0,
        30.0,
        WHITE,
    );

    // Check if mouse clicks inside the rectangle
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        if button_rect.contains(vec2(mx, my)) {
            start_battle(state);
            state.current_screen = Screen::Battle;
        }
    }
}

async fn handle_battle_input(state: &mut GameState) {
    // Placeholder for future battle input handling if needed
}

async fn draw_battlefield(state: &mut GameState) {
    let grid_boundary = Hex {
        q: GRID_WIDTH - 1,
        r: GRID_HEIGHT - 1,
    };
    let click_area = HEX_RADIUS * CLICK_AREA_FACTOR;
    let assets = match &state.assets {
        Some(a) => a,
        None => return,
    };
    let battle = match state.battle.as_mut() {
        Some(b) => b,
        None => return,
    };

    draw_hex_grid(GRID_WIDTH, GRID_HEIGHT);

    let (mx, my) = mouse_position();

    // Highlight currently selected unit
    if let Some(unit) = &battle.selected_unit {
        let hex = battle.unit_hex(unit);
        let (x, y) = hex_to_screen(hex);
        draw_poly_lines(x, y, 6, HEX_RADIUS - 1.0, 0.0, 3.0, BLUE);
    }

    // Hover overlay + preview path tiles (pure view, no mutation)
    if let Some(hover_hex) = screen_to_hex(mx, my) {
        draw_hex_overlay(hover_hex, Color::new(1.0, 0.1, 0.2, 0.35));
        if let Some(path_info) = battle.selected_unit_range.get(&hover_hex) {
            path_info.1.iter().for_each(|hex| {
                let (px, py) = hex_to_screen(*hex);
                draw_poly(px, py, 6, HEX_RADIUS, 0.0, Color::new(0.5, 1.0, 0.5, 0.5));
            });
        }
    }

    // Range tiles (selected unit movement)
    battle.selected_unit_range.keys().for_each(|hex| {
        let (x, y) = hex_to_screen(*hex);
        draw_poly(x, y, 6, HEX_RADIUS, 0.0, Color::new(0.4, 0.6, 1.0, 0.35));
    });

    // Draw units
    draw_units(&battle.heroes);
    draw_units(&battle.enemies);
    draw_objects(&battle.terrain, assets);

    if battle.is_player_turn() {
        draw_end_turn_button(battle);
    }

    let hud_clicked = draw_hud(battle);

    // Handle input (selection & movement)
    if !hud_clicked && is_mouse_button_pressed(MouseButton::Left) {
        handle_left_click(battle, mx, my, click_area, grid_boundary);
    }
}

/// Draw the hex grid using iterators
fn draw_hex_grid(width: i32, height: i32) {
    (0..width).for_each(|q| {
        (0..height).for_each(|r| {
            let (x, y) = hex_to_screen(Hex { q, r });
            draw_poly(x, y, 6, HEX_RADIUS, 0.0, LIGHTGRAY);
            draw_poly_lines(x, y, 6, HEX_RADIUS, 0.0, 1.0, DARKGRAY);
        })
    });
}

/// Generic unit drawing for both heroes and enemies
fn draw_units<T>(units: &[T])
where
    T: UnitRender,
{
    units.iter().for_each(|u| {
        let (x, y) = hex_to_screen(u.hex());
        draw_texture_ex(
            u.texture(),
            x - HEX_RADIUS * UNIT_SCALE,
            y - HEX_RADIUS * UNIT_SCALE,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    HEX_RADIUS * 2.0 * UNIT_SCALE,
                    HEX_RADIUS * 2.0 * UNIT_SCALE,
                )),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
        draw_health_bar(
            x - HEX_RADIUS * UNIT_SCALE,
            y + HEX_RADIUS * UNIT_SCALE,
            HEX_RADIUS * 2.0 * UNIT_SCALE,
            6.0,
            u.health_percent(),
        );
    });
}

fn draw_objects(terrain: &HashMap<Hex, TerrainType>, assets: &Assets) {
    let rocks = &assets.rocks;

    terrain.iter().for_each(|(hex, tile)| {
        if *tile == TerrainType::Rocks {
            let (x, y) = hex_to_screen(*hex);
            draw_texture_ex(
                rocks,
                x - HEX_RADIUS * UNIT_SCALE,
                y - HEX_RADIUS * UNIT_SCALE,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        HEX_RADIUS * 2.0 * UNIT_SCALE,
                        HEX_RADIUS * 2.0 * UNIT_SCALE,
                    )),
                    source: None,
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                },
            );
        }
    });
}

fn draw_hud(battle: &mut BattleState) -> bool {
    let mut clicked = false;

    if let Some(UnitRef::Hero(idx)) = battle.selected_unit {
        let hero = &battle.heroes[idx];
        let button_width = 150.0;
        let button_height = 40.0;
        let spacing = 10.0;
        let start_x = 50.0;
        let y = screen_height() - 60.0;
        let ability_color = if hero.action_available {
            GREEN
        } else {
            DARKGRAY
        };

        // Draw ability buttons
        for (i, ability) in hero.abilities.iter().enumerate() {
            let x = start_x + i as f32 * (button_width + spacing);
            let rect = Rect::new(x, y, button_width, button_height);

            draw_rectangle(rect.x, rect.y, rect.w, rect.h, ability_color);
            draw_text(&ability.name, rect.x + 10.0, rect.y + 25.0, 24.0, WHITE);

            if hero.action_available && is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                if rect.contains(vec2(mx, my)) {
                    battle.selected_ability = Some(i);
                    battle.input_mode = InputMode::AbilityTarget {
                        hero_idx: idx,
                        ability_idx: i,
                    };
                    clicked = true;
                }
            }
        }

        if battle.selected_ability.is_some() {
            let cancel_x = start_x + hero.abilities.len() as f32 * (button_width + spacing);
            let cancel_rect = Rect::new(cancel_x, y, 50.0, button_height);

            draw_rectangle(
                cancel_rect.x,
                cancel_rect.y,
                cancel_rect.w,
                cancel_rect.h,
                RED,
            );
            draw_text("X", cancel_rect.x + 18.0, cancel_rect.y + 26.0, 28.0, WHITE);

            if is_mouse_button_pressed(MouseButton::Left) {
                let (mx, my) = mouse_position();
                if cancel_rect.contains(vec2(mx, my)) {
                    battle.selected_ability = None;
                    battle.input_mode = InputMode::Normal;
                    clicked = true;
                }
            }
        }
    }

    // Debug info
    if let Some(selected_unit) = &battle.selected_unit {
        draw_text(
            &format!("Selected: {}", battle.unit_name(selected_unit)),
            50.0,
            50.0,
            30.0,
            BLACK,
        );
    }
    if let Some(selected_ability) = &battle.selected_ability {
        draw_text(
            &format!("Selected Ability: {}", selected_ability),
            50.0,
            80.0,
            30.0,
            RED,
        );
    }

    clicked
}

/// Trait to unify hero/enemy rendering data (no API changes required in other modules)
trait UnitRender {
    fn hex(&self) -> Hex;
    fn texture(&self) -> &Texture2D;
    fn health_percent(&self) -> f32;
}

impl UnitRender for crate::battlestate::HeroInstance {
    fn hex(&self) -> Hex {
        self.hex
    }
    fn texture(&self) -> &Texture2D {
        &self.texture
    }
    fn health_percent(&self) -> f32 {
        self.stats.hp as f32 / self.stats.max_hp as f32
    }
}

impl UnitRender for crate::battlestate::EnemyInstance {
    fn hex(&self) -> Hex {
        self.hex
    }
    fn texture(&self) -> &Texture2D {
        &self.texture
    }
    fn health_percent(&self) -> f32 {
        (self.stats.hp as f32 / self.stats.max_hp as f32).clamp(0.0, 1.0)
    }
}

/// Click handling is expressed mostly with functional helpers (find -> map -> mutate)
fn handle_left_click(
    battle: &mut BattleState,
    mx: f32,
    my: f32,
    click_area: f32,
    grid_boundary: Hex,
) {
    match &battle.input_mode {
        InputMode::Normal => {
            // normal selection logic
            if let Some(_) = check_if_hero_clicked(battle, mx, my, click_area, grid_boundary) {
                battle.input_mode = InputMode::Movement;
            } else if let Some(_) =
                check_if_enemy_clicked(battle, mx, my, click_area, grid_boundary)
            {
                battle.input_mode = InputMode::Normal;
            } else {
                // only clear selection in Normal mode!
                battle.selected_unit = None;
                battle.selected_unit_range.clear();
            }
        }

        InputMode::Movement => {
            // clicking on a hex now means "move here"
            if let Some(target_hex) = screen_to_hex(mx, my) {
                if let Some((cost, path)) = battle.selected_unit_range.get(&target_hex) {
                    // mutate hero: deduct movement and update position
                    if let Some(UnitRef::Hero(hero_idx)) = battle.selected_unit {
                        if let Some(hero_inst) = battle.heroes.get_mut(hero_idx) {
                            hero_inst.current_movement -= *cost;
                            hero_inst.hex = target_hex;

                            // recompute reachable range from new position
                            let remaining = hero_inst.current_movement;
                            battle.update_occupied_hexes();
                            battle.selected_unit_range =
                                movement_range(target_hex, remaining, grid_boundary, battle);
                        }
                    }
                }
            }

            battle.input_mode = InputMode::Normal;
        }

        InputMode::AbilityTarget {
            hero_idx,
            ability_idx,
        } => {
            // clicking on a hex now means "this is my target"
            if let Some(selected_ability_idx) = battle.selected_ability {
                if let Some(target_hex) = screen_to_hex(mx, my) {
                    if let Some(UnitRef::Hero(hero_idx)) = battle.selected_unit {
                        battle.hero_use_ability(hero_idx, selected_ability_idx, target_hex);
                        return;
                    }
                }
            }

            battle.input_mode = InputMode::Movement;
        }
    }
}

fn check_if_hero_clicked(
    battle: &mut BattleState,
    mx: f32,
    my: f32,
    click_area: f32,
    grid_boundary: Hex,
) -> Option<usize> {
    if let Some((i, _)) = battle.heroes.iter().enumerate().find(|(_, hero)| {
        let (hx, hy) = hex_to_screen(hero.hex);
        Rect::new(
            hx - click_area,
            hy - click_area,
            2.0 * click_area,
            2.0 * click_area,
        )
        .contains(vec2(mx, my))
    }) {
        battle.selected_unit = Some(UnitRef::Hero(i));
        let mov = battle.heroes[i].current_movement;
        battle.selected_unit_range =
            movement_range(battle.heroes[i].hex, mov, grid_boundary, battle);
        return Some(i);
    }
    None
}

fn check_if_enemy_clicked(
    battle: &mut BattleState,
    mx: f32,
    my: f32,
    click_area: f32,
    grid_boundary: Hex,
) -> Option<usize> {
    if let Some((i, _)) = battle.enemies.iter().enumerate().find(|(_, enemy)| {
        let (ex, ey) = hex_to_screen(enemy.hex);
        Rect::new(
            ex - click_area,
            ey - click_area,
            2.0 * click_area,
            2.0 * click_area,
        )
        .contains(vec2(mx, my))
    }) {
        battle.selected_unit = Some(UnitRef::Enemy(i));
        let mov = battle.enemies[i].current_movement;
        battle.selected_unit_range =
            movement_range(battle.enemies[i].hex, mov, grid_boundary, battle);
        return Some(i);
    }
    None
}

fn draw_end_turn_button(battle: &mut BattleState) {
    let button_rect = Rect::new(600.0, 20.0, 150.0, 50.0);
    draw_rectangle(
        button_rect.x,
        button_rect.y,
        button_rect.w,
        button_rect.h,
        DARKGRAY,
    );
    draw_text(
        "End Turn",
        button_rect.x + 20.0,
        button_rect.y + 35.0,
        30.0,
        WHITE,
    );

    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        if button_rect.contains(vec2(mx, my)) {
            battle.end_turn();
        }
    }
}

fn draw_health_bar(x: f32, y: f32, width: f32, height: f32, percent: f32) {
    draw_rectangle(x, y, width, height, Color::new(0.1, 0.1, 0.1, 0.8));

    let color = if percent > 0.5 {
        let t = (percent - 0.5) * 2.0;
        Color::new(1.0 - t, 1.0, 0.0, 1.0)
    } else {
        let t = percent * 2.0;
        Color::new(1.0, t, 0.0, 1.0)
    };

    draw_rectangle(x, y, width * percent, height, color);
}

/// Draws a filled hex overlay at `hex`
fn draw_hex_overlay(hex: Hex, color: Color) {
    let (x, y) = hex_to_screen(hex);
    draw_poly(x, y, 6, HEX_RADIUS, 0.0, color);
}

/// Convert axial hex to screen coords (pure)
// TODOOOO
pub fn hex_to_screen(hex: Hex) -> (f32, f32) {
    let (battle_width, battle_height) = battlefield_pixel_size();

    // Center the battlefield on the screen
    let offset_x = screen_width() / 2.0 - battle_width / 2.0;
    let offset_y = screen_height() / 2.0 - battle_height / 2.0;

    let x = HEX_RADIUS * 3.0 / 2.0 * hex.q as f32 + offset_x + HEX_RADIUS;
    let y = HEX_RADIUS * (3.0_f32.sqrt()) * (hex.r as f32 + 0.5 * (hex.q % 2) as f32)
        + offset_y
        + HEX_RADIUS;

    (x, y)
}

/// Convert screen pos to axial hex (pure)
pub fn screen_to_hex(x: f32, y: f32) -> Option<Hex> {
    let (battle_width, battle_height) = battlefield_pixel_size();
    let offset_x = screen_width() / 2.0 - battle_width / 2.0;
    let offset_y = screen_height() / 2.0 - battle_height / 2.0;

    let x = x - offset_x - HEX_RADIUS;
    let y = y - offset_y - HEX_RADIUS;

    let q = (2.0 / 3.0 * x / HEX_RADIUS).round() as i32;
    let r = ((y / (HEX_RADIUS * (3.0_f32.sqrt()))) - 0.5 * (q & 1) as f32).round() as i32;
    let hex = Hex { q, r };
    if q >= 0 && q < GRID_WIDTH && r >= 0 && r < GRID_HEIGHT {
        Some(hex)
    } else {
        None
    }
}

fn battlefield_pixel_size() -> (f32, f32) {
    let width = HEX_RADIUS * 3.0 / 2.0 * (GRID_WIDTH as f32 - 1.0) + 2.0 * HEX_RADIUS;
    let height = HEX_RADIUS * (3.0_f32.sqrt()) * (GRID_HEIGHT as f32 + 0.5) + 2.0 * HEX_RADIUS;
    (width, height)
}
