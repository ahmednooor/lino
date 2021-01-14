use super::*;

impl Lino {
    pub(crate) fn clear_history(&mut self) {
        self.undo_list.clear();
        self.undo_list.push(History{
            lines: self.lines.clone(),
            cursor: self.cursor.clone(),
            selection: self.selection.clone(),
        });
        self.redo_list.clear();
    }

    pub(crate) fn save_to_history(&mut self) {
        self.undo_list.push(History{
            lines: self.lines.clone(),
            cursor: self.cursor.clone(),
            selection: self.selection.clone(),
        });
        self.redo_list.clear();
    }

    pub(crate) fn perform_undo(&mut self) {
        let last_iteration = self.undo_list.pop();
        if last_iteration.is_none() {
            return;
        }
        let last_iteration = last_iteration.unwrap();

        self.redo_list.push(History{
            lines: self.lines.clone(),
            cursor: self.cursor.clone(),
            selection: self.selection.clone(),
        });
        
        self.lines = last_iteration.lines.clone();
        self.cursor = last_iteration.cursor.clone();
        self.selection = last_iteration.selection.clone();
    }

    pub(crate) fn perform_redo(&mut self) {
        let last_iteration = self.redo_list.pop();
        if last_iteration.is_none() {
            return;
        }
        let last_iteration = last_iteration.unwrap();

        self.undo_list.push(History{
            lines: self.lines.clone(),
            cursor: self.cursor.clone(),
            selection: self.selection.clone(),
        });
        
        self.lines = last_iteration.lines.clone();
        self.cursor = last_iteration.cursor.clone();
        self.selection = last_iteration.selection.clone();
    }
}