use std::io::{stdout, Write};
use crossterm;
extern crate copypasta;
use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;

static SPECIAL_CHARS: [char; 29] = 
    ['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', 
    '=', '+', '[', ']', '{', '}', ';', ':', '\'', ',', '.', '<', '>', 
    '/', '?', '\\', '|'];

#[derive(Copy, Clone)]
struct Character {
    background: crossterm::style::Color,
    foreground: crossterm::style::Color,
    character: char
}

#[derive(Copy, Clone)]
struct Cursor {
    row: usize,
    col: usize,
}

#[derive(Copy, Clone)]
struct TextFrame {
    width: usize,
    height: usize,
    start_row: usize,
    start_col: usize,
}

#[derive(Clone)]
struct LineNumsFrame {
    width: usize,
    height: usize,
    boundary_r: String,
}

#[derive(Copy, Clone)]
struct StatusFrame {
    width: usize,
    height: usize,
}

#[derive(Copy, Clone)]
struct Selection {
    is_selecting: bool,
    start_point: Cursor,
    end_point: Cursor,
}

#[derive(Clone)]
struct History {
    lines: Vec<Vec<Character>>,
    cursor: Cursor,
    selection: Selection,
}

#[derive(Clone)]
pub struct Lino {
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
}

impl Lino {
    pub fn new(characters: &Vec<char>) -> crossterm::Result<Lino> {
        let mut lines = vec![vec![]];
        for character in characters {
            if character == &'\r' {
                continue;
            }
            if character == &'\n' {
                lines.push(vec![]);
            } else {
                lines.last_mut().unwrap().push(Character{
                    background: crossterm::style::Color::Black,
                    foreground: crossterm::style::Color::White,
                    character: character.clone(),
                })
            }
        }

        let (term_width, term_height) = crossterm::terminal::size()?;
        
        let status_frame_width = term_height;
        let status_frame_height = 1;

        let line_nums_frame_boundary_r = String::from(" | ");
        let line_nums_frame_width = lines.len().to_string().len() + line_nums_frame_boundary_r.len();
        let line_nums_frame_height = term_height - status_frame_height;

        let text_frame_width = term_width as usize - line_nums_frame_width;
        let text_frame_height = term_height - status_frame_height;

        let mut lino = Lino {
            lines: lines,
            term_width: term_width as usize,
            term_height: term_height as usize,
            cursor: Cursor{
                row: 0,
                col: 0
            },
            last_cursor_col: 0,
            selection: Selection{
                is_selecting: false,
                start_point: Cursor{
                    row: 0,
                    col: 0,
                },
                end_point: Cursor{
                    row: 0,
                    col: 0,
                },
            },
            text_frame: TextFrame{
                width: text_frame_width as usize,
                height: text_frame_height as usize,
                start_row: 0,
                start_col: 0,
            },
            line_nums_frame: LineNumsFrame{
                width: line_nums_frame_width as usize,
                height: line_nums_frame_height as usize,
                boundary_r: line_nums_frame_boundary_r,
            },
            status_frame: StatusFrame{
                width: status_frame_width as usize,
                height: status_frame_height as usize,
            },
            should_exit: false,
            is_rendering: false,
            undo_list: vec![],
            redo_list: vec![],
        };

        lino.undo_list.push(History{
            lines: lino.lines.clone(),
            cursor: lino.cursor.clone(),
            selection: lino.selection.clone(),
        });

        Ok(lino)
    }

    pub fn run(&mut self) -> crossterm::Result<()> {


        ctrlc::set_handler(|| ()).expect("Error setting Ctrl-C handler");

        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(stdout(), crossterm::terminal::EnterAlternateScreen)?;
        
        self.initiate_input_event_loop()?;
        
        crossterm::execute!(stdout(), crossterm::terminal::LeaveAlternateScreen)?;
        crossterm::terminal::disable_raw_mode()?;
        
        Ok(())
    }

    fn initiate_input_event_loop(&mut self) -> crossterm::Result<()> {
        self.render()?;

        loop {
            if self.is_rendering { continue; }
            
            // `read()` blocks until an `Event` is available
            match crossterm::event::read()? {
                crossterm::event::Event::Key(event) => {
                    self.handle_key_event(&event)?;
                    self.render()?;
                },
                crossterm::event::Event::Mouse(_event) => (),
                crossterm::event::Event::Resize(width, height) => {
                    self.term_width = width as usize;
                    self.term_height = height as usize;
                    self.render()?;
                },
            }
            
            if self.should_exit { break; }
        }

        Ok(())
    }

