use super::*;

static SPECIAL_CHARS: [char; 29] = 
    ['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', 
    '=', '+', '[', ']', '{', '}', ';', ':', '\'', ',', '.', '<', '>', 
    '/', '?', '\\', '|'];

impl Lino {
    
    pub(crate) fn move_cursor_to_line_start(&mut self) {
        self.cursor.col = 0;
    }

    pub(crate) fn move_cursor_to_line_end(&mut self) {
        self.cursor.col = self.lines[self.cursor.row].len();
    }

    pub(crate) fn move_cursor_up_by_page(&mut self) {
        if self.cursor.row as isize - self.text_frame.height as isize > 0 {
            self.cursor.row = self.cursor.row - self.text_frame.height;
        } else {
            self.cursor.row = 0;
        }
        
        if self.cursor.col > self.lines[self.cursor.row].len() {
            self.cursor.col = self.lines[self.cursor.row].len();
        }
    }

    pub(crate) fn move_cursor_down_by_page(&mut self) {
        if self.cursor.row as isize + self.text_frame.height as isize <= (self.lines.len() - 1) as isize {
            self.cursor.row = self.cursor.row + self.text_frame.height;
        } else {
            self.cursor.row = self.lines.len() - 1;
        }

        if self.cursor.col > self.lines[self.cursor.row].len() {
            self.cursor.col = self.lines[self.cursor.row].len();
        }
    }

    pub(crate) fn reset_cursor(&mut self) {
        self.cursor.row = 0;
        self.cursor.col = 0;
    }

    pub(crate) fn move_cursor_left(&mut self) {
        let is_first_line = self.cursor.row == 0;
        let is_cursor_at_line_start = self.cursor.col == 0;
        let is_cursor_mid_line_or_end = !is_cursor_at_line_start
            && self.cursor.col <= self.lines[self.cursor.row].len();

        if is_first_line && is_cursor_at_line_start {
            return;
        }

        if !is_first_line && is_cursor_at_line_start {
            self.cursor.row -= 1;
            self.cursor.col = self.lines[self.cursor.row].len();
            return;
        }

        if is_cursor_mid_line_or_end {
            self.cursor.col -= 1;
            return;
        }
    }

    pub(crate) fn move_cursor_right(&mut self) {
        let is_last_line = self.cursor.row == self.lines.len() - 1;
        let is_cursor_at_line_end = self.cursor.col == self.lines[self.cursor.row].len();
        let is_cursor_mid_line_or_start = self.cursor.col < self.lines[self.cursor.row].len();
        
        if is_last_line && is_cursor_at_line_end {
            return;
        }
        
        if !is_last_line && is_cursor_at_line_end {
            self.cursor.row += 1;
            self.cursor.col = 0;
            return;
        }
        
        if is_cursor_mid_line_or_start {
            self.cursor.col += 1;
            return;
        }
    }

    pub(crate) fn move_cursor_left_by_word(&mut self) {
        if self.is_cursor_at_line_start() {
            self.move_cursor_left();
            return;
        }

        self.move_cursor_left();
        
        loop {
            if self.is_cursor_at_line_start() {
                break;
            }

            if self.lines[self.cursor.row][self.cursor.col].character != ' ' 
            && self.lines[self.cursor.row][self.cursor.col - 1].character == ' ' {
                break;
            }

            if !SPECIAL_CHARS.contains(&self.lines[self.cursor.row][self.cursor.col].character)
            && SPECIAL_CHARS.contains(&self.lines[self.cursor.row][self.cursor.col - 1].character) {
                break;
            }

            self.move_cursor_left();
        }
    }

    pub(crate) fn move_cursor_right_by_word(&mut self) {
        if self.is_cursor_at_line_end() {
            self.move_cursor_right();
            return;
        }

        self.move_cursor_right();
        
        loop {
            if self.is_cursor_at_line_end() {
                break;
            }

            if self.lines[self.cursor.row][self.cursor.col].character == ' ' 
            && self.lines[self.cursor.row][self.cursor.col - 1].character != ' ' {
                break;
            }

            if SPECIAL_CHARS.contains(&self.lines[self.cursor.row][self.cursor.col].character)
            && !SPECIAL_CHARS.contains(&self.lines[self.cursor.row][self.cursor.col - 1].character) {
                break;
            }
            
            self.move_cursor_right();
        }
    }

