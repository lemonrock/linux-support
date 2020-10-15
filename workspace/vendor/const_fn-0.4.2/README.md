# \#\[const\_fn\]

[![crates-badge]][crates-url]
[![docs-badge]][docs-url]
[![license-badge]][license]
[![rustc-badge]][rustc-url]

[crates-badge]: https://img.shields.io/crates/v/const_fn.svg
[crates-url]: https://crates.io/crates/const_fn
[docs-badge]: https://docs.rs/const_fn/badge.svg
[docs-url]: https://docs.rs/const_fn
[license-badge]: https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg
[license]: #license
[rustc-badge]: https://img.shields.io/badge/rustc-1.31+-lightgray.svg
[rustc-url]: https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html

An attribute for easy generation of const functions with conditional compilations.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
const_fn = "0.4"
```

The current const_fn requires Rust 1.31 or later.

## Examples

```rust
use const_fn::const_fn;

// function is `const` on specified version and later compiler (including beta and nightly)
#[const_fn("1.36")]
pub const fn version() {
    /* ... */
}

// function is `const` on nightly compiler (including dev build)
#[const_fn(nightly)]
pub const fn nightly() {
    /* ... */
}

// function is `const` if `cfg(...)` is true
#[const_fn(cfg(...))]
pub const fn cfg() {
    /* ... */
}

// function is `const` if `cfg(feature = "...")` is true
#[const_fn(feature = "...")]
pub const fn feature() {
    /* ... */
}
```

## Alternatives

This crate is proc-macro, but is very lightweight, and has no dependencies.

You can manually define declarative macros with similar functionality (see [`if_rust_version`](https://github.com/ogoffart/if_rust_version#examples)), or [you can define the same function twice with different cfg](https://github.com/crossbeam-rs/crossbeam/blob/0b6ea5f69fde8768c1cfac0d3601e0b4325d7997/crossbeam-epoch/src/atomic.rs#L340-L372). (Note: the former approach requires more macros to be defined depending on the number of version requirements, the latter approach requires more functions to be maintained manually)

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
