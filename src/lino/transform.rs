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

        self.lines = Lino::convert_string_to_2d_text(&input_string);
        self.saved_lines = self.lines.clone();
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

        self.saved_lines = self.lines.clone();
        self.file.is_saved = true;
        self.file.should_save_as = false;
    }

    pub(crate) fn set_file_unsaved_if_applicable(&mut self) {
        let current_text_string = Lino::convert_2d_text_to_string(&self.lines);
        let saved_text_string = Lino::convert_2d_text_to_string(&self.saved_lines);

        if current_text_string != saved_text_string {
            self.file.is_saved = false;
        } else {
            self.file.is_saved = true;
        }
    }

    pub(crate) fn perform_save(&mut self) -> crossterm::Result<()> {
        if self.file.path == "" || self.file.should_save_as {
            self.render_save_as_frame()?;
            self.handle_save_as_frame_input()?;
        } else {
            self.save_to_file();
        }

        Ok(())
    }

    pub(crate) fn exit_from_editor(&mut self) {
        self.should_exit = true;
        self.initiate_exit_procedure().unwrap();
    }

    pub(crate) fn initiate_exit_procedure(&mut self) -> crossterm::Result<()> {
        if self.file.is_saved {
            return Ok(());
        }

        self.render_unsaved_changes_frame()?;
        self.handle_unsaved_changes_frame_input()?;

        if self.file.should_save_as {
            self.render_save_as_frame()?;
            self.handle_save_as_frame_input()?;
        }

        Ok(())
    }
    
    pub(crate) fn input_character(&mut self, character: char) {
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

    pub(crate) fn input_tab(&mut self) {
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

    pub(crate) fn move_cursor_left_by_word(&mut self) {
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

    pub(crate) fn move_cursor_right_by_word(&mut self) {
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

    pub(crate) fn make_selection(&mut self, previous_cursor: &Cursor) {
        if self.is_document_empty() { 
            self.clear_selection();
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

    pub(crate) fn clear_selection(&mut self) {
        self.selection.is_selected = false;
        self.selection.start_point.row = self.cursor.row;
        self.selection.start_point.col = self.cursor.col;
        self.selection.end_point.row = self.cursor.row;
        self.selection.end_point.col = self.cursor.col;
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

        self.clear_selection();
    }

    pub(crate) fn select_all(&mut self) {
        let is_document_empty = 
            self.lines.len() == 1 && self.lines[0].len() == 0;

        if is_document_empty {
            self.clear_selection();
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
        self.cursor.row = self.selection.end_point.row;
        self.cursor.col = self.selection.end_point.col;
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
            let is_cursor_at_line_end = self.cursor.col == self.lines[self.cursor.row].len();
            let is_cursor_at_file_end = 
                self.cursor.row == self.lines.len() - 1
                && self.cursor.col == self.lines[self.cursor.row].len();
            
            if (self.cursor.row == selection.end_point.row
            && self.cursor.col > selection.end_point.col)
            || is_cursor_at_file_end {
                break;
            }

            if is_cursor_at_line_end {
                copied_string.push('\n');
            } else {
                copied_string.push(self.lines[self.cursor.row][self.cursor.col].character);
            }

            self.move_cursor_right();
            
        }

        self.cursor.row = current_cursor_backup.row;
        self.cursor.col = current_cursor_backup.col;

        let mut clipboard_ctx = ClipboardContext::new().unwrap();
        clipboard_ctx.set_contents(copied_string).unwrap();
    }

    pub(crate) fn perform_paste(&mut self) {
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

    pub(crate) fn update_status_frame(&mut self) {
        self.status_frame.width = self.term_width;
        self.status_frame.height = 1;
    }
}