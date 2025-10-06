use std::fs;
use crate::utils::*;

use std::time::{Duration, Instant};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{poll, read, Event};
use serde_json::to_string_pretty;
use crate::models::{AppState, GameState, MenusEnum, ScreensEnum, State, FIELD_SIZE, SCREEN_SIZE};

mod utils;
mod models;
mod menu;
mod game;
mod leaderboard;

fn main() {
    let mut state = init_game_state();
    let mut last_update = Instant::now();
    let mut ticks = 0;
    let update_interval = Duration::from_millis(100);

    toggle_cursor_visibility(false);
    enable_raw_mode().unwrap();
    check_file_exists("leaderboard.txt");
    check_file_exists("settings.json");
    set_initial_state(&mut state);

    while state.app_state.app_running {
        ticks += 1;

        check_for_settings_update(&mut state);
        check_new_leaderboard_score(&mut state);
        check_view_offset(&mut state.app_state);
        check_for_keyboard_event(&mut state);
        update_game_state(&mut state, &mut ticks, &mut last_update, update_interval);
        render(&mut state, &mut last_update, update_interval);
    }

    toggle_cursor_visibility(true);
    disable_raw_mode().unwrap();
}

fn init_game_state() -> State {
    let difficulty = 1;
    let level = 2;
    let game_state = GameState::new(difficulty, level);
    let app_state = AppState {
        difficulty,
        level,
        dirty: false,
        selected_menu: MenusEnum::MainMenu,
        app_running: true,
        selected_screen: ScreensEnum::Menu,
        selected_menu_item: 0,
        view_offset: (0, 0),
        screen_changed: false,
        game_started: false,
        new_score: None,
        leaderboard: None,
    };

    State { game_state, app_state }
}

fn set_initial_state(state: &mut State) {
    let content = fs::read_to_string("settings.json").unwrap();
    let parsed = serde_json::from_str(&content);

    if content.is_empty() || parsed.is_err() {
        check_for_settings_update(state);
        return;
    }

    let saved_state: State = parsed.unwrap();

    state.game_state = saved_state.game_state;
    state.app_state.leaderboard = saved_state.app_state.leaderboard;
    state.app_state.difficulty = saved_state.app_state.difficulty;
    state.app_state.level = saved_state.app_state.level;
    state.app_state.game_started = saved_state.app_state.game_started;
}

fn check_for_keyboard_event(state: &mut State) {
    let has_event = poll(Duration::from_millis(10)).unwrap();

    if !has_event { return; }

    let event = read().unwrap();
    let Event::Key(key_event) = event else { return; };

    if !key_event.is_press() { return; }

    match state.app_state.selected_screen {
        ScreensEnum::Menu => menu::process_keyboard_event(state, key_event),
        ScreensEnum::Game => game::process_keyboard_event(state, key_event),
        ScreensEnum::Leaderboard => leaderboard::process_keyboard_event(state, key_event),
    }
}

fn render(state: &mut State, last_update: &mut Instant, update_interval: Duration) {
    if last_update.elapsed() < update_interval { return; }

    check_if_screen_changed(&mut state.app_state);
    set_row_position(state.app_state.view_offset.1);
    render_logo(state.app_state.view_offset.0);

    match state.app_state.selected_screen {
        ScreensEnum::Game => game::render(state),
        ScreensEnum::Menu => menu::render(state),
        ScreensEnum::Leaderboard => leaderboard::render(state),
    }

    *last_update = Instant::now();
}

fn update_game_state(state: &mut State, ticks: &mut u64, last_update: &mut Instant, update_interval: Duration) {
    if last_update.elapsed() < update_interval { return; }
    if *ticks < state.game_state.required_ticks { return; }

    match state.app_state.selected_screen {
        ScreensEnum::Game => game::update_game_state(state),
        _ => {}
    }

    *ticks = 0;
}

fn check_view_offset(game_state: &mut AppState) {
    let (offset_left, offset_top) = get_view_offset(SCREEN_SIZE.0, SCREEN_SIZE.1);
    let (curr_offset_left, curr_offset_top) = game_state.view_offset;

    if offset_left != curr_offset_left || offset_top != curr_offset_top {
        clear_screen();
        game_state.view_offset = (offset_left, offset_top);
    }
}

fn check_if_screen_changed(app_state: &mut AppState) {
    if !app_state.screen_changed { return; }

    set_row_position(app_state.view_offset.1);

    for _ in 0..SCREEN_SIZE.1 {
        set_col_position(app_state.view_offset.0);

        for _ in 0..SCREEN_SIZE.0 { print!(" "); }

        println!();
    }

    app_state.screen_changed = false;
}

fn check_file_exists(file_name: &str) {
    let dir = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    let file_path = dir.join(file_name);

    if !file_path.exists() {
        fs::File::create(&file_path).unwrap();
    }
}

fn check_new_leaderboard_score(state: &mut State) {
    let new_score = state.app_state.new_score;

    if new_score.is_some() {
        leaderboard::check_if_new_record(state, new_score.unwrap());
        state.app_state.new_score = None;
    }
}

fn check_for_settings_update(state: &mut State) {
    if !state.app_state.dirty { return; }

    let json = to_string_pretty(state).unwrap();

    fs::write("settings.json", json).unwrap();
    state.app_state.dirty = false;
}
