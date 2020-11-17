// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) struct VecEnumServices(Vec<String>);

impl GenerateParseTreeCallback for VecEnumServices
{
	const NoMatchingPattern: &'static str = "Err(NoMatchingEnumServicespec)";
	
	fn push_function_start(code: &mut Code<Self>) -> io::Result<()>
	{
		code.push_line("fn e164_naming_authority_pointer_service_field_parse_servicespec<'message>(enum_services: &mut IndexSet<EnumService>, services_field: &[u8]) -> Result<NamingAuthorityPointer<NamingAuthorityPointer<ParsedName<'message>, ParsedUri<'message>, ParsedCharacterString<'message>, ParsedTypeEquality>>, IgnoredServiceFieldReason>")?;
		code.push_line("{")?;
		code.push_function_line("use self::IgnoredServiceFieldReason::*;")?;
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
		let exact_match_string = match byte_index
		{
			0 =>
			{
				debug_assert!(naive_trie_node_value.is_none());
				
				String::from("Err(EmptyEnumServicespec)")
			},
			
			_ => match naive_trie_node_value
			{
				None => String::from(Self::NoMatchingPattern),
				
				Some(enum_services) =>
				{
					let mut string = String::from("{ ");
					for enum_service in enum_services.0.iter()
					{
						string.push_str(&format!("e164_naming_authority_pointer_service_field_parse_add_enum_service(enum_services, {})?; ", enum_service));
					}
					string.push_str("Ok() }");
					string
				},
			}
		};
		
		("==", exact_match_string)
	}
}
