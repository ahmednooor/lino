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
            lines: vec![vec![]],
            input_char_buf: None,
            saved_text: "".to_string(),
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
            task_feedback: TaskFeedback{
                bg: crossterm::style::Color::Rgb{r: 0x23, g: 0x29, b: 0x3c},
                fg: crossterm::style::Color::Rgb{r: 0x88, g: 0x88, b: 0x88},
                text: "".to_string(),
                default_text: "[Ctrl+Q] Quit, [Alt+G] Guide".to_string(),
            },
            should_exit: false,
            undo_list: vec![],
            redo_list: vec![],
            file: FileData{
                path: "".to_string(),
                is_saved: true,
                should_save_as: true,
                save_error: "".to_string(),
            },
            clipboard: "".to_string(),
            settings: Settings{
                tab_width: 4,
                show_line_nums_frame: true,
                show_status_frame: true,
                read_only: false,
            },
            error: Error{
                is_occured: false,
                message: "".to_string(),
                code: 0,
            },
            theming: Theming {
                line_nums_frame_bg: crossterm::style::Color::Rgb{r: 0x28, g: 0x30, b: 0x42},
                line_nums_frame_fg: crossterm::style::Color::Rgb{r: 0x88, g: 0x88, b: 0x88},
                line_nums_frame_highlighted_bg: crossterm::style::Color::Rgb{r: 0x38, g: 0x42, b: 0x52},
                line_nums_frame_highlighted_fg: crossterm::style::Color::Rgb{r: 0xff, g: 0xff, b: 0xff},
                
                text_frame_bg: crossterm::style::Color::Rgb{r: 0x23, g: 0x29, b: 0x3c},
                text_frame_fg: crossterm::style::Color::Rgb{r: 0xff, g: 0xff, b: 0xff},
                text_frame_highlighted_bg: crossterm::style::Color::Rgb{r: 0x38, g: 0x42, b: 0x52},
                text_frame_highlighted_fg: crossterm::style::Color::Rgb{r: 0xff, g: 0xff, b: 0xff},
                text_frame_selection_bg: crossterm::style::Color::Rgb{r: 0xff, g: 0xff, b: 0xff},
                text_frame_selection_fg: crossterm::style::Color::Rgb{r: 0x23, g: 0x29, b: 0x3c},
                text_frame_found_text_bg: crossterm::style::Color::Rgb{r: 0xa4, g: 0x54, b: 0x0e},
                text_frame_found_text_fg: crossterm::style::Color::Rgb{r: 0xff, g: 0xff, b: 0xff},

                status_frame_bg: crossterm::style::Color::Rgb{r: 0xff, g: 0xff, b: 0xff},
                status_frame_fg: crossterm::style::Color::Rgb{r: 0x23, g: 0x29, b: 0x3c},

                error_red: crossterm::style::Color::Rgb{r: 0xff, g: 0x58, b: 0x58},
            },
            highlighting: Highlighting{
                start_row: 0,
                end_row: 0,
            },
            rendering: Rendering{
                is_rendering: false,
                buffer: vec![],
            },
            keybindings: std::collections::HashMap::new(),
            find: Find{
                is_finding: false,
                find_string: "".to_string(),
                find_error: "".to_string(),
                found_instances: vec![],
                selected_instance_index: 0,
                keybindings_backup: std::collections::HashMap::new(),
            },
            replace: Replace{
                replace_string: "".to_string(),
            }
        };

        // lino.load_theming_defaults_from_syntect_theme();
        
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
        lino.init_new_render_buffer();
        lino.clear_all_keybindings();
        lino.add_default_keybindings();
        lino.clear_history();

        lino
    }

    pub fn run(&mut self) {
        self.bind_ctrlc_handler();
        self.enter_alt_screen_and_enable_raw_mode();
        self.start();
    }

    pub fn run_as_read_only(&mut self) {
        self.settings.read_only = true;
        self.clear_all_keybindings();
        self.add_read_only_mode_keybindings();
        self.run();
    }
    
    pub(crate) fn start(&mut self) {
        let mut syntect_config = self.create_syntect_config();

        self.apply_syntax_highlighting_on_all_lines(&mut syntect_config);
        self.clear_history();

        self.initiate_input_event_loop(&mut syntect_config);
    }

    pub(crate) fn bind_ctrlc_handler(&mut self) {
        ctrlc::set_handler(|| ())
            .unwrap_or_else(|_| self.panic_gracefully(&Error::err1()));
        // ctrlc::set_handler(|| ())
        //     .unwrap_or_else(|_| {
        //         self.set_task_feedback_error(self.error.message.clone());
        //     });
    }

    pub(crate) fn enter_alt_screen_and_enable_raw_mode(&mut self) {
        crossterm::terminal::enable_raw_mode()
            .unwrap_or_else(|_| self.panic_gracefully(&Error::err2()));
        
        // crossterm::execute!(stdout(), crossterm::terminal::EnterAlternateScreen)
        //     .unwrap_or_else(|_| self.panic_gracefully(&Error::err3()));
        crossterm::execute!(
            stdout(),
            crossterm::style::ResetColor,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0),
        ).unwrap_or_else(|_| self.panic_gracefully(&Error::err3()));
    }

    pub(crate) fn leave_alt_screen_and_disable_raw_mode(&mut self) {
        // crossterm::execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).unwrap_or(());
        crossterm::execute!(
            stdout(),
            crossterm::style::ResetColor,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0),
        ).unwrap_or(());
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
        self.leave_alt_screen_and_disable_raw_mode();
        
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
            println!("\n\n{}\n\n", exiting_message);
        }
    }
}
