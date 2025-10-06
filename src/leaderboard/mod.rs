use std::fs;
use crossterm::event::{KeyCode, KeyEvent};
use crate::models::{AppState, ScreensEnum, State, SCREEN_SIZE};
use crate::utils::{print_empty_lines, set_col_position};

pub fn process_keyboard_event(state: &mut State, key_event: KeyEvent) {
    let app_state = &mut state.app_state;

    match key_event.code {
        KeyCode::Esc => back_to_menu(app_state),
        _ => {}
    }
}

pub fn render(state: &mut State) {
    let app_state = &mut state.app_state;
    let leaderboard = app_state.leaderboard.get_or_insert_with(get_leaderboard);
    let title_offset = app_state.view_offset.0 + (SCREEN_SIZE.0 - 12) / 2;
    let no_records_instructions = [
        "There are no records set..",
        "Play Snake to set new scores",
    ];

    set_col_position(title_offset);
    print!("LEADERBOARD");

    print_empty_lines(3);

    if leaderboard.len() == 0 {
        for instruction in no_records_instructions {
            set_col_position(app_state.view_offset.0);
            print!("{}", instruction);
            println!();
        }

        return;
    }

    for i in 0..10 {
        let score = leaderboard
            .get(i)
            .map(|s| s.to_string())
            .unwrap_or(" ".to_string());

        set_col_position(app_state.view_offset.0);
        print!("{}. {}", i + 1, score);
        println!();
    }
}

pub fn check_if_new_record(state: &mut State, score: u64) {
    let mut top_scores = get_leaderboard();
    let is_new_record = match top_scores.last() {
        None => true,
        Some(last) => top_scores.len() < 10 || score > *last,
    };

    if !is_new_record || score == 0 || top_scores.contains(&score) { return; }

    top_scores.push(score);
    top_scores.sort_by(|a, b| b.cmp(a));
    top_scores.truncate(10);

    save_new_leaderboard(&top_scores);
    state.app_state.leaderboard = Some(top_scores);
}

fn back_to_menu(app_state: &mut AppState) {
    app_state.selected_screen = ScreensEnum::Menu;
    app_state.screen_changed = true;
}

fn get_leaderboard() -> Vec<u64> {
    let path = "leaderboard.txt";
    let content = fs::read_to_string(path).unwrap();
    let mut records: Vec<u64> = content
        .lines()
        .filter_map(|line| line.trim().parse::<u64>().ok())
        .collect();

    records.sort_by(|a, b| b.cmp(a));

    records
}

fn save_new_leaderboard(records: &Vec<u64>) {
    let path = "leaderboard.txt";
    let content = records
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>().join("\n");

    fs::write(path, content).unwrap();
}