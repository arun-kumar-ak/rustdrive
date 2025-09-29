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
    Download { filename: String },
    List,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Upload { path } => {
            modules::file_crud_op::upload(path.as_path())?;
            println!("File uploaded successfully.");
        }
        Commands::Download { filename } => {
            println!("Downloading file: {}", filename);
        }
        Commands::List => {
            println!("Listing files...");
        }
    }

    Ok(())
}
