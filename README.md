Precise float to string
=======================

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/precise-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/precise)
[<img alt="crates.io" src="https://img.shields.io/crates/v/precise.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/precise)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-precise-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/precise)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/dtolnay/precise/CI/master?style=for-the-badge" height="20">](https://github.com/dtolnay/precise/actions?query=branch%3Amaster)

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
