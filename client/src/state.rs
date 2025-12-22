use crate::*;

pub const CANVAS_WIDTH: u32 = 256;
pub const CANVAS_HEIGHT: u32 = 144;
pub const DOGE_WIDTH: f32 = 16.0;
pub const DOGE_HEIGHT: f32 = 32.0;
pub const BORK_WIDTH: f32 = 8.0;
pub const BORK_HEIGHT: f32 = 8.0;
pub const ENEMY_WIDTH: f32 = 16.0;
pub const ENEMY_HEIGHT: f32 = 16.0;
pub const POWERUP_WIDTH: f32 = 16.0;
pub const POWERUP_HEIGHT: f32 = 16.0;
#[allow(dead_code)]
pub const BAT_RANGE: f32 = 10.0;
#[allow(dead_code)]
pub const BAT_COOLDOWN: u32 = 60; // total ticks between swings
#[allow(dead_code)]
pub const BAT_ACTIVE_WINDOW: u32 = BAT_COOLDOWN / 2; // ticks during which hits register

pub const GIFT_SPRITE: &str = "gift";
pub const GIFT_SCALE: f32 = 0.125;
pub const SANTA_SPRITE: &str = "santa";
pub const SANTA_SCALE: f32 = 0.5;
pub const SANTA_SIZE: f32 = 128.0;

pub const SNOWBALL_SPRITE: &str = "snowball";
pub const SNOWBALL_SCALE: f32 = 0.09;

#[turbo::serialize]
pub struct Bork {
    pub owner: usize,
    pub x: f32,
    pub y: f32,
    pub vel_x: f32,
}
impl Bork {
    #[allow(dead_code)]
    pub fn new(owner: usize, x: f32, y: f32) -> Self {
        Self {
            owner,
            x: x + (DOGE_WIDTH / 2.),
            y: y - (8. - (DOGE_HEIGHT / 2.)),
            vel_x: 5.0, // Set a constant velocity for the bork
        }
    }

    // Method to update bork's position
    pub fn update(&mut self) {
        self.x += self.vel_x;
    }

    // Method to draw the bork
    pub fn draw(&self) {
        sprite!("bork", x = self.x, y = self.y);
    }
}

#[turbo::serialize]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub vel_x: f32,
    pub hits: u32,
    pub max_hits: u32,
}
impl Enemy {
    // Constructor for an enemy
    pub fn new(vel_x: f32) -> Self {
        let ground_y = CANVAS_HEIGHT as f32 - DOGE_HEIGHT - 8.0;
        let y = ground_y + (DOGE_HEIGHT - ENEMY_HEIGHT);
        Self {
            x: 256.0,
            y,
            vel_x,
            hits: 0,
            max_hits: 1,
        }
    }

    // Method to update enemy's position
    pub fn update(&mut self) {
        self.x += self.vel_x;
    }

    // Method to draw the enemy
    pub fn draw(&self) {
        sprite!(SNOWBALL_SPRITE, x = self.x, y = self.y, scale = SNOWBALL_SCALE);
    }
}

#[turbo::serialize]
#[allow(dead_code)]
pub struct Powerup {
    pub x: f32,
    pub y: f32,
    pub vel_y: f32,
    pub angle: f32,
    pub powerup_type: PowerupType,
}
impl Powerup {
    #[allow(dead_code)]
    pub fn new(x: f32, y: f32, angle: f32, vel_y: f32, powerup_type: PowerupType) -> Self {
        Self {
            x,
            y,
            angle,
            vel_y,
            powerup_type,
        }
    }

