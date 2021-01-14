use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::*;

impl Lino {
    pub(crate) fn read_from_file(&mut self) {
        // Create a path to the desired file
        let path = Path::new(self.file.path.as_str());
        let display = path.display();

        if !path.is_file() {
            self.file.should_save_as = false;
            return;
        }

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut input_string = String::new();
        match file.read_to_string(&mut input_string) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => (),
        }

        for character in input_string.chars() {
            self.input_character(character);
        }
        self.reset_cursor();
        self.last_cursor_col = self.cursor.col;
        self.saved_text = Lino::convert_2d_text_to_string(&self.lines);
        self.file.should_save_as = false;
    }

    pub(crate) fn save_to_file(&mut self) {
        let path_str = &self.file.path;
        let path = Path::new(&path_str);
        let display = path.display();

        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        
        let output_string = Lino::convert_2d_text_to_string(&self.lines);

        match file.write_all(output_string.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => (),
        }

        self.saved_text = Lino::convert_2d_text_to_string(&self.lines);
        self.file.is_saved = true;
        self.file.should_save_as = false;
    }
}