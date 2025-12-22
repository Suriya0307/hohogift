// lobby.rs
use turbo::*;
use crate::*;

// ===== BUTTON CONSTANTS (MUST BE f32) =====
const BTN_W: f32 = 180.0;
const BTN_H: f32 = 28.0;

const BTN_X: f32 = 38.0;
const BTN_CREATE_Y: f32 = 60.0;
const BTN_JOIN_Y: f32 = 96.0;

pub struct LobbySystem;

impl LobbySystem {
    pub fn handle_input(game_state: &mut GameState) {
        // Only handle input in Lobby screen
        if game_state.screen != Screen::Lobby {
            return;
        }

        let pointer = pointer::screen();

        // ===== MOUSE CLICK =====
        if pointer.just_pressed() {
            let mx: f32 = pointer.x as f32;
            let my: f32 = pointer.y as f32;


            // ===== CREATE ROOM =====
            if mx >= BTN_X
                && mx <= BTN_X + BTN_W
                && my >= BTN_CREATE_Y
                && my <= BTN_CREATE_Y + BTN_H
            {
                println!("CREATE ROOM CLICKED");

                let code = generate_room_code();
                game_state.room_code = code.clone();
                game_state.channel_id = code;

                game_state.is_online = true;
                game_state.is_host = true;
                LobbySystem::start_game(game_state);
                return;
            }

            // ===== JOIN ROOM =====
            if mx >= BTN_X
                && mx <= BTN_X + BTN_W
                && my >= BTN_JOIN_Y
                && my <= BTN_JOIN_Y + BTN_H
            {
                println!("JOIN ROOM CLICKED");

                if game_state.room_code.is_empty() {
                    return;
                }

                game_state.channel_id = game_state.room_code.clone();
                game_state.is_online = true;
                game_state.is_host = false;
                LobbySystem::start_game(game_state);
                return;
            }
        }

        // ===== KEYBOARD FALLBACK (ENTER) =====
        if keyboard::get().enter().just_pressed() {
            if !game_state.room_code.is_empty() {
                game_state.channel_id = game_state.room_code.clone();
                game_state.is_online = true;
                game_state.is_host = false;
                LobbySystem::start_game(game_state);
            }
        }
    }

    pub fn start_game(game_state: &mut GameState) {
        let t = time::tick() as u32;

        game_state.screen = Screen::Playing;
        game_state.round_start_tick = t;

        // ===== ONLINE MULTIPLAYER =====
        game_state.is_online = true;

        // Reset gameplay state
        game_state.p1_health = HEALTH_MAX;
        game_state.p2_health = HEALTH_MAX;
        game_state.p1_gifts = 0;
        game_state.p2_gifts = 0;

        game_state.p1_x = 40.0;
        game_state.p2_x = 180.0;

        game_state.last_game_over = 0;
        game_state.winner = 0;
        game_state.is_ready = false;

        game_state.gifts.clear();
        game_state.falling_enemies.clear();
        game_state.borks.clear();
        game_state.enemies.clear();
    }

    pub fn draw(game_state: &GameState, _restart_pressed: bool) {
        if game_state.screen != Screen::Lobby {
            return;
        }

        clear(0x00ffffff);

        // Title
        text!(
            "ONLINE LOBBY",
            x = 80.0,
            y = 20.0,
            font = "large",
            color = 0x000000ff
        );

        // Room code label
        text!(
            "ROOM CODE:",
            x = 60.0,
            y = 45.0,
            font = "small",
            color = 0x000000ff
        );

        // Room code value
        text!(
            game_state.room_code.as_str(),
            x = 130.0,
            y = 45.0,
            font = "small",
            color = 0xffee88ff
        );

        // CREATE ROOM button
        rect!(
            x = BTN_X,
            y = BTN_CREATE_Y,
            w = BTN_W,
            h = BTN_H,
            color = 0x001144ff
        );
        text!(
            "CREATE ROOM",
            x = BTN_X + 36.0,
            y = BTN_CREATE_Y + 10.0,
            font = "small",
            color = 0xffffffff
        );

        // JOIN ROOM button
        rect!(
            x = BTN_X,
            y = BTN_JOIN_Y,
            w = BTN_W,
            h = BTN_H,
            color = 0x223333ff
        );
        text!(
            "JOIN ROOM",
            x = BTN_X + 48.0,
            y = BTN_JOIN_Y + 10.0,
            font = "small",
            color = 0xffffffff
        );

        // Hint
        text!(
            "Click CREATE or JOIN",
            x = 60.0,
            y = 140.0,
            font = "small",
            color = 0x666666ff
        );
    }
}
