Precise float to string
=======================

[![Build Status](https://img.shields.io/github/workflow/status/dtolnay/precise/CI/master)](https://github.com/dtolnay/precise/actions?query=branch%3Amaster)
[![Latest Version](https://img.shields.io/crates/v/precise.svg)](https://crates.io/crates/precise)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/precise)

This crate computes the decimal representation of the single rational number
whose value is mathematically equal to a given float.

*Note that this is almost never an appropriate way to print user-facing values.
For that, use the [ryu] crate instead.*

[ryu]: https://github.com/dtolnay/ryu

```toml
[dependencies]
precise = "0.1"
```

## Example

```rust
fn main() {
    // 0.1000000000000000055511151231257827021181583404541015625
    println!("{}", precise::to_string(0.1));
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
