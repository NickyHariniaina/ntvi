mod actions;
mod arguments;
mod ui;

use std::error::Error;

use actions::act::init::interactive_init;
use arguments::args::Args;
use clap::Parser;

use crate::{
    actions::act::{
        create::create_new_file,
        init::{init, read_config_file_for_doc_path},
    },
    arguments::args::MainAction,
    ui::editor::create_editor,
};

fn main() -> Result<(), Box<dyn Error>> {
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
            MainAction::Create { file_name } => {
                create_new_file(file_name)?;
                Ok(())
            }
            MainAction::Open { file_name } => {
                create_editor()?;
                Ok(())
            }
            MainAction::New => {
                create_editor()?;
                Ok(())
            }
            MainAction::List => {
                println!("This is a list of every ntvi notes");
                Ok(())
            }
        },
        None => Ok(()),
    }
}
