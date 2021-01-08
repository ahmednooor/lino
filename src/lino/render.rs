use std::io::{stdout, Write};
use crossterm;
extern crate copypasta;

use super::*;

impl Lino {
    pub(crate) fn render(&mut self, syntect_config: &mut SyntectConfig, previous_cursor: Cursor) {
        self.is_rendering = true;
        self.update_terminal_size();
        self.update_status_frame();
        self.update_line_nums_frame();
        self.update_text_frame();

        crossterm::queue!(
            stdout(),
            // crossterm::style::SetBackgroundColor(crossterm::style::Color::White),
            // crossterm::style::SetForegroundColor(crossterm::style::Color::White),
            // crossterm::style::Print(' '),
            crossterm::cursor::Hide,
            crossterm::cursor::MoveTo(0, 0),
        ).unwrap_or_else(|_| self.panic_gracefully(&Error::err7()));

        // self.apply_syntax_highlighting_on_current_line(syntect_config);
        // self.apply_syntax_highlighting_on_changed_lines(syntect_config, previous_lines);
        self.apply_syntax_highlighting_on_lines_range(syntect_config, previous_cursor);

        self.render_line_nums_frame_content();
        self.render_text_frame_content();
        self.render_status_frame_content();
        self.update_visible_cursor();

        stdout().flush()
            .unwrap_or_else(|_| self.panic_gracefully(&Error::err8()));
        
        self.is_rendering = false;
    }

    pub(crate) fn render_line_nums_frame_content(&mut self) {
        crossterm::queue!(
            stdout(),
            crossterm::style::SetBackgroundColor(self.theming.line_nums_frame_bg),
            crossterm::style::SetForegroundColor(self.theming.line_nums_frame_fg),
        ).unwrap_or_else(|_| self.panic_gracefully(&Error::err9()));

        let visible_frame_starting_line_num = self.text_frame.start_row;
        let mut visible_frame_ending_line_num = visible_frame_starting_line_num + self.text_frame.height;
        
        if visible_frame_ending_line_num > self.lines.len() {
            visible_frame_ending_line_num = self.lines.len();
        }
        
        let line_num_width = self.lines.len().to_string().len();
        
        for i in visible_frame_starting_line_num..visible_frame_ending_line_num {
            let rendered_lines_row = (i - visible_frame_starting_line_num) as usize;
            let rendered_lines_col = 
                (self.term_width - self.text_frame.width - self.line_nums_frame.width) as usize;

            let num_string = format!(" {:width$}", i + 1, width = line_num_width);
            let num_string = String::from(num_string) + &self.line_nums_frame.boundary_r;

            // for line_num_char in num_string.chars() {
            //     crossterm::queue!(
            //         stdout(),
            //         crossterm::cursor::MoveTo(rendered_lines_col as u16, rendered_lines_row as u16),
            //         crossterm::style::Print(line_num_char),
            //     )?;
            //     rendered_lines_col += 1;
            // }
            if i == self.cursor.row {
                crossterm::queue!(
                    stdout(),
                    crossterm::style::SetBackgroundColor(self.theming.line_nums_frame_highlighted_bg),
                    crossterm::style::SetForegroundColor(self.theming.line_nums_frame_highlighted_fg)
                ).unwrap_or_else(|_| self.panic_gracefully(&Error::err29()));
            }

            crossterm::queue!(
                stdout(),
                crossterm::cursor::MoveTo(rendered_lines_col as u16, rendered_lines_row as u16),
                crossterm::style::Print(num_string),
            ).unwrap_or_else(|_| self.panic_gracefully(&Error::err10()));
            
            if i == self.cursor.row {
                crossterm::queue!(
                    stdout(),
                    crossterm::style::SetBackgroundColor(self.theming.line_nums_frame_bg),
                    crossterm::style::SetForegroundColor(self.theming.line_nums_frame_fg)
                ).unwrap_or_else(|_| self.panic_gracefully(&Error::err30()));
            }
        }

        let remaining_lines_start_row = (visible_frame_ending_line_num - visible_frame_starting_line_num) as usize;
        let line_nums_frame_start_col = 
            (self.term_width - self.text_frame.width - self.line_nums_frame.width) as usize;
        let line_nums_frame_end_col = (line_nums_frame_start_col + self.line_nums_frame.width) as usize;

        for i in remaining_lines_start_row..self.line_nums_frame.height {
            // for j in line_nums_frame_start_col..line_nums_frame_end_col {
            //     crossterm::queue!(
            //         stdout(),
            //         crossterm::cursor::MoveTo(j as u16, i as u16),
            //         crossterm::style::Print(' '),
            //     )?;
            // }
            let mut empty_string = String::new();
            for _ in line_nums_frame_start_col..line_nums_frame_end_col {
                empty_string.push(' ');
            }
            crossterm::queue!(
                stdout(),
                crossterm::cursor::MoveTo(line_nums_frame_start_col as u16, i as u16),
                crossterm::style::Print(empty_string),
            ).unwrap_or_else(|_| self.panic_gracefully(&Error::err11()));
        }
    }

