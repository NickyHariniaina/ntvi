use std::{error::Error, fs};

use crate::commands::init::read_config_file_for_doc_path;

pub fn list_files() -> Result<(), Box<dyn Error>> {
    if let Ok(doc_path) = read_config_file_for_doc_path() {
        for entry in fs::read_dir(doc_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(file) = path.file_name() {
                    println!("{}", file.to_string_lossy());
                }
            }
        }
    }
    Ok(())
}
