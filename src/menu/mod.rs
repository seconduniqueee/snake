use crossterm::event::{KeyCode, KeyEvent};
use crate::models::{GameState, MainMenuItemsEnum, MenuItem, MenusEnum, ScreensEnum, State, DIFFICULTY_MENU_ITEMS, LEVEL_MENU_ITEMS, MAIN_MENU_ITEMS, SCREEN_SIZE};
use crate::utils::{set_col_position};

pub fn process_keyboard_event(state: &mut State, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Down => set_selected_menu(state, true),
        KeyCode::Up => set_selected_menu(state, false),
        KeyCode::Enter => process_menu_selection(state),
        KeyCode::Esc => {
            state.app_state.selected_menu = MenusEnum::MainMenu;
            state.app_state.selected_menu_item = 0;
        },
        _ => {}
    }
}

pub fn render(state: &mut State) {
    let menu_labels: Vec<&str> = match state.app_state.selected_menu {
        MenusEnum::MainMenu => get_main_menu_items(state).iter().map(|x| x.label).collect(),
        MenusEnum::SelectDifficulty => DIFFICULTY_MENU_ITEMS.iter().map(|x| x.label).collect(),
        MenusEnum::SelectLevel => LEVEL_MENU_ITEMS.iter().map(|x| x.label).collect(),
    };

    let app_state = &mut state.app_state;

    for _ in 0..5 { println!(); }
    for i in 0..menu_labels.len() {
        let option_offset = (SCREEN_SIZE.0 - (menu_labels[i].len() as u16 + 5)) / 2;
        let is_selected = app_state.selected_menu_item == i;

        set_col_position(app_state.view_offset.0 + option_offset);

        if is_selected {
            println!(">> {} <<", menu_labels[i]);
        } else {
            println!("   {}   ", menu_labels[i]);
        }

        println!()
    }
}

fn process_menu_selection(state: &mut State) {
    match state.app_state.selected_menu {
        MenusEnum::MainMenu => process_main_menu_selection(state),
        MenusEnum::SelectDifficulty => process_difficulty_menu_selection(state),
        MenusEnum::SelectLevel => process_level_menu_selection(state)
    }

    state.app_state.screen_changed = true;
}

fn process_main_menu_selection(state: &mut State) {
    let menu = get_main_menu_items(state);
    let app_state = &mut state.app_state;
    let game_state = &mut state.game_state;
    let menu_index = app_state.selected_menu_item;

    match menu[menu_index].value {
        MainMenuItemsEnum::Continue => {
            app_state.selected_menu_item = 0;
            app_state.selected_screen = ScreensEnum::Game;
            game_state.freeze = true;
        }
        MainMenuItemsEnum::NewGame => {
            app_state.selected_screen = ScreensEnum::Game;
            app_state.game_started = true;
            app_state.selected_menu_item = 0;
            state.game_state = GameState::new(app_state.difficulty, app_state.level);
        }
        MainMenuItemsEnum::Leaderboard => {
            app_state.selected_menu_item = 0;
            app_state.selected_screen = ScreensEnum::Leaderboard;
        }
        MainMenuItemsEnum::Difficulty => {
            app_state.selected_menu = MenusEnum::SelectDifficulty;
            app_state.selected_menu_item = DIFFICULTY_MENU_ITEMS
                .iter()
                .position(|x| x.value == app_state.difficulty)
                .unwrap()
        }
        MainMenuItemsEnum::LevelSelection => {
            app_state.selected_menu = MenusEnum::SelectLevel;
            app_state.selected_menu_item = LEVEL_MENU_ITEMS
                .iter()
                .position(|x| x.value == app_state.level)
                .unwrap();
        }
        MainMenuItemsEnum::Exit => {
            app_state.app_running = false;
            app_state.selected_menu_item = 0;
        }
    }
}

fn process_difficulty_menu_selection(state: &mut State) {
    let menu_index = state.app_state.selected_menu_item;
    let difficulty = DIFFICULTY_MENU_ITEMS[menu_index].value;

    state.app_state.game_started = false;
    state.app_state.difficulty = difficulty;
    state.app_state.selected_menu = MenusEnum::MainMenu;
    state.app_state.selected_menu_item = main_menu_item_index(state, MainMenuItemsEnum::Difficulty);
    state.app_state.dirty = true;
}

fn process_level_menu_selection(state: &mut State) {
    let menu_index = state.app_state.selected_menu_item;
    let level = LEVEL_MENU_ITEMS[menu_index].value;

    state.app_state.game_started = false;
    state.app_state.level = level;
    state.app_state.selected_menu = MenusEnum::MainMenu;
    state.app_state.selected_menu_item = main_menu_item_index(state, MainMenuItemsEnum::LevelSelection);
    state.app_state.dirty = true;
}

fn set_selected_menu(state: &mut State, is_increment: bool) {
    let max_items = match state.app_state.selected_menu {
        MenusEnum::MainMenu => get_main_menu_items(state).len(),
        MenusEnum::SelectDifficulty => DIFFICULTY_MENU_ITEMS.len(),
        MenusEnum::SelectLevel => LEVEL_MENU_ITEMS.len(),
    };

    let app_state = &mut state.app_state;
    let selected = app_state.selected_menu_item;
    let is_first = selected == 0;
    let is_last = selected == max_items - 1;

    if is_increment {
        app_state.selected_menu_item = if is_last { 0 } else { selected + 1};
    } else {
        app_state.selected_menu_item = if is_first { max_items - 1 } else { selected - 1 };
    }
}

fn get_main_menu_items(state: &State) -> Vec<MenuItem<MainMenuItemsEnum>> {
    let game_started = state.app_state.game_started;

    MAIN_MENU_ITEMS
        .iter()
        .filter(|item| game_started || item.value != MainMenuItemsEnum::Continue)
        .cloned()
        .collect()
}

fn main_menu_item_index(state: &mut State, item_type: MainMenuItemsEnum) -> usize {
    get_main_menu_items(state)
        .iter()
        .position(|a| a.value == item_type)
        .unwrap()
}
