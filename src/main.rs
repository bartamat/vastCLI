mod api;
mod cli;
mod config;
mod handlers;
mod models;
mod ui;

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;

fn main() {
    let cli = Cli::parse();

    let config = Config::new();
    let api_key = match config.get_api_key() {
        Ok(key) => key,
        Err(e) => {
            ui::print_error(&format!("Failed to get API key: {}", e));
            return;
        }
    };

    match cli.command {
        Commands::Maint { id } => handlers::handle_maint(id, &api_key),
        Commands::List => handlers::handle_list(&api_key),
        Commands::Unlist { id } => handlers::handle_unlist(id, &api_key),
    }
}
