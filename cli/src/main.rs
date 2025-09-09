use std::sync::Arc;

use clap::Parser;
use ferrisfleet_core::{
    application::services::FerrisFleetService,
    domain::{error::FleetError, ports::cluster::ClusterService},
};

use crate::args::{Cli, ClusterAction, Commands};

pub mod args;

#[tokio::main]
async fn main() -> Result<(), FleetError> {
    println!("hello world cli");
    let cli = Cli::parse();
    let service = FerrisFleetService::new().await?;
    let service = Arc::new(service);

    match cli.command {
        Commands::Cluster { action } => handle_cluster_command(action, service).await,
    }

    Ok(())
}

async fn handle_cluster_command(action: ClusterAction, service: Arc<FerrisFleetService>) {
    match action {
        ClusterAction::Create { name } => {
            println!("✅ Created cluster: {}", name);
        }
        ClusterAction::List => {
            println!("✅ Listing clusters:");
            let t = service.list_vms().await.unwrap();
            for v in t {
                println!(" - {:?}", v);
            }
        }
        ClusterAction::Show { identifier } => {
            println!("✅ Showing cluster: {}", identifier);
        }
    }
}
