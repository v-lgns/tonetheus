use std::sync::atomic::AtomicU64;

use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;

// validator metrics label format
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct ValidatorLabels {
    pub validator_name: String,
    pub validator_address: Option<String>,
}

// validator metrics
#[derive(Clone, Default)]
pub struct ValidatorMetrics {
    pub validator_index: Family<ValidatorLabels, Gauge>,
    pub validator_balance: Family<ValidatorLabels, Gauge<f64, AtomicU64>>,
    pub validator_outofsync: Family<ValidatorLabels, Gauge>,
}

// pool metrics label format
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct PoolLabels {
    pub validator_name: String,
    pub pool_name: Option<String>,
    pub pool_address: Option<String>,
}

// pool metrics
#[derive(Clone, Default)]
pub struct PoolMetrics {
    pub pool_active: Family<PoolLabels, Gauge>,
    pub pool_balance: Family<PoolLabels, Gauge<f64, AtomicU64>>,
}
