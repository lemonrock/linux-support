// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use maplit::hashmap;
use maplit::hashset;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ffi::OsString;
use std::io;
use std::ops::Deref;
use std::ops::DerefMut;


include!("all_combinations_and_permutations_of_application_protocols.rs");
include!("all_combinations_and_permutations_of_modern_diameter_application_protocols.rs");
include!("application_protocol_permutation_to_string.rs");
include!("AllPermutationsOfASet.rs");
include!("Code.rs");
include!("Combination.rs");
include!("GenerateParseTree.rs");
include!("Input.rs");
include!("legacy_diameter.rs");
include!("legacy_sip.rs");
include!("MaximumServiceFieldSize.rs");
include!("modern_diameter.rs");
include!("NaiveTrie.rs");
include!("NaiveTrieNode.rs");
include!("NaiveTrieNodeIterator.rs");
include!("modern_diameter_application_protocols.rs");
include!("modern_diameter_application_services.rs");
include!("Permutation.rs");
include!("Permutations.rs");
include!("modern_diameter_application_identifiers.rs");


pub fn main(manifest_dir: &OsString, out_dir: &OsString) -> io::Result<()>
{
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/all_combinations_and_permutations_of_application_protocols.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/all_combinations_and_permutations_of_modern_diameter_application_protocols.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/application_protocol_permutation_to_string.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/AllPermutationsOfASet.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/Code.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/Combination.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/GenerateParseTree.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/Input.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/legacy_diameter.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/legacy_sip.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/MaximumServiceFieldSize.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/modern_diameter.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/NaiveTrie.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/NaiveTrieNode.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/NaiveTrieNodeIterator.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/modern_diameter_application_protocols.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/modern_diameter_application_services.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/Permutation.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/Permutations.rs");
	println!("cargo:rerun-if-changed={}", "src/build/naptr_service_parser/modern_diameter_application_identifiers.rs");
	
	let mut code = String::with_capacity(1024 * 1024);
	
	let mut all = NaiveTrie::new();
	all.add(legacy_diameter());
	all.add(legacy_sip());
	all.add(modern_diameter(&mut code));
	
	let generate_parse_tree = GenerateParseTree::new(&mut code);
	generate_parse_tree.generate(&all);
	
	// TODO: E2U enum services, and remaining S-NAPTR applications eg radius
	
	// TODO: Write out code!
	panic!(code);
	
	Ok(())
}
