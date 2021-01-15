use super::*;

impl Lino {
    pub(crate) fn operation_move_up(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_up();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn operation_move_down(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_down();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn operation_move_left(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_left();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn operation_move_right(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_right();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn operation_move_left_by_word(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_left_by_word();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn operation_move_right_by_word(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_right_by_word();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn operation_move_up_by_page(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_up_by_page();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn operation_move_down_by_page(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_down_by_page();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn operation_move_to_line_start(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_to_line_start();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn operation_move_to_line_end(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_to_line_end();
        self.clear_selection(&previous_cursor);
    }
    
    
    
    pub(crate) fn operation_select_up(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_up();
        self.make_selection(&previous_cursor);
    }
    pub(crate) fn operation_select_down(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_down();
        self.make_selection(&previous_cursor);
    }
    pub(crate) fn operation_select_left(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_left();
        self.make_selection(&previous_cursor);
    }
    pub(crate) fn operation_select_right(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_right();
        self.make_selection(&previous_cursor);
    }
    pub(crate) fn operation_select_left_by_word(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_left_by_word();
        self.make_selection(&previous_cursor);
    }
    pub(crate) fn operation_select_right_by_word(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_right_by_word();
        self.make_selection(&previous_cursor);
    }
    pub(crate) fn operation_select_up_by_page(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_up_by_page();
        self.make_selection(&previous_cursor);
    }
    pub(crate) fn operation_select_down_by_page(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_down_by_page();
        self.make_selection(&previous_cursor);
    }
    pub(crate) fn operation_select_to_line_start(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_to_line_start();
        self.make_selection(&previous_cursor);
    }
    pub(crate) fn operation_select_to_line_end(&mut self) {
        let previous_cursor = self.cursor.clone();
        self.move_cursor_to_line_end();
        self.make_selection(&previous_cursor);
    }
    pub(crate) fn operation_select_all(&mut self) {
        self.clear_selection(&self.cursor.clone());
        self.select_all();
    }
    pub(crate) fn operation_clear_selection(&mut self) {
        self.clear_selection(&self.cursor.clone());
    }
    



    pub(crate) fn operation_delete_left_character(&mut self) {
        self.save_to_history();
        if self.selection.is_selected {
            self.delete_selected();
        } else {
            self.perform_backspace();
        }
    }
    pub(crate) fn operation_delete_right_character(&mut self) {
        self.save_to_history();
        if self.selection.is_selected {
            self.delete_selected();
        } else {
            self.perform_delete();
        }
    }
    pub(crate) fn operation_delete_left_word(&mut self) {
        self.save_to_history();
        if self.selection.is_selected {
            self.delete_selected();
        } else {
            self.delete_left_by_word();
        }
    }
    pub(crate) fn operation_delete_right_word(&mut self) {
        self.save_to_history();
        if self.selection.is_selected {
            self.delete_selected();
        } else {
            self.delete_right_by_word();
        }
    }
    pub(crate) fn operation_delete_current_line(&mut self) {
        self.save_to_history();
        if self.selection.is_selected {
            self.delete_selected();
        } else {
            self.delete_current_line();
        }
    }


    
    pub(crate) fn operation_move_current_line_up(&mut self) {
        self.save_to_history();
        self.swap_line_upward();
    }
    pub(crate) fn operation_move_current_line_down(&mut self) {
        self.save_to_history();
        self.swap_line_downward();
    }



    pub(crate) fn operation_duplicate_current_line_up(&mut self) {
        self.save_to_history();
        self.duplicate_line_upward();
    }
    pub(crate) fn operation_duplicate_current_line_down(&mut self) {
        self.save_to_history();
        self.duplicate_line_downward();
    }



    pub(crate) fn operation_increase_indentation(&mut self) {
        self.save_to_history();
        self.increase_indentation();
    }
    pub(crate) fn operation_decrease_indentation(&mut self) {
        self.save_to_history();
        self.decrease_indentation();
    }



    pub(crate) fn operation_enter_character(&mut self) {
        if self.input_char_buf.is_none() {
            return;
        }
        self.save_to_history();
        if self.selection.is_selected {
            self.delete_selected();
        }
        self.input_character(self.input_char_buf.unwrap());
        self.input_char_buf = None;
    }
    pub(crate) fn operation_enter_tab(&mut self) {
        self.save_to_history();
        if self.selection.is_selected {
            self.delete_selected();
        }
        self.input_tab();
    }
    pub(crate) fn operation_enter_new_line(&mut self) {
        self.save_to_history();
        if self.selection.is_selected {
            self.delete_selected();
        }
        self.enter_newline();
    }
    pub(crate) fn operation_enter_auto_indented_new_line(&mut self) {
        self.save_to_history();
        if self.selection.is_selected {
            self.delete_selected();
        }
        self.enter_newline();
        self.auto_indent_if_applicable();
    }



    pub(crate) fn operation_cut(&mut self) {
        self.save_to_history();
        self.perform_copy();
        if self.selection.is_selected {
            self.delete_selected();
        }
    }
    pub(crate) fn operation_copy(&mut self) {
        self.perform_copy();
    }
    pub(crate) fn operation_paste(&mut self) {
        self.save_to_history();
        if self.selection.is_selected {
            self.delete_selected();
        }
        self.perform_paste();
    }
    
    
    
    pub(crate) fn operation_undo(&mut self) {
        self.perform_undo();
    }
    pub(crate) fn operation_redo(&mut self) {
        self.perform_redo();
    }



    pub(crate) fn operation_save(&mut self) {
        self.perform_save();
    }
    pub(crate) fn operation_quit(&mut self) {
        self.exit_from_editor();
    }
}
