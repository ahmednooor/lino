use std::io::{stdout, Write};
use crossterm;
extern crate copypasta;
// use copypasta::ClipboardContext;
// use copypasta::ClipboardProvider;
// use std::fs::File;
// use std::io::prelude::*;
use std::path::Path;

use super::*;

impl Lino {
    pub fn new() -> Lino {
        Lino::from_string(&"".to_string())
    }
    
    pub fn from_file(file_path: &String) -> Lino {
        let mut lino = Lino::new();
        lino.file.path = Path::new(file_path.as_str()).to_str().unwrap().to_string();
        lino.read_from_file();
        lino.set_file_unsaved_if_applicable();
        lino.clear_history();
        lino.save_to_history();

        lino
    }
    
    pub fn from_string(input_string: &String) -> Lino {
        Lino::init(&input_string)
    }

    pub(crate) fn init(input_string: &String) -> Lino {
        let mut lino = Lino {
            saved_lines: vec![vec![]],
            lines: vec![vec![]],
            term_width: 0,
            term_height: 0,
            cursor: Cursor{
                row: 0,
                col: 0
            },
            last_cursor_col: 0,
            selection: Selection{
                is_selected: false,
                start_point: Cursor{
                    row: 0,
                    col: 0,
                },
                end_point: Cursor{
                    row: 0,
                    col: 0,
                },
            },
            text_frame: TextFrame{
                width: 0,
                height: 0,
                start_row: 0,
                start_col: 0,
            },
            line_nums_frame: LineNumsFrame{
                width: 0,
                height: 0,
                boundary_r: String::from(" | "),
            },
            status_frame: StatusFrame{
                width: 0,
                height: 0,
            },
            should_exit: false,
            is_rendering: false,
            undo_list: vec![],
            redo_list: vec![],
            file: FileData{
                path: "".to_string(),
                is_saved: true,
                should_save_as: true,
            },
            clipboard: "".to_string(),
            settings: Settings{
                tab_width: 4,
            },
            error: Error{
                is_occured: false,
                code: 0,
                message: "".to_string()
            },
        };

        for character in input_string.chars() {
            lino.input_character(character);
        }
        
        lino.reset_cursor();
        lino.saved_lines = lino.lines.clone();
        lino.update_terminal_size();
        lino.update_status_frame();
        lino.update_line_nums_frame();
        lino.update_text_frame();
        lino.clear_history();
        lino.save_to_history();

        lino
    }

    pub fn run(&mut self) -> crossterm::Result<()> {
        ctrlc::set_handler(|| ())
            .unwrap_or_else(|_| self.panic_gracefully(errors::ERR1.0.to_string(), errors::ERR1.1));
        crossterm::terminal::enable_raw_mode()
            .unwrap_or_else(|_| self.panic_gracefully(errors::ERR2.0.to_string(), errors::ERR2.1));
        crossterm::execute!(stdout(), crossterm::terminal::EnterAlternateScreen)
            .unwrap_or_else(|_| self.panic_gracefully(errors::ERR3.0.to_string(), errors::ERR3.1));
        
        self.initiate_input_event_loop();
        
        // crossterm::terminal::disable_raw_mode()?;
        // crossterm::execute!(stdout(), crossterm::terminal::LeaveAlternateScreen)?;
        
        Ok(())
    }

    pub(crate) fn panic_gracefully(&mut self, error_message: String, error_code: isize) {
        self.error.is_occured = true;
        self.error.message = error_message;
        self.error.code = error_code;
        
        let mut temp_file_path = std::env::current_dir().unwrap();
        temp_file_path.push("lino_recov.tmp.txt");
        self.file.path = temp_file_path.to_str().unwrap().to_string();
        self.file.is_saved = false;
        self.save_to_file();
        
        panic!();
    }
}

impl Drop for Lino {
    fn drop(&mut self) {
        crossterm::execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).unwrap_or(());
        crossterm::terminal::disable_raw_mode().unwrap_or(());
        
        let mut exiting_message = String::new();
        
        if self.error.is_occured {
            let err_str = format!("[ERROR] {} (code: {})\n", self.error.message.clone(), self.error.code);
            exiting_message.push_str(&err_str);
        }
        
        if self.error.is_occured && self.file.is_saved {
            let err_str = format!(
                "[RECOVERY] Your unsaved data has been saved at \"{}\" , You can recover it from there.\n", 
                self.file.path);
            exiting_message.push_str(&err_str);
        }

        println!("{}", exiting_message);
    }
}
