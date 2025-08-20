use std::{error::Error, fs};

use crate::commands::init::read_config_file_for_doc_path;

pub fn create_new_file(file_name: String) -> Result<(), Box<dyn Error>> {
    if let Ok(mut doc_path) = read_config_file_for_doc_path() {
        doc_path.push(file_name);
        fs::File::create(doc_path)?;
    }
    Ok(())
}
