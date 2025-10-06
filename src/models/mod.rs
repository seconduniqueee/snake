use serde::{Deserialize, Serialize};

pub const FIELD_SIZE: usize = 15;
pub const SCREEN_SIZE: (u16, u16) = (30, 30);
pub const SNAKE: u16 = 1;
pub const FOOD: u16 = 2;
pub const BONUS: u16 = 3;
pub const WALL: u16 = 4;

pub static MAIN_MENU_ITEMS: [MenuItem<MainMenuItemsEnum>; 6] = [
    MenuItem { label: "Continue", value: MainMenuItemsEnum::Continue },
    MenuItem { label: "New Game", value: MainMenuItemsEnum::NewGame },
    MenuItem { label: "Leaderboard", value: MainMenuItemsEnum::Leaderboard },
    MenuItem { label: "Difficulty", value: MainMenuItemsEnum::Difficulty },
    MenuItem { label: "Select Level", value: MainMenuItemsEnum::LevelSelection },
    MenuItem { label: "Exit", value: MainMenuItemsEnum::Exit },
];

pub static DIFFICULTY_MENU_ITEMS: [MenuItem<u16>; 4] = [
    MenuItem { label: "Easy", value: 1 },
    MenuItem { label: "Medium", value: 2 },
    MenuItem { label: "Hard", value: 3 },
    MenuItem { label: "Extreme", value: 4 }
];

pub static LEVEL_MENU_ITEMS: [MenuItem<u16>; 5] = [
    MenuItem { label: "Plain Field", value: 0 },
    MenuItem { label: "Box", value: 1 },
    MenuItem { label: "Labyrinth", value: 2 },
    MenuItem { label: "Two Sides", value: 3 },
    MenuItem { label: "Roundabout", value: 4 }
];

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub enum DirectionsEnum {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub enum MenusEnum {
    MainMenu,
    SelectDifficulty,
    SelectLevel,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub enum ScreensEnum {
    Game,
    Menu,
    Leaderboard,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Deserialize, Serialize)]
pub enum MainMenuItemsEnum {
    Continue,
    NewGame,
    Leaderboard,
    Difficulty,
    LevelSelection,
    Exit
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct MenuItem<T> {
    pub label: &'static str,
    pub value: T,
}

pub type Coord = (usize, usize);

#[derive(Serialize, Deserialize)]
pub struct State {
    pub game_state: GameState,
    pub app_state: AppState,
}

#[derive(Serialize, Deserialize)]
pub struct AppState {
    pub app_running: bool,
    pub selected_screen: ScreensEnum,
    pub selected_menu: MenusEnum,
    pub selected_menu_item: usize,
    pub view_offset: (u16, u16),
    pub screen_changed: bool,
    pub difficulty: u16,
    pub level: u16,
    pub game_started: bool,
    pub new_score: Option<u64>,
    pub leaderboard: Option<Vec<u64>>,
    pub dirty: bool,
}

#[derive(Serialize, Deserialize)]
pub struct GameState {
    pub grid: Vec<Vec<u16>>,
    pub snake_direction: DirectionsEnum,
    pub next_direction: DirectionsEnum,
    pub snake_body: Vec<Coord>,
    pub food_position: Coord,
    pub bonus_position: Option<Coord>,
    pub bonus_value: u64,
    pub food_eaten: u64,
    pub ate_food: bool,
    pub game_over: bool,
    pub score: u64,
    pub required_ticks: u64,
    pub food_for_bonus_needed: u64,
    pub freeze: bool,
}

pub static LEVELS: [&[Coord]; 5] = [
    &[],
    &[
        (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8), (0, 9), (0, 10), (0, 11), (0, 12), (0, 13), (0, 14),
        (14, 0), (14, 1), (14, 2), (14, 3), (14, 4), (14, 5), (14, 6), (14, 7), (14, 8), (14, 9), (14, 10), (14, 11), (14, 12), (14, 13), (14, 14),
        (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0), (9, 0), (10, 0), (11, 0), (12, 0), (13, 0),
        (1, 14), (2, 14), (3, 14), (4, 14), (5, 14), (6, 14), (7, 14), (8, 14), (9, 14), (10, 14), (11, 14), (12, 14), (13, 14),
    ],
    &[
        (6, 0), (6, 1), (6, 2), (6, 3), (6, 4), (6, 5), (6, 6), (6, 7), (6, 8), (6, 9), (6,10), (6,11), (6,12), (6,13), (6,14),
        (7, 0), (7, 1), (7, 2), (7, 3), (7, 4), (7, 5), (7, 6), (7, 7), (7, 8), (7, 9), (7,10), (7,11), (7,12), (7,13), (7,14),
        (8, 0), (8, 1), (8, 2), (8, 3), (8, 4), (8, 5), (8, 6), (8, 7), (8, 8), (8, 9), (8,10), (8,11), (8,12), (8,13), (8,14),
        (0, 6), (1, 6), (2, 6), (3, 6), (4, 6), (5, 6), (9, 6), (10,6), (11,6), (12,6), (13,6), (14,6),
        (0, 7), (1, 7), (2, 7), (3, 7), (4, 7), (5, 7), (9, 7), (10,7), (11,7), (12,7), (13,7), (14,7),
        (0, 8), (1, 8), (2, 8), (3, 8), (4, 8), (5, 8), (9, 8), (10,8), (11,8), (12,8), (13,8), (14,8),
        (0, 0), (0, 1), (0, 2), (1, 0), (2, 0),
        (0, 12), (0, 13), (0, 14), (1, 14), (2, 14),
        (12, 0), (13, 0), (14, 0), (14, 1), (14, 2),
        (12, 14), (13, 14), (14, 14), (14, 12), (14, 13),
    ],
    &[
        (0, 6), (0, 7), (0, 8),
        (1, 6), (1, 7), (1, 8),
        (2, 6), (2, 7), (2, 8),
        (3, 6), (3, 7), (3, 8),
        (4, 6), (4, 7), (4, 8),
        (5, 6), (5, 7), (5, 8),
        (6, 6), (6, 7), (6, 8),
        (7, 6), (7, 7), (7, 8),
        (8, 6), (8, 7), (8, 8),
        (9, 6), (9, 7), (9, 8),
        (10, 6), (10, 7), (10, 8),
        (11, 6), (11, 7), (11, 8),
        (12, 6), (12, 7), (12, 8),
        (13, 6), (13, 7), (13, 8),
        (14, 6), (14, 7), (14, 8),
    ],
    &[
        (5, 5), (5, 6), (5, 7), (5, 8), (5, 9),
        (6, 5), (6, 6), (6, 7), (6, 8), (6, 9),
        (7, 5), (7, 6), (7, 7), (7, 8), (7, 9),
        (8, 5), (8, 6), (8, 7), (8, 8), (8, 9),
        (9, 5), (9, 6), (9, 7), (9, 8), (9, 9),
    ],
];


