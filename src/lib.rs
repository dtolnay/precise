//! [![github]](https://github.com/dtolnay/precise)&ensp;[![crates-io]](https://crates.io/crates/precise)&ensp;[![docs-rs]](https://docs.rs/precise)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! Decimal representation of the single rational number whose value is
//! mathematically equal to the input float.
//!
//! Note that this is almost never an appropriate way to print user-facing
//! values. For that, use the [ryu] crate instead.
//!
//! [ryu]: https://github.com/dtolnay/ryu
//!
//! # Example
//!
//! ```
//! fn main() {
//!     // 0.1000000000000000055511151231257827021181583404541015625
//!     println!("{}", precise::to_string(0.1));
//! }
//! ```

#![allow(
    clippy::must_use_candidate,
    clippy::needless_doctest_main,
    clippy::redundant_else,
    clippy::unseparated_literal_suffix
)]

use num_bigint::BigUint;
use num_traits::Pow;

pub fn to_string(n: f64) -> String {
    if n.is_nan() {
        return "NaN".to_owned();
    }

    if n.is_infinite() {
        if n.is_sign_positive() {
            return "inf".to_owned();
        } else {
            return "-inf".to_owned();
        }
    }

    let bits = n.to_bits();
    let is_negative = (bits & 0x8000_0000_0000_0000) != 0;
    let biased_exponent = (bits & 0x7FF0_0000_0000_0000) >> 52;
    let significand = (bits & 0x000F_FFFF_FFFF_FFFF) + (1 << 52);

    // value
    //   = significand * 2^(biased_exponent - 1075)
    //   = significand * 2^biased_exponent * 5^1075 / 10^1075
    let significand = BigUint::from(significand);
    let two = BigUint::from(2u8);
    let five = BigUint::from(5u8);
    let numerator = significand * two.pow(biased_exponent) * five.pow(1075u16);

    let mut repr = numerator.to_string();
    let more_zeros = 1076usize.saturating_sub(repr.len());
    let leading_zeros = "0".repeat(more_zeros);
    repr = leading_zeros + &repr;

    let point = repr.len() - 1075;
    repr = format!("{}.{}", &repr[..point], &repr[point..]);
    repr = repr.trim_end_matches('0').to_owned();
    if repr.ends_with('.') {
        repr.push('0');
    }

    if is_negative {
        repr.insert(0, '-');
    }

    repr
}
