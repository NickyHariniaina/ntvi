use std::{error::Error, io::stdout};

use crossterm::{
    cursor::{self, MoveTo, MoveToColumn, MoveToPreviousLine},
    execute, terminal,
};

use crate::editor::{events::key::handle_key, ui::create_editor};

pub fn open_editor(buffer: String, has_file_attached: bool) -> Result<(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    create_editor()?;
    move_cursor_at_start()?;

    if has_file_attached {
        print_text(&buffer)?;
        move_cursor_at_end_of_line(&buffer)?;
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

pub fn move_cursor_at_end_of_line(buffer: &str) -> Result<(), Box<dyn Error>> {
    let last_line_length = buffer.lines().last().map(|l| l.len()).unwrap_or(0);
    execute!(stdout(), MoveToColumn(last_line_length as u16))?;
    Ok(())
}

pub fn print_text(buffer: &str) -> Result<(), Box<dyn Error>> {
    for i in 0..buffer.len() {
        match buffer.get(i..i + 1) {
            Some("\n") => {
                println!();
                execute!(stdout(), MoveToColumn(0))?;
            }
            Some(char) => {
                print!("{}", char);
            }
            None => {
                break;
            }
        }
    }
    Ok(())
}
