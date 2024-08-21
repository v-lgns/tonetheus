use crate::prometheus::{ValidatorLabels, ValidatorMetrics};
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
        network: validator_status.network.into(),
    };

    // set metrics
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

    // send encoded prometheus metrics
    let mut encoded = String::new();
    encode(&mut encoded, &state.registry).unwrap();

    Ok(encoded.into())
}
