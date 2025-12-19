use crate::battlefield::Terrain;
use crate::battlestate::{
    BattleCommand, BattleResult, BattleState, InputMode, UnitRef,
};
use crate::button::Button;
use crate::gamestate::{GameState, Screen};
use crate::hexgrid::Hex;
use macroquad::prelude::*;

const HEX_RADIUS: f32 = 40.0;
const UNIT_SCALE: f32 = 0.8;

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
        Screen::Menu => {}
        Screen::Battle => {
            if let Some(battle) = &mut state.battle {
                battle.tick();
                handle_input(battle);

                if let Some(result) = battle.result {
                    match result {
                        BattleResult::Victory => state.current_screen = Screen::Victory,
                        BattleResult::Defeat => state.current_screen = Screen::Defeat,
                    }
                    state.battle = None; // clear the battle state
                }
            }
        }
        Screen::Victory => {}
        Screen::Defeat => {}
    }
}

pub fn handle_input(battle: &mut BattleState) {
    let ui_consumed = hud_handle_input(battle);

    if !ui_consumed {
        battlefield_handle_input(battle);
    }
}

pub fn hud_handle_input(battle: &mut BattleState) -> bool {
    if is_mouse_button_pressed(MouseButton::Left) {
        if end_turn_handle_input(battle) {
            println!("UI end turn consumed click");
            return true;
        }

        if abilities_handle_input(battle) {
            println!("UI end turn consumed click");
            return true;
        }
    }

    false
}

fn end_turn_handle_input(battle: &mut BattleState) -> bool {
    let button = end_turn_button(battle);

    button.draw();

    if button.clicked() {
        battle.handle_command(BattleCommand::RequestEndTurn);
        return true;
    }
    false
}

fn abilities_handle_input(battle: &mut BattleState) -> bool {
    for (i, button) in ability_buttons(battle).into_iter().enumerate() {
        if button.clicked() {
            if let Some(UnitRef::Hero(_)) = battle.selected_unit {
                if i < battle
                    .unit_abilities(battle.selected_unit.unwrap())
                    .unwrap()
                    .len()
                {
                    battle.handle_command(BattleCommand::SelectAbility(i));
                } else {
                    battle.handle_command(BattleCommand::CancelAction);
                }
                return true;
            }
        }
    }
    false
}

pub fn battlefield_handle_input(battle: &mut BattleState) {
    if is_mouse_button_pressed(MouseButton::Left) {
        if let Some(hex) = screen_to_hex(
            mouse_position().0,
            mouse_position().1,
            battle.grid_width,
            battle.grid_height,
        ) {
            match battle.input_mode {
                InputMode::Normal => {
                    battle.handle_command(BattleCommand::SelectUnitAtHex(hex));
                }
                InputMode::Movement => {
                    battle.handle_command(BattleCommand::MoveSelectedUnit(hex));
                }
                InputMode::AbilityTarget { .. } => {
                    battle.handle_command(BattleCommand::UseAbility(hex))
                }
            }
        }
    }

    if is_key_pressed(KeyCode::Enter) {
        battle.handle_command(BattleCommand::RequestEndTurn);
    }
}

async fn draw(state: &mut GameState) {
    clear_background(LIGHTGRAY);

    match state.current_screen {
        Screen::Menu => draw_menu(state).await,
        Screen::Battle => {
            if let Some(battle) = &state.battle {
                draw_battle(battle).await;
            }
        }
        Screen::Victory => draw_victory_screen(state).await,
        Screen::Defeat => draw_defeat_screen(state).await,
    }
}

async fn draw_menu(state: &mut GameState) {
    draw_text("MAIN MENU", 250.0, 100.0, 50.0, BLACK);

    let button = Button {
        rect: Rect::new(250.0, 200.0, 200.0, 60.0),
        label: "Start Game".to_string(),
        color: LIME,
    };

    button.draw();

    if button.clicked() {
        state.start_battle();
        state.current_screen = Screen::Battle;
    };
}

async fn draw_victory_screen(_state: &mut GameState) {
    draw_text("VICTORY!", 250.0, 100.0, 50.0, GREEN);

    let exit_button = Button {
        rect: Rect::new(250.0, 200.0, 200.0, 60.0),
        label: "Exit Game".to_string(),
        color: LIME,
    };

    exit_button.draw();

    if exit_button.clicked() {
        std::process::exit(0);
    }
}

