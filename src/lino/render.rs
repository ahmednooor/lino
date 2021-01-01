use std::io::{stdout, Write};
use crossterm;
extern crate copypasta;

use super::*;

impl Lino {
    pub(crate) fn render(&mut self) -> crossterm::Result<()> {
        self.is_rendering = true;
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
        )?;

        self.render_line_nums_frame_content()?;
        self.render_text_frame_content()?;
        self.render_status_frame_content()?;
        self.update_visible_cursor()?;

        stdout().flush()?;
        
        self.is_rendering = false;
        Ok(())
    }

    pub(crate) fn render_line_nums_frame_content(&mut self) -> crossterm::Result<()> {
        crossterm::queue!(
            stdout(),
            crossterm::style::SetBackgroundColor(crossterm::style::Color::Black),
            crossterm::style::SetForegroundColor(crossterm::style::Color::DarkGrey),
        )?;

        let visible_frame_starting_line_num = self.text_frame.start_row;
        let mut visible_frame_ending_line_num = visible_frame_starting_line_num + self.text_frame.height;
        
        if visible_frame_ending_line_num > self.lines.len() {
            visible_frame_ending_line_num = self.lines.len();
        }
        
        let line_num_width = self.lines.len().to_string().len();
        
        for i in visible_frame_starting_line_num..visible_frame_ending_line_num {
            let rendered_lines_row = (i - visible_frame_starting_line_num) as usize;
            let mut rendered_lines_col = 
                (self.term_width - self.text_frame.width - self.line_nums_frame.width) as usize;

            let num_string = format!(" {:width$}", i + 1, width = line_num_width);
            let num_string = String::from(num_string) + &self.line_nums_frame.boundary_r;

            for line_num_char in num_string.chars() {
                crossterm::queue!(
                    stdout(),
                    crossterm::cursor::MoveTo(rendered_lines_col as u16, rendered_lines_row as u16),
                    crossterm::style::Print(line_num_char),
                )?;
                rendered_lines_col += 1;
            }
        }

        let remaining_lines_start_row = (visible_frame_ending_line_num - visible_frame_starting_line_num) as usize;
        let line_nums_frame_start_col = 
            (self.term_width - self.text_frame.width - self.line_nums_frame.width) as usize;
        let line_nums_frame_end_col = (line_nums_frame_start_col + self.line_nums_frame.width) as usize;

        for i in remaining_lines_start_row..self.line_nums_frame.height {
            for j in line_nums_frame_start_col..line_nums_frame_end_col {
                crossterm::queue!(
                    stdout(),
                    crossterm::cursor::MoveTo(j as u16, i as u16),
                    crossterm::style::Print(' '),
                )?;
            }
        }

        Ok(())
    }

    pub(crate) fn render_text_frame_content(&mut self) -> crossterm::Result<()> {
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
            )?;
         
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
         
            let mut prev_background = crossterm::style::Color::Black;
            let mut prev_foreground = crossterm::style::Color::DarkGrey;

            for j in visible_frame_starting_col..visible_frame_ending_col {
                let mut background = self.lines[i][j].background;
                let mut foreground = self.lines[i][j].foreground;

                match &sorted_selection_points {
                    Some(selection) => {
                        if self.is_cursor_inside_selection(&selection, &Cursor{row: i, col: j}) {
                            background = crossterm::style::Color::White;
                            foreground = crossterm::style::Color::Black;
                        }
                    },
                    _ => ()
                }

                if prev_background != background {
                    crossterm::queue!(
                        stdout(),
                        crossterm::style::SetBackgroundColor(background),
                    )?;
                    prev_background = background;
                }
                
                if prev_foreground != foreground {
                    crossterm::queue!(
                        stdout(),
                        crossterm::style::SetForegroundColor(foreground),
                    )?;
                    prev_foreground = foreground;
                }

                crossterm::queue!(
                    stdout(),
                    crossterm::cursor::MoveTo(rendered_lines_col as u16, rendered_lines_row as u16),
                    crossterm::style::Print(self.lines[i][j].character),
                )?;
                rendered_lines_col += 1;
            }
            
            let text_frame_start_col = (self.term_width - self.text_frame.width) as usize;
            let remaining_chars_start_col = text_frame_start_col
                + (visible_frame_ending_col - visible_frame_starting_col) as usize;
            let text_frame_end_col = (text_frame_start_col + self.text_frame.width) as usize;

            crossterm::queue!(
                stdout(),
                crossterm::style::SetBackgroundColor(crossterm::style::Color::Black),
                crossterm::style::SetForegroundColor(crossterm::style::Color::DarkGrey),
            )?;
            for j in remaining_chars_start_col..text_frame_end_col {
                crossterm::queue!(
                    stdout(),
                    crossterm::cursor::MoveTo(j as u16, rendered_lines_row as u16),
                    crossterm::style::Print(' '),
                )?;
            }
        }

        let remaining_lines_start_row = (visible_frame_ending_row - visible_frame_starting_row) as usize;
        let text_frame_start_col = (self.term_width - self.text_frame.width) as usize;
        let text_frame_end_col = (text_frame_start_col + self.text_frame.width) as usize;

        crossterm::queue!(
            stdout(),
            crossterm::style::SetBackgroundColor(crossterm::style::Color::Black),
            crossterm::style::SetForegroundColor(crossterm::style::Color::DarkGrey),
        )?;

        for i in remaining_lines_start_row..self.text_frame.height {
            for j in text_frame_start_col..text_frame_end_col {
                crossterm::queue!(
                    stdout(),
                    crossterm::cursor::MoveTo(j as u16, i as u16),
                    crossterm::style::Print(' '),
                )?;
            }
        }

        Ok(())
    }

    pub(crate) fn render_status_frame_content(&mut self) -> crossterm::Result<()> {
        crossterm::queue!(
            stdout(),
            crossterm::style::SetBackgroundColor(crossterm::style::Color::White),
            crossterm::style::SetForegroundColor(crossterm::style::Color::Black),
        )?;
        
        for i in 0..self.status_frame.width {
            crossterm::queue!(
                stdout(),
                crossterm::cursor::MoveTo(i as u16, (self.term_height - 1) as u16),
                crossterm::style::Print(' '),
            )?;
        }

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
        )?;

        Ok(())
    }

    pub(crate) fn render_unsaved_changes_frame(&mut self) -> crossterm::Result<()> {
        crossterm::queue!(
            stdout(),
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
        )?;

        stdout().flush()?;

        Ok(())
    }

    pub(crate) fn render_save_as_frame(&mut self) -> crossterm::Result<()> {
        crossterm::queue!(
            stdout(),
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
        )?;

        stdout().flush()?;

        Ok(())
    }

    pub(crate) fn update_visible_cursor(&mut self) -> crossterm::Result<()> {
        let col = (self.cursor.col - self.text_frame.start_col) + (self.line_nums_frame.width);
        let row = self.cursor.row - self.text_frame.start_row;

        let mut background = crossterm::style::Color::Black;
        let mut foreground = crossterm::style::Color::White;
        let mut cursor_char = ' ';

        if self.cursor.col < self.lines[self.cursor.row].len() {
            background = self.lines[self.cursor.row][self.cursor.col].background;
            foreground = self.lines[self.cursor.row][self.cursor.col].foreground;
            cursor_char = self.lines[self.cursor.row][self.cursor.col].character;
        }

        if self.selection.is_selected {
            if self.is_cursor_inside_selection(&self.get_sorted_selection_points().unwrap(), &self.cursor) {
                background = crossterm::style::Color::White;
                foreground = crossterm::style::Color::Black;
            }
        }

        crossterm::queue!(
            stdout(),
            crossterm::cursor::MoveTo(col as u16, row as u16),
            crossterm::style::SetBackgroundColor(background),
            crossterm::style::SetForegroundColor(foreground),
            crossterm::style::SetAttribute(crossterm::style::Attribute::Underlined),
            crossterm::style::Print(cursor_char),
            crossterm::style::SetAttribute(crossterm::style::Attribute::Reset),
            crossterm::cursor::MoveTo(col as u16, row as u16),
            crossterm::cursor::Show,
        )?;

        Ok(())
    }
}