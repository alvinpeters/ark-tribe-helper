mod config;

use tokio::runtime::Runtime;

use anyhow::{Context, Result};
use serenity::all::Http;
use serenity::Client;
use serenity::interactions_endpoint::Verifier;
use tracing::info;
use crate::config::Config;

struct ServerState {
    api_client: Http,
    endpoint_verifier: Verifier
}

fn main() -> Result<()> {
    // Initialise logger
    tracing_subscriber::fmt().init();
    // Start FoundationDB client thread
    // Safe as long as it's dropped
    let fdb_client_guard = unsafe { foundationdb::boot() };

    // Get compiled config
    let config = Config::new();

    // Set up the server state
    let server_state = ServerState::new(&config);

    //


    // Start tokio runtime
    let runtime = Runtime::new().context("failed to initialise tokio runtime")?;

    runtime.block_on(run(server_state));

    drop(fdb_client_guard);
    Ok(())
}

async fn run(server_state: ServerState) {
    info!("Hello, world!");

    experiment_time().await
}


async fn experiment_time() {

}

impl ServerState {
    fn new(config: &Config) -> Self {
        let api_client = Http::new(&config.bot_token);
        let endpoint_verifier = Verifier::new(&config.public_key);

        Self {
            api_client,
            endpoint_verifier
        }
    }
}