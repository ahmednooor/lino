use super::*;
use super::input_prompt::InputPrompt;

impl Lino {
    pub(crate) fn initiate_find_routine(&mut self) {
        let mut input_prompt = InputPrompt{
            is_active: false,
            title: "FIND".to_string(),
            description: "Enter text to find. (Case Sensitive)".to_string(),
            key_hints: "[Enter] Find, [Esc] Go Back".to_string(),
            input: self.find.find_string.chars().collect(),
            cursor_col_offset: self.find.find_string.chars().count(),
            error: self.find.find_error.clone(),
            editor_theming: self.theming.clone(),
        };
        
        loop {
            input_prompt.input = self.find.find_string.chars().collect();
            input_prompt.error = self.find.find_error.clone();

            let prompt_result = input_prompt.collect_input();
            match prompt_result {
                Err(e) => self.panic_gracefully(&e),
                Ok(_) => ()
            };

            self.find.find_string = input_prompt.input.iter().collect();
            self.find.found_instances = vec![];

            if !input_prompt.is_active {
                self.reset_find();
                break;
            }

            if self.find.find_string != "" {
                self.find_string_in_text();
            } else {
                continue;
            }

            if self.find.found_instances.len() > 0 {
                break;
            }
        }
    }
    
    pub(crate) fn find_string_in_text(&mut self) {
        self.find.found_instances = vec![];
        
        for i in 0..self.lines.len() {
            let line = Lino::convert_2d_text_to_string(&self.lines[i..i+1].to_vec());
            let matched_instances: Vec<_> = 
                line.match_indices(&self.find.find_string).collect();
            
            for matched in matched_instances {
                self.find.found_instances.push(CursorRange{
                    start: Cursor{
                        row: i,
                        col: line[0..matched.0].chars().count(),
                    },
                    end: Cursor{
                        row: i,
                        col: line[0..matched.0].chars().count() + matched.1.chars().count() - 1,
                    },
                });
            }
        }

        if self.find.found_instances.len() < 1 {
            self.find.find_error = "No instances found.".to_string();
            return;
        }

        self.find.find_error = "".to_string();
        self.find.is_finding = true;
        self.find.selected_instance_index = self.find.found_instances.len();
        self.select_next_found_instance();
        self.clear_all_keybindings();
        self.add_find_mode_keybindings();
    }

    pub(crate) fn select_next_found_instance(&mut self) {
        if self.find.found_instances.len() < 1 {
            return;
        }

        self.find.selected_instance_index += 1;
        if self.find.selected_instance_index >= self.find.found_instances.len() {
            self.find.selected_instance_index = 0;
        }

        self.selection.is_selected = true;
        self.selection.start_point = 
            self.find.found_instances[self.find.selected_instance_index].start.clone();
        self.selection.end_point = 
            self.find.found_instances[self.find.selected_instance_index].end.clone();
        
        self.cursor = self.selection.end_point.clone();
        self.move_cursor_right();
        self.update_last_cursor_col();
    }

    pub(crate) fn select_previous_found_instance(&mut self) {
        if self.find.found_instances.len() < 1 {
            return;
        }

        if self.find.selected_instance_index < 1 {
            self.find.selected_instance_index = self.find.found_instances.len() - 1;
        } else {
            self.find.selected_instance_index -= 1;
        }

        self.selection.is_selected = true;
        self.selection.start_point = 
            self.find.found_instances[self.find.selected_instance_index].start.clone();
        self.selection.end_point = 
            self.find.found_instances[self.find.selected_instance_index].end.clone();
        
        self.cursor = self.selection.end_point.clone();
        self.move_cursor_right();
        self.update_last_cursor_col();
    }

    pub(crate) fn reset_find(&mut self) {
        self.find = Find{
            is_finding: false,
            find_string: "".to_string(),
            find_error: "".to_string(),
            found_instances: vec![],
            selected_instance_index: 0,
        };
        self.clear_all_keybindings();
        self.add_default_keybindings();
    }

}
