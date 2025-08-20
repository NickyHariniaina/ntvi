use std::{error::Error, fs};

use crate::commands::init::read_config_file_for_doc_path;

pub fn save(file_name: String, buffer: String) -> Result<(), Box<dyn Error>> {
    let mut path = read_config_file_for_doc_path()?;
    path.push(file_name);
    fs::write(path, buffer)?;
    Ok(())
}
