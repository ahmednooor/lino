use std::io::{stdout};
use crossterm;
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

        lino
    }
    
    pub fn from_string(input_string: &String) -> Lino {
        Lino::init(&input_string)
    }

    pub(crate) fn init(input_string: &String) -> Lino {
        let mut lino = Lino {
            saved_text: "".to_string(),
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
                message: "".to_string(),
                code: 0,
            },
            theming: Theming {
                line_nums_frame_bg: crossterm::style::Color::Rgb{r: 0x23, g: 0x25, b: 0x37},
                line_nums_frame_fg: crossterm::style::Color::DarkGrey,
                line_nums_frame_highlighted_bg: crossterm::style::Color::Rgb{r: 0x23, g: 0x25, b: 0x37},
                line_nums_frame_highlighted_fg: crossterm::style::Color::White,
                
                text_frame_bg: crossterm::style::Color::Rgb{r: 0x23, g: 0x25, b: 0x37},
                text_frame_fg: crossterm::style::Color::White,
                text_frame_highlighted_bg: crossterm::style::Color::Rgb{r: 0x23, g: 0x25, b: 0x37},
                text_frame_highlighted_fg: crossterm::style::Color::White,
                text_frame_selection_bg: crossterm::style::Color::White,
                text_frame_selection_fg: crossterm::style::Color::Rgb{r: 0x23, g: 0x25, b: 0x37},

                status_frame_bg: crossterm::style::Color::White,
                status_frame_fg: crossterm::style::Color::Rgb{r: 0x23, g: 0x25, b: 0x37},
            },
            highlighting: Highlighting{
                start_row: 0,
                end_row: 0,
            }
        };

        for character in input_string.chars() {
            lino.input_character(character);
        }
        
        lino.reset_cursor();
        lino.last_cursor_col = lino.cursor.col;
        lino.saved_text = Lino::convert_2d_text_to_string(&lino.lines);
        lino.update_terminal_size();
        lino.update_status_frame();
        lino.update_line_nums_frame();
        lino.update_text_frame();
        lino.clear_history();

        lino
    }

    pub fn run(&mut self) {
        ctrlc::set_handler(|| ())
            .unwrap_or_else(|_| self.panic_gracefully(&Error::err1()));
        
        crossterm::terminal::enable_raw_mode()
            .unwrap_or_else(|_| self.panic_gracefully(&Error::err2()));
        
        crossterm::execute!(stdout(), crossterm::terminal::EnterAlternateScreen)
            .unwrap_or_else(|_| self.panic_gracefully(&Error::err3()));
        
        let mut syntect_config = self.create_syntect_config();

        self.apply_syntax_highlighting_on_all_lines(&mut syntect_config);
        self.clear_history();

        self.initiate_input_event_loop(&mut syntect_config);
    }

    pub fn close(&mut self) {
        crossterm::execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).unwrap_or(());
        crossterm::terminal::disable_raw_mode().unwrap_or(());
    }

    pub(crate) fn panic_gracefully(&mut self, error: &Error) {
        self.error.is_occured = error.is_occured;
        self.error.message = error.message.clone();
        self.error.code = error.code;
        
        let mut temp_file_path = std::env::current_dir().unwrap();
        temp_file_path.push("lino.tmp.txt");
        self.file.path = temp_file_path.to_str().unwrap().to_string();
        self.file.is_saved = false;
        self.save_to_file();
        
        panic!();
    }
}

impl Drop for Lino {
    fn drop(&mut self) {
        self.close();
        
        let mut exiting_message = String::new();
        
        if self.error.is_occured {
            let err_str = format!("[ERROR] {} (code: {})\n", self.error.message.clone(), self.error.code);
            exiting_message.push_str(&err_str);
        }
        
        if self.error.is_occured && self.file.is_saved {
            let err_str = format!(
                "[RECOVERY] Your data has been saved in \"{}\" , You can recover it from there.\n", 
                self.file.path);
            exiting_message.push_str(&err_str);
        }

        if !exiting_message.is_empty() {
            println!("{}", exiting_message);
        }
    }
}
