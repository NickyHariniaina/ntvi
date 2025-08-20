use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use std::error::Error;

pub fn handle_key() -> Result<(), Box<dyn Error>> {
    'handle_key: loop {
        let event = event::read()?;
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Esc => {
                    println!("Key pressed");
                    break 'handle_key;
                }
                _ => {
                    println!("pressed");
                }
            }
        }
    }
    Ok(())
}
