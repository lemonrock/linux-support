# const_fn_assert

[![Build Status](https://travis-ci.com/powlpy/const_fn_assert.svg?branch=master)](https://travis-ci.com/powlpy/const_fn_assert)
[![Crate](https://img.shields.io/crates/v/const_fn_assert.svg)](https://crates.io/crates/const_fn_assert)
[![Documentation](https://docs.rs/const_fn_assert/badge.svg)](https://docs.rs/const_fn_assert)
![Minimum rustc version](https://img.shields.io/badge/rustc-1.31+-yellow.svg)
[![License](https://img.shields.io/crates/l/const_fn_assert.svg)](https://github.com/powlpy/const_fn_assert/blob/master/LICENSE)

This crate provide macros assertions who can be used in `const` function.

- [Documentation](https://docs.rs/const_fn_assert)
- [Release notes](https://github.com/powlpy/const_fn_assert/releases)

## Example

```rust
const fn my_const_fn(x: u8) -> u8 {
    cfn_assert!(x < 5);
    x + 1
}

const _CONST: u8 = my_const_fn(1);

fn main() {
    let _var = my_const_fn(2);
}
```

The function below panic when running :
```rust
fn fail() {
    let _var = my_const_fn(6); //thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1'
}
```
And this code don't compile :
```rust
const _CONST: u8 = my_const_fn(6); //~ ERROR any use of this value will cause an error
```

Available macros are `cfn_assert`, `cfn_assert_eq`, `cfn_assert_ne`, `cfn_debug_assert`, `cfn_debug_assert_eq` and `cfn_debug_assert_ne`.

## Installation

This crate is available
[on crates.io](https://crates.io/crates/const_fn_assert) and can be used by
adding the following to your project's
[`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html):

```toml
[dependencies]
const_fn_assert = "0.1"
```

and this to your crate root (`main.rs` or `lib.rs`):

```rust
#[macro_use]
extern crate const_fn_assert;
```
