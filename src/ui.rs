pub mod editor {
    use std::io::stdout;

    use crossterm::{
        execute,
        style::{Color, SetBackgroundColor},
        terminal::{Clear, ClearType},
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
}
