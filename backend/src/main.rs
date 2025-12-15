use agforest_backend::*;
use anyhow::{Context, Result};
use config::{Config as ConfigLoader, Environment};
use tracing_subscriber::{self, fmt::format::FmtSpan};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let config: Config = ConfigLoader::builder()
        .add_source(Environment::with_prefix("AGFOREST"))
        .build()
        .context("Failed to build configuration")?
        .try_deserialize()
        .context("Failed to deserialize configuration")?;

    start_server(&config)?;

    Ok(())
}
