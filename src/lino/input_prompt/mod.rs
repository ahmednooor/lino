mod render;
mod handle;

use super::*;

#[derive(Clone)]
pub(crate) struct InputPrompt {
    pub(crate) is_active: bool,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) key_hints: String,
    pub(crate) input: Vec<char>,
    pub(crate) cursor_col_offset: usize,
    pub(crate) error: String,
    pub(crate) editor_theming: super::Theming,
}

impl InputPrompt {
    pub fn collect_input(&mut self) -> Result<(), Error> {
        self.is_active = true;
        self.initiate_event_handling_loop()
    }
}
