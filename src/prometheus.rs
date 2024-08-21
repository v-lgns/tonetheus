use prometheus_client::encoding::EncodeLabelSet;
use prometheus_client::metrics::family::Family;
use prometheus_client::metrics::gauge::Gauge;

// prometheus metrics label format
#[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
pub struct Labels {
    pub network: String,
    pub validator_name: Option<String>,
    pub validator_address: Option<String>,
}

// prometheus metrics
#[derive(Clone, Default)]
pub struct Metrics {
    pub validator_index: Family<Labels, Gauge>,
    pub validator_balance: Family<Labels, Gauge>,
    pub validator_outofsync: Family<Labels, Gauge>,
}
