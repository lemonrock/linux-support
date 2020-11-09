// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(missing_docs)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]



use std::env::var_os;
use std::io;


#[path = "src/build/mod.rs"]
pub mod build;


/// Build.
pub fn main() -> io::Result<()>
{
	let manifest_dir = var_os("CARGO_MANIFEST_DIR").unwrap();
	let out_dir = var_os("OUT_DIR").unwrap();
	
	println!("cargo:rerun-if-changed={}", "src/build");
	println!("cargo:rerun-if-changed={}", "src/build/mod.rs");
	
	println!("cargo:rerun-if-changed={}", "src/build/top_level_domains");
	println!("cargo:rerun-if-changed={}", "src/build/top_level_domains/mod.rs");
	build::top_level_domains::main(&manifest_dir, &out_dir)?;
	
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/mod.rs");
	build::naptr_service_parser::main(&manifest_dir, &out_dir)?;
	
	Ok(())
}
