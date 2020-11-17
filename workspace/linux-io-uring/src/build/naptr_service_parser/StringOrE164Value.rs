// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// E2U (ENUM) has 2^384 matches and so can not be matched as part of a generic parse tree byte-by-byte.
///
/// This does mean that if there are overlapping E2U matches in U-NAPTR or S-NAPTR, these will fail.
/// There are none currently.
enum StringOrE164Value
{
	StringValue(String),

	E164Value,
}

impl GenerateParseTreeCallback for StringOrE164Value
{
	const NoMatchingPattern: &'static str = "Err(NoMatchingPattern)";
	
	fn push_function_start(code: &mut Code<Self>) -> io::Result<()>
	{
		code.push_line("fn naptr_service_field_parse<'message>(services_field: &[u8], replacement_domain_name_or_raw_regular_expression: Either<ParsedName<'message>, ParsedCharacterString<'message>>, mutually_exclusive_flag: Option<NamingAuthorityMutuallyExclusiveFlag>) -> Result<NamingAuthorityPointer, IgnoredServiceFieldReason>")?;
		code.push_line("{")?;
		code.push_function_line("use self::HypertextTransportProtocol::*;")?;
		code.push_function_line("use self::IgnoredServiceFieldReason::*;")?;
		code.push_function_line("use self::NamingAuthorityPointer::*;")?;
		code.push_function_line("")?;
		code.push_function_line("let length = services_field.len();")?;
		code.push_function_line("")?;
		
		Ok(())
	}
	
	fn push_function_end(mut code: Code<Self>) -> io::Result<()>
	{
		code.push_line("}")
	}
	
	fn length_comparison_and_exact_match_string(byte_index: usize, naive_trie_node_value: Option<&Self>) -> (&str, String)
	{
		use self::StringOrE164Value::*;
		
		match byte_index
		{
			0 =>
			{
				debug_assert!(naive_trie_node_value.is_none());
				
				("==", String::from("RegularExpressionResolvingToDomainNameOrQueryNaptrResourceRecord::parse(replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag)"))
			},
			
			_ => match naive_trie_node_value
			{
				None => ("==", String::from(Self::NoMatchingPattern)),
				
				Some(E164Value) => (">=", String::from("e164_naming_authority_pointer_service_field_parse(services_field, replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag)")),
				
				Some(StringValue(value)) => ("==", format!("Ok({})", value.as_str())),
			}
		}
	}
}
