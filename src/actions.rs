pub mod act {
    pub mod init {
        use dialoguer::{Confirm, Input};
        use serde::Deserialize;
        use serde::Serialize;
        use std::{fs, io::ErrorKind, path::PathBuf, process};

        #[derive(Clone, Deserialize, Serialize)]
        pub struct Data {
            folder_path: PathBuf,
            config_path: PathBuf,
        }

        pub fn initialize_config_data(config_data: &mut Data, folder_path: String) -> &Data {
            if let Ok(path) = create_folder_path(folder_path) {
                config_data.folder_path = path;
            }

            if let Ok(path) = create_config_path() {
                config_data.config_path = path;
            }
            config_data
        }

        pub fn init(path: String) -> std::io::Result<()> {
            let mut config_data: Data = Data {
                folder_path: PathBuf::new(),
                config_path: PathBuf::new(),
            };

            let config_data = initialize_config_data(&mut config_data, path);
            println!("Config files path {}", config_data.config_path.display());
            println!("Naoty files path {}", config_data.folder_path.display());
            save_config(true, config_data)?;
            create_config_file(config_data)?;
            write_config_file(config_data)?;

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

            let config_data = initialize_config_data(&mut config_data, folder_path);

            println!("Config files path {}", config_data.config_path.display());
            println!("Naoty files path {}", config_data.folder_path.display());

            let confirm_initialization = Confirm::new()
                .with_prompt("Do you want to save your new configuration?")
                .interact()
                .unwrap();

            save_config(confirm_initialization, config_data)?;
            if confirm_initialization {
                create_config_file(config_data)?;
                write_config_file(config_data)?;
            }
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

        fn create_config_folder(config_path: &PathBuf) -> std::io::Result<()> {
            fs::create_dir_all(config_path)?;
            Ok(())
        }

        fn create_naoty_folder(folder_path: &PathBuf) -> std::io::Result<()> {
            fs::create_dir_all(folder_path)?;
            Ok(())
        }

        fn create_config_file(config_data: &Data) -> std::io::Result<()> {
            let mut config_folder = config_data.config_path.clone();
            config_folder.push("config.toml");
            fs::File::create(config_folder)?;
            Ok(())
        }

        pub fn get_config_file() -> std::io::Result<PathBuf> {
            let config_file_path = dirs::config_dir();
            if let Some(mut config_file) = config_file_path {
                config_file.push(".naoty");
                config_file.push("config.toml");
                Ok(config_file)
            } else {
                Err(std::io::Error::new(
                    ErrorKind::NotFound,
                    "No config file found",
                ))
            }
        }

        fn write_config_file(config_data: &Data) -> std::io::Result<String> {
            let config_file = get_config_file()?;
            let config_content =
                toml::to_string(config_data).map_err(|_e| std::io::Error::other("Cannot write"))?;

            println!("{}", config_content);
            println!("Config path {}", config_file.display());
            fs::write(config_file, config_content)?;
            Ok("File created".to_string())
        }

        fn save_config(confirmation: bool, config_data: &Data) -> std::io::Result<()> {
            if confirmation {
                if let Ok(()) = create_config_folder(&config_data.config_path) {
                    println!("Config file created...");
                } else {
                    println!("Config file already exists there so naoty didn't create new one.");
                }

                if let Ok(()) = create_naoty_folder(&config_data.folder_path) {
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

    pub mod create {
        pub fn create_new_file(path: &String) -> std::io::Result<()> {
            Ok(())
        }
    }
}
