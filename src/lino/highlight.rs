use super::*;
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};

use syntect;
// use syntect::easy::HighlightLines;
// use syntect::parsing::SyntaxSet;
// use syntect::highlighting::{ThemeSet, Style};
// use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

pub(crate) static SYNTECT_THEME_NAME: &str = "base16-eighties.dark";

pub(crate) struct SyntectConfig {
    pub(crate) syntax_set: syntect::parsing::SyntaxSet,
    pub(crate) theme_set: syntect::highlighting::ThemeSet,
    pub(crate) syntax: syntect::parsing::SyntaxReference,
    // highlighter: syntect::easy::HighlightLines<'a>,
}

impl SyntectConfig {
    pub(crate) fn new(ext: &str) -> SyntectConfig {
        let syntax_set = syntect::parsing::SyntaxSet::load_defaults_newlines();
        let theme_set = syntect::highlighting::ThemeSet::load_defaults();
        let syntax = syntax_set.find_syntax_by_extension(ext)
            .unwrap_or(syntax_set.find_syntax_plain_text()).to_owned();
        // let mut highlighter = syntect::easy::HighlightLines::new(&syntax.clone(), &theme_set.themes[SYNTECT_THEME_NAME].clone());

        SyntectConfig{
            syntax_set: syntax_set,
            theme_set: theme_set,
            syntax: syntax,
            // highlighter: highlighter,
        }
    }
}

impl Lino {
    pub(crate) fn spawn_highlighting_thread(&mut self) 
        -> (Sender<(HighlightingThreadMessage, Cursor)>, 
            Receiver<(std::vec::Vec<(syntect::highlighting::Style, usize)>, Cursor)>
    ) {
        
        let (main_thread_tx, highlight_thread_rx) = channel::<(HighlightingThreadMessage, Cursor)>();
        let (highlight_thread_tx, main_thread_rx) = channel::<(Vec<(syntect::highlighting::Style, usize)>, Cursor)>();
        let syntect_config = self.create_syntect_config();
        
        thread::spawn(move || {
            loop {
                let (text, cursor) = match highlight_thread_rx
                    .try_iter().last().unwrap_or((HighlightingThreadMessage::Idle, Cursor{row: 0, col: 0})) {
                        (HighlightingThreadMessage::Start(s), cursor) => (s, cursor),
                        (HighlightingThreadMessage::Idle, _) => continue,
                        (HighlightingThreadMessage::Terminate, _) => break,
                    };

                let highlight_thread_tx_clone = highlight_thread_tx.clone();

                let syntax = syntect_config.syntax.clone();
                let theme_set = syntect_config.theme_set.themes[highlight::SYNTECT_THEME_NAME].clone();
                let syntax_set = syntect_config.syntax_set.clone();

                let mut highlighter = syntect::easy::HighlightLines::new(&syntax, &theme_set);
                    
                let ranges: Vec<(syntect::highlighting::Style, &str)> = 
                    highlighter.highlight(&text, &syntax_set);

                let mut highlighted_words: Vec<(syntect::highlighting::Style, usize)> = vec![];
                
                for word in ranges {
                    highlighted_words.push((word.0, word.1.len()));
                }
                
                highlight_thread_tx_clone.send((highlighted_words, cursor)).unwrap();
            }
        });

        (main_thread_tx, main_thread_rx)
    }

    pub(crate) fn send_text_to_highlighting_thread(&mut self, 
        main_thread_tx: &Sender<(HighlightingThreadMessage, Cursor)>
    ) {
        if !self.highlighting.should_send_text_to_highlighting_thread { return; }

        main_thread_tx.send((
            HighlightingThreadMessage::Start(Lino::convert_2d_text_to_string(&self.lines)), 
            self.cursor.clone())).unwrap_or(());
        
        self.highlighting.should_send_text_to_highlighting_thread = false;
    }

