use std::error::Error;

use crate::editor::events::screen::open_editor;

pub fn new() -> Result<(), Box<dyn Error>> {
    let buffer: String = String::new();
    open_editor(buffer, false)?;
    Ok(())
}
