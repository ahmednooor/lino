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
    pub(crate) fn bind_operations_to_keys(&mut self) {
        let kbs = &mut self.keybindings;
        
        kbs.insert(format!("{}",         UP),                    Lino::operation_move_up);
        kbs.insert(format!("{}",         DOWN),                  Lino::operation_move_down);
        kbs.insert(format!("{}",         LEFT),                  Lino::operation_move_left);
        kbs.insert(format!("{}",         RIGHT),                 Lino::operation_move_right);
        kbs.insert(format!("{}+{}",      CTRL, LEFT),            Lino::operation_move_left_by_word);
        kbs.insert(format!("{}+{}",      CTRL, RIGHT),           Lino::operation_move_right_by_word);
        kbs.insert(format!("{}",         PAGE_UP),               Lino::operation_move_up_by_page);
        kbs.insert(format!("{}",         PAGE_DOWN),             Lino::operation_move_down_by_page);
        kbs.insert(format!("{}",         HOME),                  Lino::operation_move_to_line_start);
        kbs.insert(format!("{}",         END),                   Lino::operation_move_to_line_end);
        
        kbs.insert(format!("{}+{}",      SHIFT, UP),             Lino::operation_select_up);
        kbs.insert(format!("{}+{}",      SHIFT, DOWN),           Lino::operation_select_down);
        kbs.insert(format!("{}+{}",      SHIFT, LEFT),           Lino::operation_select_left);
        kbs.insert(format!("{}+{}",      SHIFT, RIGHT),          Lino::operation_select_right);
        kbs.insert(format!("{}+{}+{}",   CTRL, SHIFT, LEFT),     Lino::operation_select_left_by_word);
        kbs.insert(format!("{}+{}+{}",   CTRL, SHIFT, RIGHT),    Lino::operation_select_right_by_word);
        kbs.insert(format!("{}+{}",      SHIFT, PAGE_UP),        Lino::operation_select_up_by_page);
        kbs.insert(format!("{}+{}",      SHIFT, PAGE_DOWN),      Lino::operation_select_down_by_page);
        kbs.insert(format!("{}+{}",      SHIFT, HOME),           Lino::operation_select_to_line_start);
        kbs.insert(format!("{}+{}",      SHIFT, END),            Lino::operation_select_to_line_end);
        kbs.insert(format!("{}+{}",      CTRL, 'a'),             Lino::operation_select_all);
        
        kbs.insert(format!("{}",         BACKSPACE),             Lino::operation_delete_left_character);
        kbs.insert(format!("{}",         DELETE),                Lino::operation_delete_right_character);
        kbs.insert(format!("{}+{}",      CTRL, BACKSPACE),       Lino::operation_delete_left_word);
        kbs.insert(format!("{}+{}",      ALT, BACKSPACE),        Lino::operation_delete_left_word);
        kbs.insert(format!("{}+{}",      CTRL, DELETE),          Lino::operation_delete_right_word);
        kbs.insert(format!("{}+{}",      ALT, DELETE),           Lino::operation_delete_right_word);
        kbs.insert(format!("{}+{}",      SHIFT, DELETE),         Lino::operation_delete_current_line);
        
        kbs.insert(format!("{}+{}",      CTRL, UP),              Lino::operation_move_current_line_up);
        kbs.insert(format!("{}+{}",      ALT, UP),               Lino::operation_move_current_line_up);
        kbs.insert(format!("{}+{}",      CTRL, DOWN),            Lino::operation_move_current_line_down);
        kbs.insert(format!("{}+{}",      ALT, DOWN),             Lino::operation_move_current_line_down);
        
        kbs.insert(format!("{}+{}+{}",   CTRL, SHIFT, UP),       Lino::operation_duplicate_current_line_up);
        kbs.insert(format!("{}+{}+{}",   ALT, SHIFT, UP),        Lino::operation_duplicate_current_line_up);
        kbs.insert(format!("{}+{}+{}",   CTRL, SHIFT, DOWN),     Lino::operation_duplicate_current_line_down);
        kbs.insert(format!("{}+{}+{}",   ALT, SHIFT, DOWN),      Lino::operation_duplicate_current_line_down);

        kbs.insert(format!("{}+{}",      ALT, ']'),              Lino::operation_increase_indentation);
        kbs.insert(format!("{}+{}",      ALT, RIGHT),            Lino::operation_increase_indentation);
        kbs.insert(format!("{}+{}",      ALT, '['),              Lino::operation_decrease_indentation);
        kbs.insert(format!("{}+{}",      ALT, LEFT),            Lino::operation_decrease_indentation);

        kbs.insert(format!("{}",         TAB),                   Lino::operation_enter_tab);
        kbs.insert(format!("{}",         ENTER),                 Lino::operation_enter_new_line);
        kbs.insert(format!("{}+{}",      CTRL, ENTER),           Lino::operation_enter_auto_indented_new_line);

        kbs.insert(format!("{}+{}",      CTRL, 'x'),             Lino::operation_cut);
        kbs.insert(format!("{}+{}",      CTRL, 'c'),             Lino::operation_copy);
        kbs.insert(format!("{}+{}",      CTRL, 'v'),             Lino::operation_paste);
        kbs.insert(format!("{}+{}",      CTRL, 'z'),             Lino::operation_undo);
        kbs.insert(format!("{}+{}",      CTRL, 'y'),             Lino::operation_redo);
        kbs.insert(format!("{}+{}",      CTRL, 's'),             Lino::operation_save);
        kbs.insert(format!("{}+{}",      CTRL, 'q'),             Lino::operation_quit);

        kbs.insert(format!("{}",         ESC),                   Lino::operation_clear_selection);
    }
}