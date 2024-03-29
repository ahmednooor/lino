use crossterm;

mod init;
mod handle;
mod cursor;
mod edit;
mod highlight;
mod render;
mod util;
mod errors;
mod file;
mod history;
mod selection;
mod commands;
mod keybindings;
mod frames;
mod task_feedback;
mod exit;
mod save;
mod find;
mod replace;
mod guide;
mod input_prompt;
mod confirmation_prompt;

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub(crate) struct Character {
    background: crossterm::style::Color,
    foreground: crossterm::style::Color,
    character: char,
    width: u8,
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
    save_error: String,
}

#[derive(Clone)]
pub(crate) struct Settings {
    tab_width: usize,
    show_line_nums_frame: bool,
    show_status_frame: bool,
    read_only: bool,
}

#[derive(Clone)]
pub(crate) struct Error {
    is_occured: bool,
    message: String,
    code: isize,
}

#[derive(Clone)]
pub(crate) struct Theming {
    line_nums_frame_bg: crossterm::style::Color,
    line_nums_frame_fg: crossterm::style::Color,
    line_nums_frame_highlighted_bg: crossterm::style::Color,
    line_nums_frame_highlighted_fg: crossterm::style::Color,
    
    text_frame_bg: crossterm::style::Color,
    text_frame_fg: crossterm::style::Color,
    text_frame_highlighted_bg: crossterm::style::Color,
    text_frame_highlighted_fg: crossterm::style::Color,
    text_frame_selection_bg: crossterm::style::Color,
    text_frame_selection_fg: crossterm::style::Color,
    text_frame_found_text_bg: crossterm::style::Color,
    text_frame_found_text_fg: crossterm::style::Color,

    status_frame_bg: crossterm::style::Color,
    status_frame_fg: crossterm::style::Color,

    error_red: crossterm::style::Color,
}

pub(crate) enum HighlightingThreadMessage {
    Start(String),
    Idle,
    Terminate,
}

#[derive(Clone)]
pub(crate) struct Highlighting {
    start_row: usize,
    end_row: usize,
    should_send_text_to_highlighting_thread: bool,
}

#[derive(Clone)]
pub(crate) struct Rendering {
    is_rendering: bool,
    should_render: bool,
    buffer: Vec<Vec<Character>>,
}

#[derive(Clone)]
pub(crate) struct TaskFeedback {
    bg: crossterm::style::Color,
    fg: crossterm::style::Color,
    text: String,
    default_text: String,
}

#[derive(Clone)]
pub(crate) struct CursorRange {
    start: Cursor,
    end: Cursor,
}

#[derive(Clone)]
pub(crate) struct Find {
    is_finding: bool,
    find_string: String,
    find_error: String,
    found_instances: Vec<CursorRange>,
    selected_instance_index: usize,
    keybindings_backup: std::collections::HashMap<String, fn(&mut Lino) -> ()>,
}

#[derive(Clone)]
pub(crate) struct Replace {
    replace_string: String,
}

#[derive(Clone)]
pub struct Lino {
    lines: Vec<Vec<Character>>,
    input_char_buf: Option<char>,
    saved_text: String,
    term_width: usize,
    term_height: usize,
    cursor: Cursor,
    last_cursor_col: usize,
    selection: Selection,
    text_frame: TextFrame,
    line_nums_frame: LineNumsFrame,
    status_frame: StatusFrame,
    task_feedback: TaskFeedback,
    should_exit: bool,
    undo_list: Vec<History>,
    redo_list: Vec<History>,
    file: FileData,
    clipboard: String,
    settings: Settings,
    error: Error,
    theming: Theming,
    highlighting: Highlighting,
    rendering: Rendering,
    keybindings: std::collections::HashMap<String, fn(&mut Lino) -> ()>,
    find: Find,
    replace: Replace,
}
