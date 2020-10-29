// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![deny(missing_docs)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]


//! The following top-level domains (TLDs) are generated from the database at <https://data.iana.org/TLD/tlds-alpha-by-domain.txt>
//!
//! This was consulted on 29th October 2020 and `Version 2020102900, Last Updated Thu Oct 29 07:07:01 2020 UTC` was used; the root zone was last updated on 22nd October 2020 (with serial number `2020102201`).
//!
//! Other sources are:-
//!
//! * <https://www.iana.org/domains/root/files> which contains
//! 		* The root zone file from <https://www.internic.net/domain/root.zone> which includes records but requires writing a master file parser;
//! 		* The hints file at <https://www.internic.net/domain/named.root>;
//!		* The root trust anchor <https://data.iana.org/root-anchors/>;
//!		* The top-level domain list at <https://data.iana.org/TLD/tlds-alpha-by-domain.txt>.
//! * <https://www.iana.org/domains/root/db>, which includes test domains and clarifies domains by type (and is a super set):-
//! 		* `country-code`.
//! 		* `generic`.
//! 		* `generic-restricted`.
//!	 	* `infrastructure`.
//! 		* `sponsored`.
//! 		* `test`. (not present in the root zone file).
//! * The daily reports at <http://stats.research.icann.org/dns/tld_report/index.html> were consulted, which gives insight into insecure domains.


use std::env::var_os;
use std::ffi::OsString;
use std::fs::File;
use std::fs::read;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;


const TopLevelDomainsRelativeFilePath: &'static str = "src/coroutines/domain_name_service/domain_model/name/tlds-alpha-by-domain.txt";


/// Build.
pub fn main() -> io::Result<()>
{
	println!("cargo:rerun-if-changed={}", TopLevelDomainsRelativeFilePath);
	
	let manifest_dir = var_os("CARGO_MANIFEST_DIR").unwrap();
	
	let (version_and_updated_at, top_level_domains) = top_level_domains(&manifest_dir)?;
	
	let out_dir = var_os("OUT_DIR").unwrap();
	
	write_case_folded_labels(&out_dir, &top_level_domains)?;
	write_case_folded_names(&out_dir, &top_level_domains, &version_and_updated_at)?;
	
	Ok(())
}

fn write_case_folded_labels(out_dir: &OsString, top_level_domains: &Vec<CaseFoldedLabel>) -> io::Result<()>
{
	let mut file = BufWriter::new(File::create(&Path::new(&out_dir).join("CaseFoldedLabel.top-level-domains.rs"))?);
	
	writeln!(file, "impl CaseFoldedLabel<'static>")?;
	writeln!(file, "{{")?;
	
		for case_folded_label in top_level_domains
		{
			writeln!(file, "\tconst {}: Self = Self::new(b\"{}\");", case_folded_label.with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit(), case_folded_label.as_str())?;
		}
	
	writeln!(file, "}}")?;
	
	Ok(())
}

