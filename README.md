# Snake Game in Rust (Terminal)

A terminal-based Snake game implemented in Rust, featuring a centered, refreshable UI, dynamic levels, difficulty settings, and persistent state.

## Features

* Control the snake to collect food and avoid walls. Bonus appears once every 5 food eaten and decreases in value over time.
* Tracks the top 10 local scores. If your score ranks in the top 10, it is automatically saved.
* Four difficulty levels: Easy, Medium, Hard, Extreme. Each affects snake speed and points per food.
* Five unique levels with different wall layouts.
* Navigate back to the menu during a game and resume an in-progress game using the "Continue" option.
* Saves difficulty, level, and in-progress games automatically. Reopen the game to continue where you left off.

---


## Dependencies

This project uses the following Rust crates:

* [`crossterm`](https://crates.io/crates/crossterm) = "0.29.0"
* [`rand`](https://crates.io/crates/rand) = "0.9.2"
* [`log`](https://crates.io/crates/log) = "0.4.28"
* [`serde`](https://crates.io/crates/serde) = "1.0.228"
* [`serde_json`](https://crates.io/crates/serde_json) = "1.0.145"

---

## Por Structure
```bash
snake_game/
├─ main.rs
├─ game/              # Core game loop and mechanics
│  └─ mod.rs           
├─ leaderboard/       # Leaderboard logic
│  └─ mod.rs           
├─ menu/              # Menu navigation and options
│  └─ mod.rs           
├─ models/            # Enums, constants, and lists
│  └─ mod.rs           
└─ utils/             # Helper functions
   └─ mod.rs
```


## Gameplay Preview

https://github.com/user-attachments/assets/35b188a3-6c65-4013-93b0-e6f1cf795781

---

## Running the Game

The game runs via the compiled executable:

1. Build the project:

```
cargo build --release
```

2. Run the compiled executable (`.exe` on Windows, in `target/release`):

```
./target/release/snake.exe
```

---

## Controls

* **Arrow keys** – Move the snake
* **Esc** – Go back to the menu
* **Enter** - Option selection

---

## Saving and Leaderboard

* Settings, difficulty, level, and in-progress games are saved automatically in `settings.json` locally.
* Leaderboard scores are stored locally in `leaderboard.txt`.

---

## License

This project is **MIT Licensed**. Feel free to use, modify, and share.
