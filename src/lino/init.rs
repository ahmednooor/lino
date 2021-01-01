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
        let lines = Lino::convert_string_to_2d_text(&input_string);

        let (term_width, term_height) = crossterm::terminal::size().unwrap();
        
        let status_frame_width = term_height;
        let status_frame_height = 1;

        let line_nums_frame_boundary_r = String::from(" | ");
        let line_nums_frame_width = lines.len().to_string().len() + 1 + line_nums_frame_boundary_r.len();
        let line_nums_frame_height = term_height - status_frame_height;

        let text_frame_width = term_width as usize - line_nums_frame_width;
        let text_frame_height = term_height - status_frame_height;

        let mut lino = Lino {
            saved_lines: lines.clone(),
            lines: lines.clone(),
            term_width: term_width as usize,
            term_height: term_height as usize,
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
                width: text_frame_width as usize,
                height: text_frame_height as usize,
                start_row: 0,
                start_col: 0,
            },
            line_nums_frame: LineNumsFrame{
                width: line_nums_frame_width as usize,
                height: line_nums_frame_height as usize,
                boundary_r: line_nums_frame_boundary_r,
            },
            status_frame: StatusFrame{
                width: status_frame_width as usize,
                height: status_frame_height as usize,
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
        };

        lino.clear_history();
        lino.save_to_history();

        lino
    }

    pub fn run(&mut self) -> crossterm::Result<()> {
        ctrlc::set_handler(|| ()).expect("Error setting Ctrl-C handler");

        crossterm::execute!(stdout(), crossterm::terminal::EnterAlternateScreen)?;
        crossterm::terminal::enable_raw_mode()?;
        
        self.initiate_input_event_loop()?;
        
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(stdout(), crossterm::terminal::LeaveAlternateScreen)?;
        
        Ok(())
    }
}