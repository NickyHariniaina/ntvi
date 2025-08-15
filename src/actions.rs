pub mod act {
    pub mod init {
        use std::{io::ErrorKind, path::PathBuf};

        use dialoguer::Input;

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
                    path.push(".ntvim");
                    Ok(path)
                }
                None => Err(std::io::Error::new(
                    ErrorKind::NotFound,
                    "Config file not found",
                )),
            }
        }
    }
}