    pub(crate) fn colorize_text_based_on_highlighting(&mut self, 
        main_thread_rx: &Receiver<(Vec<(syntect::highlighting::Style, usize)>, Cursor)>
    ) {
        let (ranges, cursor_before_highlighting) = 
            main_thread_rx.try_iter().last().unwrap_or((vec![], Cursor{row: 0, col: 0}));
        
        if ranges.len() < 1 { return; }

        let cursor_backup = self.cursor.clone();
        let last_cursor_col = self.last_cursor_col;

        self.cursor.col = 0;
        self.cursor.row = 0;

        for word in ranges {
            let foreground = crossterm::style::Color::Rgb{
                r: word.0.foreground.r,
                g: word.0.foreground.g,
                b: word.0.foreground.b,
            };
            
            for _ in 0..word.1 {
                if self.is_cursor_equal_to(&cursor_before_highlighting) {
                    loop {
                        if self.is_cursor_equal_to(&cursor_backup) {
                            break;
                        } else if self.is_cursor_lesser_than(&cursor_backup) {
                            self.move_cursor_right();
                        } else if self.is_cursor_greater_than(&cursor_backup) {
                            self.move_cursor_left();
                        }
                    }
                }

                if self.is_cursor_at_line_end() {
                    self.move_cursor_right();
                    continue;
                }

                self.lines[self.cursor.row][self.cursor.col].foreground = foreground;
                self.move_cursor_right();
            }
        }

        self.cursor = cursor_backup;
        self.last_cursor_col = last_cursor_col;

        self.rendering.should_render = true;
    }

    // pub(crate) fn load_theming_defaults_from_syntect_theme(&mut self) {
    //     let theme_settings = syntect::highlighting::ThemeSet::load_defaults()
    //         .themes[super::highlight::SYNTECT_THEME_NAME].settings.clone();
    //     let editor_bg = crossterm::style::Color::Rgb{
    //         r: theme_settings.background.unwrap().r,
    //         g: theme_settings.background.unwrap().g,
    //         b: theme_settings.background.unwrap().b,
    //     };
    //     let editor_fg = crossterm::style::Color::Rgb{
    //         r: theme_settings.foreground.unwrap().r,
    //         g: theme_settings.foreground.unwrap().g,
    //         b: theme_settings.foreground.unwrap().b,
    //     };
    //     let highlighted_bg = crossterm::style::Color::Rgb{
    //         r: theme_settings.line_highlight.unwrap().r,
    //         g: theme_settings.line_highlight.unwrap().g,
    //         b: theme_settings.line_highlight.unwrap().b,
    //     };
    //     self.theming.line_nums_frame_bg = editor_bg;
    //     self.theming.line_nums_frame_fg = editor_fg;
    //     self.theming.line_nums_frame_highlighted_bg = highlighted_bg;
    //     self.theming.text_frame_bg = editor_bg;
    //     self.theming.text_frame_fg = editor_fg;
    //     self.theming.text_frame_highlighted_bg = highlighted_bg;
    // }

    pub(crate) fn create_syntect_config(&mut self) -> SyntectConfig {
        let mut file_ext = "txt";
        if !std::path::Path::new(self.file.path.as_str()).extension().is_none() {
            file_ext = std::path::Path::new(self.file.path.as_str()).extension().unwrap().to_str().unwrap();
        }
        SyntectConfig::new(&file_ext)
    }
    
    // pub(crate) fn update_editor_theme_from_syntect_config(&mut self, syntect_config: &mut SyntectConfig) {
    //     self.theming.text_frame_highlighted_bg = crossterm::style::Color::Rgb{
    //         r: syntect_config.theme_set.themes[SYNTECT_THEME_NAME].settings.line_highlight.unwrap().r,
    //         g: syntect_config.theme_set.themes[SYNTECT_THEME_NAME].settings.line_highlight.unwrap().g,
    //         b: syntect_config.theme_set.themes[SYNTECT_THEME_NAME].settings.line_highlight.unwrap().b,
    //     };
    //     self.theming.text_frame_selection_bg = crossterm::style::Color::Rgb{
    //         r: syntect_config.theme_set.themes[SYNTECT_THEME_NAME].settings.selection.unwrap().r,
    //         g: syntect_config.theme_set.themes[SYNTECT_THEME_NAME].settings.selection.unwrap().g,
    //         b: syntect_config.theme_set.themes[SYNTECT_THEME_NAME].settings.selection.unwrap().b,
    //     };
    //     self.theming.text_frame_highlighted_fg = crossterm::style::Color::Rgb{
    //         r: syntect_config.theme_set.themes[SYNTECT_THEME_NAME].settings.line_highlight.unwrap().r,
    //         g: syntect_config.theme_set.themes[SYNTECT_THEME_NAME].settings.line_highlight.unwrap().g,
    //         b: syntect_config.theme_set.themes[SYNTECT_THEME_NAME].settings.line_highlight.unwrap().b,
    //     };
    // }