    // Method to draw the powerup
    #[allow(dead_code)]
    pub fn draw(&self) {
        match self.powerup_type {
            PowerupType::DoubleJump => {
                sprite!("double_jump", x = self.x, y = self.y);
                circ!(d = 16, color = 0x00ffffff, x = self.x, y = self.y);
                rect!(
                    w = POWERUP_WIDTH,
                    h = POWERUP_HEIGHT,
                    color = 0xff000fff,
                    x = self.x,
                    y = self.y
                );
            }
            PowerupType::SpeedBoost => {
                sprite!("speed_boost", x = self.x, y = self.y);
                circ!(d = 16, color = 0x00ff00ff, x = self.x, y = self.y);
                rect!(
                    w = POWERUP_WIDTH,
                    h = POWERUP_HEIGHT,
                    color = 0xffff00ff,
                    x = self.x,
                    y = self.y
                );
            }
            PowerupType::MultiBork => {
                sprite!("multi_bork", x = self.x, y = self.y);
                circ!(d = 16, color = 0xffff00ff, x = self.x, y = self.y);
                rect!(
                    w = POWERUP_WIDTH,
                    h = POWERUP_HEIGHT,
                    color = 0xff00ffff,
                    x = self.x,
                    y = self.y
                );
            }
            PowerupType::Bat => {
                sprite!(GIFT_SPRITE, x = self.x, y = self.y, scale = GIFT_SCALE);
                // rect!(
                //     w = POWERUP_WIDTH as u32,
                //     h = POWERUP_HEIGHT as u32,
                //     color = 0x0000ffff,
                //     x = self.x as i32,
                //     y = self.y as i32
                // );
            }
        }
    }
}

#[turbo::serialize]
#[derive(PartialEq)]
#[allow(dead_code)]
pub enum PowerupType {
    DoubleJump,
    SpeedBoost,
    MultiBork,
    Bat,
}

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub enum Weapon {
    Basic,
    Triple,
    Rapid,
}

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub enum Character {
    Default,
    Worried,
    Sad,
    Player1,
    Player2,
}

#[turbo::serialize]
pub struct Gift {
    pub x: f32,
    pub y: f32,
    pub vel_y: f32,
}
impl Gift {
    pub fn new(x: f32, y: f32) -> Self {
        let santa_w = SANTA_SIZE * SANTA_SCALE;
        let spawn_x = x + (santa_w / 2.0) - (POWERUP_WIDTH / 2.0);
        let spawn_y = y + santa_w - POWERUP_HEIGHT;
        Self {
            x: spawn_x,
            y: spawn_y,
            vel_y: 0.6 + ((random::u32() % 100) as f32 / 100.0) * 0.8,
        }
    }

    pub fn update(&mut self) {
        self.y += self.vel_y;
        self.vel_y = (self.vel_y + 0.02).min(2.0);
    }

    pub fn draw(&self) {
        sprite!(GIFT_SPRITE, x = self.x, y = self.y, scale = GIFT_SCALE);
    }
}

#[turbo::serialize]
pub struct FallingEnemy {
    pub x: f32,
    pub y: f32,
    pub vel_y: f32,
}
impl FallingEnemy {
    pub fn new() -> Self {
        let max_x = CANVAS_WIDTH as f32 - ENEMY_WIDTH;
        let x = ((random::u32() % 10_000) as f32 / 10_000.0) * max_x;
        Self {
            x,
            y: -ENEMY_HEIGHT,
            vel_y: 0.75 + ((random::u32() % 100) as f32 / 100.0) * 0.9,
        }
    }

    pub fn update(&mut self) {
        self.y += self.vel_y;
        self.vel_y = (self.vel_y + 0.02).min(2.2);
    }

    pub fn draw(&self) {
        sprite!(SNOWBALL_SPRITE, x = self.x, y = self.y, scale = SNOWBALL_SCALE);
    }
}

#[turbo::serialize]
pub struct Santa {
    pub x: f32,
    pub y: f32,
}
impl Santa {
    pub fn new() -> Self {
        Self { x: 0.0, y: 6.0 }
    }

    pub fn update(&mut self, t: u32) {
        let max_x = CANVAS_WIDTH as f32 - (SANTA_SIZE * SANTA_SCALE);
        self.x = ((t as f32) * 0.6) % max_x;
    }

    pub fn draw(&self) {
        sprite!(SANTA_SPRITE, x = self.x, y = self.y, scale = SANTA_SCALE);
    }
}
