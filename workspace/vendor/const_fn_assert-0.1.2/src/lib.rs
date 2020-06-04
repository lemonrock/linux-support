// Copyright 2019 const_fn_assert Developers
//
// Licensed under the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>. This file may not be
// copied, modified, or distributed except according to those terms.

/*!
This crate provide macros assertions who can be used in `const` functions.

# Examples

```rust
# #[macro_use]
# extern crate const_fn_assert;
const fn my_const_fn(x: u8) -> u8 {
    cfn_assert!(x < 5);
    x + 1
}

const _CONST: u8 = my_const_fn(1);

fn main() {
    let mut _var = my_const_fn(2);
}
```

Inputs are type-checked as booleans:

```compile_fail
# #[macro_use] extern crate const_fn_assert;
fn main() {
    cfn_assert!(!0);
}
```
Despite this being a macro, we see this produces a type error:

```txt
  | cfn_assert!(!0);
  |             ^^ expected bool, found integral variable
  |
  = note: expected type `bool`
             found type `{integer}`
```

The function below panic when running :
```should_panic
# #[macro_use] extern crate const_fn_assert;
# const fn my_const_fn(x: u8) -> u8 { cfn_assert!(x < 5); x + 1 }
fn fail() {
    let _var = my_const_fn(6); //thread 'main' panicked at 'index out of bounds: the len is 1 but the index is 1'
}
# fn main() { fail(); }
```

And this code don't compile :
```compile_fail
# #[macro_use] extern crate const_fn_assert;
# const fn my_const_fn(x: u8) -> u8 { cfn_assert!(x < 5); x + 1 }
const _CONST: u8 = my_const_fn(6); //~ ERROR any use of this value will cause an error
# fn main() { }
```

# Advices

Since the panic message is not really descriptive, it is advisable to create
a non-constant version of your functions using normal assertions.
*/

#![no_std]
#![doc(html_root_url = "https://docs.rs/const_fn_assert")]

#![forbid(const_err)]

#[allow(dead_code)]
/// Used by the macros of this crate to check the assertions.
pub const ASSERT: [(); 1] = [()];

#[inline(always)]
#[allow(dead_code)]
/// Used by the macros of this crate to check that the inputs are boolean.
pub const fn bool_assert(x: bool) -> bool { x }

#[macro_export]
macro_rules! cfn_assert {
    ($x:expr $(,)*) => {
        let _ = $crate::ASSERT[!$crate::bool_assert($x) as usize];
    };
}

#[macro_export]
macro_rules! cfn_assert_eq {
    ($x:expr, $y:expr $(,)*) => {
        $crate::cfn_assert!($x == $y)
    };
}

#[macro_export]
macro_rules! cfn_assert_ne {
    ($x:expr, $y:expr $(,)*) => {
        $crate::cfn_assert!($x != $y)
    };
}

#[macro_export]
macro_rules! cfn_debug_assert {
    ($x:expr $(,)*) => {
        #[cfg(debug_assertions)]
        $crate::cfn_assert!($x)
    };
}

#[macro_export]
macro_rules! cfn_debug_assert_eq {
    ($x:expr, $y:expr $(,)*) => {
        #[cfg(debug_assertions)]
        $crate::cfn_assert_eq!($x, $y)
    };
}

#[macro_export]
macro_rules! cfn_debug_assert_ne {
    ($x:expr, $y:expr $(,)*) => {
        #[cfg(debug_assertions)]
        $crate::cfn_assert_ne!($x, $y)
    };
}
