use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "vastai")]
#[command(about = "VastAI API Command Line Tool", long_about = None)]
pub struct Cli {
    /// Show detailed request information
    #[arg(short, long, global = true)]
    pub info: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Set maintenance mode for an instance
    Maint {
        /// Instance ID (optional, will prompt if not provided)
        #[arg(short = 'd', long)]
        id: Option<u64>,
    },
    /// List machines
    List,
    /// Unlist an instance
    Unlist {
        /// Instance ID (optional, will prompt if not provided)
        #[arg(short = 'd', long)]
        id: Option<u64>,
    },
}
