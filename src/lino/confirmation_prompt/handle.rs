use super::*;
use super::super::Error;

impl ConfirmationPrompt {
    pub(crate) fn initiate_event_handling_loop(&mut self) -> Result<(), Error>{
        let render_res = self.render();
        if !render_res.is_ok() {
            let mut err = Error::err11();
            err.message = "[".to_string() + &self.title + "] " + &err.message;
            return Err(err);
        }
        
        loop {
            let event = crossterm::event::read();

            if event.is_err() {
                let mut err = Error::err12();
                err.message = "[".to_string() + &self.title + "] " + &err.message;
                return Err(err);
            }

            match event.unwrap() { // read is a blocking call
                crossterm::event::Event::Key(key_event) => {
                    match key_event.code {
                        crossterm::event::KeyCode::Char(c) => {
                            if c == 'y' || c == 'Y' {
                                self.input = Some(true);
                                break;
                            }
                            if c == 'n' || c == 'N' {
                                self.input = Some(false);
                                break;
                            }
                        },
                        crossterm::event::KeyCode::Esc => {
                            self.input = None;
                            break;
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        };

        Ok(())
    }
}
