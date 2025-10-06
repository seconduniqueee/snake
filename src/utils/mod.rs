use std::io::stdout;
use crossterm::cursor::{Hide, MoveToColumn, MoveToRow, Show};
use crossterm::{execute};
use crossterm::terminal::{size, Clear, ClearType};
use rand::{random_range};
use crate::FIELD_SIZE;

pub fn clear_screen() {
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

pub fn set_col_position(col: u16) {
    execute!(stdout(), MoveToColumn(col)).unwrap();
}

pub fn set_row_position(row: u16) {
    execute!(stdout(), MoveToRow(row)).unwrap();
}

pub fn get_random_int(min: usize, max: usize) -> usize {
    random_range(min..max)
}

pub fn toggle_cursor_visibility(show_cursor: bool) {
    if show_cursor {
        execute!(stdout(), Show).unwrap();
    } else {
        execute!(stdout(), Hide).unwrap();
    }
}

pub fn get_random_free_cell(grid: & Vec<Vec<u16>>) -> (usize, usize) {
    let mut free_cells: Vec<(usize, usize)> = Vec::new();

    for i in 0..FIELD_SIZE {
        for j in 0..FIELD_SIZE {
            if grid[i][j] == 0 {
                free_cells.push((i, j));
            }
        }
    }

    free_cells[get_random_int(0, free_cells.len())]
}

pub fn print_empty_lines(amount: u16) {
    for _ in 0..amount { println!(); }
}

pub fn get_view_offset(view_width: u16, view_height: u16) -> (u16, u16) {
    let (cols, rows) = size().unwrap();

    let offset_left = cols.saturating_sub(view_width) / 2;
    let offset_top = rows.saturating_sub(view_height) / 2;

    (offset_left, offset_top)
}

pub fn render_logo(offset_left: u16) {
    let lines = [
        "    Welcome to Snake Game!   ",
        "*****************************",
        "                             ",
        "                             ",
    ];

    for i in 0..lines.len() {
        println!();
        set_col_position(offset_left);
        print!("{}", lines[i]);
    }
}