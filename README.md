This attribute macro will retry a test multiple times, failing only if all attempts fail.
Useful for situations when a test is known to be flaky due to external conditions.

```rust
use test_retry::retry;

#[test]
#[retry]
fn my_test() {
  assert_eq!(1, 2);
}
```
