use std::sync::atomic::{AtomicUsize, Ordering};
use test_retry::retry;

#[retry(count = 5)]
#[test]
fn test_succeeds_with_enough_retries() {
    static ATTEMPTS: AtomicUsize = AtomicUsize::new(0);
    let attempts = ATTEMPTS.fetch_add(1, Ordering::SeqCst);
    if attempts < 2 {
        panic!("Test failed on attempt {}", attempts + 1);
    }
}

#[retry(count = 2)]
#[test]
#[should_panic]
fn test_fails_with_insufficient_retries() {
    static ATTEMPTS: AtomicUsize = AtomicUsize::new(0);
    let attempts = ATTEMPTS.fetch_add(1, Ordering::SeqCst);
    if attempts < 2 {
        panic!("Test failed on attempt {}", attempts + 1);
    }
}

#[retry(count = 3)]
#[test]
fn test_runs_only_once_on_success() {
    static ATTEMPTS: AtomicUsize = AtomicUsize::new(0);
    let attempts = ATTEMPTS.fetch_add(1, Ordering::SeqCst);
    assert_eq!(attempts, 0, "Test should only run once");
}

#[retry(count = 3)]
#[test]
#[should_panic]
fn test_fails_after_all_retries() {
    panic!("Test failed.");
}

#[retry]
#[test]
fn test_succeeds_with_default_retries() {
    static ATTEMPTS: AtomicUsize = AtomicUsize::new(0);
    let attempts = ATTEMPTS.fetch_add(1, Ordering::SeqCst);
    if attempts < 2 {
        panic!("Test failed on attempt {}", attempts + 1);
    }
}

#[retry]
#[test]
#[should_panic]
fn test_fails_with_default_retries() {
    static ATTEMPTS: AtomicUsize = AtomicUsize::new(0);
    let attempts = ATTEMPTS.fetch_add(1, Ordering::SeqCst);
    if attempts < 3 {
        panic!("Test failed on attempt {}", attempts + 1);
    }
}
