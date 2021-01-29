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

        
        for instance in self.find.found_instances.clone() {
            self.selection.is_selected = true;
            self.selection.start_point = instance.start;
            self.selection.end_point = instance.end;
            self.cursor = self.selection.start_point;
            {
                self.delete_selected();
            }
            for character in self.replace.replace_string.clone().chars() {
                self.input_character(character);
            }
        }
        
        self.highlighting.start_row = 0;
        self.highlighting.end_row = self.lines.len() - 1;
        self.replace.replace_string = "".to_string();
        self.reset_find();
    }

}