    // pub(crate) fn apply_syntax_highlighting_on_all_lines(&mut self, syntect_config: &mut SyntectConfig) {
    //     let cursor_backup = self.cursor.clone();
    //     let last_cursor_col = self.last_cursor_col;

    //     self.cursor.col = 0;
    //     self.cursor.row = 0;

    //     let s = Lino::convert_2d_text_to_string(&self.lines);
        
    //     let mut highlighter = syntect::easy::HighlightLines::new(
    //         &syntect_config.syntax, &syntect_config.theme_set.themes[SYNTECT_THEME_NAME]);
        
    //     let ranges: Vec<(syntect::highlighting::Style, &str)> = 
    //         highlighter.highlight(&s, &syntect_config.syntax_set);
        
    //     for word in ranges {
    //         // if self.is_cursor_lesser_than(&Cursor{row: self.text_frame.start_row, col: self.text_frame.start_col}) {
    //         //     let new_line_chars: Vec<&str> = word.1.matches("\n").collect();
                
    //         //     if new_line_chars.len() > 0 {
    //         //         self.cursor.row += new_line_chars.len();
    //         //         self.cursor.col = 0;
    //         //         continue;
    //         //     }

    //         //     self.cursor.col += word.1.len();
    //         //     continue;
    //         // }

    //         if self.is_cursor_greater_than(&Cursor{row: self.text_frame.start_row + self.text_frame.height - 1, col: self.text_frame.start_col + self.text_frame.width - 1}) {
    //             break;
    //         }

    //         // let background = crossterm::style::Color::Rgb{
    //         //     r: word.0.background.r,
    //         //     g: word.0.background.g,
    //         //     b: word.0.background.b,
    //         // };

    //         let foreground = crossterm::style::Color::Rgb{
    //             r: word.0.foreground.r,
    //             g: word.0.foreground.g,
    //             b: word.0.foreground.b,
    //         };
            
    //         for c in word.1.chars() {
    //             if c == '\n' || self.is_cursor_at_line_end() || self.is_cursor_lesser_than(&Cursor{row: self.text_frame.start_row, col: self.text_frame.start_col}) {
    //                 self.move_cursor_right();
    //                 continue;
    //             }
    //             // self.lines[self.cursor.row][self.cursor.col].background = background;
    //             self.lines[self.cursor.row][self.cursor.col].foreground = foreground;
    //             self.move_cursor_right();
    //         }

            
    //     }

    //     self.cursor = cursor_backup;
    //     self.last_cursor_col = last_cursor_col;
    // }

    // pub(crate) fn apply_syntax_highlighting_on_all_lines(&mut self, syntect_config: &mut SyntectConfig) {
    //     if !self.highlighting.should_send_text_to_highlighting_thread { return; }

    //     let cursor_backup = self.cursor.clone();
    //     let last_cursor_col = self.last_cursor_col;

    //     self.cursor.col = 0;
    //     self.cursor.row = 0;

    //     let s = Lino::convert_2d_text_to_string(&self.lines);
        
    //     let mut highlighter = syntect::easy::HighlightLines::new(
    //         &syntect_config.syntax, &syntect_config.theme_set.themes[SYNTECT_THEME_NAME]);
        
    //     let ranges: Vec<(syntect::highlighting::Style, &str)> = 
    //         highlighter.highlight(&s, &syntect_config.syntax_set);
        
