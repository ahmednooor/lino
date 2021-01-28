// use std::io::{stdout, Write};
extern crate copypasta;
use copypasta::ClipboardContext;
use copypasta::ClipboardProvider;

use super::*;

impl Lino {
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
                width: 1,
            }
        );
        
        self.cursor.col += 1;
        self.last_cursor_col = self.cursor.col;
    }

    pub(crate) fn input_tab(&mut self) {
        let tab_width = self.calculate_tab_width();
                    
        for _ in 0..tab_width {
            self.input_character(' ');
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

        if self.lines[self.cursor.row - 1].len() > 0 
        && ['(', '[', '{', '<', ':'].contains(&self.lines[self.cursor.row - 1].last().unwrap().character) {
            for _ in 0..4 {
                self.input_character(' ');
            }
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


    
    pub(crate) fn delete_left_by_word(&mut self) {
        if self.is_cursor_at_first_line() && self.is_cursor_at_line_start() {
            return;
        }
        let previous_cursor = self.cursor.clone();
        self.clear_selection(&previous_cursor);
        self.move_cursor_left_by_word();
        self.make_selection(&previous_cursor);
        self.delete_selected();
    }

    pub(crate) fn delete_right_by_word(&mut self) {
        if self.is_cursor_at_last_line() && self.is_cursor_at_line_end() {
            return;
        }
        let previous_cursor = self.cursor.clone();
        self.clear_selection(&previous_cursor);
        self.move_cursor_right_by_word();
        self.make_selection(&previous_cursor);
        self.delete_selected();
    }



    pub(crate) fn increase_indentation(&mut self) {
        if self.selection.is_selected {
            self.increase_selected_lines_indentation();
        } else {
            self.increase_current_line_indentation();
        }
    }

    pub(crate) fn increase_selected_lines_indentation(&mut self) {
        if !self.selection.is_selected {
            return;
        }

        let selection = self.get_sorted_selection_points().unwrap();
        let backup_cursor = self.cursor.clone();
        
        self.cursor = selection.start_point.clone();

        for _ in selection.start_point.row..=selection.end_point.row {
            self.increase_current_line_indentation();
            self.move_cursor_down();
        }

        self.cursor = backup_cursor;
        self.cursor.col += self.settings.tab_width;
        self.selection.start_point.col += self.settings.tab_width;
        self.selection.end_point.col += self.settings.tab_width;
    }

    pub(crate) fn increase_current_line_indentation(&mut self) {
        let backup_cursor = self.cursor.clone();
        self.cursor.col = 0;
        for _ in 0..self.settings.tab_width {
            self.input_character(' ');
        }
        self.cursor.col = backup_cursor.col + self.settings.tab_width;
    }



    pub(crate) fn decrease_indentation(&mut self) {
        if self.selection.is_selected {
            self.decrease_selected_lines_indentation();
        } else {
            self.decrease_current_line_indentation();
        }
    }

    pub(crate) fn decrease_selected_lines_indentation(&mut self) {
        if !self.selection.is_selected {
            return;
        }

        let selection = self.get_sorted_selection_points().unwrap();
        let backup_cursor = self.cursor.clone();
        let mut should_adjust_selection_start = false;
        let mut should_adjust_selection_end = false;

        self.cursor = self.selection.start_point.clone();
        if !self.is_current_line_empty() && self.lines[self.cursor.row][0].character == ' ' {
            should_adjust_selection_start = true;
        }
        self.cursor = self.selection.end_point.clone();
        if !self.is_current_line_empty() && self.lines[self.cursor.row][0].character == ' ' {
            should_adjust_selection_end = true;
        }

        self.cursor = selection.start_point.clone();

        for _ in selection.start_point.row..=selection.end_point.row {
            self.decrease_current_line_indentation();
            self.move_cursor_down();
        }

        self.cursor = backup_cursor;

        if self.cursor.col >= self.settings.tab_width && should_adjust_selection_start{
            self.cursor.col -= self.settings.tab_width;
        } else if should_adjust_selection_start {
            self.cursor.col = 0;
        }
        
        if self.selection.start_point.col >= self.settings.tab_width 
        && should_adjust_selection_start {
            self.selection.start_point.col -= self.settings.tab_width;
        } else if should_adjust_selection_start {
            self.selection.start_point.col = 0;
        }

        if self.selection.end_point.col >= self.settings.tab_width 
        && should_adjust_selection_end {
            self.selection.end_point.col -= self.settings.tab_width;
        } else if should_adjust_selection_end {
            self.selection.end_point.col = 0;
        }
    }
    
    pub(crate) fn decrease_current_line_indentation(&mut self) {
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
                if !self.is_cursor_at_line_start() {
                    self.move_cursor_left();
                }
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

    pub(crate) fn delete_selected(&mut self) {
        if !self.selection.is_selected { return; }
        
        let selection = self.get_sorted_selection_points();
        if selection.is_none() { return; }
        let selection = selection.unwrap();

        self.highlighting.start_row = selection.start_point.row;
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
            if self.is_cursor_greater_than(&selection.end_point)
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
}
