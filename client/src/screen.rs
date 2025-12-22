use turbo::*;

/* =========================
   SCREEN ENUM
========================= */

#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    Home,
    ModeSelect,
    OnlineMenu,
    WaitingRoom,
    Playing,
}

impl Screen {
    pub fn new() -> Self {
        Screen::HoHoHome
    }
}

/* =========================
   MAIN SCREEN LOGIC
========================= */

impl Screen {
    pub fn match_screen(&self) {
        clear(0x0fe3f4ff); // light winter blue background

        match self {
            Screen::HoHoHome => draw_home(),
            Screen::ModeSelect => draw_mode_select(),
            Screen::OnlineMenu => draw_online_menu(),
            Screen::WaitingRoom => draw_waiting_room(),
            Screen::Playing => {
                text!(
                    "GAME RUNNING",
                    x = 80,
                    y = 60,
                    font = "large",
                    color = 0x000000ff
                );
            }
        }
    }

    pub fn state_machine(&mut self, _state: &crate::GameState) {
        let p = pointer::screen();

        match self {
            Screen::HoHoHome => {
                if button(80, 70, 100, 18, "PLAY", p) {
                    *self = Screen::ModeSelect;
                }
            }

            Screen::ModeSelect => {
                if button(60, 50, 140, 16, "SINGLE PLAYER", p) {
                    *self = Screen::Playing;
                }
                if button(60, 72, 140, 16, "LOCAL MULTIPLAYER", p) {
                    *self = Screen::Playing;
                }
                if button(60, 94, 140, 16, "ONLINE MULTIPLAYER", p) {
                    *self = Screen::OnlineMenu;
                }
            }

            Screen::OnlineMenu => {
                if button(60, 60, 140, 16, "CREATE ROOM", p) {
                    *self = Screen::WaitingRoom;
                }
                if button(60, 82, 140, 16, "JOIN ROOM", p) {
                    *self = Screen::WaitingRoom;
                }
            }

            Screen::WaitingRoom => {
                if button(60, 100, 140, 16, "START GAME", p) {
                    *self = Screen::Playing;
                }
            }

            Screen::Playing => {}
        }
    }
}

/* =========================
   UI HELPERS
========================= */

fn button(x: i32, y: i32, w: i32, h: i32, label: &str, p: Pointer) -> bool {
    let hovered =
        p.x >= x && p.x <= x + w &&
        p.y >= y && p.y <= y + h;

    let bg = if hovered { 0xffffffff } else { 0xffffffaa };

    rect!(x = x, y = y, w = w, h = h, color = bg);
    rect!(x = x, y = y, w = w, h = h, color = 0x000000ff);

    text!(
        label,
        x = x + 8,
        y = y + 5,
        font = "medium",
        color = 0x000000ff
    );

    hovered && p.just_pressed()
}

/* =========================
   SCREEN DRAWING
========================= */

fn draw_home() {
    text!(
        "🎄 SNOW GIFT RUN 🎄",
        x = 40,
        y = 30,
        font = "large",
        color = 0x000000ff
    );

    text!(
        "Collect gifts. Avoid snowballs.",
        x = 36,
        y = 50,
        font = "small",
        color = 0x000000ff
    );
}

fn draw_mode_select() {
    text!(
        "SELECT MODE",
        x = 80,
        y = 30,
        font = "large",
        color = 0x000000ff
    );
}

fn draw_online_menu() {
    text!(
        "ONLINE MULTIPLAYER",
        x = 50,
        y = 30,
        font = "large",
        color = 0x000000ff
    );
}

fn draw_waiting_room() {
    text!(
        "WAITING ROOM",
        x = 70,
        y = 30,
        font = "large",
        color = 0x000000ff
    );

    text!(
        "ROOM CODE:",
        x = 80,
        y = 55,
        font = "small",
        color = 0x000000ff
    );

    text!(
        "ABCD",
        x = 100,
        y = 70,
        font = "large",
        color = 0xff0000ff
    );

    text!(
        "Waiting for players...",
        x = 50,
        y = 90,
        font = "small",
        color = 0x000000ff
    );
}
