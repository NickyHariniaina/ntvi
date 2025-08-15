mod actions;
mod arguments;
use actions::act::init::interactive_init;
use arguments::args::Args;
use clap::Parser;

use crate::{actions::act::init::init, arguments::args::MainAction};

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    match args.main_action {
        MainAction::Init { path } => {
            println!("This is initialization.");
            if let Some(content) = path {
                init(content)?;
            } else {
                interactive_init()?;
            }
            Ok(())
        }
    }
}
