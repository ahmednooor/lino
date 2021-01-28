use super::*;
use super::input_dialog::InputDialog;

impl Lino {
    pub(crate) fn set_file_unsaved_if_applicable(&mut self) {
        let current_text = Lino::convert_2d_text_to_string(&self.lines);
        // let saved_text_string = Lino::convert_2d_text_to_string(&self.saved_text);

        if current_text != self.saved_text {
            self.file.is_saved = false;
        } else {
            self.file.is_saved = true;
        }
    }
    
    pub(crate) fn initiate_save_as_routine(&mut self) {
        self.file.should_save_as = true;
        self.initiate_save_routine();
    }

    pub(crate) fn initiate_save_routine(&mut self) {
        self.file.save_error = "".to_string();

        if self.file.path == "" || self.file.should_save_as {
            self.ask_file_path_and_save();
        } else {
            self.save_to_file();
        }
    }

    pub(crate) fn ask_file_path_and_save(&mut self) {
        let file_data_backup = self.file.clone();

        let mut input_dialog = InputDialog{
            is_active: false,
            title: "SAVE FILE".to_string(),
            description: "Enter file name.".to_string(),
            key_hints: "[Enter] Save, [Esc] Go Back".to_string(),
            input: self.file.path.chars().collect(),
            cursor_col_offset: self.file.path.chars().count(),
            error: self.file.save_error.clone(),
            editor_theming: self.theming.clone(),
        };

        loop {
            input_dialog.input = self.file.path.chars().collect();
            input_dialog.error = self.file.save_error.clone();

            let dialog_result = input_dialog.collect_input();
            match dialog_result {
                Err(e) => self.panic_gracefully(&e),
                Ok(_) => ()
            };

            self.file.path = input_dialog.input.iter().collect();
            
            if !input_dialog.is_active {
                self.file.path = file_data_backup.path;
                self.should_exit = false;
                self.file.should_save_as = false;
                break;
            }

            if self.file.path != "" {
                self.save_to_file();
            }

            if self.file.is_saved && self.file.save_error == "" {
                break;
            }
        }
    }
}
