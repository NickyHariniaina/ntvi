use std::{error::Error, io::stdout};

use crossterm::{
    execute,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
};

pub fn create_editor() -> Result<(), Box<dyn Error>> {
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
