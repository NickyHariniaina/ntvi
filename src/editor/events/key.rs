use crossterm::cursor::MoveDown;
use crossterm::cursor::MoveTo;
use crossterm::cursor::MoveToColumn;
use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::terminal;
use crossterm::terminal::Clear;
use dialoguer::Confirm;
use dialoguer::Input;
use std::error::Error;
use std::io::Write;
use std::io::stdout;

use crate::editor::events::file::save;

pub fn handle_key(mut buffer: String) -> Result<(), Box<dyn Error>> {
    'handle_key: loop {
        let event = event::read()?;
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Esc => {
                    break 'handle_key;
                }
                KeyCode::Char(key_pressed) => {
                    print!("{}", key_pressed);
                    buffer.push(key_pressed);
                }
                KeyCode::Backspace => {
                    print!("\x08");
                    print!(" ");
                    print!("\x08");
                    buffer.pop();
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