    //     for word in ranges {
    //         let foreground = crossterm::style::Color::Rgb{
    //             r: word.0.foreground.r,
    //             g: word.0.foreground.g,
    //             b: word.0.foreground.b,
    //         };
            
    //         for c in word.1.chars() {
    //             if c == '\n' || self.is_cursor_at_line_end() {
    //                 self.move_cursor_right();
    //                 continue;
    //             }
    //             self.lines[self.cursor.row][self.cursor.col].foreground = foreground;
    //             self.move_cursor_right();
    //         }
    //     }

    //     self.cursor = cursor_backup;
    //     self.last_cursor_col = last_cursor_col;
    //     self.highlighting.should_send_text_to_highlighting_thread = false;
    // }

    // pub(crate) fn apply_syntax_highlighting_on_visible_lines(&mut self, syntect_config: &mut SyntectConfig) {
    //     if !self.highlighting.should_send_text_to_highlighting_thread { return; }

    //     let cursor_backup = self.cursor.clone();
    //     let last_cursor_col = self.last_cursor_col;

    //     self.cursor.col = 0;
    //     self.cursor.row = 0;

    //     let s = Lino::convert_2d_text_to_string(&self.lines);
        
    //     let mut highlighter = syntect::easy::HighlightLines::new(
    //         &syntect_config.syntax, &syntect_config.theme_set.themes[SYNTECT_THEME_NAME]);
        
    //     let ranges: Vec<(syntect::highlighting::Style, &str)> = 
    //         highlighter.highlight(&s, &syntect_config.syntax_set);
        
    //     for word in ranges {
    //         if self.is_cursor_greater_than(&Cursor{row: self.text_frame.start_row + self.text_frame.height - 1, col: self.text_frame.start_col + self.text_frame.width - 1}) {
    //             break;
    //         }

    //         let foreground = crossterm::style::Color::Rgb{
    //             r: word.0.foreground.r,
    //             g: word.0.foreground.g,
    //             b: word.0.foreground.b,
    //         };
            
    //         for c in word.1.chars() {
    //             if c == '\n' || self.is_cursor_at_line_end() || self.is_cursor_lesser_than(&Cursor{row: self.text_frame.start_row, col: self.text_frame.start_col}) {
    //                 self.move_cursor_right();
    //                 continue;
    //             }
    //             self.lines[self.cursor.row][self.cursor.col].foreground = foreground;
    //             self.move_cursor_right();
    //         }
    //     }

    //     self.cursor = cursor_backup;
    //     self.last_cursor_col = last_cursor_col;
    //     self.highlighting.should_send_text_to_highlighting_thread = false;
    // }
    
    // pub(crate) fn apply_syntax_highlighting_on_lines_range(&mut self, syntect_config: &mut SyntectConfig) {
    //     let cursor_backup = self.cursor.clone();
    //     let last_cursor_col = self.last_cursor_col;

    //     self.cursor.row = self.highlighting.start_row;

    //     // let selection = self.get_sorted_selection_points().unwrap_or(self.selection.clone());
    //     self.cursor.col = 0;

    //     // for _ in self.highlighting.start_row..=self.highlighting.end_row {
    //     //     if selection.is_selected && self.is_cursor_inside_selection(&selection, &self.cursor) {
    //     //         self.move_cursor_down();
    //     //         continue;
    //     //     }
    //     //     self.apply_syntax_highlighting_on_current_line(syntect_config);
    //     //     self.move_cursor_down();
    //     // }
    //     // let cursor_backup = self.cursor.clone();
    //     // let last_cursor_col = self.last_cursor_col;
        

    //     let s = Lino::convert_2d_text_to_string(
    //         &self.lines[self.highlighting.start_row..self.highlighting.end_row + 1].to_vec()) + "\n";
        
    //     let mut highlighter = syntect::easy::HighlightLines::new(
    //         &syntect_config.syntax, &syntect_config.theme_set.themes[SYNTECT_THEME_NAME]);
        
