use std::{error::Error, io::stdout};

use crossterm::{cursor::MoveTo, execute, terminal};

use crate::editor::{events::key::handle_key, ui::create_editor};

pub fn open_editor() -> Result<(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    create_editor()?;
    move_cursor_at_start()?;
    handle_key()?;
    terminal::disable_raw_mode()?;
    Ok(())
}

pub fn move_cursor_at_start() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 1))?;
    Ok(())
}
