#![doc(
    html_logo_url = "https://github.com/supernovahs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/supernovahs/eigen-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
use eigen_metrics_derive::Metrics;
use metrics::{Counter, Histogram, Key, Label};

#[derive(Clone, Metrics)]
#[metrics(scope = "eigen.rpcmetrics")]
pub struct RpcCalls {
    #[metric(
        rename = "eigen_rpc_request_duration_seconds",
        describe = " Duration of json-rpc <method> in seconds from Ethereum Execution client <client> "
    )]
    rpc_request_duration_seconds: Histogram,
    #[metric(
        rename = "eigen_rpc_request_total",
        describe = "Total of json-rpc <method> requests from Ethereum Execution client <client> "
    )]
    rpc_request_total: Counter,
}

impl RpcCalls {
    pub fn new() -> Self {
        let rpc_calls = Self {
            rpc_request_duration_seconds: metrics::register_histogram!(
                "eigen_rpc_request_duration_seconds"
            ),
            rpc_request_total: metrics::register_counter!("eigen_rpc_request_total"),
        };

        RpcCalls::describe();

        rpc_calls
    }

    pub fn rpc_request_duration_seconds(&self) -> Histogram {
        self.rpc_request_duration_seconds.clone()
    }

    pub fn rpc_request_total(&self) -> Counter {
        self.rpc_request_total.clone()
    }

    pub fn set_rpc_request_duration_seconds(
        &self,
        method: &str,
        client_version: &str,
        duration: f64,
    ) {
        let key = Key::from_parts(
            "eigen_rpc_request_duration_seconds",
            vec![
                Label::new("method", method.to_string()),
                Label::new("client_version", client_version.to_string()),
            ],
        );

        metrics::histogram!(key.to_string(), duration);
    }

    pub fn set_rpc_request_total(
        &self,
        method: &str,
        client_version: &str,
        rpc_request_total: u64,
    ) {
        let key = Key::from_parts(
            "eigen_rpc_request_total",
            vec![
                Label::new("method", method.to_string()),
                Label::new("client_version", client_version.to_string()),
            ],
        );

        metrics::counter!(key.to_string(), rpc_request_total);
    }
}