    //     // for line in syntect::util::LinesWithEndings::from(&s) {
    //         let ranges: Vec<(syntect::highlighting::Style, &str)> = 
    //             highlighter.highlight(&s, &syntect_config.syntax_set);
            
    //         for word in ranges {
    //             // let background = crossterm::style::Color::Rgb{
    //             //     r: word.0.background.r,
    //             //     g: word.0.background.g,
    //             //     b: word.0.background.b,
    //             // };
    //             let foreground = crossterm::style::Color::Rgb{
    //                 r: word.0.foreground.r,
    //                 g: word.0.foreground.g,
    //                 b: word.0.foreground.b,
    //             };
                
    //             for _ in 0..word.1.len() {
    //                 if self.is_cursor_at_line_end() {
    //                     self.move_cursor_right();
    //                     continue;
    //                 }
    //                 // self.lines[self.cursor.row][self.cursor.col].background = background;
    //                 self.lines[self.cursor.row][self.cursor.col].foreground = foreground;
    //                 self.move_cursor_right();
    //             }
    //         // }
    //     }

    //     self.cursor = cursor_backup;
    //     self.last_cursor_col = last_cursor_col;
    // }

    // pub(crate) fn apply_syntax_highlighting_on_changed_lines(
    //     &mut self, syntect_config: &mut SyntectConfig, previous_lines: &Vec<Vec<Character>>
    // ) {
    //     let cursor_backup = self.cursor.clone();
    //     let last_cursor_col = self.last_cursor_col;
    //     self.cursor.row = 0;
        
    //     for i in 0..self.lines.len() {
    //         if i >= previous_lines.len() {
    //             self.apply_syntax_highlighting_on_current_line(syntect_config);
    //             self.move_cursor_down();
    //             continue;
    //         }
    //         if Lino::convert_2d_text_to_string(&previous_lines[i..i + 1].to_vec()) 
    //         != Lino::convert_2d_text_to_string(&self.lines[i..i + 1].to_vec()) {
    //             self.apply_syntax_highlighting_on_current_line(syntect_config);
    //             self.move_cursor_down();
    //         }
    //     }

    //     self.cursor = cursor_backup;
    //     self.last_cursor_col = last_cursor_col;
    // }

    // pub(crate) fn apply_syntax_highlighting_on_current_line(&mut self, syntect_config: &mut SyntectConfig) {
    //     let cursor_backup = self.cursor.clone();
    //     let last_cursor_col = self.last_cursor_col;
        
    //     self.cursor.col = 0;
        
    //     let s = String::from("\n") + &Lino::convert_2d_text_to_string(
    //         &self.lines[self.cursor.row..self.cursor.row + 1].to_vec()) + "\n";
        
    //     let mut highlighter = syntect::easy::HighlightLines::new(
    //         &syntect_config.syntax, &syntect_config.theme_set.themes[SYNTECT_THEME_NAME]);
    //     let ranges: Vec<(syntect::highlighting::Style, &str)> = 
    //         highlighter.highlight(&s, &syntect_config.syntax_set);

    //     let mut first_new_line_done = false;
        
    //     for word in ranges {
    //         // let background = crossterm::style::Color::Rgb{
    //         //     r: word.0.background.r,
    //         //     g: word.0.background.g,
    //         //     b: word.0.background.b,
    //         // };
    //         let foreground = crossterm::style::Color::Rgb{
    //             r: word.0.foreground.r,
    //             g: word.0.foreground.g,
    //             b: word.0.foreground.b,
    //         };
            
    //         for c in word.1.chars() {
    //             if !first_new_line_done {
    //                 first_new_line_done = true;
    //                 continue;
    //             }
    //             if c == '\n' || self.is_cursor_at_line_end() {
    //                 self.move_cursor_right();
    //                 continue;
    //             }
    //             // self.lines[self.cursor.row][self.cursor.col].background = background;
    //             self.lines[self.cursor.row][self.cursor.col].foreground = foreground;
    //             self.move_cursor_right();
    //         }
    //     }
    //     self.cursor = cursor_backup;
    //     self.last_cursor_col = last_cursor_col;
    // }
}