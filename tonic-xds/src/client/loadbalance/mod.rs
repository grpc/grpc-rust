// `LbChannel` is not constructed by the current P2C wiring (the LoadBalancer
// tracks load via `ReadyChannel` + `EndpointChannel`); retained for now.
#[allow(dead_code)]
pub(crate) mod channel;
pub(crate) mod channel_state;
pub(crate) mod errors;
pub(crate) mod keyed_futures;
pub(crate) mod loadbalancer;
pub(crate) mod outlier_detection;
pub(crate) mod pickers;
pub(crate) mod service;
