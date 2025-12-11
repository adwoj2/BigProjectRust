use macroquad::prelude::*;
use crate::hexgrid::Hex;
use crate::gamestate::{GameState, Screen};
use crate::pathfinding::{bfs_path, movement_range};
use crate::battlestate::{BattleState, UnitRef, start_battle};

pub struct Assets {
    pub hero: Texture2D,
    pub enemy: Texture2D,
}

impl Assets {
    pub async fn load() -> Self {
        let hero = load_texture("assets/fighter.png").await.unwrap();
        let enemy = load_texture("assets/goblin.png").await.unwrap();
        hero.set_filter(FilterMode::Nearest);   // optional: prevents blurry scaling
        enemy.set_filter(FilterMode::Nearest);
        Self { hero, enemy }
    }
}


pub async fn run(state: &mut GameState) {
    loop {
        clear_background(LIGHTGRAY);

        match state.current_screen {
            Screen::Menu => draw_menu(state).await,
            Screen::Battle => draw_battlefield(state).await,
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}

// ----------------- Menu -----------------
async fn draw_menu(state: &mut GameState) {
    draw_text("MAIN MENU", 200.0, 100.0, 50.0, BLACK);

    let button_rect = Rect::new(250.0, 200.0, 200.0, 60.0);
    draw_rectangle(button_rect.x, button_rect.y, button_rect.w, button_rect.h, BLUE);
    draw_text("Start Game", button_rect.x + 20.0, button_rect.y + 40.0, 30.0, WHITE);

    // Check if mouse clicks inside the rectangle
    if is_mouse_button_pressed(MouseButton::Left) {
        let (mx, my) = mouse_position();
        if button_rect.contains(vec2(mx, my)) {
            start_battle(state);
            state.current_screen = Screen::Battle;
        }
    }
}


async fn draw_battlefield(state: &mut GameState) {
    let hex_radius = 40.0;
    let grid_width = 10;
    let grid_height = 10;
    let grid_boundary = Hex { q: grid_width - 1, r: grid_height - 1 };
    let unit_scale = 0.8;
    let battle = state.battle.as_mut().unwrap();


    for q in 0..grid_width {
        for r in 0..grid_height {
            let (x, y) = hex_to_screen(Hex { q, r }, hex_radius);
            draw_poly(x, y, 6, hex_radius, 0.0, LIGHTGRAY);
            draw_poly_lines(x, y, 6, hex_radius, 0.0, 1.0, DARKGRAY);
        }
    }

    let click_area = hex_radius * 0.8;
    let (mx, my) = mouse_position();



    if let Some(unit) = &battle.selected_unit {
        let hex = battle.unit_hex(unit);
        let (x, y) = hex_to_screen(hex, hex_radius);
        draw_poly_lines(x, y, 6, hex_radius - 1.0, 0.0, 3.0, BLUE);
    }

    if let Some(hover_hex) = screen_to_hex(mx, my, hex_radius, grid_boundary) {
        let (x, y) = hex_to_screen(hover_hex, hex_radius);
        draw_poly(x, y, 6, hex_radius, 0.0, Color::new(1.0, 0.1, 0.2, 0.35)); // blue transparent
        if let Some(path_info) = battle.selected_unit_range.get(&hover_hex) {
            for hex in &path_info.1 {
                let (px, py) = hex_to_screen(*hex, hex_radius);
                draw_poly(px, py, 6, hex_radius, 0.0, Color::new(0.5, 1.0, 0.5, 0.5)); // green transparent
            }
        }
    }
    

    // range
    for (hex, _) in &battle.selected_unit_range {
        let (x, y) = hex_to_screen(*hex, hex_radius);
        draw_poly(x, y, 6, hex_radius, 0.0, Color::new(0.4, 0.6, 1.0, 0.35)); // blue transparent
    }
    
    // units
    for hero in &battle.heroes {
        let texture = &hero.texture;
        let (hx, hy) = hex_to_screen(hero.hex, hex_radius);

        draw_texture_ex(
            texture,
            hx - hex_radius * unit_scale,
            hy - hex_radius * unit_scale,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(hex_radius * 2.0 * unit_scale, hex_radius * 2.0 * unit_scale)),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }

    for enemy in &battle.enemies {
        let texture = &enemy.texture;
        let (ex, ey) = hex_to_screen(enemy.hex, hex_radius);
        draw_texture_ex(
            texture,
            ex - hex_radius * unit_scale,
            ey - hex_radius * unit_scale,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(hex_radius * 2.0 * unit_scale, hex_radius * 2.0 * unit_scale)),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: false,
                pivot: None,
            },
        );
    }

    

    let (mx, my) = mouse_position();

    if is_mouse_button_pressed(MouseButton::Left) {
        for (i, hero) in battle.heroes.iter().enumerate() {
            let (hx, hy) = hex_to_screen(hero.hex, hex_radius);
            let rect = Rect::new(hx - click_area, hy - click_area, 2.0 * click_area, 2.0 * click_area);

            if rect.contains(vec2(mx, my)) {
                let unit = UnitRef::Hero(i);

                battle.selected_unit = Some(unit);

                // Compute movement range
                let movement = hero.current_movement;
                battle.selected_unit_range = movement_range(hero.hex, movement, grid_boundary);

                return; // selection done
            }
        }

        for (i, enemy) in battle.enemies.iter().enumerate() {
            let (ex, ey) = hex_to_screen(enemy.hex, hex_radius);
            let rect = Rect::new(ex - click_area, ey - click_area, 2.0 * click_area, 2.0 * click_area);

            if rect.contains(vec2(mx, my)) {
                let unit = UnitRef::Enemy(i);

                battle.selected_unit = Some(unit);

                let movement = enemy.current_movement;
                battle.selected_unit_range = movement_range(enemy.hex, movement, grid_boundary);
                
                return;
            }
        }

        if let Some(UnitRef::Hero(hero_idx)) = battle.selected_unit {
            if let Some(target_hex) = screen_to_hex(mx, my, hex_radius, grid_boundary) {
                if let Some(path_info) = battle.selected_unit_range.get(&target_hex) {
                    battle.heroes[hero_idx].current_movement -= path_info.0;
                    println!("Hero {} moved to {:?}, which took {}, remaining movement: {}", hero_idx, target_hex, path_info.0, battle.heroes[hero_idx].current_movement);
                    battle.heroes[hero_idx].hex = target_hex;
                    let movement = battle.heroes[hero_idx].current_movement;
                    battle.selected_unit_range = movement_range(target_hex, movement, grid_boundary);
                    
                    return;
                }
            }
        }

    // Clicking empty space clears selection
        battle.selected_unit = None;
        battle.selected_unit_range.clear();
    }
    
    if let Some(selected_unit) = &battle.selected_unit {
        draw_text(&format!("Selected: {}", battle.unit_name(selected_unit)), 50.0, 50.0, 30.0, BLACK);
    }
}

fn hex_to_screen(hex: Hex, radius: f32) -> (f32, f32) {
    let x = radius * 3.0 / 2.0 * hex.q as f32 + 100.0;
    let y = radius * (3.0f32.sqrt()) * (hex.r as f32 + 0.5 * (hex.q % 2) as f32) + 100.0;
    (x, y)
}

pub fn screen_to_hex(x: f32, y: f32, radius: f32, grid_boundary: Hex) -> Option<Hex> {
    let x = x - 100.0;
    let y = y - 100.0;

    let q = (2.0/3.0 * x / radius).round() as i32;
    let r = ((y / (radius * (3.0_f32.sqrt()))) - 0.5 * (q & 1) as f32).round() as i32;

    let hex = Hex { q, r };

    if q >= 0 && q < grid_boundary.q && r >= 0 && r < grid_boundary.r {
        Some(hex)
    } else {
        None
    }
}