    pub(crate) fn render_text_frame_content(&mut self) {
        let visible_frame_starting_row = self.text_frame.start_row;
        let mut visible_frame_ending_row = visible_frame_starting_row + self.text_frame.height;
        
        if visible_frame_ending_row > self.lines.len() {
            visible_frame_ending_row = self.lines.len();
        }

        let sorted_selection_points = self.get_sorted_selection_points();

        for i in visible_frame_starting_row..visible_frame_ending_row {
            crossterm::queue!(
                stdout(),
                crossterm::cursor::MoveTo(
                    (self.term_width - self.text_frame.width) as u16,
                    (i - visible_frame_starting_row) as u16),
            ).unwrap_or_else(|_| self.panic_gracefully(&Error::err12()));
         
            let mut visible_frame_starting_col = self.text_frame.start_col;
            let mut visible_frame_ending_col = visible_frame_starting_col + self.text_frame.width;
            
            if visible_frame_starting_col > self.lines[i].len() {
                visible_frame_starting_col = self.lines[i].len();
            }
            
            if visible_frame_ending_col > self.lines[i].len() {
                visible_frame_ending_col = self.lines[i].len();
            }

            let rendered_lines_row = (i - visible_frame_starting_row) as usize;
            let mut rendered_lines_col = (self.term_width - self.text_frame.width) as usize;
         
            let mut prev_background = self.theming.text_frame_bg;
            let mut prev_foreground = self.theming.text_frame_fg;
            
            crossterm::queue!(
                stdout(),
                crossterm::style::SetBackgroundColor(prev_background),
                crossterm::style::SetForegroundColor(prev_foreground),
            ).unwrap_or_else(|_| self.panic_gracefully(&Error::err13()));

            let mut same_styled_text = String::new();

            for j in visible_frame_starting_col..visible_frame_ending_col {
                let mut background = self.lines[i][j].background;
                let mut foreground = self.lines[i][j].foreground;

                if i == self.cursor.row {
                    // background = crossterm::style::Color::DarkGrey;
                    // ^ use a darker than darkgrey and uncomment
                    background = self.theming.text_frame_highlighted_bg;
                    // foreground = self.theming.text_frame_highlighted_fg;
                }

                match &sorted_selection_points {
                    Some(selection) => {
                        if self.is_cursor_inside_selection(&selection, &Cursor{row: i, col: j}) {
                            background = self.theming.text_frame_selection_bg;
                            foreground = self.theming.text_frame_selection_fg;
                        }
                    },
                    _ => ()
                }

                if (prev_background != background || prev_foreground != foreground)
                && same_styled_text.len() > 0 {
                    crossterm::queue!(
                        stdout(),
                        crossterm::cursor::MoveTo(rendered_lines_col as u16, rendered_lines_row as u16),
                        crossterm::style::Print(same_styled_text.clone()),
                    ).unwrap_or_else(|_| self.panic_gracefully(&Error::err25()));

                    rendered_lines_col += same_styled_text.chars().count();
                    same_styled_text = String::new();
                }

                if prev_background != background {
                    crossterm::queue!(
                        stdout(),
                        crossterm::style::SetBackgroundColor(background),
                    ).unwrap_or_else(|_| self.panic_gracefully(&Error::err14()));

                    prev_background = background;
                }
                
                if prev_foreground != foreground {
                    crossterm::queue!(
                        stdout(),
                        crossterm::style::SetForegroundColor(foreground),
                    ).unwrap_or_else(|_| self.panic_gracefully(&Error::err15()));

                    prev_foreground = foreground;
                }

                

                // crossterm::queue!(
                //     stdout(),
                //     crossterm::cursor::MoveTo(rendered_lines_col as u16, rendered_lines_row as u16),
                //     crossterm::style::Print(self.lines[i][j].character),
                // )?;
                same_styled_text.push(self.lines[i][j].character);
                // rendered_lines_col += 1;
            }

            if same_styled_text.len() > 0 {
                crossterm::queue!(
                    stdout(),
                    crossterm::cursor::MoveTo(rendered_lines_col as u16, rendered_lines_row as u16),
                    crossterm::style::Print(same_styled_text.clone()),
                ).unwrap_or_else(|_| self.panic_gracefully(&Error::err16()));
            }
            
            let text_frame_start_col = (self.term_width - self.text_frame.width) as usize;
            let remaining_chars_start_col = text_frame_start_col
                + (visible_frame_ending_col - visible_frame_starting_col) as usize;
            let text_frame_end_col = (text_frame_start_col + self.text_frame.width) as usize;

            // crossterm::queue!(
            //     stdout(),
            //     crossterm::style::SetBackgroundColor(crossterm::style::Color::Black),
            //     crossterm::style::SetForegroundColor(crossterm::style::Color::DarkGrey),
            // )?;
            let mut empty_string = String::new();
            for _ in remaining_chars_start_col..text_frame_end_col {
                empty_string.push(' ');
            }
            
            if i == self.cursor.row {
                crossterm::queue!(
                    stdout(),
                    // crossterm::style::SetBackgroundColor(crossterm::style::Color::DarkGrey),
                    // ^ use a darker than darkgrey and uncomment
                    crossterm::style::SetBackgroundColor(self.theming.text_frame_highlighted_bg),
                    // crossterm::style::SetForegroundColor(self.theming.text_frame_highlighted_fg),
                ).unwrap_or_else(|_| self.panic_gracefully(&Error::err28()));
            } else {
                crossterm::queue!(
                    stdout(),
                    crossterm::style::SetBackgroundColor(self.theming.text_frame_bg),
                    // crossterm::style::SetForegroundColor(self.theming.text_frame_fg),
                ).unwrap_or_else(|_| self.panic_gracefully(&Error::err28()));
            }

            crossterm::queue!(
                stdout(),
                crossterm::cursor::MoveTo(remaining_chars_start_col as u16, rendered_lines_row as u16),
                crossterm::style::Print(empty_string),
            ).unwrap_or_else(|_| self.panic_gracefully(&Error::err17()));
        }

        let remaining_lines_start_row = (visible_frame_ending_row - visible_frame_starting_row) as usize;
        let text_frame_start_col = (self.term_width - self.text_frame.width) as usize;
        let text_frame_end_col = (text_frame_start_col + self.text_frame.width) as usize;

        crossterm::queue!(
            stdout(),
            crossterm::style::SetBackgroundColor(self.theming.text_frame_bg),
            crossterm::style::SetForegroundColor(self.theming.text_frame_fg),
        ).unwrap_or_else(|_| self.panic_gracefully(&Error::err18()));

        for i in remaining_lines_start_row..self.text_frame.height {
            // for j in text_frame_start_col..text_frame_end_col {
            //     crossterm::queue!(
            //         stdout(),
            //         crossterm::cursor::MoveTo(j as u16, i as u16),
            //         crossterm::style::Print(' '),
            //     )?;
            // }
            let mut empty_string = String::new();
            for _ in text_frame_start_col..text_frame_end_col {
                empty_string.push(' ');
            }
            crossterm::queue!(
                stdout(),
                crossterm::cursor::MoveTo(text_frame_start_col as u16, i as u16),
                crossterm::style::Print(empty_string),
            ).unwrap_or_else(|_| self.panic_gracefully(&Error::err19()));
        }
    }

