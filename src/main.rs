use quests_tracker::config::config_loader;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dotenvy_env = match config_loader::load() {
        Ok(config) => config,
        Err(e) => {
            error!("Error loading config: {:?}", e);
            std::process::exit(1);
        }
    };

    info!("ENV has been loaded: {:?}", dotenvy_env);
}
