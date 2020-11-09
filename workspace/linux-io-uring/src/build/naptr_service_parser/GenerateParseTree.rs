// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct GenerateParseTree<'a>
{
	code: &'a mut String,
	
	stack_depth: usize,
}

impl<'a> GenerateParseTree<'a>
{
	fn new(code: &'a mut Code) -> Self
	{
		Self
		{
			code,
			
			stack_depth: 0,
		}
	}

	fn generate(mut self, trie: &NaiveTrie<String>)
	{
		self.push_line("fn parse(services_field: &[u8]) -> Result<Either<IgnoreServiceFieldReason>, ServiceFieldParseError>");
		self.push_line("{");
		self.push_function_line("use self::IgnoreServiceFieldReason::*;");
		self.push_function_line("use self::ServiceField::*;");
		self.push_function_line("use self::ServiceFieldParseError::*;");
		self.push_function_line("");
		self.push_function_line("let length = services_field.len();");
		self.push_function_line("");
		self.generate_recursive(&trie);
		self.push_line("}");
	}

	fn generate_recursive(&mut self, naive_trie_node: &NaiveTrieNode<String>)
	{
		let byte_index = self.stack_depth;

		const NoMatchingPattern: &str = "Ok(Right(NoMatchingPattern))";
		
		let exact_match_string = if byte_index == 0
		{
			String::from("Ok(Left(None))")
		}
		else
		{
			match naive_trie_node.value()
			{
				None => String::from(NoMatchingPattern),
				Some(value) => format!("Ok(Left(Some({})))", value.as_str()),
			}
		};
		
		self.push_str(&format!("match (if length == {} {{ return {} }} else {{ unsafe {{ *services_field.get_unchecked({}) }} }})", byte_index, exact_match_string, byte_index));
		self.push_new_line();
		self.push_tab_indented_line("{");
		
		for (case_folded_ascii_byte, child_naive_trie_node) in naive_trie_node.iter()
		{
			self.push_match_pattern(case_folded_ascii_byte);
			
			self.stack_depth += 1;
			self.generate_recursive(child_naive_trie_node);
			self.stack_depth -= 1;
		}
		
		let always_invalid_bytes = "byte @ 0x00 ..= b'*' | byte @ b',' | byte @ b'/' | byte @ b';' ..= b'<' | byte @ b'>' ..= b'@' | byte @ b'[' ..= b'`' | byte @ b'{' ..= 0xFF";
		self.push_tab_indented_line(&format!("\t{} => Err(ServiceFieldParseError::OutOfRange(byte, {})),", always_invalid_bytes, byte_index));
		self.push_tab_indented_line("");
		self.push_tab_indented_line(&format!("\t_ => {},", NoMatchingPattern));
		
		if self.stack_depth == 0
		{
			self.push_tab_indented_line("}");
		}
		else
		{
			self.push_tab_indented_line("},");
			self.push_tab_indented_line("");
		}
	}
	
	fn push_match_pattern(&mut self, case_folded_ascii_byte: u8)
	{
		self.push_tabs();
		self.push_tab();
		self.push_str("b'");
		self.push_byte_as_char(case_folded_ascii_byte);
		if matches!(case_folded_ascii_byte, b'a' ..= b'z')
		{
			self.push_str("' | b'");
			self.push_byte_as_char(case_folded_ascii_byte - 0x20);
		}
		self.push_str("'");
		self.push_str(" => ");
	}
	
	fn push_function_line(&mut self, value: &str)
	{
		self.push_tab();
		self.push_line(value)
	}
	
	fn push_line(&mut self, value: &str)
	{
		self.push_str(value);
		self.push_new_line()
	}
	
	fn push_tab_indented_line(&mut self, value: &str)
	{
		self.push_tab_indented(value);
		self.push_new_line()
	}
	
	fn push_tab_indented(&mut self, value: &str)
	{
		self.push_tabs();
		self.push_str(value);
	}
	
	fn push_new_line(&mut self)
	{
		self.push_char('\n');
	}
	
	fn push_tabs(&mut self)
	{
		self.push_tab();
		for _ in 0 .. self.stack_depth
		{
			self.push_tab();
		}
	}
	
	fn push_tab(&mut self)
	{
		self.push_char('\t');
	}
	
	fn push_byte_as_char(&mut self, byte: u8)
	{
		self.push_char(char::from(byte))
	}
	
	fn push_char(&mut self, value: char)
	{
		self.code.push(value)
	}
	
	fn push_str(&mut self, value: &str)
	{
		self.code.push_str(value)
	}
}
