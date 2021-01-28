use super::*;
use super::super::Error;

impl InputDialog {
    pub(crate) fn initiate_event_handling_loop(&mut self) -> Result<(), Error>{
        loop {
            let render_res = self.render();
            if !render_res.is_ok() {
                let mut err = Error::err9();
                err.message = "[".to_string() + &self.title + "] " + &err.message;
                return Err(err);
            }
            
            let event = crossterm::event::read();
            if event.is_err() {
                let mut err = Error::err10();
                err.message = "[".to_string() + &self.title + "] " + &err.message;
                return Err(err);
            }

            match event.unwrap() { // read is a blocking call
                crossterm::event::Event::Key(key_event) => {
                    match key_event.code {
                        crossterm::event::KeyCode::Char(c) => {
                            self.input.insert(self.cursor_col_offset, c);
                            self.cursor_col_offset += 1;
                        },
                        crossterm::event::KeyCode::Backspace => {
                            if self.input.len() > 0 && self.cursor_col_offset > 0 {
                                self.cursor_col_offset -= 1;
                                self.input.remove(self.cursor_col_offset);
                            }
                        },
                        crossterm::event::KeyCode::Delete => {
                            if self.input.len() > 0 
                            && self.cursor_col_offset < self.input.len() {
                                self.input.remove(self.cursor_col_offset);
                            }
                        },
                        crossterm::event::KeyCode::Left => {
                            if self.cursor_col_offset > 0 {
                                self.cursor_col_offset -= 1;
                            }
                        },
                        crossterm::event::KeyCode::Right => {
                            if self.cursor_col_offset < self.input.len() {
                                self.cursor_col_offset += 1;
                            }
                        },
                        crossterm::event::KeyCode::Home => {
                            self.cursor_col_offset = 0;
                        },
                        crossterm::event::KeyCode::End => {
                            self.cursor_col_offset = self.input.len();
                        },
                        crossterm::event::KeyCode::Enter => {
                            if self.input.len() > 0 {
                                break;
                            }
                        },
                        crossterm::event::KeyCode::Esc => {
                            self.is_active = false;
                            break;
                        },
                        _ => ()
                    }
                },
                _ => ()
            };
        };

        Ok(())
    }
}
