use std::time::{Duration, Instant};

use timed_option::{TimedOption, TimedValue};

#[test]
fn std_instant_backend() {
    let ttl = Duration::from_secs(3500);
    let mut token = TimedOption::<_, Instant>::new("space_patato", ttl);

    assert!(token.is_some());
    assert!(!token.is_none());

    assert_eq!(token.into_option(), Some("space_patato"));
    assert_eq!(token.into_timed_value(), TimedValue::Valid("space_patato"));

    token.expire();

    assert_eq!(token.into_option(), None);
    assert_eq!(
        token.into_timed_value(),
        TimedValue::Expired("space_patato")
    );

    assert!(!token.is_some());
    assert!(token.is_none());
}
