use serde::{Serialize, Deserialize};
use std::{fs::{self, File}, io::{self, Read, Write}, path::{Path, PathBuf}};
use anyhow::{Error, Ok, Result};
use serde_json;

const FILES_META_DATA_FILE_NAME: &str = "files_metadata_internals.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
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

    let file_name = get_file_name(path, &storage_path);

    let dest = storage_path.join(&file_name);
    let copied = fs::copy(path, &dest)?; // returns number of bytes copied

    add_file_name_to_metadata(file_name, copied)
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

pub fn get_files() -> Result<Vec<FileMeta>> {
    let storage_path: PathBuf = get_storage_folder_path()?;
    let meta_file_path = storage_path.join(FILES_META_DATA_FILE_NAME);

    if !storage_path.exists() || !meta_file_path.exists() {
        return Ok(vec![]); // No files if storage doesn't exist
    }

    let mut meta_file_content = String::new();
    let mut meta_file = fs::OpenOptions::new().create(true).read(true).write(true).open(meta_file_path)?;
    meta_file.read_to_string(&mut meta_file_content)?;

    if let std::result::Result::Ok(output) = serde_json::from_str(&meta_file_content) {
        Ok(output)
    } else {
        Ok(vec![])
    }
}

pub fn delete_files(file_names: Vec<String>) -> Result<()> {
    let storage_path = get_storage_folder_path()?;
    let mut file_meta = get_files_from_meta_file()?;

    for file_name in file_names.iter() {
        let file_path = storage_path.join(file_name);
    
        if !file_path.exists() {
            Err(Error::from(io::Error::new(io::ErrorKind::NotFound, "File not found in storage")))?
        }
    
        fs::remove_file(file_path)?;
        
        if let Some(file_index) = file_meta.iter().position(|file_item| file_item.filename == *file_name) {
            file_meta.remove(file_index);
        }

        println!("{} deleted!", file_name);
    }

    set_meta_to_meta_file(file_meta)?;

    Ok(())
}

fn get_file_name(file_path: &Path, storage_path: &PathBuf) -> String {
    let mut file_name = if let Some(name) = file_path.file_name() {
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

    file_name
}

fn get_storage_folder_path() -> Result<PathBuf> {
    let cwd = std::env::current_dir()?;
    let storage_folder_name = "storage";

    Ok(cwd.join(storage_folder_name))
}

fn get_meta_file() -> Result<File> {
    let storage_path: PathBuf = get_storage_folder_path()?;
    let meta_file_path = storage_path.join(FILES_META_DATA_FILE_NAME);
    
    Ok(fs::OpenOptions::new().create(true).read(true).write(true).open(meta_file_path)?)
}

fn get_files_from_meta_file() -> Result<Vec<FileMeta>> {
    let mut meta_file = get_meta_file()?;
    let mut meta_file_content = String::new();

    meta_file.read_to_string(&mut meta_file_content)?;

    if let std::result::Result::Ok(output) = serde_json::from_str(&meta_file_content) {
        Ok(output)
    } else {
        Ok(vec![])
    }
}

fn add_file_name_to_metadata(filename: String, file_size: u64) -> Result<FileMeta>{
    let mut parsed_meta_file_content: Vec<FileMeta> = get_files_from_meta_file()?;

    let new_file_meta = FileMeta { filename, size: file_size };

    parsed_meta_file_content.push(new_file_meta.clone());

    set_meta_to_meta_file(parsed_meta_file_content)?;

    Ok(new_file_meta)
}

fn set_meta_to_meta_file(file_meta: Vec<FileMeta>) -> Result<()> {
    let mut meta_file = get_meta_file()?;
    let de_serialized_content = serde_json::to_string_pretty(&file_meta)?;

    meta_file.set_len(0)?;

    meta_file.write_all(de_serialized_content.as_bytes())?;

    Ok(())
}