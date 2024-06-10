use timed_option::TimedValue;

const TIMED_VALUE_NONE: TimedValue<()> = TimedValue::None;

#[test]
fn timed_value() {
    assert!(TimedValue::Valid("But just remember how we shook, shook").is_valid());
    assert!(!TimedValue::Expired("And all the things we took, took").is_valid());
    assert!(!TIMED_VALUE_NONE.is_valid());

    assert!(!TimedValue::Valid("This town's the oldest friend of mine").is_expired());
    assert!(TimedValue::Expired("duu du-du du-du du-du --").is_expired());
    assert!(!TIMED_VALUE_NONE.is_expired());

    assert!(TimedValue::Valid("The sky is turning purple").has_value());
    assert!(TimedValue::Expired("Then orange, then pink and yellow").has_value());
    assert!(!TIMED_VALUE_NONE.has_value());

    assert!(!TimedValue::Valid("But instead I stand still").is_none());
    assert!(!TimedValue::Expired("heart cracking").is_none());
    assert!(TIMED_VALUE_NONE.is_none());

    assert_eq!(
        TimedValue::Valid("I feel like summer").as_ref(),
        TimedValue::Valid(&"I feel like summer")
    );
    assert_eq!(
        TimedValue::Expired("We dream like we need to see").as_ref(),
        TimedValue::Expired(&"We dream like we need to see")
    );
}

#[test]
fn timed_value_eq() {
    assert_eq!(
        TimedValue::Valid("thousand-year blood war"),
        TimedValue::Valid("thousand-year blood war")
    );

    assert_eq!(
        TimedValue::Expired("thousand-year blood war"),
        TimedValue::Expired("thousand-year blood war")
    );

    assert_eq!(TIMED_VALUE_NONE, TimedValue::None);

    assert_ne!(
        TimedValue::Valid("thousand-year blood war"),
        TimedValue::Expired("thousand-year blood war")
    );
    assert_ne!(
        TimedValue::Valid("day eight thousand one hundred eighteen"),
        TimedValue::None
    );

    assert_ne!(
        TimedValue::Expired("But he replies with, 'Okay', every time, every time"),
        TimedValue::None
    );
}
