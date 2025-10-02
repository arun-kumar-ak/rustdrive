use clap::{Parser, Subcommand};
use anyhow::Result;
use rustdrive_core::modules;

/// RustDrive CLI - manage files locally and remotely
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Upload { path: std::path::PathBuf },
    Download { filename: String, path: std::path::PathBuf },
    List,
    Delete { filenames: Vec<String> },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Upload { path } => {
            modules::file_crud_op::upload(path.as_path())?;
            println!("File uploaded successfully.");
        }
        Commands::Download { filename, path } => {
            println!("Downloading file: {}", filename);
            modules::file_crud_op::download(&filename, path.as_path())?;
        }
        Commands::List => {
            println!("Listing files...");
            if let Ok(all_files) = modules::file_crud_op::get_files()  {
                if all_files.len() == 0 {
                    println!("No files found.");
                    return Ok(());
                }

                for file in all_files.iter() {
                    println!("fileName: {}, size: {}", file.filename, file.size);
                }
            }
        }
        Commands::Delete { filenames} => {
            println!("deleting files...");

            modules::file_crud_op::delete_files(filenames)?;
        }
    }

    Ok(())
}
