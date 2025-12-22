//Player.rs
use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Player {
    pub name: String,
    pub max_hp: usize,
    pub current_hp: usize,
    pub current_level: usize,
    pub max_exp: usize,
    pub current_exp: usize,
}

impl Player {
    pub fn new() -> Self {
        Self {
            name: "Hero".to_string(),
            max_hp: 25,
            current_hp: 25,
            current_level: 1,
            max_exp: 100,
            current_exp: 25,
        }
    }
}