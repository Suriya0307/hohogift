// leaderboard.rs
use crate::*;

pub struct LeaderboardSystem;

impl LeaderboardSystem {
    pub fn handle_input(game_state: &mut GameState, restart_pressed: bool) {
        match game_state.screen {
            Screen::Leaderboard => { // Leaderboard
                if restart_pressed {
                    game_state.screen = Screen::Lobby; // Lobby
                }
            }
            _ => {}
        }
    }

    pub fn draw(game_state: &GameState) {
        match game_state.screen {
            Screen::Leaderboard => { // Leaderboard
                clear(0x00ffffff);
                
                // Draw title
                text!("LEADERBOARD", x = 80, y = 20, font = "large", color = 0x000000ff);
                
                // Draw total gifts earned
                text!("TOTAL GIFTS EARNED: {}", game_state.total_gifts_earned; x = 40, y = 40, font = "small", color = 0x000000ff);
                
                // Draw number of games played
                text!("GAMES PLAYED: {}", game_state.leaderboard_entries; x = 20, y = 60, font = "medium", color = 0x000000ff);
                
                // Draw last run stats (simplified - just show current game stats if available)
                if game_state.leaderboard_recorded {
                    text!("LAST RUN", x = 20, y = 80, font = "medium", color = 0x000000ff);
                    text!("Total Gifts: {}", game_state.p1_gifts + game_state.p2_gifts; x = 20, y = 95, font = "small", color = 0x000000ff);
                    text!("P1 Gifts: {}", game_state.p1_gifts; x = 20, y = 105, font = "small", color = 0x000000ff);
                    text!("P2 Gifts: {}", game_state.p2_gifts; x = 20, y = 115, font = "small", color = 0x000000ff);
                    text!("P1 Life: {}", game_state.p1_health; x = 20, y = 125, font = "small", color = 0x000000ff);
                }
                
                text!("PRESS ANY KEY TO RETURN", x = 50, y = 140, font = "small", color = 0x000000ff);
            }
            _ => {}
        }
    }

    pub fn record_game_end(game_state: &mut GameState, round_t: u32) {
        // Record leaderboard entry
        if !game_state.leaderboard_recorded {
            let total_gifts = game_state.p1_gifts + game_state.p2_gifts;
            game_state.total_gifts_earned += total_gifts;
            game_state.leaderboard_entries += 1;
            game_state.leaderboard_recorded = true;

            // Transition to leaderboard after a delay
            if round_t - game_state.last_game_over > 180 { // 3 seconds delay
                game_state.screen = Screen::Leaderboard; // Leaderboard
            }
        }
    }
}
