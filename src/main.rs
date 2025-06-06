use clap::{Parser, Subcommand};

mod docker;

#[derive(Parser)]
#[command(name = "dnetmap", version = "1.0", about = "Mapping tool for Docker networks")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// lists docker networks
    All,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::All => docker::list_all_networks(),
    }
}
