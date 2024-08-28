use crate::prometheus::{PoolLabels, PoolMetrics, ValidatorLabels, ValidatorMetrics};
use crate::ton::MyTonCtrl;

use std::sync::Arc;

use tide::Request;

use prometheus_client::encoding::text::encode;
use prometheus_client::registry::Registry;

// shared webserver context format
#[derive(Clone)]
pub struct State {
    pub name: Arc<String>,
    pub registry: Arc<Registry>,
    pub validator_metrics: Arc<ValidatorMetrics>,
    pub pool_metrics: Arc<PoolMetrics>,
}

// fetch all metrics
pub async fn handle_metrics(req: Request<State>) -> tide::Result {
    // server context
    let state = Arc::new(req.state().clone());

    // instantiate mytonctrl process
    let mut mytonctrl = MyTonCtrl::new();

    // fetch validator metrics
    let validator_status = mytonctrl.validator_status();
    let validator_labels = ValidatorLabels {
        validator_name: state.name.to_string(),
        validator_address: validator_status.address.into(),
    };

    // set validator metrics
    state
        .validator_metrics
        .validator_index
        .get_or_create(&validator_labels)
        .set(validator_status.index.into());
    state
        .validator_metrics
        .validator_balance
        .get_or_create(&validator_labels)
        .set(validator_status.balance.into());
    state
        .validator_metrics
        .validator_outofsync
        .get_or_create(&validator_labels)
        .set(validator_status.outofsync.into());

    // fetch pool metrics
    let pool_status = mytonctrl.pool_status();
    for pool in pool_status {
        let pool_labels = PoolLabels {
            validator_name: state.name.to_string(),
            pool_name: pool.name.into(),
            pool_address: pool.address.into(),
        };

        // set pool metrics
        state
            .pool_metrics
            .pool_active
            .get_or_create(&pool_labels)
            .set(pool.active.into());
        state
            .pool_metrics
            .pool_balance
            .get_or_create(&pool_labels)
            .set(pool.balance.into());
    }

    // send encoded prometheus metrics
    let mut encoded = String::new();
    encode(&mut encoded, &state.registry).unwrap();

    Ok(encoded.into())
}
