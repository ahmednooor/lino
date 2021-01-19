use super::*;

pub(crate) mod keys {
    pub(crate) static UP: &str = "up";
    pub(crate) static DOWN: &str = "down";
    pub(crate) static LEFT: &str = "left";
    pub(crate) static RIGHT: &str = "right";
    pub(crate) static PAGE_UP: &str = "page_up";
    pub(crate) static PAGE_DOWN: &str = "page_down";
    pub(crate) static HOME: &str = "home";
    pub(crate) static END: &str = "end";
    pub(crate) static BACKSPACE: &str = "backspace";
    pub(crate) static DELETE: &str = "delete";
    pub(crate) static CTRL: &str = "ctrl";
    pub(crate) static SHIFT: &str = "shift";
    pub(crate) static ALT: &str = "alt";
    pub(crate) static ENTER: &str = "enter";
    pub(crate) static TAB: &str = "tab";
    pub(crate) static ESC: &str = "esc";
}

use keys::*;

impl Lino {
    pub(crate) fn bind_keys_to_commands(&mut self) {
        let kbs = &mut self.keybindings;
        
        kbs.insert(format!("{}",         UP),                    Lino::command_move_up);
        kbs.insert(format!("{}",         DOWN),                  Lino::command_move_down);
        kbs.insert(format!("{}",         LEFT),                  Lino::command_move_left);
        kbs.insert(format!("{}",         RIGHT),                 Lino::command_move_right);
        kbs.insert(format!("{}+{}",      CTRL, LEFT),            Lino::command_move_left_by_word);
        kbs.insert(format!("{}+{}",      CTRL, RIGHT),           Lino::command_move_right_by_word);
        kbs.insert(format!("{}",         PAGE_UP),               Lino::command_move_up_by_page);
        kbs.insert(format!("{}",         PAGE_DOWN),             Lino::command_move_down_by_page);
        kbs.insert(format!("{}",         HOME),                  Lino::command_move_to_line_start);
        kbs.insert(format!("{}",         END),                   Lino::command_move_to_line_end);
        
        kbs.insert(format!("{}+{}",      SHIFT, UP),             Lino::command_select_up);
        kbs.insert(format!("{}+{}",      SHIFT, DOWN),           Lino::command_select_down);
        kbs.insert(format!("{}+{}",      SHIFT, LEFT),           Lino::command_select_left);
        kbs.insert(format!("{}+{}",      SHIFT, RIGHT),          Lino::command_select_right);
        kbs.insert(format!("{}+{}+{}",   CTRL, SHIFT, LEFT),     Lino::command_select_left_by_word);
        kbs.insert(format!("{}+{}+{}",   CTRL, SHIFT, RIGHT),    Lino::command_select_right_by_word);
        kbs.insert(format!("{}+{}",      SHIFT, PAGE_UP),        Lino::command_select_up_by_page);
        kbs.insert(format!("{}+{}",      SHIFT, PAGE_DOWN),      Lino::command_select_down_by_page);
        kbs.insert(format!("{}+{}",      SHIFT, HOME),           Lino::command_select_to_line_start);
        kbs.insert(format!("{}+{}",      SHIFT, END),            Lino::command_select_to_line_end);
        kbs.insert(format!("{}+{}",      CTRL, 'a'),             Lino::command_select_all);
        
        kbs.insert(format!("{}",         BACKSPACE),             Lino::command_delete_left_character);
        kbs.insert(format!("{}",         DELETE),                Lino::command_delete_right_character);
        kbs.insert(format!("{}+{}",      CTRL, BACKSPACE),       Lino::command_delete_left_word);
        kbs.insert(format!("{}+{}",      ALT, BACKSPACE),        Lino::command_delete_left_word);
        kbs.insert(format!("{}+{}",      CTRL, DELETE),          Lino::command_delete_right_word);
        kbs.insert(format!("{}+{}",      ALT, DELETE),           Lino::command_delete_right_word);
        kbs.insert(format!("{}+{}",      SHIFT, DELETE),         Lino::command_delete_current_line);
        
        kbs.insert(format!("{}+{}",      CTRL, UP),              Lino::command_move_current_line_up);
        kbs.insert(format!("{}+{}",      ALT, UP),               Lino::command_move_current_line_up);
        kbs.insert(format!("{}+{}",      CTRL, DOWN),            Lino::command_move_current_line_down);
        kbs.insert(format!("{}+{}",      ALT, DOWN),             Lino::command_move_current_line_down);
        
        kbs.insert(format!("{}+{}+{}",   CTRL, SHIFT, UP),       Lino::command_duplicate_current_line_up);
        kbs.insert(format!("{}+{}+{}",   ALT, SHIFT, UP),        Lino::command_duplicate_current_line_up);
        kbs.insert(format!("{}+{}+{}",   CTRL, SHIFT, DOWN),     Lino::command_duplicate_current_line_down);
        kbs.insert(format!("{}+{}+{}",   ALT, SHIFT, DOWN),      Lino::command_duplicate_current_line_down);

        kbs.insert(format!("{}+{}",      ALT, ']'),              Lino::command_increase_indentation);
        kbs.insert(format!("{}+{}",      ALT, RIGHT),            Lino::command_increase_indentation);
        kbs.insert(format!("{}+{}",      ALT, '['),              Lino::command_decrease_indentation);
        kbs.insert(format!("{}+{}",      ALT, LEFT),             Lino::command_decrease_indentation);

        kbs.insert(format!("{}",         TAB),                   Lino::command_enter_tab);
        kbs.insert(format!("{}",         ENTER),                 Lino::command_enter_new_line);
        kbs.insert(format!("{}+{}",      CTRL, ENTER),           Lino::command_enter_auto_indented_new_line);

        kbs.insert(format!("{}+{}",      CTRL, 'x'),             Lino::command_cut);
        kbs.insert(format!("{}+{}",      CTRL, 'c'),             Lino::command_copy);
        kbs.insert(format!("{}+{}",      CTRL, 'v'),             Lino::command_paste);
        kbs.insert(format!("{}+{}",      CTRL, 'z'),             Lino::command_undo);
        kbs.insert(format!("{}+{}",      CTRL, 'y'),             Lino::command_redo);
        kbs.insert(format!("{}+{}",      CTRL, 's'),             Lino::command_save);
        kbs.insert(format!("{}+{}",      CTRL, 'w'),             Lino::command_quit);

        kbs.insert(format!("{}",         ESC),                   Lino::command_clear_selection);
    }
}