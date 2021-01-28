use std::io::{stdout, Write};
use crossterm;

use super::*;

impl ConfirmationDialog {
    pub(crate) fn render(&mut self) -> crossterm::Result<()> {
        crossterm::queue!(
            stdout(),
            crossterm::style::SetBackgroundColor(self.editor_theming.text_frame_bg),
            crossterm::style::SetForegroundColor(self.editor_theming.text_frame_fg),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::style::SetBackgroundColor(self.editor_theming.text_frame_fg),
            crossterm::style::SetForegroundColor(self.editor_theming.text_frame_bg),
            crossterm::cursor::MoveTo(0, 0),
            crossterm::style::Print(&self.title),
            crossterm::style::SetBackgroundColor(self.editor_theming.text_frame_bg),
            crossterm::style::SetForegroundColor(self.editor_theming.text_frame_fg),
            crossterm::style::Print("\n\n"),
            crossterm::cursor::MoveToColumn(0),
            crossterm::style::Print(&self.description),
            crossterm::style::Print("\n\n"),
            crossterm::cursor::MoveToColumn(0),
            crossterm::style::Print(&self.key_hints),
            crossterm::style::Print("\n\n"),
            crossterm::cursor::MoveToColumn(0),
            crossterm::style::Print("> "),
        )?;

        stdout().flush()?;

        Ok(())
    }
}
