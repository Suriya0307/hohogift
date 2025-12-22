use crate::protocol::*;
use std::cell::RefCell;
use serde::{Deserialize, Serialize};
use borsh::{BorshSerialize, BorshDeserialize};

thread_local! {
    // Simulated server-side state (LOCAL STUB)
    static SERVER_STATE: RefCell<Vec<PlayerState>> = RefCell::new(Vec::new());
}

#[derive(Debug, Clone, Serialize, Deserialize, BorshSerialize, BorshDeserialize)]
pub struct Network {
    connected: bool,
    room_code: String,
}

impl Network {
    // =============================
    // CREATE NETWORK INSTANCE
    // =============================
    pub fn new() -> Self {
        Self {
            connected: false,
            room_code: String::new(),
        }
    }

    // =============================
    // CREATE ROOM (HOST)
    // =============================
    pub fn create_room(&mut self) -> String {
        // Temporary hardcoded room code
        let code = "XMAS1".to_string();
        self.room_code = code.clone();
        self.connected = true;

        println!("Room created: {}", code);
        code
    }

    // =============================
    // JOIN ROOM (CLIENT)
    // =============================
    pub fn join_room(&mut self, room_code: String) {
        if self.connected {
            return;
        }

        println!("Joining room: {}", room_code);
        self.room_code = room_code;
        self.connected = true;
    }

    // =============================
    // CONNECT (USED INTERNALLY)
    // =============================
    pub fn connect(&mut self, room_code: &str) {
        if self.connected {
            return;
        }

        println!("Connecting to room: {}", room_code);
        self.room_code = room_code.to_string();
        self.connected = true;
    }

    // =============================
    // SEND PLAYER INPUT
    // =============================
    pub fn send_input(&self, x: f32, y: f32, gifts: u32, health: u8) {
        if !self.connected {
            return;
        }

        let msg = ClientMessage::Input {
            session_id: 0,
            x,
            y,
            gifts,
            health,
        };

        // 🔴 TEMP: Just log instead of real network
        println!("Sending input: {:?}", msg);

        // 🔴 TEMP: Simulate another player for testing
        SERVER_STATE.with(|state| {
            let mut players = state.borrow_mut();

            if players.is_empty() {
                players.push(PlayerState {
                    id: "remote".to_string(),
                    session_id: 0,
                    x: x + 24.0,
                    y,
                    gifts,
                    health,
                });
            } else {
                players[0].x = x + 24.0;
                players[0].y = y;
                players[0].gifts = gifts;
                players[0].health = health;
            }
        });
    }

    // =============================
    // RECEIVE REMOTE PLAYERS
    // =============================
    pub fn get_remote_players() -> Vec<PlayerState> {
        SERVER_STATE.with(|state| state.borrow().clone())
    }

    // =============================
    // CONNECTION STATE
    // =============================
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn room_code(&self) -> &str {
        &self.room_code
    }
}
