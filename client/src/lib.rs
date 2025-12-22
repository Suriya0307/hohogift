//lib.rs

use turbo::*;

use serde::{Deserialize, Serialize};
use crate::protocol::*;
use std::collections::HashMap;
use std::sync::Mutex;

use lazy_static::lazy_static;

mod state;
use state::*;

mod lobby;
mod leaderboard;
mod store;
mod pause;
mod network;
mod protocol;

use network::Network;

fn generate_room_code() -> String {
    let tick = turbo::time::tick();

    let uid = turbo::os::client::user_id()
        .unwrap_or_else(|| "GUEST".to_string());

    let short_uid = uid.chars().take(4).collect::<String>();

    format!("XMAS{}{}", short_uid, tick % 1000)
}

lazy_static! {
    static ref ROOMS: Mutex<HashMap<String, HashMap<String, PlayerSnapshot>>> =
        Mutex::new(HashMap::new());
}

#[turbo::os::channel(program = "christmas_maze", name = "game")]
pub struct GameChannel;

impl ChannelHandler for GameChannel {
    type Recv = PlayerInput;
    type Send = WorldState;

    fn new() -> Self {
        Self
    }

    fn on_data(&mut self, user_id: &str, input: PlayerInput) -> Result<(), std::io::Error> {
        let mut rooms = ROOMS.lock().unwrap();

        let room = rooms.entry("default".to_string()).or_insert_with(HashMap::new);

        let player_key = format!("{}#{}", user_id, input.session_id);

        room.insert(
            player_key,
            PlayerSnapshot {
                id: user_id.to_string(),
                session_id: input.session_id,
                x: input.x,
                y: input.y,
                gifts: input.gifts,
                health: input.health,
            },
        );

        let world = WorldState {
            players: room
                .values()
                .map(|p| PlayerSnapshot {
                    id: p.id.clone(),
                    session_id: p.session_id,
                    x: p.x,
                    y: p.y,
                    gifts: p.gifts,
                    health: p.health,
                })
                .collect(),
        };

        Self::broadcast(world)
    }
}


#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    borsh::BorshSerialize,
    borsh::BorshDeserialize,
)]
pub enum Screen {
    HoHoHome,
    Lobby,
    Multiplayer,
    Playing,
    Leaderboard,
    GameOver,
}


const GAME_DURATION_TICKS: u32 = 60 * 120;
const HEALTH_MAX: u32 = 100;
const HEALTH_HIT_DAMAGE: u32 = 5;

const BACKGROUND_AUDIO: &str = "background";
const GIFT_AUDIO: &str = "gift";
const SNOWBALL_AUDIO: &str = "snowball";

const BTN_W: i32 = 200;
const BTN_H: i32 = 32;

const BTN_X: i32 = 28;
const BTN_Y_SINGLE: i32 = 60;
const BTN_Y_ONLINE: i32 = 100;

#[turbo::game]
pub struct GameState {
    is_ready: bool,
    p1_x: f32,
    p2_x: f32,
    player_y: f32,
    round_start_tick: u32,

    last_p1_bork: u32,
    last_p2_bork: u32,
    bork_rate: u32,
    last_enemy_spawn: u32,
    enemy_spawn_rate: u32,
    santa: Santa,
    last_gift_drop: u32,
    gift_drop_rate: u32,
    last_falling_enemy_drop: u32,
    falling_enemy_drop_rate: u32,
    borks: Vec<Bork>,
    enemies: Vec<Enemy>,
    gifts: Vec<Gift>,
    falling_enemies: Vec<FallingEnemy>,
    points: u32,
    currency: u32,
    p1_gifts: u32,
    p2_gifts: u32,

    p1_health: u32,
    p2_health: u32,
    p1_weapon: Weapon,
    p2_weapon: Weapon,
    p1_character: Character,
    p2_character: Character,
    unlocked_triple: bool,
    unlocked_rapid: bool,
    unlocked_worried: bool,
    unlocked_sad: bool,
    shop_open: bool,
    shop_owner: usize,
    shop_index: usize,
    last_game_over: u32,
    winner: u8,
    pub screen: Screen,
    leaderboard_entries: u32, // number of entries
    leaderboard_recorded: bool,
    total_gifts_earned: u32,
    lobby_selected: u8, // 0=Start, 1=Store, 2=Leaderboard, 3=Quit
    store_selected: u8, // 0=Back, 1=Character1, 2=Character2, 3=Character3, 4=Character4
    unlocked_characters: u32, // bit flags for unlocked characters
    selected_p1_character: u8, // 0=Player1, 1=Worried, 2=Sad
    selected_p2_character: u8,
    is_paused: bool,
    is_muted: bool,
    pause_selected: u8, // 0=Resume, 1=Back, 2=Mute

    // ===== ONLINE MULTIPLAYER =====
    network: Network,
    pub channel_id: String,
    pub is_online: bool,
    is_host: bool,
    room_code: String,
    session_id: u64,
    last_screen: Screen,
    audio_unlocked: bool,
    last_audio_tick: u32,
}

