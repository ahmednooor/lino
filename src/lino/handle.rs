// use std::io::{stdout, Write};
use crossterm;
use super::*;
use super::keybindings::keys;
use std::time::Duration;

impl Lino {
    pub(crate) fn initiate_input_event_loop(&mut self) {
        let (main_thread_tx, main_thread_rx) = self.spawn_highlighting_thread();

        loop {
            if crossterm::event::poll(Duration::from_millis(5)).unwrap_or(false) {
                let event = crossterm::event::read();
                
                if event.is_err() {
                    self.panic_gracefully(&Error::err4());
                }
                
                match event.unwrap() {
                    crossterm::event::Event::Mouse(_) => (),
                    crossterm::event::Event::Key(key_event) => {
                        self.handle_key_event(&key_event);
                        
                        if self.should_exit { 
                            main_thread_tx.send((
                                HighlightingThreadMessage::Terminate, 
                                self.cursor.clone())).unwrap_or(());
                            break;
                        }

                        self.rendering.should_render = true;
                    },
                    crossterm::event::Event::Resize(_, _) => {
                        self.update_terminal_size();
                        self.rendering.should_render = true;
                    },
                }
            }

            self.send_text_to_highlighting_thread(&main_thread_tx);
            self.colorize_text_based_on_highlighting(&main_thread_rx);
            self.render();
        }
    }

