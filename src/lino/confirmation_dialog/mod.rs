mod render;
mod handle;

use super::*;

#[derive(Clone)]
pub(crate) struct ConfirmationDialog {
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) key_hints: String,
    pub(crate) input: Option<bool>,
    pub(crate) editor_theming: super::Theming,
}

impl ConfirmationDialog {
    pub fn collect_input(&mut self) -> Result<(), Error> {
        self.initiate_event_handling_loop()
    }
}
