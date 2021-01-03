// use std::io::{stdout, Write};
// use crossterm;
// extern crate copypasta;
// use copypasta::ClipboardContext;
// use copypasta::ClipboardProvider;
// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;

use super::*;

#[allow(dead_code)]
impl Lino {
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
        return self.cursor.row == self.lines.len();
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

    pub(crate) fn convert_2d_text_to_string(lines: &Vec<Vec<Character>>) -> String {
        let mut output_string = String::new();
        for line in lines {
            for character in line {
                output_string.push(character.character);
            }
            output_string.push('\n');
        }
        output_string.pop();
        output_string
    }

    pub(crate) fn convert_string_to_2d_text(input_string: &String) -> Vec<Vec<Character>> {
        let mut lines = vec![vec![]];
        for character in input_string.chars() {
            if character == '\r' {
                continue;
            }
            if character == '\n' {
                lines.push(vec![]);
            } else {
                lines.last_mut().unwrap().push(Character{
                    background: crossterm::style::Color::Black,
                    foreground: crossterm::style::Color::White,
                    character: character.clone(),
                })
            }
        }
        lines
        // let mut temp_lino = Lino::from_string(&"".to_string());

        // for character in input_string.chars() {
        //     temp_lino.input_character(character);
        // }
        
        // temp_lino.lines
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

    pub(crate) fn calculate_tab_width(&self) -> usize {
        // let (old_cursor_col, _old_cursor_row) = crossterm::cursor::position()?;
        
        // crossterm::execute!(
        //     stdout(),
        //     crossterm::cursor::SavePosition,
        //     crossterm::cursor::Hide,
        //     crossterm::style::Print('\t'),
        //     crossterm::style::ResetColor
        // )?;
        
        // let (new_cursor_col, _new_cursor_row) = crossterm::cursor::position()?;
        
        // crossterm::execute!(
        //     stdout(),
        //     crossterm::cursor::RestorePosition,
        //     crossterm::cursor::Show,
        // )?;
        
        // Ok((new_cursor_col - old_cursor_col) as usize)

        return self.settings.tab_width - (self.cursor.col % self.settings.tab_width);
    }
}