    pub(crate) fn move_cursor_up(&mut self) {
        let is_first_line = self.cursor.row == 0;
                
        if !is_first_line {
            self.cursor.row -= 1;

            let is_cursor_after_line_end = self.cursor.col > self.lines[self.cursor.row].len();

            if is_cursor_after_line_end {
                self.cursor.col = self.lines[self.cursor.row].len();
            }
        }
    }

    pub(crate) fn move_cursor_down(&mut self) {
        let is_last_line = self.cursor.row == self.lines.len() - 1;
        
        if !is_last_line {
            self.cursor.row += 1;

            let is_cursor_after_line_end = self.cursor.col > self.lines[self.cursor.row].len();
            
            if is_cursor_after_line_end {
                self.cursor.col = self.lines[self.cursor.row].len();
            }
        }
    }
    
    pub(crate) fn update_last_cursor_col(&mut self) {
        self.last_cursor_col = self.cursor.col;
    }

    pub(crate) fn restore_last_cursor_col_if_applicable(&mut self) {
        if self.last_cursor_col <= self.lines[self.cursor.row].len() {
            self.cursor.col = self.last_cursor_col;
        } else {
            self.cursor.col = self.lines[self.cursor.row].len();
        }
    }

    pub(crate) fn is_cursor_at_line_start(&self) -> bool {
        return self.cursor.col == 0;
    }

    pub(crate) fn is_cursor_at_line_end(&self) -> bool {
        return self.cursor.col == self.lines[self.cursor.row].len();
    }

    pub(crate) fn is_current_line_empty(&self) -> bool {
        return self.lines[self.cursor.row].len() == 0;
    }

    pub(crate) fn is_cursor_at_first_line(&self) -> bool {
        return self.cursor.row == 0;
    }
    
    pub(crate) fn is_cursor_at_last_line(&self) -> bool {
        return self.cursor.row == self.lines.len() - 1;
    }
    
    pub(crate) fn is_cursor_at_file_end(&self) -> bool {
        return self.is_cursor_at_last_line() && self.is_cursor_at_line_end();
    }

    pub(crate) fn is_document_empty(&self) -> bool {
        return self.lines.len() == 1 && self.lines[0].len() == 0;
    }

    pub(crate) fn is_cursor_greater_than(&self, other_cursor: &Cursor) -> bool {
        return self.cursor.row > other_cursor.row
        || (self.cursor.row == other_cursor.row && self.cursor.col > other_cursor.col);
    }
    
    pub(crate) fn is_cursor_lesser_than(&self, other_cursor: &Cursor) -> bool {
        return self.cursor.row < other_cursor.row
        || (self.cursor.row == other_cursor.row && self.cursor.col < other_cursor.col);
    }

    pub(crate) fn is_cursor_inside_selection(&self, selection: &Selection, cursor: &Cursor) -> bool {
        let is_single_line_selection_and_cursor_inside_points = 
            cursor.row == selection.start_point.row && cursor.row == selection.end_point.row
            && cursor.col >= selection.start_point.col && cursor.col <= selection.end_point.col;
        if is_single_line_selection_and_cursor_inside_points {
            return true;
        }

        let is_multi_line_selection_and_cursor_after_start_point_of_first_line = 
            cursor.row == selection.start_point.row && cursor.row < selection.end_point.row
            && cursor.col >= selection.start_point.col;
        if is_multi_line_selection_and_cursor_after_start_point_of_first_line {
            return true;
        }

        let is_multi_line_selection_and_cursor_before_end_point_of_last_line = 
            cursor.row > selection.start_point.row && cursor.row == selection.end_point.row
            && cursor.col <= selection.end_point.col;
        if is_multi_line_selection_and_cursor_before_end_point_of_last_line {
            return true;
        }

        let is_multi_line_selection_and_cursor_inside_points = 
            cursor.row > selection.start_point.row && cursor.row < selection.end_point.row;
        if is_multi_line_selection_and_cursor_inside_points {
            return true;
        }
        
        false
    }
}