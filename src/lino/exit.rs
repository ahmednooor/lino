use super::*;
use super::confirmation_dialog::ConfirmationDialog;

impl Lino {
    pub(crate) fn initiate_exit_routine(&mut self) {
        self.should_exit = true;

        if self.file.is_saved {
            return;
        }

        self.ask_to_save_unsaved_changes();

        if self.file.should_save_as {
            self.ask_file_path_and_save();
        }
    }

    pub(crate) fn ask_to_save_unsaved_changes(&mut self) {
        let mut confirmation_dialog = ConfirmationDialog{
            title: "UNSAVED CHANGES".to_string(),
            description: "Would you like to save changes before you quit?".to_string(),
            key_hints: "[Y] Yes, [N] No, [Esc] Go Back".to_string(),
            input: None,
            editor_theming: self.theming.clone(),
        };
        
        let dialog_result = confirmation_dialog.collect_input();
        match dialog_result {
            Err(e) => self.panic_gracefully(&e),
            Ok(_) => ()
        };

        let dialog_input = confirmation_dialog.input;
        if dialog_input.is_none() {
            self.file.should_save_as = false;
            self.should_exit = false;
            return;
        }
        
        let should_save_before_quitting = dialog_input.unwrap();

        if should_save_before_quitting {
            self.file.should_save_as = true;
            return;
        }

        self.file.should_save_as = false;
    }
    
}
