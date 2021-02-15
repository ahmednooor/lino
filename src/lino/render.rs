use std::io::{stdout, Write};
use crossterm;
// extern crate copypasta;

use super::*;

impl Lino {
    pub(crate) fn render(&mut self, syntect_config: &mut SyntectConfig) {
        self.rendering.is_rendering = true;
        self.update_terminal_size();
        self.update_status_frame();
        self.update_line_nums_frame();
        self.update_text_frame();

        // crossterm::queue!(
        //     stdout(),
        //     // crossterm::style::SetBackgroundColor(crossterm::style::Color::White),
        //     // crossterm::style::SetForegroundColor(crossterm::style::Color::White),
        //     // crossterm::style::Print(' '),
        //     crossterm::cursor::Hide,
        //     crossterm::cursor::MoveTo(0, 0),
        // ).unwrap_or_else(|_| self.panic_gracefully(&Error::err7()));

        // self.apply_syntax_highlighting_on_current_line(syntect_config);
        // self.apply_syntax_highlighting_on_changed_lines(syntect_config, previous_lines);
        self.apply_syntax_highlighting_on_lines_range(syntect_config);
        
        self.populate_line_nums_frame_in_render_buffer();
        self.populate_text_frame_in_render_buffer();
        self.populate_status_frame_in_render_buffer();
        self.render_updated_buffer();
        self.update_visible_cursor();

        stdout().flush()
            .unwrap_or_else(|_| self.panic_gracefully(&Error::err8()));
        
        self.rendering.is_rendering = false;
    }

    pub(crate) fn populate_line_nums_frame_in_render_buffer(&mut self) {
        if !self.settings.show_line_nums_frame {
            return;
        }

        let line_num_width = self.lines.len().to_string().len();

        for i in 0..self.line_nums_frame.height {
            let mut background = self.theming.line_nums_frame_bg;
            let mut foreground = self.theming.line_nums_frame_fg;
            let row = self.text_frame.start_row + i;

            if row >= self.lines.len() {
                for j in 0..self.line_nums_frame.width {
                    if j == self.line_nums_frame.width - 1 && self.line_nums_frame.boundary_r != "" {
                        background = self.theming.text_frame_bg;
                    }

                    self.rendering.buffer[i][j] = Character{
                        background: background,
                        foreground: foreground,
                        character: ' ',
                        width: 1,
                    };
                }
                continue;
            }

            let num_string_with_boundary = 
                format!(" {:width$} ", row + 1, width = line_num_width) 
                + &self.line_nums_frame.boundary_r;

            let num_string_with_boundary: Vec<char> = num_string_with_boundary.chars().collect();

            for j in 0..line_num_width+2 {
                if row == self.cursor.row {
                    background = self.theming.line_nums_frame_highlighted_bg;
                    foreground = self.theming.line_nums_frame_highlighted_fg;
                }
                self.rendering.buffer[i][j] = Character{
                    background: background,
                    foreground: foreground,
                    character: num_string_with_boundary.get(j).unwrap_or(&' ').clone(),
                    width: 1,
                };
            }
            
            for j in line_num_width+2..num_string_with_boundary.len() {
                background = self.theming.text_frame_bg;
                foreground = self.theming.line_nums_frame_fg;

                // if row == self.cursor.row {
                //     background = self.theming.line_nums_frame_highlighted_bg;
                // }

                self.rendering.buffer[i][j] = Character{
                    background: background,
                    foreground: foreground,
                    character: num_string_with_boundary.get(j).unwrap_or(&' ').clone(),
                    width: 1,
                };
            }
        }
    }

