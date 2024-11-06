mod api;
mod entities;
mod megaverse;

use api::ApiClient;
use crate::megaverse::Megaverse;
use anyhow::Result;
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "Crossmint Megaverse Populator")]
#[command(about = "Populate the Megaverse for Phase 1 or Phase 2", long_about = None)]
struct Cli {
    /// Choose the phase to run (e.g., Phase 1, phase1, p1)
    #[arg(short, long, value_enum)]
    phase: Phase,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Phase {
    #[clap(alias = "phase 1", alias = "1", alias = "p1", alias = "Phase 1", alias = "PHASE 1", alias = "P1", alias = "PH1", alias = "ph1", alias = "ph 1", alias = "PH 1", alias = "p 1", alias = "P 1", alias = "Phase1")]
    Phase1,
    #[clap(alias = "phase 2", alias = "2", alias = "p2", alias = "Phase 2", alias = "PHASE 2", alias = "P2", alias = "PH2", alias = "ph2", alias = "ph 2", alias = "PH 2", alias = "p 2", alias = "P 2", alias = "Phase2")]
    Phase2,
}



#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let candidate_id = "43ddbb13-9ff4-465e-b954-e0a0d817b615";
    let api_client = ApiClient::new(candidate_id);
    let megaverse = Megaverse::new(api_client);
    match cli.phase {
        Phase::Phase1 => {
            megaverse.populate().await?;
            println!("Phase 1: Basic Megaverse Population completed.");
        }
        Phase::Phase2 => {
            megaverse.populate_from_goal().await?;
            println!("Phase 2: Goal map populated successfully.");
        }
    }

    Ok(())
}
