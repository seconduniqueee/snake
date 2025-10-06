use crossterm::event::{KeyCode, KeyEvent};
use crate::FIELD_SIZE;
use crate::models::{AppState, Coord, DirectionsEnum, GameState, ScreensEnum, State, BONUS, FOOD, LEVELS, SCREEN_SIZE, SNAKE, WALL};
use crate::utils::{get_random_free_cell, get_random_int, print_empty_lines, set_col_position};

impl GameState {
    pub fn new(difficulty: u16, level: u16) -> Self {
        get_new_game_state(difficulty, level)
    }
}

pub fn process_keyboard_event(state: &mut State, key_event: KeyEvent) {
    let game_state = &mut state.game_state;
    let app_state = &mut state.app_state;

    game_state.freeze = false;

    match key_event.code {
        KeyCode::Down => game_state.next_direction = DirectionsEnum::Down,
        KeyCode::Up => game_state.next_direction = DirectionsEnum::Up,
        KeyCode::Left => game_state.next_direction = DirectionsEnum::Left,
        KeyCode::Right => game_state.next_direction = DirectionsEnum::Right,
        KeyCode::Esc => back_to_menu(app_state),
        KeyCode::Enter => {
            if game_state.game_over {
                *game_state = GameState::new(app_state.difficulty, app_state.level);
            }
        }
        _ => {}
    }
}

pub fn update_game_state(state: &mut State) {
    let ate_food = state.game_state.ate_food;

    if state.game_state.game_over || state.game_state.freeze { return; }

    state.game_state.ate_food = false;

    update_curr_direction(&mut state.game_state);
    update_bonus_value(&mut state.game_state);

    let head = state.game_state.snake_body[0];
    let next_head = get_next_head(&mut state.game_state, head);

    if state.game_state.snake_body.contains(&next_head) ||
        state.game_state.grid[next_head.0][next_head.1] == WALL
    {
        state.game_state.game_over = true;
        state.app_state.game_started = false;
        state.app_state.screen_changed = true;
        state.app_state.new_score = Some(state.game_state.score);
        return;
    }

    if !ate_food {
        state.game_state.snake_body.pop();
    }

    state.game_state.snake_body.insert(0, next_head);
    set_grid_values(state);

    if is_bonus(&state.game_state, next_head) {
        state.game_state.score += state.game_state.bonus_value;
        state.game_state.bonus_position = None;
        state.game_state.bonus_value = 100;
    }

    if state.game_state.food_position == next_head {
        state.game_state.ate_food = true;
        state.game_state.food_position = get_random_free_cell(&state.game_state.grid);
        state.game_state.score += get_food_value(&state.app_state);
        state.game_state.food_eaten += 1;
        state.game_state.food_for_bonus_needed = state.game_state.food_for_bonus_needed.saturating_sub(1);
    }

    if state.game_state.food_for_bonus_needed == 0 {
        state.game_state.ate_food = true;
        state.game_state.bonus_position = get_bonus_position(&mut state.game_state);
        state.game_state.bonus_value = 100;
        state.game_state.food_for_bonus_needed = 5;
    }

    set_grid_values(state);
}

pub fn render(state: &mut State) {
    if !state.game_state.game_over {
        render_game(state);
    } else {
        render_game_result(state);
    }
}

fn render_game(state: &mut State) {
    let game_state = &mut state.game_state;
    let app_state = &mut state.app_state;
    let offset = app_state.view_offset.0 + (SCREEN_SIZE.0 - FIELD_SIZE as u16 * 2) / 2;
    let head = game_state.snake_body[0];
    let dir = game_state.snake_direction;

    for i in 0..FIELD_SIZE {
        set_col_position(offset);

        for j in 0..FIELD_SIZE {
            let is_head = head == (i, j);
            let is_bonus = game_state.grid[i][j] == BONUS;
            let is_snake = game_state.grid[i][j] == SNAKE;
            let is_food = game_state.grid[i][j] == FOOD;
            let is_wall = game_state.grid[i][j] == WALL;

            let symbol = if is_bonus { "▒▒" }
                else if is_head && dir == DirectionsEnum::Down { "V "}
                else if is_head && dir == DirectionsEnum::Left { "< " }
                else if is_head && dir == DirectionsEnum::Right { "> " }
                else if is_head && dir == DirectionsEnum::Up { "^ " }
                else if is_snake { "o " }
                else if is_food { "■ " }
                else if is_wall { "X "}
                else {"˙ "};

            print!("{}", symbol);
        }

        println!();
    }

    println!();
    set_col_position(offset);
    println!("Score: {}", game_state.score);
}

fn render_game_result(state: &mut State) {
    print_empty_lines(3);

    let offset = state.app_state.view_offset.0;
    let instructions = [
        String::from("GAME OVER"),
        format!("YOUR SCORE: {}", state.game_state.score),
        String::from(""),
        String::from("'Enter' to start a new game"),
        String::from("'Esc' to open main manu")
    ];

    for instruction in instructions.iter() {
        let actual_offset = offset + (SCREEN_SIZE.0.saturating_sub(instruction.len() as u16)) / 2;

        set_col_position(actual_offset);
        println!("{}", instruction);
    }
}

