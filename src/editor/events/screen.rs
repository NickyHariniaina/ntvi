use std::{error::Error, io::stdout, thread::current};

use crossterm::{
    cursor::{self, MoveTo, MoveToColumn, MoveToPreviousLine},
    execute,
    terminal::{self, DisableLineWrap, EnableLineWrap},
};

use crate::editor::{events::key::handle_key, ui::create_editor};

pub fn open_editor(buffer: String, has_file_attached: bool) -> Result<(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    create_editor()?;
    move_cursor_at_start()?;

    if has_file_attached {
        let current_col = print_text(&buffer)?;
        move_cursor_at_end_of_line(&current_col + 1)?;
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

pub fn move_cursor_at_end_of_line(current_col: u16) -> Result<(), Box<dyn Error>> {
    execute!(stdout(), MoveToColumn(current_col))?;
    Ok(())
}

pub fn print_text(buffer: &str) -> Result<u16, Box<dyn Error>> {
    let window_size = terminal::size()?;
    let max_col = window_size.0;
    let mut current_col: u16 = 0;
    for i in 0..buffer.len() {
        match buffer.get(i..i + 1) {
            Some("\n") => {
                println!();
                current_col = 0;
                execute!(stdout(), MoveToColumn(0))?;
            }
            Some(char) => {
                if current_col == max_col {
                    println!();
                    current_col = 0;
                } else {
                    current_col += 1;
                }
                print!("{}", char);
            }
            None => {
                break;
            }
        }
    }
    Ok(current_col)
}