async fn draw_defeat_screen(_state: &mut GameState) {
    draw_text("DEFEAT...", 250.0, 100.0, 50.0, RED);

    let exit_button = Button {
        rect: Rect::new(250.0, 200.0, 200.0, 60.0),
        label: "Exit Game".to_string(),
        color: RED,
    };

    exit_button.draw();

    if exit_button.clicked() {
        std::process::exit(0);
    }
}

async fn draw_battle(battle: &BattleState) {
    draw_hex_grid(battle.grid_width, battle.grid_height);
    draw_terrain(battle);

    draw_active_unit(battle);
    draw_selected_unit(battle);

    draw_preview(battle);

    draw_units(battle);

    draw_hud(battle);
}

fn draw_hex_grid(grid_width: i32, grid_height: i32) {
    (0..grid_width).for_each(|q| {
        (0..grid_height).for_each(|r| {
            let (x, y) = hex_to_screen(Hex { q, r }, grid_width, grid_height);
            draw_poly(x, y, 6, HEX_RADIUS, 0.0, LIGHTGRAY);
            draw_poly_lines(x, y, 6, HEX_RADIUS, 0.0, 1.0, DARKGRAY);
        })
    });
}

fn draw_preview(battle: &BattleState) {
    match battle.input_mode {
        InputMode::Normal => {}
        InputMode::Movement => draw_movement_preview(battle),
        InputMode::AbilityTarget(_) => draw_ability_preview(battle),
    }
}

fn draw_ability_preview(battle: &BattleState) {
    for hex in &battle.selected_ability_range {
        let (x, y) = hex_to_screen(*hex, battle.grid_width, battle.grid_height);
        draw_poly(x, y, 6, HEX_RADIUS, 0.0, Color::new(1.0, 0.0, 0.0, 0.35));
    }
}

fn draw_movement_preview(battle: &BattleState) {
    for hex in battle.selected_unit_range.keys() {
        let (x, y) = hex_to_screen(*hex, battle.grid_width, battle.grid_height);
        draw_poly(x, y, 6, HEX_RADIUS, 0.0, Color::new(0.4, 0.6, 1.0, 0.35));
    }
}

fn draw_selected_unit(battle: &BattleState) {
    if let Some(hex) = battle.selected_unit_hex() {
        let (x, y) = hex_to_screen(hex, battle.grid_width, battle.grid_height);
        draw_poly_lines(x, y, 6, HEX_RADIUS - 2.0, 0.0, 3.0, BLUE);
    }
}

fn draw_active_unit(battle: &BattleState) {
    if let Some(hex) = battle.active_unit_hex() {
        let (x, y) = hex_to_screen(hex, battle.grid_width, battle.grid_height);
        draw_poly_lines(x, y, 6, HEX_RADIUS - 2.0, 0.0, 3.0, GREEN);
    }
}

fn terrain_color(terrain: Terrain) -> Color {
    match terrain {
        Terrain::Plain => Color::new(0.7, 0.9, 0.7, 1.0), // light green
        Terrain::Forest => Color::new(0.2, 0.6, 0.2, 1.0), // dark green
        Terrain::Mountain => Color::new(0.5, 0.5, 0.5, 1.0), // gray
        Terrain::Water => Color::new(0.2, 0.4, 0.8, 1.0), // blue
    }
}

fn draw_terrain(battle: &BattleState) {
    battle.hex_map.iter().for_each(|(hex, info)| {
        let (x, y) = hex_to_screen(*hex, battle.grid_width, battle.grid_height);

        draw_poly(x, y, 6, HEX_RADIUS, 0.0, terrain_color(info.terrain));
        draw_poly_lines(x, y, 6, HEX_RADIUS, 0.0, 1.0, DARKGRAY);

        if !info.passable {
            draw_text("X", x - 5.0, y + 5.0, 20.0, RED);
        }
    });
}