    pub(crate) fn populate_text_frame_in_render_buffer(&mut self) {
        let sorted_selection_points = self.get_sorted_selection_points();

        for i in 0..self.text_frame.height {
            let mut background = self.theming.text_frame_bg;
            let mut foreground = self.theming.text_frame_fg;
            let row = self.text_frame.start_row + i;

            if row >= self.lines.len() {
                for j in self.line_nums_frame.width..self.line_nums_frame.width+self.text_frame.width {
                    self.rendering.buffer[i][j] = Character{
                        background: background,
                        foreground: foreground,
                        character: ' ',
                        width: 1,
                    };
                }
                continue;
            }

            for j in self.line_nums_frame.width..self.line_nums_frame.width+self.text_frame.width {
                let col = self.text_frame.start_col + j - self.line_nums_frame.width;
                let mut character = ' ';
                let mut width = 1;
                background = self.theming.text_frame_bg;

                if row == self.cursor.row {
                    background = self.theming.text_frame_highlighted_bg;
                }

                if col < self.lines[row].len() {
                    foreground = self.lines[row][col].foreground;
                    character = self.lines[row][col].character;
                    width = self.lines[row][col].width;
                }

                if self.find.is_finding {
                    for instance in &self.find.found_instances {
                        if row < instance.start.row || row > instance.end.row {
                            continue;
                        }
                        let selection = Selection{
                            is_selected: true,
                            start_point: instance.start.clone(),
                            end_point: instance.end.clone(),
                        };
                        if self.is_cursor_inside_selection(&selection, &Cursor{row: row, col: col}) {
                            background = self.theming.text_frame_found_text_bg;
                            foreground = self.theming.text_frame_found_text_fg;
                        }
                    }
                }

                match &sorted_selection_points {
                    Some(selection) => {
                        if self.is_cursor_inside_selection(&selection, &Cursor{row: row, col: col})
                        && ((row < self.lines.len() - 1 && col <= self.lines[row].len())
                        || (row == self.lines.len() - 1 && col < self.lines[row].len())) {
                            background = self.theming.text_frame_selection_bg;
                            foreground = self.theming.text_frame_selection_fg;
                        }
                    },
                    _ => ()
                }

                self.rendering.buffer[i][j] = Character{
                    background: background,
                    foreground: foreground,
                    character: character,
                    width: width as u8,
                };

            }
        }
    }
    
    pub(crate) fn populate_status_frame_in_render_buffer(&mut self) {
        if !self.settings.show_status_frame {
            return;
        }

        let status_string = self.make_status_bar_string();

        let background = self.theming.status_frame_bg;
        let foreground = self.theming.status_frame_fg;

        for i in self.term_height-self.status_frame.height..self.term_height-1 {
            for j in 0..self.status_frame.width {
                let character = status_string.chars().nth(j);
                
                if character.is_none() {
                    self.rendering.buffer[i][j] = Character{
                        background: background,
                        foreground: foreground,
                        character: ' ',
                        width: 1,
                    };
                    continue;
                }

                let character = character.unwrap();
                
                self.rendering.buffer[i][j] = Character{
                    background: background,
                    foreground: foreground,
                    character: character,
                    width: 1,
                };
            }
        }

        let task_feedback = self.get_task_feedback();

        let background = self.task_feedback.bg;
        let foreground = self.task_feedback.fg;

        for i in self.term_height-self.status_frame.height+1..self.term_height {
            for j in 0..self.status_frame.width {
                let character = task_feedback.chars().nth(j);
                
                if character.is_none() {
                    self.rendering.buffer[i][j] = Character{
                        background: background,
                        foreground: foreground,
                        character: ' ',
                        width: 1,
                    };
                    continue;
                }
                
                let character = character.unwrap();
                
                self.rendering.buffer[i][j] = Character{
                    background: background,
                    foreground: foreground,
                    character: character,
                    width: 1,
                };
            }
        }
    }

    pub(crate) fn make_status_bar_string(&mut self) -> String {
        if self.find.is_finding {
            return self.make_find_mode_status_bar_string();
        }
        
        let file_name = 
            if self.file.path != "" { String::from(&self.file.path) }
            else { String::from("[NEW]") };
        let status_string = 
            file_name + if !self.file.is_saved { &"* - "} else { &" - "}
            + &String::from("Ln:") + &(self.cursor.row + 1).to_string()
            + &String::from(", Col:") + &(self.cursor.col + 1).to_string();
        
        let status_string = if status_string.len() > self.term_width {
            status_string[status_string.len()-self.term_width..].to_string()
        } else {
            status_string
        };
        
        status_string
    }

    pub(crate) fn make_find_mode_status_bar_string(&mut self) -> String {
        if !self.find.is_finding || self.find.found_instances.len() < 1 {
            return "".to_string();
        }

        let status_string = 
            format!("Finds: {}/{} - [Down] Next, [Up] Previous",
            self.find.selected_instance_index + 1, self.find.found_instances.len());
        
        let status_string = if status_string.len() > self.term_width {
            status_string[..self.term_width].to_string()
        } else {
            status_string
        };
        
        status_string
    }

