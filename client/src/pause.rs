// pause.rs
use turbo::*;
use crate::*;

pub struct PauseSystem;

impl PauseSystem {
    pub fn handle_input(game_state: &mut GameState) {
        let gp0 = gamepad::get(0);
        let gp1 = gamepad::get(1);
        let kb = keyboard::get();
        
        // Check for pause toggle (ESC or Start button)
        if kb.escape().just_pressed() || gp0.start.just_pressed() || gp1.start.just_pressed() {
            if game_state.screen == Screen::Playing && !game_state.is_paused { // Only pause during playing screen
                game_state.is_paused = true;
            } else if game_state.is_paused {
                // Handle pause menu navigation
                if kb.arrow_up().just_pressed() || gp0.up.just_pressed() {
                    match game_state.pause_selected {
                        0 => game_state.pause_selected = 2, // Resume -> Mute
                        1 => game_state.pause_selected = 0, // Back -> Resume
                        2 => game_state.pause_selected = 1, // Mute -> Back
                        _ => game_state.pause_selected = 0,
                    }
                }
                if kb.arrow_down().just_pressed() || gp0.down.just_pressed() {
                    match game_state.pause_selected {
                        0 => game_state.pause_selected = 1, // Resume -> Back
                        1 => game_state.pause_selected = 2, // Back -> Mute
                        2 => game_state.pause_selected = 0, // Mute -> Resume
                        _ => game_state.pause_selected = 0,
                    }
                }
                
                // Handle pause menu selection
                if kb.enter().just_pressed() || gp0.a.just_pressed() || gp1.a.just_pressed() {
                    match game_state.pause_selected {
                        0 => { // Resume
                            game_state.is_paused = false;
                        }
                        1 => { // Back to Lobby
                            game_state.is_paused = false;
                            game_state.screen = Screen::HoHoHome; // Lobby
                        }
                        2 => { // Mute/Unmute
                            game_state.is_muted = !game_state.is_muted;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn draw(game_state: &GameState) {
        if game_state.is_paused && game_state.screen == Screen::Playing { // Only show during paused gameplay
            // Draw semi-transparent overlay
            rect!(w = 256, h = 144, x = 0, y = 0, color = 0x00000088);
            
            // Draw pause menu
            rect!(w = 200, h = 100, x = 28, y = 22, color = 0xffffffdd);
            text!("PAUSED", x = 100, y = 30, font = "large", color = 0x000000ff);
            
            // Draw menu options
            let resume_color = if game_state.pause_selected == 0 { 0xff0000ff } else { 0x000000ff };
            let back_color = if game_state.pause_selected == 1 { 0xff0000ff } else { 0x000000ff };
            let mute_color = if game_state.pause_selected == 2 { 0xff0000ff } else { 0x000000ff };
            
            text!("RESUME", x = 85, y = 55, font = "medium", color = resume_color);
            text!("BACK TO LOBBY", x = 70, y = 70, font = "medium", color = back_color);
            
            let mute_text = if game_state.is_muted { "UNMUTE" } else { "MUTE" };
            text!(mute_text, x = 90, y = 85, font = "medium", color = mute_color);
            
            text!("USE ARROW KEYS TO NAVIGATE", x = 35, y = 105, font = "small", color = 0x000000ff);
            text!("ENTER TO SELECT", x = 75, y = 115, font = "small", color = 0x000000ff);
        }
    }

    pub fn draw_back_button(game_state: &GameState) {
        if game_state.screen == Screen::Playing && !game_state.is_paused { // Only show during active gameplay
            // Draw back button in top-left corner
            rect!(w = 35, h = 12, x = 2, y = 2, color = 0x000000ff);
            rect!(w = 33, h = 10, x = 3, y = 3, color = 0xffffffdd);
            text!("BACK", x = 8, y = 5, font = "small", color = 0x000000ff);
        }
    }

    pub fn draw_mute_button(game_state: &GameState) {
        if game_state.screen == Screen::Playing { // Show during gameplay (paused or not)
            // Draw mute button in top-right corner
            rect!(w = 35, h = 12, x = 219, y = 2, color = 0x000000ff);
            rect!(w = 33, h = 10, x = 220, y = 3, color = 0xffffffdd);
            
            let mute_text = if game_state.is_muted { "UNMUTE" } else { "MUTE" };
            let font_size = if game_state.is_muted { "small" } else { "small" };
            text!(mute_text, x = 223, y = 5, font = font_size, color = 0x000000ff);
        }
    }

    pub fn handle_back_button_click(game_state: &mut GameState) {
        if game_state.screen == Screen::Playing && !game_state.is_paused {
            let pointer = pointer::screen();
            if pointer.just_pressed() {
                let px = pointer.x as f64;
                let py = pointer.y as f64;
                // Check if back button was clicked (top-left corner)
                if px >= 2.0 && px <= 37.0 && py >= 2.0 && py <= 14.0 {
                    game_state.screen = Screen::HoHoHome; // Back to lobby
                }
            }
        }
    }

    pub fn handle_mute_button_click(game_state: &mut GameState) {
        if game_state.screen == Screen::Playing {
            let pointer = pointer::screen();
            if pointer.just_pressed() {
                let px = pointer.x as f64;
                let py = pointer.y as f64;
                // Check if mute button was clicked (top-right corner)
                if px >= 219.0 && px <= 254.0 && py >= 2.0 && py <= 14.0 {
                    game_state.is_muted = !game_state.is_muted;
                }
            }
        }
    }
}
