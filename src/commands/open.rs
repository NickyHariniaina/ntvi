use std::{error::Error, fs};

use crate::{commands::init::read_config_file_for_doc_path, editor::events::screen::open_editor};

pub fn open(file_name: String) -> Result<(), Box<dyn Error>> {
    let mut file_path = read_config_file_for_doc_path()?;
    file_path.push(file_name);
    let file_content = fs::read_to_string(file_path)?;
    open_editor(file_content, true)?;
    Ok(())
}

