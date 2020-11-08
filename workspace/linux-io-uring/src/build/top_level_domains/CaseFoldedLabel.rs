// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.



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
