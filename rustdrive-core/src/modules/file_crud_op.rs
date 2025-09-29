use serde::{Serialize, Deserialize};
use std::{fs, path::Path, io};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMeta {
    pub id: i32,
    pub filename: String,
    pub size: u64,
}

pub fn upload(path: &Path) -> io::Result<FileMeta> {
    if !path.is_file() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file path"));
    }

    let cwd = std::env::current_dir()?;
    let storage_folder_name = "storage";
    let storage_path = cwd.join(storage_folder_name);

    if !storage_path.exists() {
        fs::create_dir(&storage_path)?;
    }

    let mut file_name = if let Some(name) = path.file_name() {
        name.to_string_lossy().to_string()
    } else {
        "file.txt".to_string()
    };

    if storage_path.join(&file_name).exists() {
        for i in 1.. {
            let new_file_name = format!("{}_{}", i, file_name);
            if !storage_path.join(&new_file_name).exists() {
                file_name = new_file_name;
                break;
            }
        }
    }

    let dest = storage_path.join(&file_name);
    let copied = fs::copy(path, &dest)?; // returns number of bytes copied

    Ok(FileMeta {
        id: 0,
        filename: file_name,
        size: copied,
    })
}