fn set_grid_values(state: &mut State) {
    let game_state = &mut state.game_state;

    for i in 0..FIELD_SIZE {
        for j in 0..FIELD_SIZE {
            if game_state.grid[i][j] == WALL { continue; }

            game_state.grid[i][j] = if (i, j) == game_state.food_position { FOOD }
                else if game_state.snake_body.contains(&(i, j)) { SNAKE }
                else if is_bonus(game_state, (i, j)) { BONUS }
                else { 0 };
        }
    }
}

fn update_curr_direction(state: &mut GameState) {
    match (state.snake_direction, state.next_direction) {
        (DirectionsEnum::Up, DirectionsEnum::Down) |
        (DirectionsEnum::Down, DirectionsEnum::Up) |
        (DirectionsEnum::Left, DirectionsEnum::Right) |
        (DirectionsEnum::Right, DirectionsEnum::Left) => return,
        _ => {}
    }

    state.snake_direction = state.next_direction;
}

fn back_to_menu(app_state: &mut AppState) {
    app_state.selected_screen = ScreensEnum::Menu;
    app_state.screen_changed = true;
    app_state.dirty = true;
}

fn get_bonus_position(game_state: &mut GameState) -> Option<(usize, usize)> {
    let mut free_cells: Vec<(usize, usize)> = Vec::new();

    for i in 0..FIELD_SIZE - 1 {
        for j in 0..FIELD_SIZE - 1 {
            let free = game_state.grid[i][j] == 0 &&
                game_state.grid[i][j + 1] == 0 &&
                game_state.grid[i + 1][j] == 0 &&
                game_state.grid[i + 1][j + 1] == 0;

            if free { free_cells.push((i, j)); }
        }
    }

    if free_cells.len() != 0 {
        Some(free_cells[get_random_int(0, free_cells.len())])
    } else {
        None
    }
}

fn update_bonus_value(game_state: &mut GameState) {
    if game_state.bonus_position.is_none() { return; }

    game_state.bonus_value = game_state.bonus_value.saturating_sub(3);

    if game_state.bonus_value == 0 {
        game_state.bonus_position = None;
    }
}

fn is_bonus(game_state: &GameState, coord: Coord) -> bool {
    if game_state.bonus_position.is_none() { return false; }

    let (x, y) = game_state.bonus_position.unwrap();

    coord == (x, y) ||
        coord == (x + 1, y) ||
        coord == (x, y + 1) ||
        coord == (x + 1, y + 1)
}

fn get_new_game_state(difficulty: u16, level: u16) -> GameState {
    let grid = gen_grid(FIELD_SIZE, level);
    let food_position = get_random_free_cell(&grid);
    let required_ticks = match difficulty {
        1 => 20,
        2 => 15,
        3 => 10,
        4 => 2,
        _ => 2,
    };

    let game_state = GameState {
        snake_direction: DirectionsEnum::Right,
        next_direction: DirectionsEnum::Right,
        snake_body: Vec::from([get_starting_position(level)]),
        ate_food: false,
        game_over: false,
        score: 0,
        bonus_position: None,
        food_eaten: 0,
        bonus_value: 50,
        food_for_bonus_needed: 5,
        freeze: false,
        grid,
        food_position,
        required_ticks,
    };

    game_state
}

fn gen_grid(size: usize, level: u16) -> Vec<Vec<u16>> {
    let level = LEVELS[level as usize];
    let mut grid = vec![vec![0; size]; size];

    for (x, y) in level {
        grid[*x][*y] = WALL;
    }

    grid
}

fn get_starting_position(level: u16) -> Coord {
    match level {
        0 | 1 => (7, 7),
        2 => (4, 0),
        3 | 4 => (0, 0),
        _ => (0, 0),
    }
}

fn get_next_head(game_state: &GameState, head: Coord) -> Coord {
    let max = FIELD_SIZE - 1;
    let (mut curr_row, mut curr_col) = head;

    match game_state.snake_direction {
        DirectionsEnum::Up => {
            curr_row = if curr_row == 0 { max } else { curr_row - 1 };
        }
        DirectionsEnum::Down => {
            curr_row = if curr_row == max { 0 } else { curr_row + 1 };
        }
        DirectionsEnum::Left => {
            curr_col = if curr_col == 0 { max } else { curr_col - 1 };
        }
        DirectionsEnum::Right => {
            curr_col = if curr_col == max { 0 } else { curr_col + 1 };
        }
    }

    (curr_row, curr_col)
}

fn get_food_value(state: &AppState) -> u64 {
    match state.difficulty {
        1 => 1,
        2 => 3,
        3 => 5,
        4 => 8,
        _ => 8,
    }
}
