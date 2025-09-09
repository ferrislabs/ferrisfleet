use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "🦀 FerrisFleet CLI")]
#[command(about = "A CLI for testing FerrisFleet core functionality")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Cluster {
        #[command(subcommand)]
        action: ClusterAction,
    },
}

#[derive(Subcommand)]
pub enum ClusterAction {
    /// Create a new cluster
    Create {
        /// Name of the cluster
        name: String,
    },
    /// List all clusters
    List,
    /// Show details of a specific cluster
    Show {
        /// Cluster ID or name
        identifier: String,
    },
}
