use std::sync::Arc;

use tonetheus::constants::METRICS_PREFIX;
use tonetheus::http::{handle_metrics, State};
use tonetheus::prometheus::{PoolMetrics, ValidatorMetrics};
use tonetheus::utils::Args;

use clap::Parser;

use prometheus_client::registry::Registry;

#[async_std::main]
async fn main() -> tide::Result<()> {
    // parse CLI args
    let args = Args::parse();

    // initialize registry
    let mut registry = Registry::default();

    // register prometheus metrics
    let validator_metrics = ValidatorMetrics::default();
    registry.register(
        format!("{METRICS_PREFIX}_validator_index"),
        "Current validator index",
        validator_metrics.validator_index.clone(),
    );
    registry.register(
        format!("{METRICS_PREFIX}_validator_balance"),
        "Validator wallet balance",
        validator_metrics.validator_balance.clone(),
    );
    registry.register(
        format!("{METRICS_PREFIX}_validator_outofsync"),
        "Validator Out of Sync duration (in seconds)",
        validator_metrics.validator_outofsync.clone(),
    );

    let pool_metrics = PoolMetrics::default();
    registry.register(
        format!("{METRICS_PREFIX}_pool_active"),
        "Is the current pool active?",
        pool_metrics.pool_active.clone(),
    );
    registry.register(
        format!("{METRICS_PREFIX}_pool_balance"),
        "Pool balance",
        pool_metrics.pool_balance.clone(),
    );

    // initialize tide app
    tide::log::start();
    let mut app = tide::with_state(State {
        name: Arc::new(args.name),
        registry: Arc::new(registry),
        validator_metrics: Arc::new(validator_metrics),
        pool_metrics: Arc::new(pool_metrics),
    });

    // register endpoints
    app.at("/metrics").get(handle_metrics);

    // start webserver
    app.listen(format!("{}:{}", &args.host, &args.port)).await?;

    Ok(())
}
