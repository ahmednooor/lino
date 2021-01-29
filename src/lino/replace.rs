use super::*;
use super::input_prompt::InputPrompt;

impl Lino {
    pub(crate) fn initiate_replace_routine(&mut self) {
        if !self.find.is_finding {
            return;
        }

        let mut input_prompt = InputPrompt{
            is_active: false,
            title: "REPLACE ALL".to_string(),
            description: format!("Enter replacement text for: `{}`", self.find.find_string),
            key_hints: "[Enter] Replace, [Esc] Go Back".to_string(),
            input: self.replace.replace_string.chars().collect(),
            cursor_col_offset: self.replace.replace_string.chars().count(),
            error: "".to_string(),
            editor_theming: self.theming.clone(),
        };
        
        let prompt_result = input_prompt.collect_input();
        match prompt_result {
            Err(e) => self.panic_gracefully(&e),
            Ok(_) => ()
        };

        self.replace.replace_string = input_prompt.input.iter().collect();
        
        if !input_prompt.is_active {
            self.replace.replace_string = "".to_string();
            return;
        }

        self.replace_text();
    }
    
    pub(crate) fn replace_text(&mut self) {
        if self.find.found_instances.len() < 1 {
            return;
        }

        let found_instances: Vec<CursorRange> = self.find.found_instances.clone().into_iter().rev().collect();
        let mut last_instance_col_offset_multiplier: isize = 0;
        let last_instance_row: usize = found_instances[0].end.row;

        for instance in &found_instances {
            if instance.end.row == last_instance_row {
                last_instance_col_offset_multiplier += 1;
            }
            self.selection.is_selected = true;
            self.selection.start_point = instance.start;
            self.selection.end_point = instance.end;
            self.cursor = self.selection.start_point;
            self.delete_selected();
            for character in self.replace.replace_string.clone().chars() {
                self.input_character(character);
            }
        }
        
        self.cursor = found_instances[0].end;

        let find_replace_string_len_diff = 
            self.replace.replace_string.len() as isize - self.find.find_string.len() as isize;

        self.cursor.col = (self.cursor.col as isize + 1 + 
            (last_instance_col_offset_multiplier * find_replace_string_len_diff)) as usize;

            self.replace.replace_string = "".to_string();
        self.reset_find();
    }

}
