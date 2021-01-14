use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::*;

impl Lino {
    pub(crate) fn read_from_file(&mut self) {
        // Create a path to the desired file
        let path = Path::new(self.file.path.as_str());
        let display = path.display();

        if path.is_dir() {
            panic!("[ERROR] Couldn't open \"{}\": {}", display, "Is a directory.");
        }

        if !path.is_file() {
            self.file.should_save_as = false;
            return;
        }

        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match File::open(&path) {
            Err(why) => panic!("[ERROR] Couldn't open \"{}\": {}", display, why),
            Ok(file) => file,
        };

        // Read the file contents into a string, returns `io::Result<usize>`
        let mut input_string = String::new();
        match file.read_to_string(&mut input_string) {
            Err(why) => panic!("[ERROR] Couldn't read \"{}\": {}", display, why),
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

        if path.is_dir() {
            self.file.save_error = format!("[ERROR] Couldn't save at \"{}\": {}", display, "Is a directory.");
        }

        let file = File::create(&path);
        
        if file.is_err() {
            self.file.save_error = format!("[ERROR] Couldn't create \"{}\": {}", display, file.unwrap_err());
            return;
        }

        let mut file = file.unwrap();
        let output_string = Lino::convert_2d_text_to_string(&self.lines);

        match file.write_all(output_string.as_bytes()) {
            Err(why) => {
                self.file.save_error = format!("[ERROR] Couldn't write to \"{}\": {}", display, why);
                return;
            },
            Ok(_) => (),
        }

        self.saved_text = Lino::convert_2d_text_to_string(&self.lines);
        self.file.is_saved = true;
        self.file.should_save_as = false;
        self.file.save_error = "".to_string();
    }

    pub(crate) fn set_file_unsaved_if_applicable(&mut self) {
        let current_text = Lino::convert_2d_text_to_string(&self.lines);
        // let saved_text_string = Lino::convert_2d_text_to_string(&self.saved_text);

        if current_text != self.saved_text {
            self.file.is_saved = false;
        } else {
            self.file.is_saved = true;
        }
    }

    pub(crate) fn perform_save(&mut self) {
        if self.file.path == "" || self.file.should_save_as {
            self.render_save_as_frame();
            self.handle_save_as_frame_input();
        } else {
            self.save_to_file();
        }
    }
}