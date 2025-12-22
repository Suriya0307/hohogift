// store.rs
use crate::*;

pub struct StoreSystem;

impl StoreSystem {
    pub fn draw_store_overlay(game_state: &GameState) {
        // Draw store overlay
        rect!(w = 240, h = 120, x = 8, y = 12, color = 0xffffffdd);
        text!("CHARACTER STORE", x = 75, y = 18, font = "medium", color = 0x000000ff);
        text!("YOUR GIFTS: {}", game_state.total_gifts_earned; x = 20, y = 30, font = "small", color = 0x000000ff);
        
        // Draw character options using bit flags
        let characters = [
            ("WORRIED", 0, 1000),
            ("SAD", 1, 2000), 
            ("TRIPLE SHOT", 2, 3000),
            ("RAPID FIRE", 3, 4000),
        ];
        
        for (i, (name, idx, cost)) in characters.iter().enumerate() {
            let y = 45 + (i as i32 * 12);
            let is_unlocked = (game_state.unlocked_characters >> idx) & 1 == 1;
            
            let status = if is_unlocked { "OWNED" } else { &format!("{} GIFTS", cost) };
            let color = if is_unlocked { 0x00cc00ff } else { 
                if game_state.total_gifts_earned >= *cost { 0x000000ff } else { 0x888888ff }
            };
            
            text!("{}: {}", name, status; x = 20, y = y, font = "small", color = color);
        }
        
        text!("PRESS ANY KEY TO RETURN", x = 60, y = 110, font = "small", color = 0x000000ff);
    }

    #[allow(dead_code)]
    pub fn handle_character_purchase(game_state: &mut GameState, character_index: usize, cost: u32) -> bool {
        // Check if player has enough gifts
        if game_state.total_gifts_earned >= cost {
            // Check if already unlocked
            let is_unlocked = (game_state.unlocked_characters >> character_index) & 1 == 1;
            if !is_unlocked {
                // Unlock the character
                game_state.unlocked_characters |= 1 << character_index;
                game_state.total_gifts_earned -= cost;
                return true;
            }
        }
        false
    }

    #[allow(dead_code)]
    pub fn is_character_unlocked(game_state: &GameState, character_index: usize) -> bool {
        (game_state.unlocked_characters >> character_index) & 1 == 1
    }

    #[allow(dead_code)]
    pub fn get_unlocked_characters(game_state: &GameState) -> Vec<usize> {
        let mut unlocked = Vec::new();
        for i in 0..4 {
            if Self::is_character_unlocked(game_state, i) {
                unlocked.push(i);
            }
        }
        unlocked
    }

    #[allow(dead_code)]
    pub fn get_character_cost(character_index: usize) -> u32 {
        match character_index {
            0 => 1000, // Worried
            1 => 2000, // Sad
            2 => 3000, // Triple Shot
            3 => 4000, // Rapid Fire
            _ => 0,
        }
    }

    #[allow(dead_code)]
    pub fn get_character_name(character_index: usize) -> &'static str {
        match character_index {
            0 => "WORRIED",
            1 => "SAD",
            2 => "TRIPLE SHOT",
            3 => "RAPID FIRE",
            _ => "UNKNOWN",
        }
    }
}
