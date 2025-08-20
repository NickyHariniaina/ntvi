use std::{error::Error, io::stdout};

use crossterm::{cursor::MoveTo, execute, terminal};

use crate::editor::{events::key::handle_key, ui::create_editor};

pub fn open_editor(buffer: String, has_file_attached: bool) -> Result<(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    create_editor()?;
    move_cursor_at_start()?;
    if has_file_attached {
        print!("{}", buffer);
    }
    handle_key(buffer)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

pub fn move_cursor_at_start() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    execute!(stdout, MoveTo(0, 0))?;
    Ok(())
}
