pub mod act {
    pub mod init {
        use std::{fs, io::ErrorKind, path::PathBuf, process};

        use dialoguer::{Confirm, Input};

        pub struct Data {
            folder_path: PathBuf,
            config_path: PathBuf,
        }

        pub fn init() -> std::io::Result<()> {
            Ok(())
        }

        pub fn interactive_init() -> std::io::Result<()> {
            let mut config_data: Data = Data {
                folder_path: PathBuf::new(),
                config_path: PathBuf::new(),
            };

            let folder_path: String = Input::new()
                .with_prompt("Where do you want to store your naoty file?")
                .interact_text()
                .unwrap();

            if let Ok(path) = create_folder_path(folder_path) {
                config_data.folder_path = path;
            }

            if let Ok(path) = create_config_path() {
                config_data.config_path = path;
            }

            println!("{}", config_data.folder_path.display());
            println!("{}", config_data.config_path.display());

            let confirm_initialization = Confirm::new()
                .with_prompt("Do you want to save your new configuration?")
                .interact()
                .unwrap();

            save_config(confirm_initialization, config_data)?;
            Ok(())
        }

        fn create_folder_path(folder_path: String) -> std::io::Result<PathBuf> {
            let home_folder = dirs::home_dir();

            match home_folder {
                Some(mut path) => {
                    path.push(&folder_path);
                    Ok(path)
                }
                None => Err(std::io::Error::new(
                    ErrorKind::NotFound,
                    "Cannot find folder path",
                )),
            }
        }

        fn create_config_path() -> std::io::Result<PathBuf> {
            let config_folder = dirs::config_dir();

            match config_folder {
                Some(mut path) => {
                    path.push(".naoty");
                    Ok(path)
                }
                None => Err(std::io::Error::new(
                    ErrorKind::NotFound,
                    "Config file not found",
                )),
            }
        }

        fn create_config_folder(config_path: PathBuf) -> std::io::Result<()> {
            fs::create_dir(config_path)?;
            Ok(())
        }

        fn create_naoty_folder(folder_path: PathBuf) -> std::io::Result<()> {
            fs::create_dir(folder_path)?;
            Ok(())
        }

        fn save_config(confirmation: bool, config_data: Data) -> std::io::Result<()> {
            if confirmation {
                if let Ok(()) = create_config_folder(config_data.config_path) {
                    println!("Config file created...");
                } else {
                    println!("Config file already exists there so naoty didn't create new one.");
                }

                if let Ok(()) = create_naoty_folder(config_data.folder_path) {
                    println!("Naoty folder created...");
                } else {
                    println!("This file already exists so naoty didn't create it.");
                }
            } else {
                println!("Configuration aborted.");
                process::exit(0);
            }
            Ok(())
        }
    }
}
