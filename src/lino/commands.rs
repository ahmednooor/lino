use super::*;

impl Lino {
    pub(crate) fn command_move_up(&mut self) {
        self.clear_task_feedback();
        if self.selection.is_selected {
            self.set_task_feedback_normal("Selection cancelled.".to_string());
        }
        let previous_cursor = self.cursor.clone();
        self.move_cursor_up();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn command_move_down(&mut self) {
        self.clear_task_feedback();
        if self.selection.is_selected {
            self.set_task_feedback_normal("Selection cancelled.".to_string());
        }
        let previous_cursor = self.cursor.clone();
        self.move_cursor_down();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn command_move_left(&mut self) {
        self.clear_task_feedback();
        if self.selection.is_selected {
            self.set_task_feedback_normal("Selection cancelled.".to_string());
        }
        let previous_cursor = self.cursor.clone();
        self.move_cursor_left();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn command_move_right(&mut self) {
        self.clear_task_feedback();
        if self.selection.is_selected {
            self.set_task_feedback_normal("Selection cancelled.".to_string());
        }
        let previous_cursor = self.cursor.clone();
        self.move_cursor_right();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn command_move_left_by_word(&mut self) {
        self.clear_task_feedback();
        if self.selection.is_selected {
            self.set_task_feedback_normal("Selection cancelled.".to_string());
        }
        let previous_cursor = self.cursor.clone();
        self.move_cursor_left_by_word();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn command_move_right_by_word(&mut self) {
        self.clear_task_feedback();
        if self.selection.is_selected {
            self.set_task_feedback_normal("Selection cancelled.".to_string());
        }
        let previous_cursor = self.cursor.clone();
        self.move_cursor_right_by_word();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn command_move_up_by_page(&mut self) {
        self.clear_task_feedback();
        if self.selection.is_selected {
            self.set_task_feedback_normal("Selection cancelled.".to_string());
        }
        let previous_cursor = self.cursor.clone();
        self.move_cursor_up_by_page();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn command_move_down_by_page(&mut self) {
        self.clear_task_feedback();
        if self.selection.is_selected {
            self.set_task_feedback_normal("Selection cancelled.".to_string());
        }
        let previous_cursor = self.cursor.clone();
        self.move_cursor_down_by_page();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn command_move_to_line_start(&mut self) {
        self.clear_task_feedback();
        if self.selection.is_selected {
            self.set_task_feedback_normal("Selection cancelled.".to_string());
        }
        let previous_cursor = self.cursor.clone();
        self.move_cursor_to_line_start();
        self.clear_selection(&previous_cursor);
    }
    pub(crate) fn command_move_to_line_end(&mut self) {
        self.clear_task_feedback();
        if self.selection.is_selected {
            self.set_task_feedback_normal("Selection cancelled.".to_string());
        }
        let previous_cursor = self.cursor.clone();
        self.move_cursor_to_line_end();
        self.clear_selection(&previous_cursor);
    }
    
    
    
    pub(crate) fn command_select_up(&mut self) {
        self.clear_task_feedback();
        let previous_cursor = self.cursor.clone();
        self.move_cursor_up();
        self.make_selection(&previous_cursor);
        if self.selection.is_selected {
            let selected_count = self.get_selected_count().to_string() + " characters selected.";
            self.set_task_feedback_normal(selected_count);
        }
    }
    pub(crate) fn command_select_down(&mut self) {
        self.clear_task_feedback();
        let previous_cursor = self.cursor.clone();
        self.move_cursor_down();
        self.make_selection(&previous_cursor);
        if self.selection.is_selected {
            let selected_count = self.get_selected_count().to_string() + " characters selected.";
            self.set_task_feedback_normal(selected_count);
        }
    }
    pub(crate) fn command_select_left(&mut self) {
        self.clear_task_feedback();
        let previous_cursor = self.cursor.clone();
        self.move_cursor_left();
        self.make_selection(&previous_cursor);
        if self.selection.is_selected {
            let selected_count = self.get_selected_count().to_string() + " characters selected.";
            self.set_task_feedback_normal(selected_count);
        }
    }
    pub(crate) fn command_select_right(&mut self) {
        self.clear_task_feedback();
        let previous_cursor = self.cursor.clone();
        self.move_cursor_right();
        self.make_selection(&previous_cursor);
        if self.selection.is_selected {
            let selected_count = self.get_selected_count().to_string() + " characters selected.";
            self.set_task_feedback_normal(selected_count);
        }
    }
    pub(crate) fn command_select_left_by_word(&mut self) {
        self.clear_task_feedback();
        let previous_cursor = self.cursor.clone();
        self.move_cursor_left_by_word();
        self.make_selection(&previous_cursor);
        if self.selection.is_selected {
            let selected_count = self.get_selected_count().to_string() + " characters selected.";
            self.set_task_feedback_normal(selected_count);
        }
    }
    pub(crate) fn command_select_right_by_word(&mut self) {
        self.clear_task_feedback();
        let previous_cursor = self.cursor.clone();
        self.move_cursor_right_by_word();
        self.make_selection(&previous_cursor);
        if self.selection.is_selected {
            let selected_count = self.get_selected_count().to_string() + " characters selected.";
            self.set_task_feedback_normal(selected_count);
        }
    }
    pub(crate) fn command_select_up_by_page(&mut self) {
        self.clear_task_feedback();
        let previous_cursor = self.cursor.clone();
        self.move_cursor_up_by_page();
        self.make_selection(&previous_cursor);
        if self.selection.is_selected {
            let selected_count = self.get_selected_count().to_string() + " characters selected.";
            self.set_task_feedback_normal(selected_count);
        }
    }
    pub(crate) fn command_select_down_by_page(&mut self) {
        self.clear_task_feedback();
        let previous_cursor = self.cursor.clone();
        self.move_cursor_down_by_page();
        self.make_selection(&previous_cursor);
        if self.selection.is_selected {
            let selected_count = self.get_selected_count().to_string() + " characters selected.";
            self.set_task_feedback_normal(selected_count);
        }
    }
    pub(crate) fn command_select_to_line_start(&mut self) {
        self.clear_task_feedback();
        let previous_cursor = self.cursor.clone();
        self.move_cursor_to_line_start();
        self.make_selection(&previous_cursor);
        if self.selection.is_selected {
            let selected_count = self.get_selected_count().to_string() + " characters selected.";
            self.set_task_feedback_normal(selected_count);
        }
    }
    pub(crate) fn command_select_to_line_end(&mut self) {
        self.clear_task_feedback();
        let previous_cursor = self.cursor.clone();
        self.move_cursor_to_line_end();
        self.make_selection(&previous_cursor);
        if self.selection.is_selected {
            let selected_count = self.get_selected_count().to_string() + " characters selected.";
            self.set_task_feedback_normal(selected_count);
        }
    }
    pub(crate) fn command_select_all(&mut self) {
        self.clear_task_feedback();
        self.clear_selection(&self.cursor.clone());
        self.select_all();
        if self.selection.is_selected {
            let selected_count = self.get_selected_count().to_string() + " characters selected.";
            self.set_task_feedback_normal(selected_count);
        }
    }
    pub(crate) fn command_clear_selection(&mut self) {
        self.clear_task_feedback();
        if self.selection.is_selected {
            self.set_task_feedback_normal("Selection cancelled.".to_string());
        }
        self.clear_selection(&self.cursor.clone());
    }
    



    pub(crate) fn command_delete_left_character(&mut self) {
        self.clear_task_feedback();
        self.save_to_history();
        let task_feedback_text: String;
        let should_show_feedback = if !self.is_document_empty() { true } else { false };
        if self.selection.is_selected {
            self.delete_selected();
            task_feedback_text = "Selection deleted.".to_string();
        } else {
            self.perform_backspace();
            task_feedback_text = "Previous character deleted.".to_string();
        }
        if should_show_feedback && task_feedback_text != "" {
            self.set_task_feedback_normal(task_feedback_text);
        }
    }
    pub(crate) fn command_delete_right_character(&mut self) {
        self.clear_task_feedback();
        self.save_to_history();
        let task_feedback_text: String;
        let should_show_feedback = if !self.is_document_empty() { true } else { false };
        if self.selection.is_selected {
            self.delete_selected();
            task_feedback_text = "Selection deleted.".to_string();
        } else {
            self.perform_delete();
            task_feedback_text = "Next character deleted.".to_string();
        }
        if should_show_feedback && task_feedback_text != "" {
            self.set_task_feedback_normal(task_feedback_text);
        }
    }
    pub(crate) fn command_delete_left_word(&mut self) {
        self.clear_task_feedback();
        self.save_to_history();
        let task_feedback_text: String;
        let should_show_feedback = if !self.is_document_empty() { true } else { false };
        if self.selection.is_selected {
            self.delete_selected();
            task_feedback_text = "Selection deleted.".to_string();
        } else {
            self.delete_left_by_word();
            task_feedback_text = "Previous word deleted.".to_string();
        }
        if should_show_feedback && task_feedback_text != "" {
            self.set_task_feedback_normal(task_feedback_text);
        }
    }
    pub(crate) fn command_delete_right_word(&mut self) {
        self.clear_task_feedback();
        self.save_to_history();
        let task_feedback_text: String;
        let should_show_feedback = if !self.is_document_empty() { true } else { false };
        if self.selection.is_selected {
            self.delete_selected();
            task_feedback_text = "Selection deleted.".to_string();
        } else {
            self.delete_right_by_word();
            task_feedback_text = "Next word deleted.".to_string();
        }
        if should_show_feedback && task_feedback_text != "" {
            self.set_task_feedback_normal(task_feedback_text);
        }
    }
    pub(crate) fn command_delete_current_line(&mut self) {
        self.clear_task_feedback();
        self.save_to_history();
        let task_feedback_text: String;
        let should_show_feedback = if !self.is_document_empty() { true } else { false };
        if self.selection.is_selected {
            self.delete_selected();
            task_feedback_text = "Selection deleted.".to_string();
        } else {
            self.delete_current_line();
            task_feedback_text = "Line deleted.".to_string();
        }
        if should_show_feedback && task_feedback_text != "" {
            self.set_task_feedback_normal(task_feedback_text);
        }
    }


    
    pub(crate) fn command_move_current_line_up(&mut self) {
        self.clear_task_feedback();
        self.save_to_history();
        self.swap_line_upward();
        if !self.is_document_empty() {
            self.set_task_feedback_normal("Line moved up.".to_string());
        }
    }
    pub(crate) fn command_move_current_line_down(&mut self) {
        self.clear_task_feedback();
        self.save_to_history();
        self.swap_line_downward();
        if !self.is_document_empty() {
            self.set_task_feedback_normal("Line moved down.".to_string());
        }
    }



    pub(crate) fn command_duplicate_current_line_up(&mut self) {
        self.clear_task_feedback();
        self.save_to_history();
        self.duplicate_line_upward();
        if !self.is_document_empty() {
            self.set_task_feedback_normal("Line duplicated.".to_string());
        }
    }
    pub(crate) fn command_duplicate_current_line_down(&mut self) {
        self.clear_task_feedback();
        self.save_to_history();
        self.duplicate_line_downward();
        if !self.is_document_empty() {
            self.set_task_feedback_normal("Line duplicated.".to_string());
        }
    }



    pub(crate) fn command_increase_indentation(&mut self) {
        self.clear_task_feedback();
        self.set_task_feedback_normal("Increased indent.".to_string());
        self.save_to_history();
        self.increase_indentation();
    }
    pub(crate) fn command_decrease_indentation(&mut self) {
        self.clear_task_feedback();
        if !self.is_cursor_at_line_start() {
            self.set_task_feedback_normal("Decreased indent.".to_string());
        }
        self.save_to_history();
        self.decrease_indentation();
    }



    pub(crate) fn command_enter_character(&mut self) {
        self.clear_task_feedback();
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
    pub(crate) fn command_enter_tab(&mut self) {
        self.clear_task_feedback();
        self.save_to_history();
        if self.selection.is_selected {
            self.delete_selected();
        }
        self.input_tab();
    }
    pub(crate) fn command_enter_new_line(&mut self) {
        self.clear_task_feedback();
        self.save_to_history();
        if self.selection.is_selected {
            self.delete_selected();
        }
        self.enter_newline();
    }
    pub(crate) fn command_enter_auto_indented_new_line(&mut self) {
        self.clear_task_feedback();
        self.save_to_history();
        if self.selection.is_selected {
            self.delete_selected();
        }
        self.enter_newline();
        self.auto_indent_if_applicable();
    }



    pub(crate) fn command_cut(&mut self) {
        self.clear_task_feedback();
        self.save_to_history();
        self.perform_copy();
        if self.selection.is_selected {
            self.delete_selected();
            self.set_task_feedback_normal("Cut.".to_string());
        }
    }
    pub(crate) fn command_copy(&mut self) {
        self.clear_task_feedback();
        self.perform_copy();
        if self.selection.is_selected {
            self.set_task_feedback_normal("Copied.".to_string());
        }
    }
    pub(crate) fn command_paste(&mut self) {
        self.clear_task_feedback();
        self.save_to_history();
        if self.selection.is_selected {
            self.delete_selected();
        }
        self.perform_paste();
        self.set_task_feedback_normal("Pasted.".to_string());
    }
    
    
    
    pub(crate) fn command_undo(&mut self) {
        self.clear_task_feedback();
        if self.undo_list.len() > 0 {
            self.set_task_feedback_normal("Undid.".to_string());
        }
        self.perform_undo();
    }
    pub(crate) fn command_redo(&mut self) {
        self.clear_task_feedback();
        if self.redo_list.len() > 0 {
            self.set_task_feedback_normal("Redid.".to_string());
        }
        self.perform_redo();
    }



    pub(crate) fn command_save(&mut self) {
        self.clear_task_feedback();
        self.perform_save();
    }
    pub(crate) fn command_quit(&mut self) {
        self.clear_task_feedback();
        self.exit_from_editor();
    }
}