fn write_case_folded_names(out_dir: &OsString, top_level_domains: &Vec<CaseFoldedLabel>, version_and_updated_at: &str) -> io::Result<()>
{
	let mut file = BufWriter::new(File::create(&Path::new(&out_dir).join("CaseFoldedName.top-level-domains.rs"))?);
	writeln!(file, "impl CaseFoldedName<'static>")?;
	writeln!(file, "{{")?;
	
		from_iana_comment(&mut file, version_and_updated_at)?;
		writeln!(file, "\t#[inline(always)]")?;
		writeln!(file, "\tpub fn top_level_domain_names() -> &'static HashSet<Self>")?;
		writeln!(file, "\t{{")?;
			writeln!(file, "\t\tlazy_static!")?;
			writeln!(file, "\t\t{{")?;
				writeln!(file, "\t\t\tstatic ref top_level_domain_names: HashSet<CaseFoldedName<'static>> = hashset!")?;
				writeln!(file, "\t\t\t{{")?;
				for case_folded_label in top_level_domains
				{
					writeln!(file, "\t\t\t\tCaseFoldedName::{}().clone(),", case_folded_label.with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit())?;
				}
				writeln!(file, "\t\t\t}};")?;
				writeln!(file, "\t\t\t")?;
			writeln!(file, "\t\t}}")?;
			writeln!(file, "\t\t&top_level_domain_names")?;
		writeln!(file, "\t}}")?;
	
		for case_folded_label in top_level_domains
		{
			writeln!(file)?;
			writeln!(file, "\t/// `{}.`", case_folded_label.as_str())?;
			from_iana_comment(&mut file, version_and_updated_at)?;
			writeln!(file, "\t#[inline(always)]")?;
			writeln!(file, "\tpub fn {}() -> &'static Self", case_folded_label.with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit())?;
			writeln!(file, "\t{{")?;
			writeln!(file, "\t\tlazy_static!")?;
			writeln!(file, "\t\t{{")?;
				writeln!(file, "\t\t\tstatic ref {}: CaseFoldedName<'static> = CaseFoldedName::top_level(CaseFoldedLabel::{});", case_folded_label.with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit(), case_folded_label.with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit())?;
			writeln!(file, "\t\t}}")?;
			writeln!(file, "\t\t")?;
			writeln!(file, "\t\t&{}", case_folded_label.with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit())?;
			writeln!(file, "\t}}")?;
		}
	
	writeln!(file, "}}")?;
	
	Ok(())
}

fn from_iana_comment(file: &mut BufWriter<File>, version_and_updated_at: &str) -> io::Result<()>
{
	writeln!(file, "\t///")?;
	writeln!(file, "\t/// From [IANA](https://data.iana.org/TLD/tlds-alpha-by-domain.txt) as of `{}`.", version_and_updated_at)?;
	Ok(())
}

fn top_level_domains(manifest_dir: &OsString) -> io::Result<(String, Vec<CaseFoldedLabel>)>
{
	let tlds_alpha_by_domain_file_path = Path::new(&manifest_dir).join(TopLevelDomainsRelativeFilePath);
	
	let mut tlds_alpha_by_domain = read(tlds_alpha_by_domain_file_path)?;
	
	// Prevent a final empty line.
	if tlds_alpha_by_domain.get(tlds_alpha_by_domain.len() - 1) == Some(&b'\n')
	{
		tlds_alpha_by_domain.pop();
	}
	
	let mut version_and_updated_at = None;
	let mut top_level_domains = Vec::with_capacity(2048);
	for line in tlds_alpha_by_domain.split(|byte| *byte == b'\n')
	{
		let line_starts_with_hash = line.starts_with(b"#");
		if version_and_updated_at.is_none()
		{
			if line_starts_with_hash
			{
				// "# "
				version_and_updated_at = Some(String::from_utf8_lossy(&line[2 ..]).into_owned());
				continue
			}
			else
			{
				panic!("First line was not a comment line")
			}
		}
		else
		{
			if line_starts_with_hash
			{
				panic!("Subsequent line started with a hash")
			}
		}
		
		let case_folded_label = CaseFoldedLabel::case_fold_label(line);
		top_level_domains.push(case_folded_label)
	}
	
	Ok((version_and_updated_at.expect("No first line"), top_level_domains))
}

struct CaseFoldedLabel(String);