    pub(crate) fn render_status_frame_content(&mut self) {
        let mut empty_string = String::new();
        for _ in 0..self.status_frame.width {
            empty_string.push(' ');
        }
        crossterm::queue!(
            stdout(),
            crossterm::style::SetBackgroundColor(self.theming.status_frame_bg),
            crossterm::style::SetForegroundColor(self.theming.status_frame_fg),
            crossterm::cursor::MoveTo(0, (self.term_height - 1) as u16),
            crossterm::style::Print(empty_string),
        ).unwrap_or_else(|_| self.panic_gracefully(&Error::err26()));

        let file_name = 
            if self.file.path != "" { String::from(&self.file.path) }
            else { String::from("[NEW]") };
        let status_string = 
            file_name + if !self.file.is_saved { &"* - "} else { &" - "}
            + &String::from("Ln:") + &(self.cursor.row + 1).to_string()
            + &String::from(", Col:") + &(self.cursor.col + 1).to_string();
        
        crossterm::queue!(
            stdout(),
            crossterm::cursor::MoveTo(0, (self.term_height - 1) as u16),
            crossterm::style::Print(status_string),
        ).unwrap_or_else(|_| self.panic_gracefully(&Error::err27()));
    }

    pub(crate) fn render_unsaved_changes_frame(&mut self) {
        crossterm::queue!(
            stdout(),
            crossterm::style::SetBackgroundColor(crossterm::style::Color::Black),
            crossterm::style::SetForegroundColor(crossterm::style::Color::White),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::style::SetBackgroundColor(crossterm::style::Color::White),
            crossterm::style::SetForegroundColor(crossterm::style::Color::Black),
            crossterm::cursor::MoveTo(0, 0),
            crossterm::style::Print("UNSAVED CHANGES"),
            crossterm::style::SetBackgroundColor(crossterm::style::Color::Black),
            crossterm::style::SetForegroundColor(crossterm::style::Color::White),
            crossterm::style::Print("\n\n"),
            crossterm::cursor::MoveToColumn(0),
            crossterm::style::Print("Would you like to save changes before you quit?"),
            crossterm::style::Print("\n\n"),
            crossterm::cursor::MoveToColumn(0),
            crossterm::style::Print("[Y] yes, [N] no, [Esc] go back"),
            crossterm::style::Print("\n\n"),
            crossterm::cursor::MoveToColumn(0),
            crossterm::style::Print("> "),
        ).unwrap_or_else(|_| self.panic_gracefully(&Error::err20()));

        stdout().flush()
            .unwrap_or_else(|_| self.panic_gracefully(&Error::err21()));
    }

