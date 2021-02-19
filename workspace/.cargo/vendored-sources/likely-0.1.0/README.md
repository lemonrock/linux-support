# likely

[likely] is a rust crate used to provide macro wrappers around the intrinsics `::std::intrinsic::likely` and `::std::intrinsic::unlikely`, which themselves are hints to the CPU's branch predictor.

These hints should only be used when the vast majority of cases would satisfy them. A typicaly example might be a highly unusual error condition, or an extreme value or outlier in a range. Another common scenario is dealing with return codes from common libc functions, many of which are highly unlikely to occur (eg `EINVAL` for many such functions).

Note that using these macro wrappers requires compiling with nightly.


## Using It.


### Firstly, Include it

To include it, add the following to your `Cargo.toml`:-

```toml
[dependencies]
likely = "^0.1"
```

In your crate's `lib.rs`, add:-

```rust
`#![feature(core_intrinsics)]`

...

#[macro_use] extern crate likely;
```


### Secondly, use it with `if` statements

```rust
fn example(age: u8)
{
	if likely!(age <= 70)
	{
		println!("Not yet entitled to this welfare benefit")
	}
	else if unlikely!(age == 99)
	{
		println!("You are entitled to a birthday card from the Queen for your next birthday")
	}
}
```


## Licensing

The license for this project is MIT.

[likely]: https://github.com/lemonrock/likely "likely GitHub page"