impl CaseFoldedLabel
{
	/// Sourced from <https://doc.rust-lang.org/reference/keywords.html> for Rust 2018.
	fn is_rust_keyword(value: &str) -> bool
	{
		match value
		{
			"as" => true,
			
			"break" => true,
			
			"const" => true,
			
			"continue" => true,
			
			"crate" => true,
			
			"else" => true,
			
			"enum" => true,
			
			"extern" => true,
			
			"false" => true,
			
			"fn" => true,
			
			"for" => true,
			
			"if" => true,
			
			"impl" => true,
			
			"in" => true,
			
			"let" => true,
			
			"loop" => true,
			
			"match" => true,
			
			"mod" => true,
			
			"move" => true,
			
			"mut" => true,
			
			"pub" => true,
			
			"ref" => true,
			
			"return" => true,
			
			"self" => true,
			
			"Self" => true,
			
			"static" => true,
			
			"struct" => true,
			
			"super" => true,
			
			"trait" => true,
			
			"true" => true,
			
			"type" => true,
			
			"unsafe" => true,
			
			"use" => true,
			
			"where" => true,
			
			"while" => true,
			
			"async" => true,
			
			"await" => true,
			
			"dyn" => true,
			
			"abstract" => true,
			
			"become" => true,
			
			"box" => true,
			
			"do" => true,
			
			"final" => true,
			
			"macro" => true,
			
			"override" => true,
			
			"priv" => true,
			
			"typeof" => true,
			
			"unsized" => true,
			
			"virtual" => true,
			
			"yield" => true,
			
			"try" => true,
			
			_ => false,
		}
	}
	
	fn is_function_or_constant_otherwise_defined_in_case_folded_label(value: &str) -> bool
	{
		match value
		{
			"new" => true,
			
			_ => false,
		}
	}
	
	fn with_hyphens_as_underscores_and_leading_underscore_if_starts_with_digit(&self) -> String
	{
		let mut with_hyphens_as_underscores = self.0.replace("-", "_");
		
		let starts_with_digit = match *with_hyphens_as_underscores.as_bytes().get(0).unwrap()
		{
			b'0' ..= b'9' => true,
			_ => false,
		};
		if starts_with_digit
		{
			with_hyphens_as_underscores.insert(0, '_');
		}
		
		if Self::is_rust_keyword(&with_hyphens_as_underscores)
		{
			with_hyphens_as_underscores.insert_str(0, "r#");
		}
		else
		{
			while Self::is_function_or_constant_otherwise_defined_in_case_folded_label(&with_hyphens_as_underscores)
			{
				with_hyphens_as_underscores.push_str("_");
			}
		}
		
		with_hyphens_as_underscores
	}
	
	fn as_str(&self) -> &str
	{
		&self.0
	}
	
	/// Processing is in accordance with RFC 952 ASSUMPTIONS, 1. and RFC 1123, Section 2.1 Host Names and Numbers, but:-
	///
	/// * single byte labels are permitted;
	/// * uppercase is assumed.
	fn case_fold_label(line: &[u8]) -> Self
	{
		let length = line.len();
		
		if length == 0
		{
			panic!("Empty line")
		}
		
		if length > 63
		{
			panic!("Over long label")
		}
		
		if line.starts_with(b"XN--")
		{
			if length == 4
			{
				panic!("Invalid IDN")
			}
		}
		
		const CaseFoldOffset: u8 = 0x20;
		
		let mut case_folded_label = Vec::with_capacity(length);
		
		let byte = line[0];
		let case_folded_byte = match byte
		{
			b'A' ..= b'Z' => byte + CaseFoldOffset,
			b'a' ..= b'z' => panic!("Non-uppercase byte"),
			b'0' ..= b'9' => byte,
			
			invalid @ _ => panic!("First character of DNS label was not letter or digit, but `{:?}`", invalid),
		};
		case_folded_label.push(case_folded_byte);
		
		for byte in &line[1 .. ]
		{
			let byte = *byte;
			let case_folded_byte = match byte
			{
				b'A' ..= b'Z' => byte + CaseFoldOffset,
				b'a' ..= b'z' => panic!("Non-uppercase byte"),
				b'0' ..= b'9' => byte,
				b'-' => byte,
				
				invalid @ _ => panic!("Subsequent character of DNS label was not letter, digit or hyphen, but `{:?}`", invalid),
			};
			case_folded_label.push(case_folded_byte);
		}
		
		Self(String::from_utf8(case_folded_label).unwrap())
	}
}