#[derive(Serialize, Deserialize)]
struct NetPlayer {
    x: f32,
    y: f32,
    gifts: u32,
    health: u8,
}

impl GameState {
    fn new_with_start(round_start_tick: u32) -> Self {
        let player_y = CANVAS_HEIGHT as f32 - DOGE_HEIGHT - 8.0;
        let session_id = ((random::u32() as u64) << 32) | (random::u32() as u64);
        Self {
            is_ready: false,
            p1_x: 40.0,
            p2_x: 180.0,
            player_y,
            round_start_tick,
            last_p1_bork: 0,
            last_p2_bork: 0,
            bork_rate: 10,
            last_enemy_spawn: 0,
            enemy_spawn_rate: 100,
            santa: Santa::new(),
            last_gift_drop: 0,
            gift_drop_rate: 120,
            last_falling_enemy_drop: 0,
            falling_enemy_drop_rate: 120,
            borks: vec![],
            enemies: vec![],
            gifts: vec![],
            falling_enemies: vec![],
            points: 0,
            currency: 0,
            p1_gifts: 0,
            p2_gifts: 0,

            p1_health: HEALTH_MAX,
            p2_health: HEALTH_MAX,
            p1_weapon: Weapon::Basic,
            p2_weapon: Weapon::Basic,
            p1_character: Character::Player1,
            p2_character: Character::Player2,
            unlocked_triple: false,
            unlocked_rapid: false,
            unlocked_worried: false,
            unlocked_sad: false,
            shop_open: false,
            shop_owner: 0,
            shop_index: 0,
            last_game_over: 0,
            winner: 0,
            screen: Screen::Lobby,
            leaderboard_entries: 0,
            leaderboard_recorded: false,
            total_gifts_earned: 0,
            lobby_selected: 0, // Start
            store_selected: 0, // Back
            unlocked_characters: 0, // No characters unlocked initially
            selected_p1_character: 0, // Player1
            selected_p2_character: 1, // Player2
            is_paused: false,
            is_muted: false,
            pause_selected: 0, // Resume

            // ===== ONLINE MULTIPLAYER =====
            network: Network::new(),
            channel_id: String::new(),
            is_online: false,
            is_host: false,
            room_code: String::new(),
            session_id,
            last_screen: Screen::Lobby,
            audio_unlocked: false,
            last_audio_tick: 0,
        }
    }

    fn new() -> Self {
        Self::new_with_start(time::tick() as u32)
    }

    fn is_host(&self) -> bool {
        self.is_online && self.is_host
    }

    fn apply_world_state(&mut self, world: WorldState) {
    // My Turbo OS user id
    let my_id = turbo::os::client::user_id().unwrap_or_default();

    for player in world.players {
        if player.id == my_id {
            continue;
        }

        self.p2_x = player.x;
        self.p2_gifts = player.gifts;
        self.p2_health = player.health as u32;
    }
}

