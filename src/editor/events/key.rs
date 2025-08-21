use crossterm::{
    cursor::{self, MoveTo, MoveToColumn},
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, Clear},
};
use dialoguer::{Confirm, Input};
use std::{
    error::Error,
    io::{Write, stdout},
};

use crate::editor::events::file::save;

pub fn handle_key(mut buffer: String, cur_col: u16) -> Result<(), Box<dyn Error>> {
    'handle_key: loop {
        let event = event::read()?;
        if let Event::Key(key_event) = event {
            let max_col = terminal::size()?.0;
            let mut current_col: u16 = cur_col;
            match key_event.code {
                KeyCode::Esc => {
                    break 'handle_key;
                }
                KeyCode::Char(key_pressed) => {
                    if current_col == max_col {
                        current_col = 0;
                    } else {
                        current_col += 1;
                    }
                    print!("{}", key_pressed);
                    buffer.push(key_pressed);
                }
                KeyCode::Backspace => {
                    if !buffer.is_empty() {
                        let deleted_char_in_buffer = buffer.pop().unwrap();

                        if deleted_char_in_buffer == '\n' {
                            let last_lines_length =
                                buffer.lines().last().map(|l| l.len()).unwrap_or(0);

                            execute!(
                                stdout(),
                                cursor::MoveToPreviousLine(1),
                                cursor::MoveToColumn(last_lines_length as u16)
                            )?;
                        } else if current_col == 0 {
                            current_col = max_col;
                            execute!(
                                stdout(),
                                cursor::MoveToPreviousLine(1),
                                cursor::MoveToColumn(current_col)
                            )?;
                            print!("\x08");
                            print!(" ");
                            print!("\x08");
                        } else {
                            print!("\x08");
                            print!(" ");
                            print!("\x08");
                        }
                    }
                }
                KeyCode::Enter => {
                    println!();
                    execute!(stdout(), MoveToColumn(0))?;
                    buffer.push('\n');
                }
                _ => {}
            }
        }
        stdout().flush()?;
    }
    let terminal_length = terminal::size();
    if let Ok((_col, row)) = terminal_length {
        execute!(stdout(), Clear(terminal::ClearType::All))?;
        execute!(stdout(), MoveTo(0, row))?;
    }
    prompt_save(buffer)?;
    Ok(())
}

pub fn prompt_save(buffer: String) -> Result<(), Box<dyn Error>> {
    let confirmation = Confirm::new()
        .with_prompt("Do you want to save?")
        .interact()?;
    if confirmation {
        execute!(stdout(), Clear(terminal::ClearType::All))?;
        execute!(stdout(), MoveToColumn(0))?;
        let file_name: String = Input::new().with_prompt("Save as").interact()?;
        save(file_name, buffer)?;
    }
    Ok(())
}
