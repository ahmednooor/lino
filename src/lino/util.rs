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
                    width: 1,
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

    pub(crate) fn calculate_tab_width(&self) -> usize {
        return self.settings.tab_width - (self.cursor.col % self.settings.tab_width);
    }

    pub(crate) fn do_nothing(&mut self) {}
}