fn draw_units(battle: &BattleState) {
    for unit in battle.units_for_render() {
        let (x, y) = hex_to_screen(unit.hex(), battle.grid_width, battle.grid_height);
        draw_texture_ex(
            unit.texture(),
            x - HEX_RADIUS * UNIT_SCALE,
            y - HEX_RADIUS * UNIT_SCALE,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    HEX_RADIUS * 2.0 * UNIT_SCALE,
                    HEX_RADIUS * 2.0 * UNIT_SCALE,
                )),
                ..Default::default()
            },
        );

        draw_health_bar(
            x - HEX_RADIUS,
            y + HEX_RADIUS * 0.8,
            HEX_RADIUS * 2.0,
            6.0,
            unit.health_percent(),
        );
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

fn draw_hud(battle: &BattleState) {
    draw_text(
        &format!("Turn: {:?}", battle.active_unit()),
        20.0,
        30.0,
        30.0,
        BLACK,
    );
    draw_end_turn_button(battle);
    draw_abilities(battle);
}

fn draw_abilities(battle: &BattleState) {
    for button in ability_buttons(battle) {
        button.draw();
    }
}

fn ability_buttons(battle: &BattleState) -> Vec<Button> {
    let mut buttons = Vec::new();

    if let Some(UnitRef::Hero(hero_id)) = battle.selected_unit {
        let hero = battle.hero(hero_id).unwrap();
        let action_available = hero.action_available;

        let button_width = 150.0;
        let button_height = 40.0;
        let spacing = 10.0;
        let start_x = 50.0;
        let y = screen_height() - 60.0;

        for (i, ability) in hero.abilities.iter().enumerate() {
            let x = start_x + i as f32 * (button_width + spacing);
            buttons.push(Button {
                rect: Rect::new(x, y, button_width, button_height),
                label: ability.name.clone(),
                color: if action_available { GREEN } else { DARKGRAY },
            });
        }

        // Optional cancel button
        if battle.selected_ability.is_some() {
            let cancel_x = start_x + hero.abilities.len() as f32 * (button_width + spacing);
            buttons.push(Button {
                rect: Rect::new(cancel_x, y, 50.0, button_height),
                label: "X".to_string(),
                color: RED,
            });
        }
    }

    buttons
}

fn draw_end_turn_button(battle: &BattleState) {
    end_turn_button(battle).draw();
}

fn end_turn_button(battle: &BattleState) -> Button {
    Button {
        rect: Rect::new(600.0, 20.0, 150.0, 50.0),
        label: "End Turn".to_string(),
        color: GRAY,
    }
}

// Mapping
pub fn hex_to_screen(hex: Hex, grid_width: i32, grid_height: i32) -> (f32, f32) {
    let (battle_width, battle_height) = battlefield_pixel_size(grid_width, grid_height);

    let offset_x = screen_width() / 2.0 - battle_width / 2.0;
    let offset_y = screen_height() / 2.0 - battle_height / 2.0;

    let x = HEX_RADIUS * 3.0 / 2.0 * hex.q as f32 + offset_x + HEX_RADIUS;
    let y = HEX_RADIUS * (3.0_f32.sqrt()) * (hex.r as f32 + 0.5 * (hex.q % 2) as f32)
        + offset_y
        + HEX_RADIUS;

    (x, y)
}

pub fn screen_to_hex(x: f32, y: f32, grid_width: i32, grid_height: i32) -> Option<Hex> {
    let (battle_width, battle_height) = battlefield_pixel_size(grid_width, grid_height);
    let offset_x = screen_width() / 2.0 - battle_width / 2.0;
    let offset_y = screen_height() / 2.0 - battle_height / 2.0;

    let x = x - offset_x - HEX_RADIUS;
    let y = y - offset_y - HEX_RADIUS;

    let q = (2.0 / 3.0 * x / HEX_RADIUS).round() as i32;
    let r = ((y / (HEX_RADIUS * (3.0_f32.sqrt()))) - 0.5 * (q & 1) as f32).round() as i32;
    let hex = Hex { q, r };
    if (0..grid_width).contains(&q) && (0..grid_height).contains(&r) {
        Some(hex)
    } else {
        None
    }
}

fn battlefield_pixel_size(grid_width: i32, grid_height: i32) -> (f32, f32) {
    let width = HEX_RADIUS * 3.0 / 2.0 * (grid_width as f32 - 1.0) + 2.0 * HEX_RADIUS;
    let height = HEX_RADIUS * (3.0_f32.sqrt()) * (grid_height as f32 + 0.5) + 2.0 * HEX_RADIUS;
    (width, height)
}
