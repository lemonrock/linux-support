// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


struct GenerateParseTree<'a>
{
	code: &'a mut Code,
}

impl<'a> Deref for GenerateParseTree<'a>
{
	type Target = Code;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.code
	}
}

impl<'a> DerefMut for GenerateParseTree<'a>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		self.code
	}
}

impl<'a> GenerateParseTree<'a>
{
	fn new(code: &'a mut Code) -> Self
	{
		Self
		{
			code,
		}
	}

	fn generate(mut self, trie: &NaiveTrie<String>) -> io::Result<()>
	{
		self.generate_recursive(&trie)
	}

	fn generate_recursive(&mut self, naive_trie_node: &NaiveTrieNode<String>) -> io::Result<()>
	{
		let byte_index = self.stack_depth();

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
		
		self.push_str(&format!("match (if length == {} {{ return {} }} else {{ unsafe {{ *services_field.get_unchecked({}) }} }})", byte_index, exact_match_string, byte_index))?;
		self.push_new_line()?;
		self.push_tab_indented_line("{")?;
		
		for (case_folded_ascii_byte, child_naive_trie_node) in naive_trie_node.iter()
		{
			self.push_match_pattern(case_folded_ascii_byte)?;
			
			self.increment_stack_depth();
			self.generate_recursive(child_naive_trie_node)?;
			self.decrement_stack_depth();
		}
		
		let always_invalid_bytes = "byte @ 0x00 ..= b'*' | byte @ b',' | byte @ b'/' | byte @ b';' ..= b'<' | byte @ b'>' ..= b'@' | byte @ b'[' ..= b'`' | byte @ b'{' ..= 0xFF";
		self.push_tab_indented_line(&format!("\t{} => Err(OutOfRange(byte, {})),", always_invalid_bytes, byte_index))?;
		self.push_tab_indented_line("")?;
		self.push_tab_indented_line(&format!("\t_ => {},", NoMatchingPattern))?;
		
		if self.stack_depth() == 0
		{
			self.push_tab_indented_line("}")?;
		}
		else
		{
			self.push_tab_indented_line("},")?;
			self.push_tab_indented_line("")?;
		}
		
		Ok(())
	}
	
	fn push_match_pattern(&mut self, case_folded_ascii_byte: u8) -> io::Result<()>
	{
		self.push_tabs()?;
		self.push_tab()?;
		self.push_str("b'")?;
		self.push_byte(case_folded_ascii_byte)?;
		if matches!(case_folded_ascii_byte, b'a' ..= b'z')
		{
			self.push_str("' | b'")?;
			self.push_byte(case_folded_ascii_byte - 0x20)?;
		}
		self.push_str("'")?;
		self.push_str(" => ")
	}
}