    pub(crate) fn handle_key_event(&mut self, event: &crossterm::event::KeyEvent) {
        let mut key_binding = format!("");

        self.highlighting.start_row = self.cursor.row;

        match event.code {
            crossterm::event::KeyCode::Char(c) => {
                if event.modifiers == crossterm::event::KeyModifiers::SHIFT
                || event.modifiers == crossterm::event::KeyModifiers::NONE {
                    self.input_char_buf = Some(c);
                    key_binding = format!("{}", keys::CHAR_INPUT);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'w' || c == 'W') {
                    key_binding = format!("{}+{}", keys::CTRL, 'w');
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'q' || c == 'Q') {
                    key_binding = format!("{}+{}", keys::CTRL, 'q');
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 's' || c == 'S') {
                    key_binding = format!("{}+{}", keys::CTRL, 's');
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::ALT
                && (c == 's' || c == 'S') {
                    key_binding = format!("{}+{}", keys::ALT, 's');
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::ALT
                && (c == 'g' || c == 'G') {
                    key_binding = format!("{}+{}", keys::ALT, 'g');
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                | crossterm::event::KeyModifiers::SHIFT && (c == 's' || c == 'S') {
                    key_binding = format!("{}+{}+{}", keys::CTRL, keys::SHIFT, 's');
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'a' || c == 'A') {
                    key_binding = format!("{}+{}", keys::CTRL, 'a');
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'c' || c == 'C') {
                    key_binding = format!("{}+{}", keys::CTRL, 'c');
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'd' || c == 'D') {
                    key_binding = format!("{}+{}", keys::CTRL, 'd');
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'x' || c == 'X') {
                    key_binding = format!("{}+{}", keys::CTRL, 'x');
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'v' || c == 'V') {
                    key_binding = format!("{}+{}", keys::CTRL, 'v');
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'z' || c == 'Z') {
                    key_binding = format!("{}+{}", keys::CTRL, 'z');
                }
                
                else if event.modifiers == (crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT) && (c == 'z' || c == 'Z') {
                    key_binding = format!("{}+{}", keys::CTRL, 'y');
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'y' || c == 'Y') {
                    key_binding = format!("{}+{}", keys::CTRL, 'y');
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'f' || c == 'F') {
                    key_binding = format!("{}+{}", keys::CTRL, 'f');
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'r' || c == 'R') {
                    key_binding = format!("{}+{}", keys::CTRL, 'r');
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'n' || c == 'N') {
                    key_binding = format!("{}+{}", keys::CTRL, 'n');
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'p' || c == 'P') {
                    key_binding = format!("{}+{}", keys::CTRL, 'p');
                }

                else if event.modifiers == crossterm::event::KeyModifiers::ALT
                && c == ']' {
                    key_binding = format!("{}+{}", keys::ALT, ']');
                }

                else if event.modifiers == crossterm::event::KeyModifiers::ALT
                && c == '[' {
                    key_binding = format!("{}+{}", keys::ALT, '[');
                }
                
            },
            crossterm::event::KeyCode::Tab => {
                key_binding = format!("{}", keys::TAB);
            },
            crossterm::event::KeyCode::BackTab => {
                key_binding = format!("{}+{}", keys::SHIFT, keys::TAB);
            },
            crossterm::event::KeyCode::Enter => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::ENTER);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                    key_binding = format!("{}+{}", keys::CTRL, keys::ENTER);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::ENTER);
                }
            },
            crossterm::event::KeyCode::Backspace => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::BACKSPACE);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                    key_binding = format!("{}+{}", keys::CTRL, keys::BACKSPACE);
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::ALT {
                    key_binding = format!("{}+{}", keys::ALT, keys::BACKSPACE);
                }
            },
            crossterm::event::KeyCode::Delete => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::DELETE);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                    key_binding = format!("{}+{}", keys::CTRL, keys::DELETE);
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::ALT {
                    key_binding = format!("{}+{}", keys::ALT, keys::DELETE);
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::DELETE);
                }
            },
            crossterm::event::KeyCode::Home => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::HOME);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::HOME);
                }
            },
            crossterm::event::KeyCode::End => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::END);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::END);
                }
            },
            crossterm::event::KeyCode::PageUp => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::PAGE_UP);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::PAGE_UP);
                }
            },
            crossterm::event::KeyCode::PageDown => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::PAGE_DOWN);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::PAGE_DOWN);
                }
            },
            crossterm::event::KeyCode::Left => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::LEFT);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                    key_binding = format!("{}+{}", keys::CTRL, keys::LEFT);
                }
                    
                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::LEFT);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::ALT {
                    key_binding = format!("{}+{}", keys::ALT, keys::LEFT);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}+{}", keys::CTRL, keys::SHIFT, keys::LEFT);
                }
            },
            crossterm::event::KeyCode::Right => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::RIGHT);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                    key_binding = format!("{}+{}", keys::CTRL, keys::RIGHT);
                }
                    
                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::RIGHT);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::ALT {
                    key_binding = format!("{}+{}", keys::ALT, keys::RIGHT);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}+{}", keys::CTRL, keys::SHIFT, keys::RIGHT);
                }
            },
            crossterm::event::KeyCode::Up => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::UP);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                    key_binding = format!("{}+{}", keys::CTRL, keys::UP);
                }
                    
                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::UP);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::ALT {
                    key_binding = format!("{}+{}", keys::ALT, keys::UP);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}+{}", keys::CTRL, keys::SHIFT, keys::UP);
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::ALT 
                | crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}+{}", keys::ALT, keys::SHIFT, keys::UP);
                }
            },
            crossterm::event::KeyCode::Down => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::DOWN);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                    key_binding = format!("{}+{}", keys::CTRL, keys::DOWN);
                }
                    
                else if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}", keys::SHIFT, keys::DOWN);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::ALT {
                    key_binding = format!("{}+{}", keys::ALT, keys::DOWN);
                }

                else if event.modifiers == crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}+{}", keys::CTRL, keys::SHIFT, keys::DOWN);
                }
                
                else if event.modifiers == crossterm::event::KeyModifiers::ALT 
                | crossterm::event::KeyModifiers::SHIFT {
                    key_binding = format!("{}+{}+{}", keys::ALT, keys::SHIFT, keys::DOWN);
                }
            },
            crossterm::event::KeyCode::Esc => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    key_binding = format!("{}", keys::ESC);
                }
            },
            _ => ()
        }

        let operation_to_perform = self.keybindings.get(&key_binding);
        
        if !operation_to_perform.is_none() {
            operation_to_perform.unwrap()(self);
        }

        self.set_file_unsaved_if_applicable();

        self.highlighting.end_row = self.cursor.row;
    }
}
