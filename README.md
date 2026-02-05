<h1 align="center">🎄 <span style="color:#ff4d4d;">HOHOGIFT</span> — <span style="color:#22c55e;">A Christmas Gift Run</span> with <span style="color:#3b82f6;">Real-Time Multiplayer</span></h1> <p align="center" style="font-size:18px;"> <span style="color:#38bdf8;">❄️ Dodge Snowballs</span> &nbsp;•&nbsp; <span style="color:#facc15;">🎁 Catch Gifts from Santa</span> &nbsp;•&nbsp; <span style="color:#a855f7;">🎮 Local & Online Co-op</span> </p> <p align="center"> <img src="https://img.shields.io/badge/Game%20Type-Arcade%20Christmas-red?style=for-the-badge" /> <img src="https://img.shields.io/badge/Multiplayer-Local%20%7C%20Online-green?style=for-the-badge" /> <img src="https://img.shields.io/badge/Engine-Turbo-blue?style=for-the-badge" /> <img src="https://img.shields.io/badge/Platform-Web-yellow?style=for-the-badge" /> </p>

                            Play locally on the same keyboard or join friends online using a room code 
                                         every gift counts, and survival is the real challenge.
---

## 📚 Table of Contents

* [Demo](#demo)
* [What Is HoHoGift?](#what-is-hohogift)
* [Game Modes](#game-modes)
* [Core Gameplay](#core-gameplay)
* [Multiplayer System (Room Codes)](#multiplayer-system-room-codes)
* [Architecture Overview](#architecture-overview)
* [Tech Stack](#tech-stack)
* [Installation](#installation)
* [Running the Game](#running-the-game)
* [How to Play Multiplayer](#how-to-play-multiplayer)
* [Deployment](#deployment)
* [Project Structure](#project-structure)
* [Screenshots](#screenshot)
* [Use Cases](#use-cases)
* [Why HoHoGift?](#why-hohogift)

---

## 🚀 Demo


**Play with your friends  - https://hohogift-neuro.netlify.app/**

🕹 **Turbo OS Live Runtime**

Run locally with:

```bash
turbo run -w
```

> Multiplayer works **in real time** when two players run the game using the **same room code**.

---

## 🎁 What Is HoHoGift?

**HoHoGift** is a **Christmas-themed 2D arcade maze runner** where players race through snowy levels, dodge enemies, collect gifts, and survive together.

It blends:

* 🎄 Festive pixel art
* ⚡ Fast arcade gameplay
* 🌐 Real-time multiplayer
* 🔑 Simple room-code matchmaking

> **Create a room, share the code, and play together instantly — no accounts required.**

---

## 🎮 Controls & Game Modes

*🧍‍♂️ Single / Local Co-op Mode (Same Keyboard)*

Play together on one device using a single keyboard.

Both players are active at the same time and can be controlled locally.

**Controls:**

Player	Keys	Action

Player 1	A / D	Move Left / Right

Player 2	← / →	Move Left / Right

✅ In this mode:

Both players share the same screen

No internet connection required

Great for local co-op gameplay

Perfect for testing and casual play

## 🌐 Online Multiplayer Mode (Room Code Based) 

Play with a friend on two different devices using a shared room code.

**How it works:**

Player 1 clicks Create Room

A unique room code is generated

Player 2 enters the same room code

Both players join the same live session

Controls in Online Mode:

**Player	Controls**

Player 1 (Host) 	 A / D

Player 2 (Guest)	 ← / → 

✅ In online mode:

Each player controls only their own character

Movements sync in real time

Gifts, health, and positions update live

Both players see each other instantly

Works across different devices
---

## 🕹 Core Gameplay

| Mechanic       | Description                      |
| -------------- | -------------------------------- |
| 🎁 Gifts       | Collect gifts to increase score  |
| ❤️ Health      | Avoid enemies to stay alive      |
| ⏱ Timer        | Limited time per run             |
| 👥 Two Players | Player 1 & Player 2 run together |
| 🧠 Strategy    | Coordinate movement to survive   |

---

## 🔐 Multiplayer System (Room Codes)

HoHoGift uses **Turbo OS Channels** to enable real-time multiplayer.

### How It Works

1. Player 1 creates a room
2. A **room code** is generated (example: `XMASAB12`)
3. Player 2 enters the same code
4. Both players join the **same live channel**
5. Player movements & stats sync instantly

✔ No servers to manage
✔ No login required
✔ Works directly in the browser

---

## 🏗 Architecture Overview

```
Client (Turbo Game)
│
├── Input System
│   ├── Keyboard
│   ├── Gamepad
│
├── Game State
│   ├── Player 1 (local)
│   ├── Player 2 (remote)
│
├── Network Layer
│   └── Turbo OS Channel (Room Code)
│
└── Renderer
    ├── Sprites
    ├── UI
    └── HUD
```

---

## 🧪 Tech Stack

### Game Engine

* **Turbo Engine**
* Canvas / Web runtime handled by Turbo

### Language

* **Rust**

### Multiplayer

* **Turbo OS Channels**
* Real-time message sync (no custom server)

### Platform

* Browser (Web)
* Desktop (Turbo runtime)

---

## ⚙ Installation

### Prerequisites

* Rust (stable)
* Turbo CLI

Install Turbo:

```bash
cargo install turbo-cli
```

---

## ▶ Running the Game (Local)

```bash
git clone https://github.com/Suriya0307/hohogift.git
cd hohogift/client
turbo run -w
```

The game will open in a Turbo window.

---

## 🌐 How to Play Multiplayer

### Player 1 (Host)

1. Launch the game
2. Select **Online Multiplayer**
3. Click **Create Room**
4. Copy the generated **room code**
5. Share the code with Player 2

### Player 2 (Guest)

1. Launch the game on another device
2. Select **Online Multiplayer**
3. Enter the same **room code**
4. Press **Enter** to join

🎉 Both players now play **in the same game world**.

---

## 🌍 Deployment

### Export for Web

```bash
turbo export
```

This creates a `www/` folder.

### Hosting Options

* **Netlify** (drag & drop `www/`)
* **Itch.io** (HTML game upload)

⚠️ Note:
Real-time multiplayer requires **Turbo OS runtime**.
Static hosting is best for **single-player demos**.

---

## 📁 Project Structure

```
hohogift/
├── client/
│   ├── src/
│   │   ├── lib.rs
│   │   ├── lobby.rs
│   │   ├── state.rs
│   │   ├── network.rs
│   │   └── protocol.rs
│   ├── sprites/
│   └── audio/
├── Cargo.toml
├── turbo.toml
├── .gitignore
└── README.md
```

---


## Screenshots



##  Game-Lobby 


<img width="1920" height="1140" alt="image" src="https://github.com/user-attachments/assets/39a7f27c-772f-4e2c-8962-80dfa0e5f8f2" />





## Single/local mode



<img width="1920" height="1140" alt="image" src="https://github.com/user-attachments/assets/5512c18f-ca90-4a87-8445-1c40a231b404" />




## Multiplayer mode 



<img width="1920" height="1140" alt="image" src="https://github.com/user-attachments/assets/c30963af-3c8b-42df-8939-92973314f2b5" />



## Two friends are in the same room 



<img width="1920" height="1200" alt="image" src="https://github.com/user-attachments/assets/9e53d8c4-7f5e-4dcd-a420-0fc3263aba1e" />





## 🎯 Use Cases

* 🎄 Christmas / seasonal arcade games
* 🧠 Learning real-time multiplayer concepts
* 🕹 Hackathon demo projects
* 🎓 Teaching Rust game development
* 🌐 Browser-based multiplayer experiments

---

## 💡 Why HoHoGift?

> “Multiplayer doesn’t have to be complicated.”



HoHoGift proves that:

* You don’t need heavy servers
* You don’t need user accounts
* You don’t need complex matchmaking

Just:
🎮 a game
🔑 a room code
👥 two players

---

<p align="center" style="font-size:18px;">
  <i><b>“A small Christmas game that brings two players together — instantly.”</b></i>
</p>

---


