use std::{error::Error, fs};

use crate::commands::init::read_config_file_for_doc_path;

pub fn remove_file(file_name: String) -> Result<(), Box<dyn Error>> {
    if let Ok(mut doc_path) = read_config_file_for_doc_path() {
        doc_path.push(file_name);
        fs::remove_file(doc_path)?;
    }
    Ok(())
}
