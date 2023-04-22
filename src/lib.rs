//! # Test Retry
//!
//! [Documentation](https://docs.rs/test_retry) |
//! [Github](https://github.com/caspervonb/test_retry) |
//! [Crate](https://crates.io/crates/test_retry)
//!
//! An attribute macro that can be used to retry non-idempotent tests multiple times before
//! resulting in a failure.
//!
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, NestedMeta};

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
pub fn retry(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);
    let mut count = 3;

    for arg in attr_args {
        if let NestedMeta::Meta(syn::Meta::NameValue(name_value)) = arg {
            if name_value.path.is_ident("count") {
                if let syn::Lit::Int(lit_int) = name_value.lit {
                    count = lit_int.base10_parse().unwrap();
                }
            }
        }
    }

    let mut input_fn = parse_macro_input!(input as ItemFn);

    let input_fn_attrs = &input_fn.attrs;
    let input_fn_sig = &input_fn.sig;
    let input_fn_block = &input_fn.block;

    let output = quote! {
        #(#input_fn_attrs)*
        #input_fn_sig {
            for i in 0..#count {
                let result = std::panic::catch_unwind(|| #input_fn_block);
                if result.is_ok() {
                    return;
                }

                if i == #count - 1 {
                    std::panic::resume_unwind(result.unwrap_err());
                }
            }
        }
    };

    output.into()
}
