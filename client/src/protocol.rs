use turbo::*;

#[turbo::serialize]
pub struct PlayerInput {
    pub session_id: u64,
    pub x: f32,
    pub y: f32,
    pub gifts: u32,
    pub health: u8,
}

#[turbo::serialize]
pub struct WorldState {
    pub players: Vec<PlayerSnapshot>,
}

#[turbo::serialize]
pub struct PlayerSnapshot {
    pub id: String,
    pub session_id: u64,
    pub x: f32,
    pub y: f32,
    pub gifts: u32,
    pub health: u8,
}

#[turbo::serialize]
pub enum ClientMessage {
    Join { room: String },
    Input {
        session_id: u64,
        x: f32,
        y: f32,
        gifts: u32,
        health: u8,
    },
}

#[turbo::serialize]
pub struct PlayerState {
    pub id: String,
    pub session_id: u64,
    pub x: f32,
    pub y: f32,
    pub gifts: u32,
    pub health: u8,
}
