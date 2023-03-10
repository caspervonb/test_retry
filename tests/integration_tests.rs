use std::sync::atomic::{AtomicUsize, Ordering};
use test_retry::retry;

#[test]
#[retry]
fn always_ok() {
    assert!(true);
}

#[test]
#[retry]
fn with_atomic_counter() {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    assert_eq!(COUNTER.fetch_add(1, Ordering::Relaxed), 3);
}

#[test]
#[retry]
#[should_panic]
fn always_panic() {
    panic!("Oops, something went wrong!");
}
