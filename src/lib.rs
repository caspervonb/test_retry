//! This crate provides an attribute macro for retrying tests multiple times before failing.
//!
//! # Examples
//!
//! By default retry will cause the test to be called three times before failing:
//! ```
//! use std::sync::atomic::{AtomicUsize, Ordering};
//!
//! #[test]
//! #[retry]
//! fn default() {
//!   static COUNTER: AtomicUsize = AtomicUsize::new(1);
//!   assert_eq!(counter.fetch_add(1, Ordering::Relaxed), 3);
//! }
//! ```
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

/// An attribute which can be used with `#[test]` in order to retry a single
/// test multiple times before failing.
///
/// # Examples
///
/// By default retry will call the test function three times before failing:
/// ```
/// use std::sync::atomic::{AtomicUsize, Ordering};
///
/// #[test]
/// #[retry]
/// fn default() {
///   static COUNTER: AtomicUsize = AtomicUsize::new(1);
///   assert_eq!(counter.fetch_add(1, Ordering::Relaxed), 3);
/// }
/// ```
#[proc_macro_attribute]
pub fn retry(_args: TokenStream, input: TokenStream) -> TokenStream {
    // TODO add count as an argument
    let count = 3;

    let input = parse_macro_input!(input as ItemFn);
    let attrs = input.attrs.clone();
    let name = input.sig.ident.clone();

    TokenStream::from(quote! {
        #(#attrs),*
        fn #name() {
            #input

            for i in 0..#count {
                let result = std::panic::catch_unwind(|| {
                    #name();
                });

                if result.is_ok() {
                    return;
                }

                if i == #count - 1 {
                    std::panic::resume_unwind(result.unwrap_err());
                }
            }
        }
    })
}
