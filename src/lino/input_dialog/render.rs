use std::io::{stdout, Write};
use crossterm;

use super::*;

impl InputDialog {
    pub(crate) fn render(&mut self) -> crossterm::Result<()> {
        crossterm::queue!(
            stdout(),
            crossterm::cursor::Hide,

            crossterm::style::SetBackgroundColor(self.editor_theming.text_frame_fg),
            crossterm::style::SetForegroundColor(self.editor_theming.text_frame_bg),
            crossterm::cursor::MoveTo(0, 0),
            crossterm::style::Print(&self.title),
            crossterm::style::SetBackgroundColor(self.editor_theming.text_frame_bg),
            crossterm::style::SetForegroundColor(self.editor_theming.text_frame_fg),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::UntilNewLine),
            

            crossterm::style::Print("\n"),
            crossterm::cursor::MoveToColumn(0),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::UntilNewLine),
            crossterm::style::Print("\n"),
            crossterm::cursor::MoveToColumn(0),
            
            crossterm::style::Print(&self.description),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::UntilNewLine),
            
            crossterm::style::Print("\n"),
            crossterm::cursor::MoveToColumn(0),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::UntilNewLine),
            crossterm::style::Print("\n"),
            
            crossterm::cursor::MoveToColumn(0),
            crossterm::style::Print(&self.key_hints),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::UntilNewLine),

            crossterm::style::Print("\n"),
            crossterm::cursor::MoveToColumn(0),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::UntilNewLine),
            crossterm::style::Print("\n"),
            
            crossterm::cursor::MoveToColumn(0),
            crossterm::style::Print("> "),
            crossterm::cursor::SavePosition,
            crossterm::terminal::Clear(crossterm::terminal::ClearType::UntilNewLine),
            
            crossterm::style::Print(&self.input.iter().collect::<String>()),
            
            crossterm::style::Print("\n"),
            crossterm::cursor::MoveToColumn(0),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::UntilNewLine),
            crossterm::style::Print("\n"),
            
            crossterm::cursor::MoveToColumn(0),
            crossterm::style::SetForegroundColor(self.editor_theming.error_red),
            crossterm::style::Print(&self.error),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::UntilNewLine),
            
            crossterm::style::SetForegroundColor(self.editor_theming.text_frame_fg),
            crossterm::style::Print("\n"),
            crossterm::cursor::MoveToColumn(0),
        )?;

        let (_, row) = crossterm::cursor::position()?;
        let (_, term_height) = crossterm::terminal::size()?;

        for _ in row..term_height {
            crossterm::queue!(
                stdout(),
                crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
                crossterm::cursor::MoveDown(1),
            )?;
        }
        
        crossterm::queue!(
            stdout(),
            crossterm::cursor::RestorePosition,
            crossterm::cursor::MoveRight(self.cursor_col_offset as u16),
            crossterm::cursor::Show,
        )?;

        stdout().flush()?;

        Ok(())
    }
}
