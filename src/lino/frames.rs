use crossterm;

use super::*;

impl Lino {
    pub(crate) fn update_terminal_size(&mut self) {
        let (term_width, term_height) = crossterm::terminal::size().unwrap_or((80, 40));
        self.term_width = term_width as usize;
        self.term_height = term_height as usize;
        self.init_new_render_buffer();
    }
    
    pub(crate) fn update_status_frame(&mut self) {
        if !self.settings.show_status_frame {
            self.status_frame.width = 0;
            self.status_frame.height = 0;
            return;
        }

        self.status_frame.width = self.term_width;
        self.status_frame.height = 2;
    }

    pub(crate) fn update_line_nums_frame(&mut self) {
        if !self.settings.show_line_nums_frame {
            self.line_nums_frame.boundary_r = String::from("");
            self.line_nums_frame.width = 0;
            self.line_nums_frame.height = 0;
            return;
        }

        let mut should_update_text_frame = false;
        
        if self.text_frame.start_col > 0 && self.line_nums_frame.boundary_r != String::from("") {
            self.line_nums_frame.boundary_r = String::from("");
            should_update_text_frame = true;
        } else if self.text_frame.start_col == 0 && self.line_nums_frame.boundary_r != String::from(" ") {
            self.line_nums_frame.boundary_r = String::from(" ");
            should_update_text_frame = true;
        }
        
        self.line_nums_frame.width = self.lines.len().to_string().len() + 2 + self.line_nums_frame.boundary_r.len();
        self.line_nums_frame.height = self.term_height - self.status_frame.height;
        
        if should_update_text_frame {
            self.update_text_frame();
        }
    }

    pub(crate) fn update_text_frame(&mut self) {
        self.text_frame.width = self.term_width - self.line_nums_frame.width;
        self.text_frame.height = self.term_height - self.status_frame.height;

        let row_offset = 2;
        let is_cursor_up_from_frame = self.cursor.row < self.text_frame.start_row + row_offset;
        let is_cursor_down_from_frame = self.cursor.row > self.text_frame.start_row + self.text_frame.height - 1 - row_offset;

        if is_cursor_up_from_frame {
            while self.text_frame.start_row + row_offset > self.cursor.row {
                if self.text_frame.start_row < 1 {
                    break;
                }
                self.text_frame.start_row -= 1;
            }
        }
        if is_cursor_down_from_frame {
            while self.text_frame.start_row + self.text_frame.height - 1 - row_offset < self.cursor.row {
                self.text_frame.start_row += 1;
            }
        }

        let col_offset = 4;
        let is_cursor_left_from_frame = self.cursor.col < self.text_frame.start_col + col_offset;
        let is_cursor_right_from_frame = self.cursor.col > self.text_frame.start_col + self.text_frame.width - 1 - col_offset;

        if is_cursor_left_from_frame {
            while self.text_frame.start_col + col_offset > self.cursor.col {
                if self.text_frame.start_col < 1 {
                    break;
                }
                self.text_frame.start_col -= 1;
            }
        }
        if is_cursor_right_from_frame {
            while self.text_frame.start_col + self.text_frame.width - 1 - col_offset < self.cursor.col {
                self.text_frame.start_col += 1;
            }
        }

        let mut unicode_width_offset = 0;
        for i in self.text_frame.start_col..self.cursor.col {
            let char_width = self.lines[self.cursor.row][i].width as usize;
            if char_width > 1 {
                unicode_width_offset += char_width - 1;
            }
        }

        let is_cursor_right_from_frame = self.cursor.col > self.text_frame.start_col + self.text_frame.width - 1 - col_offset - unicode_width_offset;

        if is_cursor_right_from_frame {
            while self.text_frame.start_col + self.text_frame.width - 1 - col_offset - unicode_width_offset < self.cursor.col {
                self.text_frame.start_col += 1;
            }
        }

        self.update_line_nums_frame();
    }
}
