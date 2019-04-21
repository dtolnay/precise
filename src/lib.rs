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
