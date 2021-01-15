use super::*;

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
            // highlighter : highlighter,
        }
    }
}

impl Lino {
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

    pub(crate) fn apply_syntax_highlighting_on_all_lines(&mut self, syntect_config: &mut SyntectConfig) {
        let cursor_backup = self.cursor.clone();
        let last_cursor_col = self.last_cursor_col;
        self.cursor.row = 0;
        
        for _ in 0..self.lines.len() {
            self.apply_syntax_highlighting_on_current_line(syntect_config);
            self.move_cursor_down();
        }

        self.cursor = cursor_backup;
        self.last_cursor_col = last_cursor_col;
    }
    
    pub(crate) fn apply_syntax_highlighting_on_lines_range(&mut self, syntect_config: &mut SyntectConfig) {
        let cursor_backup = self.cursor.clone();
        let last_cursor_col = self.last_cursor_col;

        // let start_row = if self.cursor.row < previous_cursor.row {
        //     self.cursor.row
        // } else {
        //     previous_cursor.row
        // };
        
        // let end_row = if self.cursor.row > previous_cursor.row {
        //     self.cursor.row
        // } else {
        //     previous_cursor.row
        // };

        self.cursor.row = self.highlighting.start_row;

        let selection = self.get_sorted_selection_points().unwrap_or(self.selection.clone());

        for _ in self.highlighting.start_row..=self.highlighting.end_row {
            if selection.is_selected && self.is_cursor_inside_selection(&selection, &self.cursor) {
                self.move_cursor_down();
                continue;
            }
            self.apply_syntax_highlighting_on_current_line(syntect_config);
            self.move_cursor_down();
        }

        self.cursor = cursor_backup;
        self.last_cursor_col = last_cursor_col;
    }

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

    pub(crate) fn apply_syntax_highlighting_on_current_line(&mut self, syntect_config: &mut SyntectConfig) {
        let cursor_backup = self.cursor.clone();
        let last_cursor_col = self.last_cursor_col;
        
        self.cursor.col = 0;
        
        let s = Lino::convert_2d_text_to_string(
            &self.lines[self.cursor.row..self.cursor.row + 1].to_vec()) + "\n";
        
        let mut highlighter = syntect::easy::HighlightLines::new(
            &syntect_config.syntax, &syntect_config.theme_set.themes[SYNTECT_THEME_NAME]);
        let ranges: Vec<(syntect::highlighting::Style, &str)> = 
            highlighter.highlight(&s, &syntect_config.syntax_set);
        
        for word in ranges {
            // let background = crossterm::style::Color::Rgb{
            //     r: word.0.background.r,
            //     g: word.0.background.g,
            //     b: word.0.background.b,
            // };
            let foreground = crossterm::style::Color::Rgb{
                r: word.0.foreground.r,
                g: word.0.foreground.g,
                b: word.0.foreground.b,
            };
            
            for c in word.1.chars() {
                if c == '\n' || self.is_cursor_at_line_end() {
                    self.move_cursor_right();
                    continue;
                }
                // self.lines[self.cursor.row][self.cursor.col].background = background;
                self.lines[self.cursor.row][self.cursor.col].foreground = foreground;
                self.move_cursor_right();
            }
        }
        self.cursor = cursor_backup;
        self.last_cursor_col = last_cursor_col;
    }
}