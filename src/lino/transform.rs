// use std::io::{stdout, Write};
use crossterm;
extern crate copypasta;
use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::*;

static SPECIAL_CHARS: [char; 29] = 
    ['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', 
    '=', '+', '[', ']', '{', '}', ';', ':', '\'', ',', '.', '<', '>', 
    '/', '?', '\\', '|'];

impl Lino {
    pub(crate) fn read_from_file(&mut self) {
        // Create a path to the desired file
        let path = Path::new(self.file.path.as_str());
        let display = path.display();

        if !path.is_file() {
            self.file.should_save_as = false;
            return;
        }

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut input_string = String::new();
        match file.read_to_string(&mut input_string) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => (),
        }

        for character in input_string.chars() {
            self.input_character(character);
        }
        self.reset_cursor();
        self.last_cursor_col = self.cursor.col;
        self.saved_text = Lino::convert_2d_text_to_string(&self.lines);
        self.file.should_save_as = false;
    }

    pub(crate) fn save_to_file(&mut self) {
        let path_str = &self.file.path;
        let path = Path::new(&path_str);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        
        let output_string = Lino::convert_2d_text_to_string(&self.lines);

        match file.write_all(output_string.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => (),
        }

        self.saved_text = Lino::convert_2d_text_to_string(&self.lines);
        self.file.is_saved = true;
        self.file.should_save_as = false;
    }

    pub(crate) fn set_file_unsaved_if_applicable(&mut self) {
        let current_text = Lino::convert_2d_text_to_string(&self.lines);
        // let saved_text_string = Lino::convert_2d_text_to_string(&self.saved_text);

        if current_text != self.saved_text {
            self.file.is_saved = false;
        } else {
            self.file.is_saved = true;
        }
    }

    pub(crate) fn perform_save(&mut self) {
        if self.file.path == "" || self.file.should_save_as {
            self.render_save_as_frame();
            self.handle_save_as_frame_input();
        } else {
            self.save_to_file();
        }
    }

    pub(crate) fn exit_from_editor(&mut self) {
        self.should_exit = true;
        self.initiate_exit_procedure();
    }

    pub(crate) fn initiate_exit_procedure(&mut self) {
        if self.file.is_saved {
            return;
        }

        self.render_unsaved_changes_frame();
        self.handle_unsaved_changes_frame_input();

        if self.file.should_save_as {
            self.render_save_as_frame();
            self.handle_save_as_frame_input();
        }
    }
    
    pub(crate) fn input_character(&mut self, character: char) {
        if character == '\r' { return; }
        if character == '\n' { self.enter_newline(); return; }
        if character == '\t' { self.input_tab(); return; }

        self.lines[self.cursor.row].insert(
            self.cursor.col,
            Character{
                background: self.theming.text_frame_bg,
                foreground: self.theming.text_frame_fg,
                character: character,
            }
        );
        
        self.cursor.col += 1;
        self.last_cursor_col = self.cursor.col;
    }

    pub(crate) fn input_tab(&mut self) {
        let tab_width = self.calculate_tab_width();
                    
        for _ in 0..tab_width {
            self.lines[self.cursor.row].insert(
                self.cursor.col,
                Character{
                    background: self.theming.text_frame_bg,
                    foreground: self.theming.text_frame_fg,
                    character: ' ',
                });
            self.cursor.col += 1;
        }
        self.last_cursor_col = self.cursor.col;
    }

    pub(crate) fn enter_newline(&mut self) {
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

    pub(crate) fn auto_indent_if_applicable(&mut self) {
        if self.is_cursor_at_first_line() {
            return;
        }

        for i in 0..self.lines[self.cursor.row - 1].len() {
            if self.lines[self.cursor.row - 1][i].character != ' ' {
                break;
            }

            self.input_character(' ');
        }
    }

    pub(crate) fn perform_backspace(&mut self) {
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

    pub(crate) fn perform_delete(&mut self) {
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

    pub(crate) fn goto_line_start(&mut self) {
        self.cursor.col = 0;
        self.last_cursor_col = self.cursor.col;
    }

    pub(crate) fn goto_line_end(&mut self) {
        self.cursor.col = self.lines[self.cursor.row].len();
        self.last_cursor_col = self.cursor.col;
    }

    pub(crate) fn scroll_up(&mut self) {
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

    pub(crate) fn scroll_down(&mut self) {
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

    pub(crate) fn reset_cursor(&mut self) {
        self.cursor.row = 0;
        self.cursor.col = 0;
        // self.last_cursor_col = 0;
    }

    pub(crate) fn move_cursor_left(&mut self) {
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

    pub(crate) fn move_cursor_right(&mut self) {
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

    pub(crate) fn move_cursor_left_by_word(&mut self) {
        if self.is_cursor_at_line_start() {
            self.move_cursor_left();
            return;
        }

        self.move_cursor_left();
        
        loop {
            if self.is_cursor_at_line_start() {
                break;
            }

            if self.lines[self.cursor.row][self.cursor.col].character != ' ' 
            && self.lines[self.cursor.row][self.cursor.col - 1].character == ' ' {
                break;
            }

            if !SPECIAL_CHARS.contains(&self.lines[self.cursor.row][self.cursor.col].character)
            && SPECIAL_CHARS.contains(&self.lines[self.cursor.row][self.cursor.col - 1].character) {
                break;
            }

            self.move_cursor_left();
        }
    }

    pub(crate) fn move_cursor_right_by_word(&mut self) {
        if self.is_cursor_at_line_end() {
            self.move_cursor_right();
            return;
        }

        self.move_cursor_right();
        
        loop {
            if self.is_cursor_at_line_end() {
                break;
            }

            if self.lines[self.cursor.row][self.cursor.col].character == ' ' 
            && self.lines[self.cursor.row][self.cursor.col - 1].character != ' ' {
                break;
            }

            if SPECIAL_CHARS.contains(&self.lines[self.cursor.row][self.cursor.col].character)
            && !SPECIAL_CHARS.contains(&self.lines[self.cursor.row][self.cursor.col - 1].character) {
                break;
            }
            
            self.move_cursor_right();
        }
    }

    pub(crate) fn delete_left_by_word(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.clear_selection(&previous_cursor);
        self.move_cursor_left_by_word();
        self.make_selection(&previous_cursor);
        self.delete_selected();
    }

    pub(crate) fn delete_right_by_word(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.clear_selection(&previous_cursor);
        self.move_cursor_right_by_word();
        self.make_selection(&previous_cursor);
        self.delete_selected();
    }

    pub(crate) fn move_cursor_up(&mut self) {
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

    pub(crate) fn move_cursor_down(&mut self) {
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

    pub(crate) fn increase_indentation(&mut self) {
        for _ in 0..self.settings.tab_width {
            self.lines[self.cursor.row].insert(0, Character{
                background: self.theming.text_frame_bg,
                foreground: self.theming.text_frame_fg,
                character: ' ',
            });
            self.move_cursor_right();
        }
    }

    pub(crate) fn decrease_indentation(&mut self) {
        if self.is_current_line_empty() {
            return;
        }

        if self.lines[self.cursor.row][0].character != ' ' {
            return;
        }

        let mut indent_width = self.settings.tab_width;
        if self.lines[self.cursor.row].len() < self.settings.tab_width {
            indent_width = self.lines[self.cursor.row].len();
        }

        for _ in 0..indent_width {
            if self.lines[self.cursor.row][0].character == ' ' {
                self.move_cursor_left();
                self.lines[self.cursor.row].remove(0);
            } else {
                break;
            }
        }
    }

    pub(crate) fn swap_line_upward(&mut self) {
        if self.is_cursor_at_first_line() {
            return;
        }

        let current_line = self.lines[self.cursor.row].clone();
        self.lines[self.cursor.row] = self.lines[self.cursor.row - 1].clone();
        self.lines[self.cursor.row - 1] = current_line;

        self.move_cursor_up();
    }

    pub(crate) fn swap_line_downward(&mut self) {
        if self.is_cursor_at_last_line() {
            return;
        }

        let current_line = self.lines[self.cursor.row].clone();
        self.lines[self.cursor.row] = self.lines[self.cursor.row + 1].clone();
        self.lines[self.cursor.row + 1] = current_line;

        self.move_cursor_down();
    }

    pub(crate) fn delete_current_line(&mut self) {
        if self.is_document_empty() {
            return;
        }

        let should_move_curor_up = self.is_cursor_at_last_line();

        self.lines.remove(self.cursor.row);

        if should_move_curor_up && self.lines.len() > 0 {
            self.move_cursor_up();
        } else if self.lines.len() < 1 {
            self.lines.push(vec![]);
            self.reset_cursor();
        }

        if self.cursor.col > self.lines[self.cursor.row].len() {
            self.cursor.col = self.lines[self.cursor.row].len();
        }
    }

    pub(crate) fn duplicate_line_upward(&mut self) {
        self.lines.insert(self.cursor.row, self.lines[self.cursor.row].clone());
    }

    pub(crate) fn duplicate_line_downward(&mut self) {
        self.lines.insert(self.cursor.row, self.lines[self.cursor.row].clone());
        self.move_cursor_down();
    }

    pub(crate) fn make_selection(&mut self, previous_cursor: &Cursor) {
        if self.is_document_empty() { 
            self.clear_selection(&self.cursor.clone());
            return;
        }

        if !self.selection.is_selected {
            self.selection.is_selected = true;
            self.selection.start_point = previous_cursor.clone();

            let is_selecting_backward = 
                self.is_cursor_lesser_than(&self.selection.start_point);
            if is_selecting_backward {
                let cursor_backup = self.cursor.clone();
                self.cursor = self.selection.start_point.clone();
                self.move_cursor_left();
                self.selection.start_point = self.cursor.clone();
                self.cursor = cursor_backup.clone();
            }
        }

        self.selection.is_selected = true;
        self.selection.end_point = self.cursor.clone();

        let is_selecting_forward = 
            self.is_cursor_greater_than(&self.selection.start_point);
        if is_selecting_forward {
            self.move_cursor_left();
            self.selection.end_point = self.cursor.clone();
            self.move_cursor_right();
        }
    }

    pub(crate) fn clear_selection(&mut self, previous_cursor: &Cursor) {
        if self.selection.is_selected == false {
            self.selection.start_point = self.cursor.clone();
            self.selection.end_point = self.cursor.clone();
            return;
        }

        let sorted_selection_points = self.get_sorted_selection_points();
        
        if !sorted_selection_points.is_none() {
            self.selection = sorted_selection_points.unwrap();
        }

        let is_cursor_going_forward_from_start_point = 
            self.is_cursor_greater_than(&previous_cursor)
            && self.is_cursor_lesser_than(&self.selection.end_point);
        let is_cursor_going_backward_from_end_point = 
            self.is_cursor_lesser_than(&previous_cursor)
            && self.is_cursor_greater_than(&self.selection.start_point);
        let is_cursor_going_forward_from_end_point = 
            self.is_cursor_greater_than(&previous_cursor)
            && self.is_cursor_greater_than(&self.selection.end_point);
        let is_cursor_going_backward_from_start_point = 
            self.is_cursor_lesser_than(&previous_cursor)
            && self.is_cursor_lesser_than(&self.selection.start_point);
        
        if is_cursor_going_forward_from_start_point {
            self.cursor = self.selection.end_point.clone();
            self.move_cursor_right();
        } else if is_cursor_going_backward_from_end_point {
            self.cursor = self.selection.start_point.clone();
        } else if is_cursor_going_forward_from_end_point {
            self.cursor = self.selection.end_point.clone();
            self.move_cursor_right();
        } else if is_cursor_going_backward_from_start_point {
            self.cursor = self.selection.start_point.clone();
        }
        
        self.selection.is_selected = false;
        self.selection.start_point = self.cursor.clone();
        self.selection.end_point = self.cursor.clone();
    }

    pub(crate) fn delete_selected(&mut self) {
        if !self.selection.is_selected { return; }
        
        let selection = self.get_sorted_selection_points();
        if selection.is_none() { return; }
        let selection = selection.unwrap();

        self.cursor = selection.end_point.clone();
        self.move_cursor_right();

        loop {
            self.perform_backspace();
            
            if self.cursor.row == selection.start_point.row
            && self.cursor.col == selection.start_point.col {
                break;
            }
        }

        self.clear_selection(&self.cursor.clone());
    }

    pub(crate) fn select_all(&mut self) {
        if self.is_document_empty() {
            self.clear_selection(&self.cursor.clone());
            return;
        }
        
        self.selection.is_selected = true;
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
        self.cursor = self.selection.end_point.clone();
    }

    pub(crate) fn perform_copy(&mut self) {
        if !self.selection.is_selected { return; }
        
        let selection = self.get_sorted_selection_points();
        if selection.is_none() { return; }
        let selection = selection.unwrap();
        let current_cursor_backup = self.cursor.clone();
        let mut copied_string = String::new();

        self.cursor.row = selection.start_point.row;
        self.cursor.col = selection.start_point.col;

        loop {
            // let is_cursor_at_line_end = self.cursor.col == self.lines[self.cursor.row].len();
            // let is_cursor_at_file_end = 
            //     self.cursor.row == self.lines.len() - 1
            //     && self.cursor.col == self.lines[self.cursor.row].len();
            
            if self.is_cursor_greater_than(&self.selection.end_point)
            || self.is_cursor_at_file_end() { 
                break;
            }

            if self.is_cursor_at_line_end() {
                copied_string.push('\n');
            } else {
                copied_string.push(self.lines[self.cursor.row][self.cursor.col].character);
            }

            self.move_cursor_right();
            
        }

        self.cursor.row = current_cursor_backup.row;
        self.cursor.col = current_cursor_backup.col;

        let clipboard_ctx = ClipboardContext::new();
        if clipboard_ctx.is_ok() {
            let mut clipboard_ctx = clipboard_ctx.unwrap();
            clipboard_ctx.set_contents(copied_string.clone()).unwrap();
        } else {
            self.clipboard = copied_string.clone();
        }
    }

    pub(crate) fn perform_paste(&mut self) {
        let clipboard_ctx = ClipboardContext::new();
        let copied_string: String;
        if clipboard_ctx.is_ok() {
            let mut clipboard_ctx = clipboard_ctx.unwrap();
            copied_string = clipboard_ctx.get_contents().unwrap();
        } else {
            copied_string = self.clipboard.clone();
        }

        for character in copied_string.chars() {
            self.input_character(character);
        }
    }

    pub(crate) fn perform_undo(&mut self) {
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

    pub(crate) fn perform_redo(&mut self) {
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

    pub(crate) fn clear_history(&mut self) {
        self.undo_list.clear();
        self.undo_list.push(History{
            lines: self.lines.clone(),
            cursor: self.cursor.clone(),
            selection: self.selection.clone(),
        });
        self.redo_list.clear();
    }

    pub(crate) fn save_to_history(&mut self) {
        self.undo_list.push(History{
            lines: self.lines.clone(),
            cursor: self.cursor.clone(),
            selection: self.selection.clone(),
        });
        self.redo_list.clear();
    }

    pub(crate) fn restore_last_cursor_col_if_applicable(&mut self) {
        if self.last_cursor_col <= self.lines[self.cursor.row].len() {
            self.cursor.col = self.last_cursor_col;
        }
    }
    
    pub(crate) fn update_terminal_size(&mut self) {
        let (term_width, term_height) = crossterm::terminal::size().unwrap_or((80, 40));
        self.term_width = term_width as usize;
        self.term_height = term_height as usize;
    }
    
    pub(crate) fn update_status_frame(&mut self) {
        self.status_frame.width = self.term_width;
        self.status_frame.height = 1;
    }

    pub(crate) fn update_line_nums_frame(&mut self) {
        let mut should_update_text_frame = false;
        
        if self.text_frame.start_col > 0 && self.line_nums_frame.boundary_r != String::from(" |") {
            self.line_nums_frame.boundary_r = String::from(" |");
            should_update_text_frame = true;
        } else if self.text_frame.start_col == 0 && self.line_nums_frame.boundary_r != String::from(" | ") {
            self.line_nums_frame.boundary_r = String::from(" | ");
            should_update_text_frame = true;
        }
        
        self.line_nums_frame.width = self.lines.len().to_string().len() + 1 + self.line_nums_frame.boundary_r.len();
        self.line_nums_frame.height = self.term_height - self.status_frame.height;
        
        if should_update_text_frame {
            self.update_text_frame();
        }
    }

    pub(crate) fn update_text_frame(&mut self) {
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

        let is_cursor_left_from_frame = self.cursor.col < self.text_frame.start_col + 2;
        let is_cursor_right_from_frame = self.cursor.col > self.text_frame.start_col + self.text_frame.width - 2;

        if is_cursor_left_from_frame {
            while self.text_frame.start_col + 2 > self.cursor.col {
                if self.text_frame.start_col < 1 {
                    break;
                }
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

}
