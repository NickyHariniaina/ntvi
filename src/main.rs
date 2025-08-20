use std::error::Error;

use clap::Parser;

use crate::{
    arguments::args::{Args, MainAction},
    commands::{
        create::create_new_file,
        init::{init, interactive_init},
        list::list_files,
        new::new,
        remove::remove_file,
    },
    editor::events::screen::open_editor,
};
mod arguments;
mod commands;
mod editor;

fn main() -> Result<(), Box<dyn Error>> {
    run()?;
    Ok(())
}

fn run() -> Result<(), Box<(dyn Error)>> {
    let args = Args::parse();
    if let Some(action) = args.main_action {
        match action {
            MainAction::Init { path } => {
                if let Some(file_path) = path {
                    init(file_path)?;
                } else {
                    interactive_init()?;
                }
                return Ok(());
            }
            MainAction::Create { file_name } => {
                create_new_file(file_name)?;
                return Ok(());
            }
            MainAction::Remove { file_name } => {
                remove_file(file_name)?;
                return Ok(());
            }
            MainAction::New => {
                new()?;
                return Ok(());
            }
            MainAction::List => {
                list_files()?;
                return Ok(());
            }
            MainAction::Open { file_name } => {
                return Ok(());
            }
        }
    }
    Ok(())
}
