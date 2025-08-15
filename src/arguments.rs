pub mod args {
    use clap::{Parser, Subcommand};

    #[derive(Parser)]
    #[command(name = "Naotivy (ntvim)", version = "v1.0")]
    pub struct Args {
        #[command(subcommand)]
        pub main_action: MainAction,
    }

    #[derive(Subcommand)]
    pub enum MainAction {
        Init,
    }
}
