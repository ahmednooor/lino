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
            }
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
        ctrlc::set_handler(|| ()).expect("Error setting Ctrl-C handler");

        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(stdout(), crossterm::terminal::EnterAlternateScreen)?;
        
        self.initiate_input_event_loop()?;
        
        // crossterm::terminal::disable_raw_mode()?;
        // crossterm::execute!(stdout(), crossterm::terminal::LeaveAlternateScreen)?;
        
        Ok(())
    }
}

impl Drop for Lino {
    fn drop(&mut self) {
        crossterm::execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
    }
}
