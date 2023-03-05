//! [![github]](https://github.com/dtolnay/precise)&ensp;[![crates-io]](https://crates.io/crates/precise)&ensp;[![docs-rs]](https://docs.rs/precise)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
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

#![doc(html_root_url = "https://docs.rs/precise/0.1.7")]
#![allow(
    clippy::must_use_candidate,
    clippy::needless_doctest_main,
    clippy::redundant_else,
    clippy::unseparated_literal_suffix
)]

use num_bigint::BigUint;
use num_traits::Pow;

pub fn to_string(n: impl traits::Float) -> String {
    traits::Float::to_precise_string(n)
}

mod traits {
    pub trait Float: Sized {
        fn to_precise_string(n: Self) -> String;
    }

    impl Float for f32 {
        fn to_precise_string(n: Self) -> String {
            crate::f32_to_precise_string(n)
        }
    }

    impl Float for f64 {
        fn to_precise_string(n: Self) -> String {
            crate::f64_to_precise_string(n)
        }
    }
}

fn f32_to_precise_string(n: f32) -> String {
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
    let is_negative = (bits & 0x8000_0000) != 0;
    let biased_exponent = (bits & 0x7F80_0000) >> 23;
    let significand = (bits & 0x007F_FFFF) + (1 << 23);

    // value
    //   = significand * 2^(biased_exponent - 150)
    //   = significand * 2^biased_exponent * 5^150 / 10^150
    let significand = BigUint::from(significand);
    let two = BigUint::from(2u8);
    let five = BigUint::from(5u8);
    let numerator = significand * two.pow(biased_exponent) * five.pow(150u16);

    let mut repr = numerator.to_string();
    let more_zeros = 151usize.saturating_sub(repr.len());
    let leading_zeros = "0".repeat(more_zeros);
    repr = leading_zeros + &repr;

    repr.insert(repr.len() - 150, '.');
    repr.truncate(repr.trim_end_matches('0').len());
    if repr.ends_with('.') {
        repr.push('0');
    }

    if is_negative {
        repr.insert(0, '-');
    }

    repr
}

fn f64_to_precise_string(n: f64) -> String {
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

    repr.insert(repr.len() - 1075, '.');
    repr.truncate(repr.trim_end_matches('0').len());
    if repr.ends_with('.') {
        repr.push('0');
    }

    if is_negative {
        repr.insert(0, '-');
    }

    repr
}
