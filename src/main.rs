mod actions;
mod arguments;
mod ui;

use actions::act::init::interactive_init;
use arguments::args::Args;
use clap::Parser;

use crate::{actions::act::init::init, arguments::args::MainAction, ui::editor::create_editor};

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    match args.main_action {
        Some(action) => match action {
            MainAction::Init { path } => {
                match path {
                    Some(p) => init(p)?,
                    None => interactive_init()?,
                }
                Ok(())
            }
            MainAction::Create { file_name } => Ok(()),
            MainAction::Open { file_name } => {
                create_editor()?;
                Ok(())
            }
            MainAction::New => {
                create_editor()?;
                Ok(())
            }
        },
        None => Ok(()),
    }
}
