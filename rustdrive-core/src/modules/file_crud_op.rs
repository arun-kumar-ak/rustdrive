use serde::{Serialize, Deserialize};
use std::{fs, io, path::{Path, PathBuf}};
use anyhow::{Error, Ok, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileMeta {
    pub filename: String,
    pub size: u64,
}

pub fn upload(path: &Path) -> Result<FileMeta> {
    if !path.is_file() {
        return Err(Error::from(io::Error::new(io::ErrorKind::InvalidInput, "Invalid file path")));
    }

    let storage_path: PathBuf = get_storage_folder_path()?;

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
        filename: file_name,
        size: copied,
    })
}

pub fn download(filename: &str, path: &Path) -> Result<()> {
    let storage_path: PathBuf = get_storage_folder_path()?;

    if !storage_path.exists() {
        return Err(Error::from(io::Error::new(io::ErrorKind::NotFound, "Storage folder does not exist")));
    }

    let source = storage_path.join(filename);
    if !source.exists() {
        return Err(Error::from(io::Error::new(io::ErrorKind::NotFound, "File not found in storage")));
    }

    if !path.is_dir() {
        return Err(Error::from(io::Error::new(io::ErrorKind::InvalidInput, "Invalid destination path")));
    }

    let dest = path.join(filename);
    fs::copy(&source, &dest)?;

    println!("File downloaded successfully to {:?}", dest);
    Ok(())
}

pub fn get_all_files() -> Result<Vec<FileMeta>> {
    let storage_path: PathBuf = get_storage_folder_path()?;

    if !storage_path.exists() {
        return Ok(vec![]); // No files if storage doesn't exist
    }

    let mut files = Vec::new();
    for entry in fs::read_dir(storage_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_file() {
            files.push(FileMeta {
                filename: entry.file_name().to_string_lossy().to_string(),
                size: metadata.len(),
            });
        }
    }

    Ok(files)
}

pub fn delete_files(file_names: Vec<String>) -> Result<()> {
    let storage_path = get_storage_folder_path()?;

    for file_name in file_names.iter() {
        let file_path = storage_path.join(file_name);
    
        if !file_path.exists() {
            Err(Error::from(io::Error::new(io::ErrorKind::NotFound, "File not found in storage")))?
        }
    
        fs::remove_file(file_path)?;

        println!("{} deleted!", file_name);
    }

    Ok(())
}

fn get_storage_folder_path() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let storage_folder_name = "storage";

    Ok(cwd.join(storage_folder_name))
}