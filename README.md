# timed-option
A simple library for options with TTLs

```rust
use std::thread;
use std::time::{Duration, Instant};

use timed_option::{TimedOption, TimedValue};

let ttl = Duration::from_millis(10);
let access_token = TimedOption::<_, Instant>::new("token", ttl);
assert_eq!(true, access_token.is_some());
thread::sleep(ttl);
assert_eq!(false, access_token.is_some());

match access_token.into_option() {
    Some(_) => println!("is_some"),
    None => println!("is_none")
}

match access_token.into_timed_value() {
    TimedValue::Valid(_) => println!("is_valid"),
    TimedValue::Expired(_) => println!("is_expired"),
    TimedValue::None => println!("is_none")
}
```