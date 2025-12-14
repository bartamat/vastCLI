mod api;
mod cli;
mod config;
mod handlers;
mod models;
mod ui;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Maint { id } => handlers::handle_maint(id),
        Commands::List => handlers::handle_list(),
        Commands::Unlist { id } => handlers::handle_unlist(id),
    }
}
