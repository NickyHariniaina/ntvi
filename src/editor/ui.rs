use std::{error::Error, io::stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    style::{Color, SetBackgroundColor},
    terminal::{self, Clear, ClearType},
};

pub fn create_editor() -> std::io::Result<()> {
    let mut stdout = stdout();

    execute!(
        stdout,
        SetBackgroundColor(Color::Rgb {
            r: 20,
            g: 20,
            b: 19
        })
    )?;

    execute!(stdout, Clear(ClearType::All))?;
    Ok(())
}

pub fn open_editor() -> Result<(), Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    create_editor()?;
    handle_key()?;
    terminal::disable_raw_mode()?;
    Ok(())
}

pub fn handle_key() -> Result<(), Box<dyn Error>> {
    'handle_key: loop {
        let event = event::read()?;
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Esc => {
                    println!("Key pressed");
                    break 'handle_key;
                }
                _ => {}
            }
        }
    }
    Ok(())
}
