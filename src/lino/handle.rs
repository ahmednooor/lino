// use std::io::{stdout, Write};
use crossterm;
extern crate copypasta;
// use copypasta::ClipboardContext;
// use copypasta::ClipboardProvider;
// use std::fs::File;
// use std::io::prelude::*;
// use std::path::Path;

use super::*;

impl Lino {
    pub(crate) fn initiate_input_event_loop(&mut self) {
        self.render();

        loop {
            if self.is_rendering { continue; }
            
            // `read()` blocks until an `Event` is available
            let event = crossterm::event::read();

            if event.is_err() {
                self.panic_gracefully(errors::ERR4.0.to_string(), errors::ERR4.1);
            }
                
            match event.unwrap() {
                crossterm::event::Event::Key(key_event) => {
                    self.handle_key_event(&key_event);
                    // self.render()?;
                },
                crossterm::event::Event::Mouse(_) => (),
                crossterm::event::Event::Resize(_, _) => {
                    self.update_terminal_size();
                    // self.render()?;
                },
            }
            
            if self.should_exit { break; }
            
            self.render();
        }
    }

    pub(crate) fn handle_key_event(&mut self, event: &crossterm::event::KeyEvent) {
        let mut should_input_character = false;
        let mut character_input: Option<char> = None;
        let mut should_exit_from_editor = false;
        let mut should_perform_save = false;
        let mut should_input_tab = false;
        let mut should_enter_newline = false;
        let mut should_auto_indent_if_applicable = false;
        let mut should_perform_backspace = false;
        let mut should_perform_delete = false;
        let mut should_goto_line_start = false;
        let mut should_goto_line_end = false;
        let mut should_scroll_up = false;
        let mut should_scroll_down = false;
        let mut should_move_cursor_left = false;
        let mut should_move_cursor_right = false;
        let mut should_move_cursor_left_by_word = false;
        let mut should_move_cursor_right_by_word = false;
        let mut should_delete_left_by_word = false;
        let mut should_delete_right_by_word = false;
        let mut should_move_cursor_up = false;
        let mut should_move_cursor_down = false;
        let mut should_clear_selection = false;
        let mut should_make_selection = false;
        let mut should_select_all = false;
        let mut should_delete_selected = false;
        let previous_cursor = self.cursor.clone();
        let mut should_perform_copy = false;
        let mut should_perform_cut = false;
        let mut should_perform_paste = false;
        let mut should_perform_undo = false;
        let mut should_perform_redo = false;
        let mut should_save_to_history = false;
        let mut should_increase_indentation = false;
        let mut should_decrease_indentation = false;
        let mut should_swap_line_upward = false;
        let mut should_swap_line_downward = false;
        let mut should_duplicate_line_upward = false;
        let mut should_duplicate_line_downward = false;
        let mut should_delete_current_line = false;

        match event.code {
            crossterm::event::KeyCode::Char(c) => {
                if event.modifiers == crossterm::event::KeyModifiers::SHIFT
                || event.modifiers == crossterm::event::KeyModifiers::NONE {
                    should_input_character = true;
                    character_input = Some(c);
                    should_delete_selected = true;
                    
                    should_save_to_history = true;
                }

                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'q' || c == 'Q') {
                    should_exit_from_editor = true;
                }

                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 's' || c == 'S') {
                    should_perform_save = true;
                }

                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'a' || c == 'A') {
                    should_select_all = true;
                }

                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'c' || c == 'C') {
                    should_perform_copy = true;
                }
                
                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'x' || c == 'X') {
                    should_perform_cut = true;
                    should_delete_selected = true;
                    should_save_to_history = true;
                }
                
                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'v' || c == 'V') {
                    should_delete_selected = true;
                    should_perform_paste = true;
                    should_save_to_history = true;
                }
                
                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'z' || c == 'Z') {
                    should_perform_undo = true;
                }
                
                if event.modifiers == (crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT) && (c == 'z' || c == 'Z') {
                    should_perform_redo = true;
                }
                
                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                && (c == 'y' || c == 'Y') {
                    should_perform_redo = true;
                }

                if event.modifiers == crossterm::event::KeyModifiers::ALT
                && c == ']' {
                    should_increase_indentation = true;
                    should_save_to_history = true;
                }

                if event.modifiers == crossterm::event::KeyModifiers::ALT
                && c == '[' {
                    should_decrease_indentation = true;
                    should_save_to_history = true;
                }
                
            },
            crossterm::event::KeyCode::Tab => {
                if event.modifiers == crossterm::event::KeyModifiers::NONE {
                    should_input_tab = true;
                    should_delete_selected = true;
                    should_save_to_history = true;
                }
            },
            crossterm::event::KeyCode::Enter => {
                should_enter_newline = true;
                should_delete_selected = true;
                should_save_to_history = true;

                if event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                    should_auto_indent_if_applicable = true;
                }
            },
            crossterm::event::KeyCode::Backspace => {
                if !self.selection.is_selected {
                    should_perform_backspace = true;
                } else {
                    should_delete_selected = true;
                }
                
                if !self.selection.is_selected
                && (event.modifiers == crossterm::event::KeyModifiers::ALT
                || event.modifiers == crossterm::event::KeyModifiers::CONTROL) {
                    should_delete_left_by_word = true;
                    should_perform_backspace = false;
                    should_delete_selected = false;
                }
                
                should_save_to_history = true;
            },
            crossterm::event::KeyCode::Delete => {
                if !self.selection.is_selected {
                    should_perform_delete = true;
                } else {
                    should_delete_selected = true;
                }
                
                if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    should_delete_current_line = true;
                    should_perform_delete = false;
                    should_delete_selected = false;
                    should_clear_selection = true;
                }

                if !self.selection.is_selected
                && (event.modifiers == crossterm::event::KeyModifiers::ALT
                || event.modifiers == crossterm::event::KeyModifiers::CONTROL) {
                    should_delete_right_by_word = true;
                    should_perform_delete = false;
                    should_delete_selected = false;
                }

                should_save_to_history = true;
            },
            crossterm::event::KeyCode::Home => {
                should_goto_line_start = true;

                if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    should_make_selection = true;
                } else {
                    should_clear_selection = true;
                }
            },
            crossterm::event::KeyCode::End => {
                should_goto_line_end = true;

                if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    should_make_selection = true;
                } else {
                    should_clear_selection = true;
                }
            },
            crossterm::event::KeyCode::PageUp => {
                should_scroll_up = true;

                if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    should_make_selection = true;
                } else {
                    should_clear_selection = true;
                }
            },
            crossterm::event::KeyCode::PageDown => {
                should_scroll_down = true;

                if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    should_make_selection = true;
                } else {
                    should_clear_selection = true;
                }
            },
            crossterm::event::KeyCode::Left => {
                should_move_cursor_left = true;

                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                || event.modifiers == crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT {
                    should_move_cursor_left = false;
                    should_move_cursor_left_by_word = true;
                }

                if event.modifiers == crossterm::event::KeyModifiers::SHIFT
                || event.modifiers == crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT {
                    should_make_selection = true;
                } else {
                    should_clear_selection = true;
                }
            },
            crossterm::event::KeyCode::Right => {
                should_move_cursor_right = true;
                
                if event.modifiers == crossterm::event::KeyModifiers::CONTROL
                || event.modifiers == crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT {
                    should_move_cursor_right = false;
                    should_move_cursor_right_by_word = true;
                }

                if event.modifiers == crossterm::event::KeyModifiers::SHIFT
                || event.modifiers == crossterm::event::KeyModifiers::CONTROL 
                | crossterm::event::KeyModifiers::SHIFT {
                    should_make_selection = true;
                } else {
                    should_clear_selection = true;
                }
            },
            crossterm::event::KeyCode::Up => {
                should_move_cursor_up = true;
                
                if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    should_make_selection = true;
                } else {
                    should_clear_selection = true;
                }
                
                if event.modifiers == crossterm::event::KeyModifiers::ALT
                || event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                    should_swap_line_upward = true;
                    should_move_cursor_up = false;
                }
                
                if event.modifiers == crossterm::event::KeyModifiers::ALT
                | crossterm::event::KeyModifiers::SHIFT
                || event.modifiers == crossterm::event::KeyModifiers::CONTROL
                | crossterm::event::KeyModifiers::SHIFT {
                    should_duplicate_line_upward = true;
                    should_move_cursor_up = false;
                }
            },
            crossterm::event::KeyCode::Down => {
                should_move_cursor_down = true;

                if event.modifiers == crossterm::event::KeyModifiers::SHIFT {
                    should_make_selection = true;
                } else {
                    should_clear_selection = true;
                }

                if event.modifiers == crossterm::event::KeyModifiers::ALT
                || event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                    should_swap_line_downward = true;
                    should_move_cursor_down = false;
                }

                if event.modifiers == crossterm::event::KeyModifiers::ALT
                | crossterm::event::KeyModifiers::SHIFT
                || event.modifiers == crossterm::event::KeyModifiers::CONTROL
                | crossterm::event::KeyModifiers::SHIFT {
                    should_duplicate_line_downward = true;
                    should_move_cursor_down = false;
                }
            },
            crossterm::event::KeyCode::Esc => {
                should_clear_selection = true;
                should_save_to_history = true;
            },
            _ => ()
        }

        // ordering is important here
        if should_save_to_history { self.save_to_history(); }
        if should_perform_cut { self.perform_copy(); }
        if should_delete_selected { self.delete_selected(); }
        if should_input_character { self.input_character(character_input.unwrap()); }
        if should_exit_from_editor { self.exit_from_editor(); }
        if should_perform_save { self.perform_save(); }
        if should_input_tab { self.input_tab(); }
        if should_enter_newline { self.enter_newline(); }
        if should_auto_indent_if_applicable { self.auto_indent_if_applicable(); }
        if should_perform_backspace { self.perform_backspace(); }
        if should_perform_delete { self.perform_delete(); }
        if should_goto_line_start { self.goto_line_start(); }
        if should_goto_line_end { self.goto_line_end(); }
        if should_scroll_up { self.scroll_up(); }
        if should_scroll_down { self.scroll_down(); }
        if should_move_cursor_left { self.move_cursor_left(); }
        if should_move_cursor_right { self.move_cursor_right(); }
        if should_move_cursor_left_by_word { self.move_cursor_left_by_word(); }
        if should_move_cursor_right_by_word { self.move_cursor_right_by_word(); }
        if should_delete_left_by_word { self.delete_left_by_word(); }
        if should_delete_right_by_word { self.delete_right_by_word(); }
        if should_move_cursor_up { self.move_cursor_up(); }
        if should_move_cursor_down { self.move_cursor_down(); }
        if should_clear_selection { self.clear_selection(&previous_cursor); }
        if should_make_selection { self.make_selection(&previous_cursor); }
        if should_select_all { self.select_all(); }
        if should_perform_copy { self.perform_copy(); }
        if should_perform_paste { self.perform_paste(); }
        if should_perform_undo { self.perform_undo(); }
        if should_perform_redo { self.perform_redo(); }
        if should_increase_indentation { self.increase_indentation(); }
        if should_decrease_indentation { self.decrease_indentation(); }
        if should_swap_line_upward { self.swap_line_upward(); }
        if should_swap_line_downward { self.swap_line_downward(); }
        if should_duplicate_line_upward { self.duplicate_line_upward(); }
        if should_duplicate_line_downward { self.duplicate_line_downward(); }
        if should_delete_current_line { self.delete_current_line(); }

        self.set_file_unsaved_if_applicable();

    }

    pub(crate) fn handle_unsaved_changes_frame_input(&mut self) {
        loop {
            let event = crossterm::event::read();

            if event.is_err() {
                self.panic_gracefully(errors::ERR5.0.to_string(), errors::ERR5.1);
            }

            match event.unwrap() { // read is a blocking call
                crossterm::event::Event::Key(key_event) => {
                    match key_event.code {
                        crossterm::event::KeyCode::Char(c) => {
                            if c == 'y' || c == 'Y' {
                                if self.file.path == "" {
                                    self.file.should_save_as = true;
                                } else {
                                    self.file.should_save_as = false;
                                    self.save_to_file();
                                }
                                break;
                            }
                            if c == 'n' || c == 'N' {
                                self.file.should_save_as = false;
                                break;
                            }
                        },
                        crossterm::event::KeyCode::Esc => {
                            self.file.should_save_as = false;
                            self.should_exit = false;
                            break;
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        };
    }
    
    pub(crate) fn handle_save_as_frame_input(&mut self) {
        loop {
            let event = crossterm::event::read();

            if event.is_err() {
                self.panic_gracefully(errors::ERR6.0.to_string(), errors::ERR6.1);
            }

            match event.unwrap() { // read is a blocking call
                crossterm::event::Event::Key(key_event) => {
                    match key_event.code {
                        crossterm::event::KeyCode::Char(c) => {
                            self.file.path.push(c);
                        },
                        crossterm::event::KeyCode::Backspace => {
                            self.file.path.pop();
                        },
                        crossterm::event::KeyCode::Enter => {
                            if self.file.path == "" {
                                continue;
                            }
                            self.save_to_file();
                            break;
                        },
                        crossterm::event::KeyCode::Esc => {
                            self.file.path = "".to_string();
                            self.should_exit = false;
                            break;
                        },
                        _ => ()
                    }
                },
                _ => ()
            };

            self.render_save_as_frame();
        };
    }

}
