use std::collections::BTreeMap;

use happy_wakey_sidecar::config::{
    OPTO_SYNC_BASE_URL_ENV, PRODUCT_KIND_ENV, PRODUCT_PROBE_URL_ENV, SHARED_AUTH_BASE_URL_ENV,
};
use happy_wakey_sidecar::{Config, Event, Machine, Phase};

#[test]
fn external_consumer_can_construct_only_a_complete_safe_configuration() {
    let values = BTreeMap::from([
        (PRODUCT_KIND_ENV.to_owned(), "web".to_owned()),
        (
            PRODUCT_PROBE_URL_ENV.to_owned(),
            "http://127.0.0.1:8081/healthz".to_owned(),
        ),
        (
            SHARED_AUTH_BASE_URL_ENV.to_owned(),
            "https://auth.example.test".to_owned(),
        ),
        (
            OPTO_SYNC_BASE_URL_ENV.to_owned(),
            "http://opto-sync.sync.svc".to_owned(),
        ),
    ]);
    let config = Config::from_lookup(|key| values.get(key).cloned()).unwrap();
    assert!(config.product_probe.address.ip().is_loopback());

    let mut incomplete = values;
    incomplete.remove(SHARED_AUTH_BASE_URL_ENV);
    assert!(Config::from_lookup(|key| incomplete.get(key).cloned()).is_err());
}

#[test]
fn external_consumer_observes_reducer_capabilities_not_parallel_booleans() {
    let mut machine = Machine::new(1, 1);
    assert_eq!(machine.snapshot().phase, Phase::Booting);
    machine.apply(Event::Configured).unwrap();
    machine.apply(Event::ProbeSucceeded).unwrap();
    assert!(machine.is_ready());
    assert!(machine.snapshot().ready_certified);
    machine.apply(Event::ProbeFailed).unwrap();
    assert_eq!(machine.snapshot().phase, Phase::Degraded);
    assert!(!machine.is_ready());
}
