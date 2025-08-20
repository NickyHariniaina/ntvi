use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use std::error::Error;
use std::io::Write;
use std::io::stdout;

pub fn handle_key() -> Result<(), Box<dyn Error>> {
    'handle_key: loop {
        let event = event::read()?;
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Esc => {
                    print!("Key pressed");
                    break 'handle_key;
                }
                KeyCode::Char(key_pressed) => {
                    print!("{}", key_pressed);
                }
                KeyCode::Backspace => {
                    print!("\x08");
                    print!(" ");
                    print!("\x08");
                }
                _ => {}
            }
        }
        stdout().flush()?;
    }
    Ok(())
}
