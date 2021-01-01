use crossterm;

mod init;
mod handle;
mod transform;
mod render;
mod util;

#[derive(Copy, Clone)]
pub(crate) struct Character {
    background: crossterm::style::Color,
    foreground: crossterm::style::Color,
    character: char
}

#[derive(Copy, Clone)]
pub(crate) struct Cursor {
    row: usize,
    col: usize,
}

#[derive(Copy, Clone)]
pub(crate) struct TextFrame {
    width: usize,
    height: usize,
    start_row: usize,
    start_col: usize,
}

#[derive(Clone)]
pub(crate) struct LineNumsFrame {
    width: usize,
    height: usize,
    boundary_r: String,
}

#[derive(Copy, Clone)]
pub(crate) struct StatusFrame {
    width: usize,
    height: usize,
}

#[derive(Copy, Clone)]
pub(crate) struct Selection {
    is_selected: bool,
    start_point: Cursor,
    end_point: Cursor,
}

#[derive(Clone)]
pub(crate) struct History {
    lines: Vec<Vec<Character>>,
    cursor: Cursor,
    selection: Selection,
}

#[derive(Clone)]
pub(crate) struct FileData {
    path: String,
    is_saved: bool,
    should_save_as: bool,
}

#[derive(Clone)]
pub struct Lino {
    saved_lines: Vec<Vec<Character>>,
    lines: Vec<Vec<Character>>,
    term_width: usize,
    term_height: usize,
    cursor: Cursor,
    last_cursor_col: usize,
    selection: Selection,
    text_frame: TextFrame,
    line_nums_frame: LineNumsFrame,
    status_frame: StatusFrame,
    should_exit: bool,
    is_rendering: bool,
    undo_list: Vec<History>,
    redo_list: Vec<History>,
    file: FileData,
    clipboard: String,
}
