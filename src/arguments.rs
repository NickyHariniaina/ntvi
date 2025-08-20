pub mod args {

    use clap::{ArgGroup, Parser, Subcommand};
    #[derive(Parser)]
    #[command(name = "Naotivy (ntvi)", version = "v1.0")]
    pub struct Args {
        #[command(subcommand)]
        pub main_action: Option<MainAction>,

        #[arg(short, long)]
        config: Option<String>,
    }

    #[derive(Subcommand)]
    pub enum MainAction {
        Init {
            #[arg(long, short)]
            path: Option<String>,
        },
        Create {
            file_name: String,
        },
        Remove {
            file_name: String,
        },
        New,
        Open {
            file_name: String,
        },
        List,
    }
}