    fn handle_key_event(&mut self, event: &crossterm::event::KeyEvent) -> crossterm::Result<()>{
        let mut should_input_character = false;
        let mut character_input: Option<char> = None;
        let mut should_quit_editor = false;
        let mut should_input_tab = false;
        let mut should_enter_newline = false;
        let mut should_perform_backspace = false;
        let mut should_perform_delete = false;
        let mut should_goto_line_start = false;
        let mut should_goto_line_end = false;
        let mut should_scroll_up = false;
        let mut should_scroll_down = false;
        let mut should_move_cursor_left = false;
        let mut should_move_cursor_left_by_word = false;
        let mut should_move_cursor_right = false;
        let mut should_move_cursor_right_by_word = false;
        let mut should_move_cursor_up = false;
        let mut should_move_cursor_down = false;
        let mut should_clear_selection = false;
        let mut should_make_selection = false;
        let mut should_select_all = false;
        let mut should_delete_selected = false;
        let previous_cursor = Cursor{
            row: self.cursor.row,
            col: self.cursor.col,
        };
        let mut should_perform_copy = false;
        let mut should_perform_cut = false;
        let mut should_perform_paste = false;
        let mut should_perform_undo = false;
        let mut should_perform_redo = false;
        let mut should_save_to_history = false;

        match event.code {
            crossterm::event::KeyCode::Char(c) => {
                if event.modifiers == crossterm::event::KeyModifiers::SHIFT
                || event.modifiers == crossterm::event::KeyModifiers::NONE {
                    should_input_character = true;
                    character_input = Some(c);
                    should_delete_selected = true;
                    
                    if c == ' ' {
                        should_save_to_history = true;
                    }
                }

                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'q' || c == 'Q') {
                    should_quit_editor = true;
                }

                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'a' || c == 'A') {
                    should_select_all = true;
                    should_save_to_history = true;
                }

                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'c' || c == 'C') {
                    should_perform_copy = true;
                }
                
                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'x' || c == 'X') {
                    should_perform_cut = true;
                    should_delete_selected = true;
                    should_save_to_history = true;
                }
                
                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'v' || c == 'V') {
                    should_delete_selected = true;
                    should_perform_paste = true;
                    should_save_to_history = true;
                }
                
                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'z' || c == 'Z') {
                    should_perform_undo = true;
                }
                
                if event.modifiers == (crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT) && (c == 'z' || c == 'Z') {
                    should_perform_redo = true;
                }
                
                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'y' || c == 'Y') {
                    should_perform_redo = true;
                }
            },
            crossterm::event::KeyCode::Tab => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    should_input_tab = true;
                    should_delete_selected = true;
                    should_save_to_history = true;
                }
            },
            crossterm::event::KeyCode::Enter => {
                should_enter_newline = true;
                should_delete_selected = true;
                should_save_to_history = true;
            },
            crossterm::event::KeyCode::Backspace => {
                if !self.selection.is_selecting {
                    should_perform_backspace = true;
                }
                should_delete_selected = true;
                should_save_to_history = true;
            },
            crossterm::event::KeyCode::Delete => {
                if !self.selection.is_selecting {
                    should_perform_delete = true;
                }
                should_delete_selected = true;
                should_save_to_history = true;
            },
            crossterm::event::KeyCode::Home => {
                should_goto_line_start = true;

                if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    should_make_selection = true;
                } else {
                    should_clear_selection = true;
                }
            },
            crossterm::event::KeyCode::End => {
                should_goto_line_end = true;

                if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    should_make_selection = true;
                } else {
                    should_clear_selection = true;
                }
            },
            crossterm::event::KeyCode::PageUp => {
                should_scroll_up = true;

                if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    should_make_selection = true;
                } else {
                    should_clear_selection = true;
                }
            },
            crossterm::event::KeyCode::PageDown => {
                should_scroll_down = true;

                if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    should_make_selection = true;
                } else {
                    should_clear_selection = true;
                }
            },
            crossterm::event::KeyCode::Left => {
                should_move_cursor_left = true;

                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                || event.modifiers == crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT {
                    should_move_cursor_left = false;
                    should_move_cursor_left_by_word = true;
                }

                if event.modifiers == crossterm::event::KeyModifiers::SHIFT
                || event.modifiers == crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT {
                    should_make_selection = true;
                } else {
                    should_clear_selection = true;
                }
            },
            crossterm::event::KeyCode::Right => {
                should_move_cursor_right = true;
                
                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                || event.modifiers == crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT {
                    should_move_cursor_right = false;
                    should_move_cursor_right_by_word = true;
                }

                if event.modifiers == crossterm::event::KeyModifiers::SHIFT
                || event.modifiers == crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT {
                    should_make_selection = true;
                } else {
                    should_clear_selection = true;
                }
            },
            crossterm::event::KeyCode::Up => {
                should_move_cursor_up = true;
                
                if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    should_make_selection = true;
                } else {
                    should_clear_selection = true;
                }
            },
            crossterm::event::KeyCode::Down => {
                should_move_cursor_down = true;

                if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    should_make_selection = true;
                } else {
                    should_clear_selection = true;
                }
            },
            crossterm::event::KeyCode::Esc => {
                should_clear_selection = true;
                should_save_to_history = true;
            },
            _ => ()
        }

        // ordering is importtant here
        if should_save_to_history { self.save_to_history(); }
        if should_perform_cut { self.perform_copy(); }
        if should_delete_selected { self.delete_selected(); }
        if should_input_character { self.input_character(character_input.unwrap()); }
        if should_quit_editor { self.quit_editor(); }
        if should_input_tab { self.input_tab(); }
        if should_enter_newline { self.enter_newline(); }
        if should_perform_backspace { self.perform_backspace(); }
        if should_perform_delete { self.perform_delete(); }
        if should_goto_line_start { self.goto_line_start(); }
        if should_goto_line_end { self.goto_line_end(); }
        if should_scroll_up { self.scroll_up(); }
        if should_scroll_down { self.scroll_down(); }
        if should_move_cursor_left { self.move_cursor_left(); }
        if should_move_cursor_left_by_word { self.move_cursor_left_by_word(); }
        if should_move_cursor_right { self.move_cursor_right(); }
        if should_move_cursor_right_by_word { self.move_cursor_right_by_word(); }
        if should_move_cursor_up { self.move_cursor_up(); }
        if should_move_cursor_down { self.move_cursor_down(); }
        if should_clear_selection { self.clear_selection(); }
        if should_make_selection { self.make_selection(&previous_cursor); }
        if should_select_all { self.select_all(); }
        if should_perform_copy { self.perform_copy(); }
        if should_perform_paste { self.perform_paste(); }
        if should_perform_undo { self.perform_undo(); }
        if should_perform_redo { self.perform_redo(); }

        Ok(())
    }

    fn quit_editor(&mut self) {
        self.should_exit = true;
    }
    
    fn input_character(&mut self, character: char) {
        if character == '\r' { return; }
        if character == '\n' { self.enter_newline(); return; }
        if character == '\t' { self.input_tab(); return; }

        self.lines[self.cursor.row].insert(
            self.cursor.col,
            Character{
                background: crossterm::style::Color::Black,
                foreground: crossterm::style::Color::White,
                character: character,
            });
        
            self.cursor.col += 1;
    }

    fn input_tab(&mut self) {
        let tab_width = self.calculate_tab_width().unwrap();
                    
        for _i in 0..tab_width {
            self.lines[self.cursor.row].insert(
                self.cursor.col,
                Character{
                    background: crossterm::style::Color::Black,
                    foreground: crossterm::style::Color::White,
                    character: ' ',
                });
            self.cursor.col += 1;
        }
    }

    fn enter_newline(&mut self) {
        let is_cursor_at_line_end = 
            self.cursor.col == self.lines[self.cursor.row].len();
        let is_cursor_mid_line_or_start = 
            self.cursor.col < self.lines[self.cursor.row].len();
        
        if is_cursor_at_line_end {
            self.cursor.row += 1;
            self.lines.insert(self.cursor.row, vec![]);
            self.cursor.col = 0;
            return;
        }
        
        if is_cursor_mid_line_or_start {
            let rest_of_the_line = self.lines[self.cursor.row].split_off(self.cursor.col);
            self.cursor.row += 1;
            self.lines.insert(self.cursor.row, rest_of_the_line);
            self.cursor.col = 0;
            return;
        }
    }

    fn perform_backspace(&mut self) {
        let is_first_line = self.cursor.row == 0;
        let is_current_line_empty = self.lines[self.cursor.row].is_empty();
        let is_cursor_at_line_start = !is_current_line_empty && self.cursor.col == 0;
        let is_cursor_mid_line_or_end = !is_cursor_at_line_start
            && self.cursor.col <= self.lines[self.cursor.row].len();
        
        if is_first_line && is_current_line_empty {
            return;
        }

        if !is_first_line && is_current_line_empty {
            self.lines.remove(self.cursor.row);
            self.cursor.row -= 1;
            self.cursor.col = self.lines[self.cursor.row].len();
            return;
        }

        if !is_first_line && is_cursor_at_line_start {
            let mut removed_line = self.lines.remove(self.cursor.row);
            let removed_line_len = removed_line.len();
            self.cursor.row -= 1;
            self.lines[self.cursor.row].append(&mut removed_line);
            self.cursor.col = self.lines[self.cursor.row].len() - removed_line_len;
            return;
        }

        if is_cursor_mid_line_or_end {
            self.cursor.col -= 1;
            self.lines[self.cursor.row].remove(self.cursor.col);
            return;
        }
    }

    fn perform_delete(&mut self) {
        let is_last_line = self.cursor.row == self.lines.len() - 1;
        let is_current_line_empty = self.lines[self.cursor.row].is_empty();
        let is_cursor_at_line_end = !is_current_line_empty 
            && self.cursor.col == self.lines[self.cursor.row].len();
        let is_cursor_mid_line_or_start = !is_current_line_empty 
            && self.cursor.col < self.lines[self.cursor.row].len();

        if is_last_line && is_current_line_empty {
            return;
        }

        if !is_last_line && is_current_line_empty {
            self.lines.remove(self.cursor.row);
            return;
        }

        if !is_last_line && is_cursor_at_line_end {
            let mut removed_line = self.lines.remove(self.cursor.row+1);
            let removed_line_len = removed_line.len();
            self.lines[self.cursor.row].append(&mut removed_line);
            self.cursor.col = self.lines[self.cursor.row].len() - removed_line_len;
            return;
        }

        if is_cursor_mid_line_or_start {
            self.lines[self.cursor.row].remove(self.cursor.col);
            return;
        }
    }

    fn goto_line_start(&mut self) {
        self.cursor.col = 0;
        self.last_cursor_col = self.cursor.col;
    }

    fn goto_line_end(&mut self) {
        self.cursor.col = self.lines[self.cursor.row].len();
        self.last_cursor_col = self.cursor.col;
    }

    fn scroll_up(&mut self) {
        if self.cursor.row as isize - self.text_frame.height as isize > 0 {
            self.cursor.row = self.cursor.row - self.text_frame.height;
        } else {
            self.cursor.row = 0;
        }
        
        if self.cursor.col > self.lines[self.cursor.row].len() {
            self.cursor.col = self.lines[self.cursor.row].len();
        }

        self.restore_last_cursor_col_if_applicable();
    }

    fn scroll_down(&mut self) {
        if self.cursor.row as isize + self.text_frame.height as isize <= (self.lines.len() - 1) as isize {
            self.cursor.row = self.cursor.row + self.text_frame.height;
        } else {
            self.cursor.row = self.lines.len() - 1;
        }

        if self.cursor.col > self.lines[self.cursor.row].len() {
            self.cursor.col = self.lines[self.cursor.row].len();
        }

        self.restore_last_cursor_col_if_applicable();
    }

    fn move_cursor_left(&mut self) {
        let is_first_line = self.cursor.row == 0;
        let is_cursor_at_line_start = self.cursor.col == 0;
        let is_cursor_mid_line_or_end = !is_cursor_at_line_start
            && self.cursor.col <= self.lines[self.cursor.row].len();

        if is_first_line && is_cursor_at_line_start {
            return;
        }

        if !is_first_line && is_cursor_at_line_start {
            self.cursor.row -= 1;
            self.cursor.col = self.lines[self.cursor.row].len();
            self.last_cursor_col = self.cursor.col;
            return;
        }

        if is_cursor_mid_line_or_end {
            self.cursor.col -= 1;
            self.last_cursor_col = self.cursor.col;
            return;
        }
    }

    fn move_cursor_left_by_word(&mut self) {
        let mut is_cursor_at_line_start = self.cursor.col == 0;
        if is_cursor_at_line_start {
            self.move_cursor_left();
            return;
        }

        let is_cursor_at_line_end = self.cursor.col == self.lines[self.cursor.row].len();
        if is_cursor_at_line_end {
            self.move_cursor_left();
        }
        
        let is_starting_char_a_space = self.lines[self.cursor.row][self.cursor.col].character == ' ';
        let is_starting_char_a_special_char = 
            SPECIAL_CHARS.contains(&self.lines[self.cursor.row][self.cursor.col].character);
        let is_starting_char_a_normal_char = 
            !is_starting_char_a_space && !is_starting_char_a_special_char;
            
        while !is_cursor_at_line_start {
            let is_current_char_a_space = self.lines[self.cursor.row][self.cursor.col].character == ' ';
            let is_current_char_a_special_char = 
                SPECIAL_CHARS.contains(&self.lines[self.cursor.row][self.cursor.col].character);
            let is_current_char_a_normal_char = 
                !is_current_char_a_space && !is_current_char_a_special_char;
            
            if (is_starting_char_a_space && !is_current_char_a_space)
            || (is_starting_char_a_special_char && !is_current_char_a_special_char)
            || (is_starting_char_a_normal_char && !is_current_char_a_normal_char) {
                break;
            }
            
            self.move_cursor_left();
            is_cursor_at_line_start = self.cursor.col == 0;
        }
    }

    fn move_cursor_right(&mut self) {
        let is_last_line = self.cursor.row == self.lines.len() - 1;
        let is_cursor_at_line_end = self.cursor.col == self.lines[self.cursor.row].len();
        let is_cursor_mid_line_or_start = self.cursor.col < self.lines[self.cursor.row].len();
        
        if is_last_line && is_cursor_at_line_end {
            return;
        }
        
        if !is_last_line && is_cursor_at_line_end {
            self.cursor.row += 1;
            self.cursor.col = 0;
            self.last_cursor_col = self.cursor.col;
            return;
        }
        
        if is_cursor_mid_line_or_start {
            self.cursor.col += 1;
            self.last_cursor_col = self.cursor.col;
            return;
        }
    }

    fn move_cursor_right_by_word(&mut self) {
        let mut is_cursor_at_line_end = self.cursor.col == self.lines[self.cursor.row].len();
        if is_cursor_at_line_end {
            self.move_cursor_right();
            return;
        }
        
        let is_starting_char_a_space = self.lines[self.cursor.row][self.cursor.col].character == ' ';
        let is_starting_char_a_special_char = 
            SPECIAL_CHARS.contains(&self.lines[self.cursor.row][self.cursor.col].character);
        let is_starting_char_a_normal_char = 
            !is_starting_char_a_space && !is_starting_char_a_special_char;
        
        while !is_cursor_at_line_end {
            let is_current_char_a_space = self.lines[self.cursor.row][self.cursor.col].character == ' ';
            let is_current_char_a_special_char = 
                SPECIAL_CHARS.contains(&self.lines[self.cursor.row][self.cursor.col].character);
            let is_current_char_a_normal_char = 
                !is_current_char_a_space && !is_current_char_a_special_char;
            
            if (is_starting_char_a_space && !is_current_char_a_space)
            || (is_starting_char_a_special_char && !is_current_char_a_special_char)
            || (is_starting_char_a_normal_char && !is_current_char_a_normal_char) {
                break;
            }
            
            self.move_cursor_right();
            is_cursor_at_line_end = self.cursor.col == self.lines[self.cursor.row].len();
        }
    }

    fn move_cursor_up(&mut self) {
        let is_first_line = self.cursor.row == 0;
                
        if !is_first_line {
            self.cursor.row -= 1;

            let is_cursor_after_line_end = self.cursor.col > self.lines[self.cursor.row].len();

            if is_cursor_after_line_end {
                self.cursor.col = self.lines[self.cursor.row].len();
            }
        }
        
        self.restore_last_cursor_col_if_applicable();
    }

    fn move_cursor_down(&mut self) {
        let is_last_line = self.cursor.row == self.lines.len() - 1;
        
        if !is_last_line {
            self.cursor.row += 1;

            let is_cursor_after_line_end = self.cursor.col > self.lines[self.cursor.row].len();
            
            if is_cursor_after_line_end {
                self.cursor.col = self.lines[self.cursor.row].len();
            }
        }
        
        self.restore_last_cursor_col_if_applicable();
    }

    fn make_selection(&mut self, previous_cursor: &Cursor) {
        if !self.selection.is_selecting {
            self.selection.is_selecting = true;
            self.selection.start_point.row = previous_cursor.row;
            self.selection.start_point.col = previous_cursor.col;

            let is_selecting_backward = 
                (self.cursor.row < previous_cursor.row 
                || self.cursor.col < previous_cursor.col)
                && self.selection.start_point.col > 0;
            if is_selecting_backward {
                self.selection.start_point.col -= 1;
            }
        }

        self.selection.is_selecting = true;
        self.selection.end_point.row = self.cursor.row;
        self.selection.end_point.col = self.cursor.col;

        let is_selecting_forward = 
            (self.selection.end_point.row > self.selection.start_point.row 
            || self.selection.end_point.col > self.selection.start_point.col)
            && self.selection.end_point.col > 0;
        if is_selecting_forward {
            self.selection.end_point.col -= 1;
        }
        
        let is_selecting_backward = 
            (self.selection.end_point.row < self.selection.start_point.row 
            || self.selection.end_point.col < self.selection.start_point.col)
            && (self.selection.end_point.col as isize) < 
                (self.lines[self.selection.end_point.row].len() as isize) - 1
            && self.selection.end_point.col != 0;
        if is_selecting_backward {
            self.selection.end_point.col += 1;
        }
        
        let is_start_point_after_line_end = 
            self.selection.start_point.col == self.lines[self.selection.start_point.row].len()
            && self.selection.start_point.col > 0;
        if is_start_point_after_line_end {
            self.selection.start_point.col -= 1;
        }

        let is_end_point_after_line_end = 
            self.selection.end_point.col == self.lines[self.selection.end_point.row].len()
            && self.selection.end_point.col > 0;
        if is_end_point_after_line_end {
            self.selection.end_point.col -= 1;
        }
    }

    fn clear_selection(&mut self) {
        self.selection.is_selecting = false;
        self.selection.start_point.row = self.cursor.row;
        self.selection.start_point.col = self.cursor.col;
        self.selection.end_point.row = self.cursor.row;
        self.selection.end_point.col = self.cursor.col;
    }

    fn delete_selected(&mut self) {
        if !self.selection.is_selecting { return; }
        
        let selection = self.get_sorted_selection_points();
        if selection.is_none() { return; }
        let selection = selection.unwrap();

        self.cursor.row = selection.end_point.row;
        self.cursor.col = selection.end_point.col + 1;

        loop {
            self.perform_backspace();
            
            if self.cursor.row == selection.start_point.row
            && self.cursor.col == selection.start_point.col {
                break;
            }
        }

        self.cursor.row = selection.start_point.row;
        self.cursor.col = selection.start_point.col;

        self.clear_selection();
    }

    fn select_all(&mut self) {
        let is_document_empty = 
            self.lines.len() == 1 && self.lines[0].len() == 0;

        if is_document_empty {
            self.clear_selection();
            return;
        }
        
        self.selection.is_selecting = true;
        self.selection.start_point.row = 0;
        self.selection.start_point.col = 0;

        if self.lines.len() > 0 {
            self.selection.end_point.row = self.lines.len() - 1;
        } else {
            self.selection.end_point.row = self.lines.len();
        }

        self.selection.end_point.col = 0;
        if self.lines[self.selection.end_point.row].len() > 0 {
            self.selection.end_point.col = self.lines[self.selection.end_point.row].len() - 1;
        }
        self.cursor.row = self.selection.end_point.row;
        self.cursor.col = self.selection.end_point.col;
    }

    fn perform_copy(&mut self) {
        if !self.selection.is_selecting { return; }
        
        let selection = self.get_sorted_selection_points();
        if selection.is_none() { return; }
        let selection = selection.unwrap();
        let current_cursor_backup = Cursor{
            row: self.cursor.row,
            col: self.cursor.col,
        };
        let mut copied_string = String::new();

        self.cursor.row = selection.start_point.row;
        self.cursor.col = selection.start_point.col;

        loop {
            let is_cursor_at_line_end = self.cursor.col == self.lines[self.cursor.row].len();

            if is_cursor_at_line_end {
                copied_string.push('\n');
            } else {
                copied_string.push(self.lines[self.cursor.row][self.cursor.col].character);
            }

            self.move_cursor_right();
            
            if self.cursor.row == selection.end_point.row
            && self.cursor.col > selection.end_point.col {
                break;
            }
        }

        self.cursor.row = current_cursor_backup.row;
        self.cursor.col = current_cursor_backup.col;

        let mut clipboard_ctx = ClipboardContext::new().unwrap();
        clipboard_ctx.set_contents(copied_string).unwrap();
    }

    fn perform_paste(&mut self) {
        let mut clipboard_ctx = ClipboardContext::new().unwrap();
        let copied_string = clipboard_ctx.get_contents().unwrap();

        for c in copied_string.chars() {
            if c == '\n' {
                self.enter_newline();
            } else {
                self.input_character(c);
            }
        }
    }

    fn perform_undo(&mut self) {
        let last_iteration = self.undo_list.pop();
        if last_iteration.is_none() {
            return;
        }
        let last_iteration = last_iteration.unwrap();

        self.redo_list.push(History{
            lines: self.lines.clone(),
            cursor: self.cursor.clone(),
            selection: self.selection.clone(),
        });
        
        self.lines = last_iteration.lines.clone();
        self.cursor = last_iteration.cursor.clone();
        self.selection = last_iteration.selection.clone();
    }

    fn perform_redo(&mut self) {
        let last_iteration = self.redo_list.pop();
        if last_iteration.is_none() {
            return;
        }
        let last_iteration = last_iteration.unwrap();

        self.undo_list.push(History{
            lines: self.lines.clone(),
            cursor: self.cursor.clone(),
            selection: self.selection.clone(),
        });
        
        self.lines = last_iteration.lines.clone();
        self.cursor = last_iteration.cursor.clone();
        self.selection = last_iteration.selection.clone();
    }

    fn save_to_history(&mut self) {
        self.undo_list.push(History{
            lines: self.lines.clone(),
            cursor: self.cursor.clone(),
            selection: self.selection.clone(),
        });
        self.redo_list.clear();
    }

    fn restore_last_cursor_col_if_applicable(&mut self) {
        if self.last_cursor_col <= self.lines[self.cursor.row].len() {
            self.cursor.col = self.last_cursor_col;
        }
    }

    fn get_sorted_selection_points(&self) -> Option<Selection> {
        if !self.selection.is_selecting {
            return None;
        }

        let start_point_as_smaller = Some(Selection{
            is_selecting: self.selection.is_selecting,
            start_point: Cursor{
                row: self.selection.start_point.row,
                col: self.selection.start_point.col,
            },
            end_point: Cursor{
                row: self.selection.end_point.row,
                col: self.selection.end_point.col,
            },
        });
        let end_point_as_smaller = Some(Selection{
            is_selecting: self.selection.is_selecting,
            start_point: Cursor{
                row: self.selection.end_point.row,
                col: self.selection.end_point.col,
            },
            end_point: Cursor{
                row: self.selection.start_point.row,
                col: self.selection.start_point.col,
            },
        });

        let is_start_point_up_from_end_point = 
            self.selection.start_point.row < self.selection.end_point.row;
        if is_start_point_up_from_end_point {
            return start_point_as_smaller;
        }

        let is_start_point_down_from_end_point =
            self.selection.start_point.row > self.selection.end_point.row;
        if is_start_point_down_from_end_point {
            return end_point_as_smaller;
        }
        
        let is_start_point_left_from_end_point = 
            self.selection.start_point.row == self.selection.end_point.row
            && self.selection.start_point.col <= self.selection.end_point.col;
        if is_start_point_left_from_end_point {
            return start_point_as_smaller;
        }
        
        let is_start_point_right_from_end_point = 
            self.selection.start_point.row == self.selection.end_point.row
            && self.selection.start_point.col > self.selection.end_point.col;
        if is_start_point_right_from_end_point {
            return end_point_as_smaller;
        }

        return None;
    }

    fn is_cursor_inside_selection(&self, selection: &Selection, cursor: &Cursor) -> bool {
        let is_single_line_selection_and_cursor_inside_points = 
            cursor.row == selection.start_point.row && cursor.row == selection.end_point.row
            && cursor.col >= selection.start_point.col && cursor.col <= selection.end_point.col;
        if is_single_line_selection_and_cursor_inside_points {
            return true;
        }

        let is_multi_line_selection_and_cursor_after_start_point_of_first_line = 
            cursor.row == selection.start_point.row && cursor.row < selection.end_point.row
            && cursor.col >= selection.start_point.col;
        if is_multi_line_selection_and_cursor_after_start_point_of_first_line {
            return true;
        }

        let is_multi_line_selection_and_cursor_before_end_point_of_last_line = 
            cursor.row > selection.start_point.row && cursor.row == selection.end_point.row
            && cursor.col <= selection.end_point.col;
        if is_multi_line_selection_and_cursor_before_end_point_of_last_line {
            return true;
        }

        let is_multi_line_selection_and_cursor_inside_points = 
            cursor.row > selection.start_point.row && cursor.row < selection.end_point.row;
        if is_multi_line_selection_and_cursor_inside_points {
            return true;
        }
        
        false
    }
    
    fn calculate_tab_width(&self) -> crossterm::Result<usize> {
        let (old_cursor_col, _old_cursor_row) = crossterm::cursor::position()?;
        
        crossterm::execute!(
            stdout(),
            crossterm::cursor::SavePosition,
            crossterm::cursor::Hide,
            crossterm::style::Print('\t'),
            crossterm::style::ResetColor
        )?;
        
        let (new_cursor_col, _new_cursor_row) = crossterm::cursor::position()?;
        
        crossterm::execute!(
            stdout(),
            crossterm::cursor::RestorePosition,
            crossterm::cursor::Show,
        )?;
        
        Ok((new_cursor_col - old_cursor_col) as usize)
    }

    fn render(&mut self) -> crossterm::Result<()> {
        self.is_rendering = true;
        self.update_line_nums_frame();
        self.update_text_frame();
        self.update_status_frame();

        crossterm::queue!(
            stdout(),
            crossterm::style::SetBackgroundColor(crossterm::style::Color::White),
            crossterm::style::SetForegroundColor(crossterm::style::Color::White),
            crossterm::style::Print(' '),
            crossterm::cursor::Hide,
            crossterm::cursor::MoveTo(0, 0),
        )?;

        self.render_line_nums_frame_content()?;
        self.render_text_frame_content()?;
        self.render_status_frame_content()?;
        self.update_visible_cursor()?;

        stdout().flush()?;
        
        self.is_rendering = false;
        Ok(())
    }

    fn update_line_nums_frame(&mut self) {
        let mut should_update_text_frame = false;
        
        if self.text_frame.start_col > 0 && self.line_nums_frame.boundary_r != String::from(" |") {
            self.line_nums_frame.boundary_r = String::from(" |");
            should_update_text_frame = true;
        } else if self.text_frame.start_col == 0 && self.line_nums_frame.boundary_r != String::from(" | ") {
            self.line_nums_frame.boundary_r = String::from(" | ");
            should_update_text_frame = true;
        }
        
        self.line_nums_frame.width = self.lines.len().to_string().len() + self.line_nums_frame.boundary_r.len();
        self.line_nums_frame.height = self.term_height - self.status_frame.height;
        
        if should_update_text_frame {
            self.update_text_frame();
        }
    }

    fn update_text_frame(&mut self) {
        self.text_frame.width = self.term_width - self.line_nums_frame.width;
        self.text_frame.height = self.term_height - self.status_frame.height;

        let is_cursor_up_from_frame = self.cursor.row < self.text_frame.start_row;
        let is_cursor_down_from_frame = self.cursor.row > self.text_frame.start_row + self.text_frame.height - 1;

        if is_cursor_up_from_frame {
            while self.text_frame.start_row > self.cursor.row {
                self.text_frame.start_row -= 1;
            }
        }
        if is_cursor_down_from_frame {
            while self.text_frame.start_row + self.text_frame.height - 1 < self.cursor.row {
                self.text_frame.start_row += 1;
            }
        }

        let is_cursor_left_from_frame = self.cursor.col < self.text_frame.start_col;
        let is_cursor_right_from_frame = self.cursor.col > self.text_frame.start_col + self.text_frame.width - 2;

        if is_cursor_left_from_frame {
            while self.text_frame.start_col > self.cursor.col {
                self.text_frame.start_col -= 1;
            }
        }
        if is_cursor_right_from_frame {
            while self.text_frame.start_col + self.text_frame.width - 2 < self.cursor.col {
                self.text_frame.start_col += 1;
            }
        }

        self.update_line_nums_frame();
    }

    fn update_status_frame(&mut self) {
        self.status_frame.width = self.term_width;
        self.status_frame.height = 1;
    }

    fn render_line_nums_frame_content(&mut self) -> crossterm::Result<()> {
        crossterm::queue!(
            stdout(),
            crossterm::style::SetBackgroundColor(crossterm::style::Color::Black),
            crossterm::style::SetForegroundColor(crossterm::style::Color::DarkGrey),
        )?;

        let visible_frame_starting_line_num = self.text_frame.start_row;
        let mut visible_frame_ending_line_num = visible_frame_starting_line_num + self.text_frame.height;
        
        if visible_frame_ending_line_num > self.lines.len() {
            visible_frame_ending_line_num = self.lines.len();
        }
        
        let line_num_width = self.lines.len().to_string().len();
        
        for i in visible_frame_starting_line_num..visible_frame_ending_line_num {
            let rendered_lines_row = (i - visible_frame_starting_line_num) as usize;
            let mut rendered_lines_col = 
                (self.term_width - self.text_frame.width - self.line_nums_frame.width) as usize;

            let num_string = format!("{:width$}", i + 1, width = line_num_width);
            let num_string = String::from(num_string) + &self.line_nums_frame.boundary_r;

            for line_num_char in num_string.chars() {
                crossterm::queue!(
                    stdout(),
                    crossterm::cursor::MoveTo(rendered_lines_col as u16, rendered_lines_row as u16),
                    crossterm::style::Print(line_num_char),
                )?;
                rendered_lines_col += 1;
            }
        }

        let remaining_lines_start_row = (visible_frame_ending_line_num - visible_frame_starting_line_num) as usize;
        let line_nums_frame_start_col = 
            (self.term_width - self.text_frame.width - self.line_nums_frame.width) as usize;
        let line_nums_frame_end_col = (line_nums_frame_start_col + self.line_nums_frame.width) as usize;

        for i in remaining_lines_start_row..self.line_nums_frame.height {
            for j in line_nums_frame_start_col..line_nums_frame_end_col {
                crossterm::queue!(
                    stdout(),
                    crossterm::cursor::MoveTo(j as u16, i as u16),
                    crossterm::style::Print(' '),
                )?;
            }
        }

        Ok(())
    }

    fn render_text_frame_content(&mut self) -> crossterm::Result<()> {
        let visible_frame_starting_row = self.text_frame.start_row;
        let mut visible_frame_ending_row = visible_frame_starting_row + self.text_frame.height;
        
        if visible_frame_ending_row > self.lines.len() {
            visible_frame_ending_row = self.lines.len();
        }

        let sorted_selection_points = self.get_sorted_selection_points();

        for i in visible_frame_starting_row..visible_frame_ending_row {
            crossterm::queue!(
                stdout(),
                crossterm::cursor::MoveTo(
                    (self.term_width - self.text_frame.width) as u16,
                    (i - visible_frame_starting_row) as u16),
            )?;
         
            let mut visible_frame_starting_col = self.text_frame.start_col;
            let mut visible_frame_ending_col = visible_frame_starting_col + self.text_frame.width;
            
            if visible_frame_starting_col > self.lines[i].len() {
                visible_frame_starting_col = self.lines[i].len();
            }
            
            if visible_frame_ending_col > self.lines[i].len() {
                visible_frame_ending_col = self.lines[i].len();
            }

            let rendered_lines_row = (i - visible_frame_starting_row) as usize;
            let mut rendered_lines_col = (self.term_width - self.text_frame.width) as usize;
         
            let mut prev_background = crossterm::style::Color::Black;
            let mut prev_foreground = crossterm::style::Color::DarkGrey;

            for j in visible_frame_starting_col..visible_frame_ending_col {
                let mut background = self.lines[i][j].background;
                let mut foreground = self.lines[i][j].foreground;

                match &sorted_selection_points {
                    Some(selection) => {
                        if self.is_cursor_inside_selection(&selection, &Cursor{row: i, col: j}) {
                            background = crossterm::style::Color::White;
                            foreground = crossterm::style::Color::Black;
                        }
                    },
                    _ => ()
                }

                if prev_background != background {
                    crossterm::queue!(
                        stdout(),
                        crossterm::style::SetBackgroundColor(background),
                    )?;
                    prev_background = background;
                }
                
                if prev_foreground != foreground {
                    crossterm::queue!(
                        stdout(),
                        crossterm::style::SetForegroundColor(foreground),
                    )?;
                    prev_foreground = foreground;
                }

                crossterm::queue!(
                    stdout(),
                    crossterm::cursor::MoveTo(rendered_lines_col as u16, rendered_lines_row as u16),
                    crossterm::style::Print(self.lines[i][j].character),
                )?;
                rendered_lines_col += 1;
            }
            
            let text_frame_start_col = (self.term_width - self.text_frame.width) as usize;
            let remaining_chars_start_col = text_frame_start_col
                + (visible_frame_ending_col - visible_frame_starting_col) as usize;
            let text_frame_end_col = (text_frame_start_col + self.text_frame.width) as usize;

            crossterm::queue!(
                stdout(),
                crossterm::style::SetBackgroundColor(crossterm::style::Color::Black),
                crossterm::style::SetForegroundColor(crossterm::style::Color::DarkGrey),
            )?;
            for j in remaining_chars_start_col..text_frame_end_col {
                crossterm::queue!(
                    stdout(),
                    crossterm::cursor::MoveTo(j as u16, rendered_lines_row as u16),
                    crossterm::style::Print(' '),
                )?;
            }
        }

        let remaining_lines_start_row = (visible_frame_ending_row - visible_frame_starting_row) as usize;
        let text_frame_start_col = (self.term_width - self.text_frame.width) as usize;
        let text_frame_end_col = (text_frame_start_col + self.text_frame.width) as usize;

        crossterm::queue!(
            stdout(),
            crossterm::style::SetBackgroundColor(crossterm::style::Color::Black),
            crossterm::style::SetForegroundColor(crossterm::style::Color::DarkGrey),
        )?;

        for i in remaining_lines_start_row..self.text_frame.height {
            for j in text_frame_start_col..text_frame_end_col {
                crossterm::queue!(
                    stdout(),
                    crossterm::cursor::MoveTo(j as u16, i as u16),
                    crossterm::style::Print(' '),
                )?;
            }
        }

        Ok(())
    }

    fn render_status_frame_content(&mut self) -> crossterm::Result<()> {
        crossterm::queue!(
            stdout(),
            crossterm::style::SetBackgroundColor(crossterm::style::Color::White),
            crossterm::style::SetForegroundColor(crossterm::style::Color::Black),
        )?;
        
        for i in 0..self.status_frame.width {
            crossterm::queue!(
                stdout(),
                crossterm::cursor::MoveTo(i as u16, (self.term_height - 1) as u16),
                crossterm::style::Print(' '),
            )?;
        }
        let status_string = 
            String::from("Ln:") + &(self.cursor.row + 1).to_string()
            + &String::from(",Col:") + &(self.cursor.col + 1).to_string();
        
        crossterm::queue!(
            stdout(),
            crossterm::cursor::MoveTo(0, (self.term_height - 1) as u16),
            crossterm::style::SetBackgroundColor(crossterm::style::Color::White),
            crossterm::style::SetForegroundColor(crossterm::style::Color::Black),
            crossterm::style::Print(status_string),
        )?;

        Ok(())
    }

    fn update_visible_cursor(&mut self) -> crossterm::Result<()> {
        let col = (self.cursor.col - self.text_frame.start_col) + (self.line_nums_frame.width);
        let row = self.cursor.row - self.text_frame.start_row;

        // let mut cursor_char = ' ';

        // if self.cursor.col < self.lines[self.cursor.row].len() {
        //     cursor_char = self.lines[self.cursor.row][self.cursor.col].character;
        // }

        crossterm::queue!(
            stdout(),
            crossterm::cursor::MoveTo(col as u16, row as u16),
            // crossterm::style::SetBackgroundColor(crossterm::style::Color::Black),
            // crossterm::style::SetForegroundColor(crossterm::style::Color::White),
            // crossterm::style::SetAttribute(crossterm::style::Attribute::Reverse),
            // crossterm::style::Print(cursor_char),
            crossterm::cursor::Show,
            // crossterm::style::SetAttribute(crossterm::style::Attribute::Reset),
        )?;

        Ok(())
    }
}