    pub(crate) fn get_task_feedback(&mut self) -> String {
        if self.find.is_finding && !self.settings.read_only {
            self.set_task_feedback_normal("[Enter] Edit Current, [Ctrl+R] Replace All, [Esc] Cancel".to_string());
        } else if self.find.is_finding && self.settings.read_only {
            self.set_task_feedback_normal("[Esc] Cancel".to_string());
        }

        let task_feedback = if self.task_feedback.text == "" {
            self.task_feedback.default_text.clone()
        } else {
            self.task_feedback.text.clone()
        };

        let task_feedback = if task_feedback.len() > self.term_width {
            task_feedback[..self.term_width].to_string()
        } else {
            task_feedback
        };
        
        task_feedback
    }

    pub(crate) fn render_updated_buffer(&mut self) {
        let mut prev_background = self.theming.text_frame_bg;
        let mut prev_foreground = self.theming.text_frame_fg;
        
        crossterm::queue!(
            stdout(),
            crossterm::cursor::Hide,
            crossterm::cursor::MoveTo(0, 0),
            crossterm::style::SetBackgroundColor(prev_background),
            crossterm::style::SetForegroundColor(prev_foreground),
            crossterm::terminal::DisableLineWrap,
        ).unwrap_or_else(|_| self.panic_gracefully(&Error::err13()));

        for i in 0..self.rendering.buffer.len() {
            let mut same_styled_text = String::new();
            // let row = i;
            // let mut col = 0;
            
            for j in 0..self.rendering.buffer[i].len() {
                let background = self.rendering.buffer[i][j].background;
                let foreground = self.rendering.buffer[i][j].foreground;

                if (prev_background != background || prev_foreground != foreground)
                && same_styled_text.len() > 0 {
                    crossterm::queue!(
                        stdout(),
                        // crossterm::cursor::MoveTo(col as u16, row as u16),
                        crossterm::style::Print(same_styled_text),
                    ).unwrap_or_else(|_| self.panic_gracefully(&Error::err25()));

                    // col += same_styled_text.chars().count();
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

                same_styled_text.push(self.rendering.buffer[i][j].character);
            }

            // if same_styled_text.len() > 0 {
                crossterm::queue!(
                    stdout(),
                    // crossterm::cursor::MoveTo(col as u16, row as u16),
                    crossterm::style::Print(same_styled_text),
                    crossterm::cursor::MoveDown(1),
                    crossterm::cursor::MoveToColumn(0),
                    // crossterm::style::ResetColor,
                ).unwrap_or_else(|_| self.panic_gracefully(&Error::err16()));
                // }
        }
        
        crossterm::queue!(
            stdout(),
            crossterm::terminal::EnableLineWrap,
        ).unwrap_or_else(|_| self.panic_gracefully(&Error::err16()));
    }

    pub(crate) fn update_visible_cursor(&mut self) {
        let mut col = (self.cursor.col - self.text_frame.start_col) + self.line_nums_frame.width;
        let row = self.cursor.row - self.text_frame.start_row;
        
        for i in self.text_frame.start_col..self.cursor.col {
            if self.lines[self.cursor.row][i].width > 1 {
                col += (self.lines[self.cursor.row][i].width - 1) as usize;
            }
        }

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

    pub(crate) fn init_new_render_buffer(&mut self) {
        self.rendering.buffer = vec![];

        for _ in 0..self.term_height {
            self.rendering.buffer.push(vec![]);
            
            for _ in 0..self.term_width {
                self.rendering.buffer.last_mut().unwrap().push(Character{
                    background: self.theming.line_nums_frame_bg,
                    foreground: self.theming.line_nums_frame_fg,
                    character: ' ',
                    width: 1,
                });
            }
        }
    }

    // pub(crate) fn has_render_buffer_changed(&mut self, old_render_buffer: Vec<Vec<Character>>) -> bool {
    //     for i in 0..self.rendering.buffer.len() {
    //         for j in 0..self.rendering.buffer[i].len() {
                
    //             if old_render_buffer[i][j].background != self.rendering.buffer[i][j].background {
    //                 return true;
    //             }
                
    //             if old_render_buffer[i][j].foreground != self.rendering.buffer[i][j].foreground {
    //                 return true;
    //             }
                
    //             if old_render_buffer[i][j].character != self.rendering.buffer[i][j].character {
    //                 return true;
    //             }

    //             if old_render_buffer[i][j].width != self.rendering.buffer[i][j].width {
    //                 return true;
    //             }
    //         }
    //     }

    //     false
    // }
    
}