    pub(crate) fn render_save_as_frame(&mut self) {
        crossterm::queue!(
            stdout(),
            crossterm::style::SetBackgroundColor(crossterm::style::Color::Black),
            crossterm::style::SetForegroundColor(crossterm::style::Color::White),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::style::SetBackgroundColor(crossterm::style::Color::White),
            crossterm::style::SetForegroundColor(crossterm::style::Color::Black),
            crossterm::cursor::MoveTo(0, 0),
            crossterm::style::Print("SAVE FILE"),
            crossterm::style::SetBackgroundColor(crossterm::style::Color::Black),
            crossterm::style::SetForegroundColor(crossterm::style::Color::White),
            crossterm::style::Print("\n\n"),
            crossterm::cursor::MoveToColumn(0),
            crossterm::style::Print("Enter file name."),
            crossterm::style::Print("\n\n"),
            crossterm::cursor::MoveToColumn(0),
            crossterm::style::Print("[Enter] save, [Esc] go back"),
            crossterm::style::Print("\n\n"),
            crossterm::cursor::MoveToColumn(0),
            crossterm::style::Print("> ".to_string() + self.file.path.as_str()),
        ).unwrap_or_else(|_| self.panic_gracefully(&Error::err22()));

        stdout().flush()
            .unwrap_or_else(|_| self.panic_gracefully(&Error::err23()));
    }

    pub(crate) fn update_visible_cursor(&mut self) {
        let col = (self.cursor.col - self.text_frame.start_col) + (self.line_nums_frame.width);
        let row = self.cursor.row - self.text_frame.start_row;

        // let mut background = crossterm::style::Color::Black;
        // let mut foreground = crossterm::style::Color::White;
        // let mut cursor_char = ' ';

        // if self.cursor.col < self.lines[self.cursor.row].len() {
        //     background = self.lines[self.cursor.row][self.cursor.col].background;
        //     foreground = self.lines[self.cursor.row][self.cursor.col].foreground;
        //     cursor_char = self.lines[self.cursor.row][self.cursor.col].character;
        // }

        // if self.selection.is_selected {
        //     if self.is_cursor_inside_selection(&self.get_sorted_selection_points().unwrap(), &self.cursor) {
        //         background = crossterm::style::Color::White;
        //         foreground = crossterm::style::Color::Black;
        //     }
        // }

        crossterm::queue!(
            stdout(),
            crossterm::cursor::MoveTo(col as u16, row as u16),
            // crossterm::style::SetBackgroundColor(background),
            // crossterm::style::SetForegroundColor(foreground),
            // crossterm::style::SetAttribute(crossterm::style::Attribute::Underlined),
            // crossterm::style::Print(cursor_char),
            // crossterm::style::SetAttribute(crossterm::style::Attribute::Reset),
            // crossterm::cursor::MoveTo(col as u16, row as u16),
            crossterm::cursor::Show,
        ).unwrap_or_else(|_| self.panic_gracefully(&Error::err24()));
    }
}