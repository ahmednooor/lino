use super::*;

impl Lino {
    pub(crate) fn select_all(&mut self) {
        if self.is_document_empty() {
            self.clear_selection(&self.cursor.clone());
            return;
        }
        
        self.selection.is_selected = true;
        self.selection.start_point.row = 0;
        self.selection.start_point.col = 0;

        if self.lines.len() > 0 {
            self.selection.end_point.row = self.lines.len() - 1;
        } else {
            self.selection.end_point.row = self.lines.len();
        }

        self.selection.end_point.col = 0;
        if self.lines[self.selection.end_point.row].len() > 0 {
            self.selection.end_point.col = self.lines[self.selection.end_point.row].len() - 1;
        }
        self.cursor = self.selection.end_point.clone();
    }
    
    pub(crate) fn make_selection(&mut self, previous_cursor: &Cursor) {
        if self.is_document_empty() { 
            self.clear_selection(&self.cursor.clone());
            return;
        }

        if !self.selection.is_selected {
            self.selection.is_selected = true;
            self.selection.start_point = previous_cursor.clone();

            let is_selecting_backward = 
                self.is_cursor_lesser_than(&self.selection.start_point);
            if is_selecting_backward {
                let cursor_backup = self.cursor.clone();
                self.cursor = self.selection.start_point.clone();
                self.move_cursor_left();
                self.selection.start_point = self.cursor.clone();
                self.cursor = cursor_backup.clone();
            }
        }

        self.selection.is_selected = true;
        self.selection.end_point = self.cursor.clone();

        let is_selecting_forward = 
            self.is_cursor_greater_than(&self.selection.start_point);
        if is_selecting_forward {
            self.move_cursor_left();
            self.selection.end_point = self.cursor.clone();
            self.move_cursor_right();
        }
    }

    pub(crate) fn clear_selection(&mut self, previous_cursor: &Cursor) {
        if self.selection.is_selected == false {
            self.selection.start_point = self.cursor.clone();
            self.selection.end_point = self.cursor.clone();
            return;
        }

        let sorted_selection_points = self.get_sorted_selection_points();
        
        if !sorted_selection_points.is_none() {
            self.selection = sorted_selection_points.unwrap();
        }

        let is_cursor_going_forward_from_start_point = 
            self.is_cursor_greater_than(&previous_cursor)
            && self.is_cursor_lesser_than(&self.selection.end_point);
        let is_cursor_going_backward_from_end_point = 
            self.is_cursor_lesser_than(&previous_cursor)
            && self.is_cursor_greater_than(&self.selection.start_point);
        let is_cursor_going_forward_from_end_point = 
            self.is_cursor_greater_than(&previous_cursor)
            && self.is_cursor_greater_than(&self.selection.end_point);
        let is_cursor_going_backward_from_start_point = 
            self.is_cursor_lesser_than(&previous_cursor)
            && self.is_cursor_lesser_than(&self.selection.start_point);
        
        if is_cursor_going_forward_from_start_point {
            self.cursor = self.selection.end_point.clone();
            self.move_cursor_right();
        } else if is_cursor_going_backward_from_end_point {
            self.cursor = self.selection.start_point.clone();
        } else if is_cursor_going_forward_from_end_point {
            self.cursor = self.selection.end_point.clone();
            self.move_cursor_right();
        } else if is_cursor_going_backward_from_start_point {
            self.cursor = self.selection.start_point.clone();
        }
        
        self.selection.is_selected = false;
        self.selection.start_point = self.cursor.clone();
        self.selection.end_point = self.cursor.clone();
    }

    pub(crate) fn get_sorted_selection_points(&self) -> Option<Selection> {
        if !self.selection.is_selected {
            return None;
        }

        let start_point_as_smaller = Some(Selection{
            is_selected: self.selection.is_selected,
            start_point: Cursor{
                row: self.selection.start_point.row,
                col: self.selection.start_point.col,
            },
            end_point: Cursor{
                row: self.selection.end_point.row,
                col: self.selection.end_point.col,
            },
        });
        let end_point_as_smaller = Some(Selection{
            is_selected: self.selection.is_selected,
            start_point: Cursor{
                row: self.selection.end_point.row,
                col: self.selection.end_point.col,
            },
            end_point: Cursor{
                row: self.selection.start_point.row,
                col: self.selection.start_point.col,
            },
        });

        let is_start_point_up_from_end_point = 
            self.selection.start_point.row < self.selection.end_point.row;
        if is_start_point_up_from_end_point {
            return start_point_as_smaller;
        }

        let is_start_point_down_from_end_point =
            self.selection.start_point.row > self.selection.end_point.row;
        if is_start_point_down_from_end_point {
            return end_point_as_smaller;
        }
        
        let is_start_point_left_from_end_point = 
            self.selection.start_point.row == self.selection.end_point.row
            && self.selection.start_point.col <= self.selection.end_point.col;
        if is_start_point_left_from_end_point {
            return start_point_as_smaller;
        }
        
        let is_start_point_right_from_end_point = 
            self.selection.start_point.row == self.selection.end_point.row
            && self.selection.start_point.col > self.selection.end_point.col;
        if is_start_point_right_from_end_point {
            return end_point_as_smaller;
        }

        return None;
    }
}