    fn update(&mut self) {
        let t = time::tick() as u32;
        let round_t = t.saturating_sub(self.round_start_tick);

        let current_screen = self.screen;
        if current_screen != Screen::Playing && self.last_screen == Screen::Playing {
            audio::stop(BACKGROUND_AUDIO);
        }
        if current_screen == Screen::Playing {
            if self.is_muted {
                audio::stop(BACKGROUND_AUDIO);
                audio::stop("background.mp3");
                log!("Stopped background audio due to mute");
            } else {
                // Always ensure background music is playing while game is running
                if !audio::is_playing("background") {
                    audio::set_volume("background", 1.0);
                    audio::play("background");
                    log!("Starting background music; is_playing: {}", audio::is_playing("background"));
                }
            }
        }
        self.last_screen = current_screen;

        let gp0 = gamepad::get(0);
        let gp1 = gamepad::get(1);
        let kb = keyboard::get();
        let p = pointer::screen();

        // Unlock audio on first user interaction (click or key press)
        if !self.audio_unlocked {
            if p.just_pressed() || kb.space().just_pressed() || gp0.a.just_pressed() || gp1.a.just_pressed() {
                // Try to unlock with a silent approach: set volume then play
                audio::set_volume("background", 0.5);
                audio::set_volume("background.mp3", 0.5);
                audio::play("background");
                audio::play("background.mp3");
                self.audio_unlocked = true;
                log!("Audio unlocked via user interaction; set volume to 0.5");
            }
        }

        // ===== HOHOHOME SCREEN INPUT =====
        if self.screen == Screen::HoHoHome {
            let p = pointer::screen();

            if p.just_pressed() {
                let mx = p.x as i32;
                let my = p.y as i32;

                // PLAY button
                if mx >= BTN_X && mx <= BTN_X + BTN_W && my >= BTN_Y_SINGLE && my <= BTN_Y_SINGLE + BTN_H {
                    self.screen = Screen::Lobby;
                    self.is_online = false;
                }

                // ONLINE MULTIPLAYER button
                if mx >= BTN_X
                    && mx <= BTN_X + BTN_W
                    && my >= BTN_Y_ONLINE
                    && my <= BTN_Y_ONLINE + BTN_H
                {
                    self.is_online = true;
                    self.is_host = false;
                    self.room_code.clear();
                    self.channel_id.clear();
                    self.screen = Screen::Lobby;
                    return;
                }
            }
        }

// ================= LOBBY SCREEN =================
if self.screen == Screen::Lobby {
    clear(0x0b1020ff);

    text!("HOHOGIFT", x = 74, y = 18, font = "large", color = 0xffffffff);
    text!("PLAY MODE", x = 86, y = 36, font = "small", color = 0xa8b3d6ff);

    // ----- MODE SELECTION -----
    const W: i32 = 140;
    const H: i32 = 14;
    const LOBBY_X: i32 = ((CANVAS_WIDTH as i32) / 2) - (W / 2);
    const SINGLE_Y: i32 = 58;
    const MULTI_Y: i32 = 78;
    const CREATE_Y: i32 = 104;
    const JOIN_Y: i32 = 124;

    // Single player button (card style)
    rect!(x = LOBBY_X + 1, y = SINGLE_Y + 1, w = W, h = H, color = 0x00000040);
    rect!(x = LOBBY_X, y = SINGLE_Y, w = W, h = H, color = 0x1e3a8aff);
    rect!(x = LOBBY_X, y = SINGLE_Y, w = W, h = 1, color = 0xffffffff);
    text!("SINGLE PLAYER", x = LOBBY_X + 30, y = SINGLE_Y + 4, font = "small", color = 0xffffffff);

    // Multiplayer button (card style)
    rect!(x = LOBBY_X + 1, y = MULTI_Y + 1, w = W, h = H, color = 0x00000040);
    rect!(x = LOBBY_X, y = MULTI_Y, w = W, h = H, color = 0x0f766eff);
    rect!(x = LOBBY_X, y = MULTI_Y, w = W, h = 1, color = 0xffffffff);
    text!("MULTIPLAYER", x = LOBBY_X + 38, y = MULTI_Y + 4, font = "small", color = 0xffffffff);

    // ===== MOUSE INPUT =====
    let p = pointer::screen();
    if p.just_pressed() {
        let mx = p.x as i32;
        let my = p.y as i32;

        // SINGLE PLAYER button
        if mx >= LOBBY_X && mx <= LOBBY_X + W && my >= SINGLE_Y && my <= SINGLE_Y + H {
            self.is_online = false;
            self.screen = Screen::Playing;
            self.round_start_tick = time::tick() as u32;
        }

        // MULTIPLAYER button
        if mx >= LOBBY_X && mx <= LOBBY_X + W && my >= MULTI_Y && my <= MULTI_Y + H {
            self.is_online = true;
            self.is_host = false;
            self.room_code.clear();
            self.channel_id.clear();
            self.screen = Screen::Multiplayer;
        }
    }

// ===== KEYBOARD INPUT =====
let kb = keyboard::get();

// Press ESC → go back
if kb.escape().just_pressed() {
    self.room_code.clear();
    self.channel_id.clear();
    self.is_host = false;
    self.screen = Screen::HoHoHome;
}

return; // VERY IMPORTANT
}

// ================= MULTIPLAYER SCREEN =================
if self.screen == Screen::Multiplayer {
    clear(0x0b1020ff);

    text!("HOHOGIFT", x = 74, y = 18, font = "large", color = 0xffffffff);
    text!("MULTIPLAYER", x = 86, y = 36, font = "small", color = 0xa8b3d6ff);

    // ----- BUTTONS -----
    const W: i32 = 140;
    const H: i32 = 14;
    const LOBBY_X: i32 = ((CANVAS_WIDTH as i32) / 2) - (W / 2);
    const CREATE_Y: i32 = 58;
    const JOIN_Y: i32 = 78;

    // Create button (card style)
    rect!(x = LOBBY_X + 1, y = CREATE_Y + 1, w = W, h = H, color = 0x00000040);
    rect!(x = LOBBY_X, y = CREATE_Y, w = W, h = H, color = 0x1e3a8aff);
    rect!(x = LOBBY_X, y = CREATE_Y, w = W, h = 1, color = 0xffffffff);
    text!("CREATE ROOM", x = LOBBY_X + 34, y = CREATE_Y + 4, font = "small", color = 0xffffffff);

    // Join button (card style)
    rect!(x = LOBBY_X + 1, y = JOIN_Y + 1, w = W, h = H, color = 0x00000040);
    rect!(x = LOBBY_X, y = JOIN_Y, w = W, h = H, color = 0x0f766eff);
    rect!(x = LOBBY_X, y = JOIN_Y, w = W, h = 1, color = 0xffffffff);
    text!("JOIN ROOM", x = LOBBY_X + 46, y = JOIN_Y + 4, font = "small", color = 0xffffffff);

    // Room code
    text!("ROOM CODE", x = 86, y = 104, font = "small", color = 0xa8b3d6ff);
    rect!(x = 68, y = 114, w = 120, h = 16, color = 0x00000080);
    rect!(x = 68, y = 114, w = 120, h = 1, color = 0xffffffff);
    text!(
        self.room_code.as_str(),
        x = 76,
        y = 123,
        font = "large",
        color = 0xffdd55ff
    );
    if self.is_host() {
        text!(
            "PRESS ENTER TO START",
            x = 60,
            y = 138,
            font = "small",
            color = 0xaaaaaaff
        );
    }

    // ===== MOUSE INPUT =====
    let p = pointer::screen();
    if p.just_pressed() {
        let mx = p.x as i32;
        let my = p.y as i32;

        // CREATE ROOM
        if mx >= LOBBY_X && mx <= LOBBY_X + W && my >= CREATE_Y && my <= CREATE_Y + H {
            self.is_host = true;
            self.room_code = generate_room_code();
            self.channel_id = self.room_code.clone();
            log!("ROOM CREATED: {}", self.room_code);
            log!("Joined room: {}", self.channel_id);
        }

        // JOIN ROOM
        if mx >= LOBBY_X && mx <= LOBBY_X + W && my >= JOIN_Y && my <= JOIN_Y + H {
            if !self.room_code.is_empty() {
                self.is_host = false;
                self.channel_id = self.room_code.clone();
                log!("Joined room: {}", self.channel_id);
                self.round_start_tick = time::tick() as u32;
                self.screen = Screen::Playing;
            }
        }
    }
    // ===== KEYBOARD INPUT (ROOM CODE) =====
    let kb = keyboard::get();

    // Read typed characters
    let typed = kb.text();
    if !typed.is_empty() {
        for ch in typed.chars() {
            if ch.is_ascii_alphanumeric() {
                // Shift pressed → lowercase
                if kb.shift_any().pressed() {
                    self.room_code.push(ch.to_ascii_lowercase());
                }
                // Normal typing → uppercase
                else {
                    self.room_code.push(ch.to_ascii_uppercase());
                }
            }
        }
    }

    // Handle BACKSPACE
    if kb.backspace().just_pressed() {
        self.room_code.pop();
    }

    // Press ENTER → start/join room
    if kb.enter().just_pressed() && !self.room_code.is_empty() {
        if !self.is_host {
            self.is_host = false;
        }
        self.is_online = true;
        self.channel_id = self.room_code.clone();
        log!("Joined room: {}", self.channel_id);
        self.round_start_tick = time::tick() as u32;
        self.screen = Screen::Playing;
    }

    // Press ESC → go back to lobby
    if kb.escape().just_pressed() {
        self.room_code.clear();
        self.channel_id.clear();
        self.is_host = false;
        self.is_online = false;
        self.screen = Screen::Lobby;
    }

return; // VERY IMPORTANT
}

        let restart_pressed = gp0.start.just_pressed()
            || gp1.start.just_pressed()
            || pointer::screen().just_pressed()
            || !kb.text().is_empty()
            || kb.escape().just_pressed()
            || kb.tab().just_pressed()
            || kb.backspace().just_pressed()
            || kb.arrow_left().just_pressed()
            || kb.arrow_right().just_pressed()
            || kb.arrow_up().just_pressed()
            || kb.arrow_down().just_pressed();

        let enable_shop = false;
        let enable_shooting = false;
        let enable_obstacles = false;

        // Handle lobby and store input using modular systems
                leaderboard::LeaderboardSystem::handle_input(self, restart_pressed);
        
        // Handle pause system input
        pause::PauseSystem::handle_input(self);
        
        // Handle button clicks for back and mute buttons
        pause::PauseSystem::handle_back_button_click(self);
        pause::PauseSystem::handle_mute_button_click(self);

        if !self.is_ready && t >= self.enemy_spawn_rate && !self.is_paused {
            self.is_ready = true;
            self.last_gift_drop = round_t;
            self.last_falling_enemy_drop = round_t;
        }

        if self.last_game_over == 0 && self.is_ready && round_t >= GAME_DURATION_TICKS && !self.is_paused {
            self.last_game_over = round_t;
            if self.p1_gifts > self.p2_gifts {
                self.winner = 1;
            } else if self.p2_gifts > self.p1_gifts {
                self.winner = 2;
            }

            // Record leaderboard entry using modular system
            leaderboard::LeaderboardSystem::record_game_end(self, round_t);
        }

        if enable_shop && self.last_game_over == 0 && self.is_ready {
            if !self.shop_open {
                if gp0.up.just_pressed() {
                    self.shop_open = true;
                    self.shop_owner = 0;
                } else if gp1.up.just_pressed() {
                    self.shop_open = true;
                    self.shop_owner = 1;
                }
            } else {
                let shop_down = if self.shop_owner == 0 {
                    gp0.down.just_pressed()
                } else {
                    gp1.down.just_pressed()
                };
                let shop_left = if self.shop_owner == 0 {
                    gp0.left.just_pressed()
                } else {
                    gp1.left.just_pressed()
                };
                let shop_right = if self.shop_owner == 0 {
                    gp0.right.just_pressed()
                } else {
                    gp1.right.just_pressed()
                };
                let shop_start = if self.shop_owner == 0 {
                    gp0.start.just_pressed()
                } else {
                    gp1.start.just_pressed()
                };

                if shop_down {
                    self.shop_open = false;
                }
                if shop_left {
                    self.shop_index = self.shop_index.wrapping_sub(1) % 4;
                }
                if shop_right {
                    self.shop_index = (self.shop_index + 1) % 4;
                }

                if shop_start {
                    match self.shop_index {
                        0 => {
                            if self.unlocked_triple {
                                if self.shop_owner == 0 {
                                    self.p1_weapon = Weapon::Triple;
                                } else {
                                    self.p2_weapon = Weapon::Triple;
                                }
                            } else if self.currency >= 25 {
                                self.currency -= 25;
                                self.unlocked_triple = true;
                            }
                        }
                        1 => {
                            if self.unlocked_rapid {
                                if self.shop_owner == 0 {
                                    self.p1_weapon = Weapon::Rapid;
                                } else {
                                    self.p2_weapon = Weapon::Rapid;
                                }
                            } else if self.currency >= 40 {
                                self.currency -= 40;
                                self.unlocked_rapid = true;
                            }
                        }
                        2 => {
                            if self.unlocked_worried {
                                if self.shop_owner == 0 {
                                    self.p1_character = Character::Worried;
                                } else {
                                    self.p2_character = Character::Worried;
                                }
                            } else if self.currency >= 15 {
                                self.currency -= 15;
                                self.unlocked_worried = true;
                            }
                        }
                        _ => {
                            if self.unlocked_sad {
                                if self.shop_owner == 0 {
                                    self.p1_character = Character::Sad;
                                } else {
                                    self.p2_character = Character::Sad;
                                }
                            } else if self.currency >= 20 {
                                self.currency -= 20;
                                self.unlocked_sad = true;
                            }
                        }
                    }
                }
            }
        }

        if self.last_game_over > 0 {
            if restart_pressed {
                *self = Self::new_with_start(t);
                return;
            }
        }

        if self.last_game_over == 0 && self.is_ready && (!self.shop_open || !enable_shop) && !self.is_paused {
            if self.is_online {
                if self.is_host() {
                    // HOST -> Player 1 only
                    if self.p1_health > 0 {
                        if kb.key_a().pressed() {
                            self.p1_x = (self.p1_x - 2.0).max(0.0);
                        }
                        if kb.key_d().pressed() {
                            self.p1_x = (self.p1_x + 2.0).min(CANVAS_WIDTH as f32 - DOGE_WIDTH);
                        }
                    }
                } else {
                    // JOINER -> Player 2 only
                    if self.p2_health > 0 {
                        if kb.arrow_left().pressed() {
                            self.p2_x = (self.p2_x - 2.0).max(0.0);
                        }
                        if kb.arrow_right().pressed() {
                            self.p2_x = (self.p2_x + 2.0).min(CANVAS_WIDTH as f32 - DOGE_WIDTH);
                        }
                    }
                }
            } else {
                // Offline/local controls
                let p1_left = kb.key_a().pressed();
                let p1_right = kb.key_d().pressed();
                let p2_left = kb.arrow_left().pressed();
                let p2_right = kb.arrow_right().pressed();

                if self.p1_health > 0 && (p1_left || (gp0.left.pressed() && !p2_left && !p2_right)) {
                    self.p1_x = (self.p1_x - 2.0).max(0.0);
                }
                if self.p1_health > 0 && (p1_right || (gp0.right.pressed() && !p2_left && !p2_right)) {
                    self.p1_x = (self.p1_x + 2.0).min(CANVAS_WIDTH as f32 - DOGE_WIDTH);
                }
                if self.p2_health > 0 && (gp1.left.pressed() || p2_left) {
                    self.p2_x = (self.p2_x - 2.0).max(0.0);
                }
                if self.p2_health > 0 && (gp1.right.pressed() || p2_right) {
                    self.p2_x = (self.p2_x + 2.0).min(CANVAS_WIDTH as f32 - DOGE_WIDTH);
                }
            }
        }

        self.santa.update(t);
        if self.last_game_over == 0 && self.is_ready && !self.is_paused {
            if round_t - self.last_gift_drop >= self.gift_drop_rate {
                let gift_count = 2 + (random::u32() % 2);
                for _ in 0..gift_count {
                    let jitter = ((random::u32() % 101) as f32 / 100.0 - 0.5) * 24.0;
                    let x = (self.santa.x + jitter).clamp(0.0, CANVAS_WIDTH as f32 - POWERUP_WIDTH);
                    self.gifts.push(Gift::new(x, self.santa.y));
                }
                self.last_gift_drop = round_t;
            }

            if round_t - self.last_falling_enemy_drop >= self.falling_enemy_drop_rate {
                let snowball_count = 2 + (random::u32() % 2);
                for _ in 0..snowball_count {
                    self.falling_enemies.push(FallingEnemy::new());
                }
                self.last_falling_enemy_drop = round_t;
            }
        }

        if enable_shooting {
            // Update borks
            self.borks.retain_mut(|bork| {
                bork.update();
                let mut collided = false;
                for enemy in self.enemies.iter_mut() {
                    if bork.x < enemy.x + ENEMY_WIDTH
                        && bork.x + BORK_WIDTH > enemy.x
                        && bork.y < enemy.y + ENEMY_HEIGHT
                        && bork.y + BORK_HEIGHT > enemy.y
                    {
                        enemy.hits += 1; // Mark the enemy as hit by the bork
                        collided = true;
                    }
                }
                if collided {
                    self.points += 10;
                }
                !collided && bork.x < CANVAS_WIDTH as f32
            });
        } else {
            self.borks.clear();
        }

        if enable_obstacles && !self.is_paused {
            // Spawn and update enemies
            if t - self.last_enemy_spawn >= self.enemy_spawn_rate {
                let vel_x = -1.0 + ((t / 10) as f32 * -0.01).max(-1.);
                let modifier = (random::u32() % 200) as f32 / 100.;
                let vel_x = vel_x * modifier;
                self.enemies.push(Enemy::new(vel_x));
                self.last_enemy_spawn = t;
                if t > 60 * 1 && self.enemy_spawn_rate > 30 {
                    self.enemy_spawn_rate -= 2;
                }
            }
            self.enemies.retain_mut(|enemy| {
                enemy.update();
                if self.p1_health > 0
                    && self.p1_x < enemy.x + ENEMY_WIDTH
                    && self.p1_x + DOGE_WIDTH > enemy.x
                    && self.player_y < enemy.y + ENEMY_HEIGHT
                    && self.player_y + DOGE_HEIGHT > enemy.y
                {
                    self.p1_health = self.p1_health.saturating_sub(HEALTH_HIT_DAMAGE);
                    enemy.hits += 1; // Mark the enemy as hit
                    if self.p1_health == 0 && self.last_game_over == 0 {
                        self.last_game_over = round_t;
                        self.winner = if self.p2_health > 0 { 2 } else { 0 };
                    }
                }

                if self.p2_health > 0
                    && self.p2_x < enemy.x + ENEMY_WIDTH
                    && self.p2_x + DOGE_WIDTH > enemy.x
                    && self.player_y < enemy.y + ENEMY_HEIGHT
                    && self.player_y + DOGE_HEIGHT > enemy.y
                {
                    self.p2_health = self.p2_health.saturating_sub(HEALTH_HIT_DAMAGE);
                    enemy.hits += 1; // Mark the enemy as hit
                    if self.p2_health == 0 && self.last_game_over == 0 {
                        self.last_game_over = round_t;
                        self.winner = if self.p1_health > 0 { 1 } else { 0 };
                    }
                }
                if (self.p1_health == 0 || self.p2_health == 0) && self.last_game_over == 0 {
                    self.last_game_over = round_t;
                    if self.p1_health == 0 && self.p2_health > 0 {
                        self.winner = 2;
                    } else if self.p2_health == 0 && self.p1_health > 0 {
                        self.winner = 1;
                    }
                }

                if enemy.hits >= enemy.max_hits {
                    self.points += 5;
                    return false;
                }
                enemy.x > -ENEMY_WIDTH
            });
        } else {
            self.enemies.clear();
        }

        self.gifts.retain_mut(|gift| {
            gift.update();
            if self.last_game_over == 0 {
                if self.p1_x < gift.x + POWERUP_WIDTH
                    && self.p1_x + DOGE_WIDTH > gift.x
                    && self.player_y < gift.y + POWERUP_HEIGHT
                    && self.player_y + DOGE_HEIGHT > gift.y
                {
                    self.p1_gifts += 1;
                    self.currency += 1;
                    self.points += 1;
                    if !self.is_muted {
                        audio::play("gift");
                        log!("Playing gift audio");
                    }
                    return false;
                }
                if self.p2_x < gift.x + POWERUP_WIDTH
                    && self.p2_x + DOGE_WIDTH > gift.x
                    && self.player_y < gift.y + POWERUP_HEIGHT
                    && self.player_y + DOGE_HEIGHT > gift.y
                {
                    self.p2_gifts += 1;
                    self.currency += 1;
                    self.points += 1;
                    if !self.is_muted {
                        audio::play("gift");
                        log!("Playing gift audio");
                    }
                    return false;
                }
            }
            gift.y < CANVAS_HEIGHT as f32
        });

        self.falling_enemies.retain_mut(|enemy| {
            enemy.update();
            if self.last_game_over == 0 {
                if self.p1_health > 0
                    && self.p1_x < enemy.x + ENEMY_WIDTH
                    && self.p1_x + DOGE_WIDTH > enemy.x
                    && self.player_y < enemy.y + ENEMY_HEIGHT
                    && self.player_y + DOGE_HEIGHT > enemy.y
                {
                    self.p1_health = self.p1_health.saturating_sub(HEALTH_HIT_DAMAGE);
                    if !self.is_muted {
                        audio::play("snowball");
                        log!("Playing snowball audio");
                    }
                    if self.p1_health == 0 {
                        self.last_game_over = round_t;
                        self.winner = if self.p2_health > 0 { 2 } else { 0 };
                    }
                    return false;
                }

                if self.p2_health > 0
                    && self.p2_x < enemy.x + ENEMY_WIDTH
                    && self.p2_x + DOGE_WIDTH > enemy.x
                    && self.player_y < enemy.y + ENEMY_HEIGHT
                    && self.player_y + DOGE_HEIGHT > enemy.y
                {
                    self.p2_health = self.p2_health.saturating_sub(HEALTH_HIT_DAMAGE);
                    if !self.is_muted {
                        audio::play("snowball");
                        log!("Playing snowball audio");
                    }
                    if self.p2_health == 0 {
                        self.last_game_over = round_t;
                        self.winner = if self.p1_health > 0 { 1 } else { 0 };
                    }
                    return false;
                }
            }

            enemy.y < CANVAS_HEIGHT as f32
        });

        if self.last_game_over == 0 && self.is_ready && (self.p1_health == 0 || self.p2_health == 0) {
            self.last_game_over = round_t;
            if self.p1_health == 0 && self.p2_health > 0 {
                self.winner = 2;
            } else if self.p2_health == 0 && self.p1_health > 0 {
                self.winner = 1;
            }
        }

        // Draw game elements using modular systems
                leaderboard::LeaderboardSystem::draw(self);
        
        // Draw pause system elements (back button, mute button, pause menu)
        pause::PauseSystem::draw_back_button(self);
        pause::PauseSystem::draw_mute_button(self);
        pause::PauseSystem::draw(self);
        
        // Draw playing screen if in playing mode
        if self.screen == Screen::Playing { // Playing
            clear(0x00ffffff);

            // Draw speed lines (only if not paused)
            if !self.is_paused {
                let line_count = 15; // Number of speed lines
                let max_speed = 25; // Maximum speed of the lines
                let line_width = 128; // Screen width
                for i in 0..line_count {
                    let speed = (i + 1) as u32 * max_speed / line_count; // Varying speeds for each line
                    let height = 1;
                    let y_position = ((i * 28) % 144) as i32; // Vertical position of each line
                    let x_position = (t * speed) as i32 % (512) as i32 - 20; // Moving from right to left
                    rect!(
                        w = line_width,
                        h = height,
                        x = 256 + -x_position,
                        y = y_position,
                        color = 0xffffff88
                    ); // Draw the line
                }
            }
            self.santa.draw();
            
            // Draw player characters
            if self.last_game_over == 0 {
                let p1_sprite = "player1";
                let p2_sprite = "player2";
                sprite!(p1_sprite, x = self.p1_x, y = self.player_y, animation_speed = 1.0);
                sprite!(p2_sprite, x = self.p2_x, y = self.player_y, animation_speed = 1.0);
            } else {
                sprite!("player1", x = self.p1_x, y = self.player_y, animation_speed = 2.0);
                sprite!("player2", x = self.p2_x, y = self.player_y, animation_speed = 2.0);
            }
            
            // ===== ONLINE MULTIPLAYER =====
            if self.is_online {
                if self.channel_id.is_empty() {
                    self.channel_id = self.room_code.clone();
                    if !self.channel_id.is_empty() {
                        log!("Joined room: {}", self.channel_id);
                    }
                }

                if !self.channel_id.is_empty() {
                    let now = time::tick() as u32;
                    if now.saturating_sub(self.last_audio_tick) >= 60 {
                        log!(
                            "Player {} in room {}",
                            turbo::os::client::user_id().unwrap_or_default(),
                            self.channel_id
                        );
                        self.last_audio_tick = now;
                    }
                }

                if let Some(conn) = GameChannel::subscribe(&self.channel_id) {
                    while let Ok(world) = conn.recv() {
                        self.apply_world_state(world);
                    }

                    let input = if self.is_host {
                        PlayerInput {
                            session_id: self.session_id,
                            x: self.p1_x,
                            y: self.player_y,
                            gifts: self.p1_gifts,
                            health: self.p1_health as u8,
                        }
                    } else {
                        PlayerInput {
                            session_id: self.session_id,
                            x: self.p2_x,
                            y: self.player_y,
                            gifts: self.p2_gifts,
                            health: self.p2_health as u8,
                        }
                    };

                    let _ = conn.send(&input);
                }
            }
            
            // Draw game entities (only if not paused)
            if !self.is_paused {
                for gift in self.gifts.iter() {
                    gift.draw();
                }
                for enemy in self.falling_enemies.iter() {
                    enemy.draw();
                }

                for bork in self.borks.iter() {
                    bork.draw();
                }
                for enemy in self.enemies.iter() {
                    enemy.draw();
                }
            } else {
                // Still draw entities when paused but in static positions
                for gift in self.gifts.iter() {
                    gift.draw();
                }
                for enemy in self.falling_enemies.iter() {
                    enemy.draw();
                }

                for bork in self.borks.iter() {
                    bork.draw();
                }
                for enemy in self.enemies.iter() {
                    enemy.draw();
                }
            }

            // Display health and score
            rect!(w = 256, h = 24, color = 0xffffffaa);

            let seconds = if self.last_game_over > 0 {
                self.last_game_over
            } else {
                round_t
            } / 60;
            let minutes = seconds / 60;
            let seconds = seconds % 60;
            let mmss = &format!("{:02}:{:02}", minutes, seconds);
            text!(mmss, x = 108, y = 9, font = "large", color = 0x000000aa);
            text!(mmss, x = 108, y = 8, font = "large", color = 0x000000ff);

            text!("GIFTS", x = 6, y = 3, color = 0x000000ff, font = "small");
            text!("{:03}", self.currency; x = 6, y = 10, font = "medium", color = 0x000000ff);

            text!("POINTS", x = 52, y = 3, color = 0x000000ff, font = "small");
            text!("{:06}", self.points; x = 52, y = 10, font = "medium", color = 0x000000ff);

            let health_bar_w: i32 = 60;
            let health_bar_h: i32 = 6;
            let p1_health_w = ((self.p1_health.min(HEALTH_MAX) * health_bar_w as u32) / HEALTH_MAX) as i32;
            let p2_health_w = ((self.p2_health.min(HEALTH_MAX) * health_bar_w as u32) / HEALTH_MAX) as i32;

            let health_bar_x: i32 = 256 - health_bar_w - 6;
            text!("P1:{:03}", self.p1_gifts; x = health_bar_x - 36, y = 3, font = "small", color = 0x000000ff);
            text!("P2:{:03}", self.p2_gifts; x = health_bar_x - 36, y = 13, font = "small", color = 0x000000ff);

            rect!(w = health_bar_w, h = health_bar_h, x = health_bar_x, y = 4, color = 0x000000ff);
            rect!(w = (health_bar_w - 2).max(0), h = (health_bar_h - 2).max(0), x = health_bar_x + 1, y = 5, color = 0xffffffff);
            rect!(w = p1_health_w.max(0).min(health_bar_w - 2), h = (health_bar_h - 2).max(0), x = health_bar_x + 1, y = 5, color = 0x00cc00ff);

            rect!(w = health_bar_w, h = health_bar_h, x = health_bar_x, y = 14, color = 0x000000ff);
            rect!(w = (health_bar_w - 2).max(0), h = (health_bar_h - 2).max(0), x = health_bar_x + 1, y = 15, color = 0xffffffff);
            rect!(w = p2_health_w.max(0).min(health_bar_w - 2), h = (health_bar_h - 2).max(0), x = health_bar_x + 1, y = 15, color = 0x00cc00ff);

            if round_t < (60 / 2) {
                text!("3", x = 124, y = 64, font = "large", color = 0x000000ff);
            } else if round_t < (120 / 2) {
                text!("2", x = 124, y = 64, font = "large", color = 0x000000ff);
            } else if round_t < (180 / 2) {
                text!("1", x = 124, y = 64, font = "large", color = 0x000000ff);
            } else if round_t < (240 / 2) {
                text!("GO!", x = 118, y = 64, font = "large", color = 0x000000ff);
            }
        }
        // Game over logic
        if self.last_game_over > 0 && self.screen == Screen::Playing { // Playing screen
            let msg = if self.winner == 1 {
                "PLAYER 1 WON"
            } else if self.winner == 2 {
                "PLAYER 2 WON"
            } else {
                "GAME OVER"
            };

            text!(
                msg,
                x = 90,
                y = 73,
                font = "large",
                color = 0x000000aa
            );
            text!(
                msg,
                x = 90,
                y = 72,
                font = "large",
                color = 0xff0000ff
            );
            if round_t - self.last_game_over > 60 {
                if t / 2 % 32 < 16 {
                    text!(
                        "- press any key -",
                        x = 78,
                        y = 84,
                        font = "medium",
                        color = 0x000000aa
                    );
                    text!(
                        "- press any key -",
                        x = 78,
                        y = 83,
                        font = "medium",
                        color = 0x000000ff
                    );
                }
            }
        }
        
        // ===== ONLINE MULTIPLAYER =====
    }
}
