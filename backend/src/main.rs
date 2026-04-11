use agforest_backend::{Config, start_server};
use anyhow::Context;
use tracing_subscriber::fmt::format::FmtSpan;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let config: Config = config::Config::builder()
        .add_source(config::Environment::with_prefix("AGFOREST"))
        .build()
        .context("Failed to build configuration")?
        .try_deserialize()
        .context("Failed to deserialize configuration")?;

    start_server(&config)?;

    Ok(())
}
