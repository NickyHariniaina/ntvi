mod actions;
mod arguments;
use actions::act::init::interactive_init;
use arguments::args::Args;
use clap::Parser;

use crate::arguments::args::MainAction;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    match args.main_action {
        MainAction::Init => {
            println!("This is initialization.");
            interactive_init()?;
            Ok(())
        }
    }
}
