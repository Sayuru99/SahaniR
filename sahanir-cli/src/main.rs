use clap::{Parser, Subcommand};
use anyhow::Result;

mod new_project;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new SahaniR project
    New {
        /// The name of the project
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::New { name } => {
            new_project::create_new_project(name).await?;
        }
    }

    Ok(())
}
