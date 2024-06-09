# timed-option
A simple library for options with TTLs

```rust
use std::thread;
use std::time::Duration;

use timed_option::TimedOption;

let ttl = Duration::from_millis(100);
let access_token = TimedOption::some("token", ttl);
assert_eq!(true, access_token.is_some());
thread::sleep(ttl);
assert_eq!(false, access_token.is_some());
```