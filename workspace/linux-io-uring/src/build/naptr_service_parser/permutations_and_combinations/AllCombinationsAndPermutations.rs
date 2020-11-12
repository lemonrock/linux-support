// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) struct AllCombinationsAndPermutations<'a>
{
	code: &'a mut Code,
	
	prefix: &'a str,
	
	application_protocol_enum_name: &'static str
}

impl<'a> Deref for AllCombinationsAndPermutations<'a>
{
	type Target = Code;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.code
	}
}

impl<'a> DerefMut for AllCombinationsAndPermutations<'a>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		self.code
	}
}

impl<'a> AllCombinationsAndPermutations<'a>
{
	pub(super) fn process<'b, Element, V: AsRef<str>>(code: &'a mut Code, prefix: &'a str, application_protocol_enum_name: &'static str, application_protocols: &'b IndexMap<Element, V>) -> io::Result<Vec<(Permutation<'b, Element, V>, Rc<HashOrIndexSetStaticName>)>>
	{
		Self::new(code, prefix, application_protocol_enum_name).all_combinations_and_permutations_of_application_protocols(application_protocols)
	}
	
	fn new(code: &'a mut Code, prefix: &'a str, application_protocol_enum_name: &'static str) -> Self
	{
		Self
		{
			code,
			prefix,
			application_protocol_enum_name
		}
	}
	
	fn all_combinations_and_permutations_of_application_protocols<'b, Element, V: AsRef<str>>(mut self, application_protocols: &'b IndexMap<Element, V>) -> io::Result<Vec<(Permutation<'b, Element, V>, Rc<HashOrIndexSetStaticName>)>>
	{
		let all_possible_combinations_and_permutations_for_combination_for_length = AllPermutationsOfASet::from_index_map(application_protocols, true);
		
		let mut permutations_with_same_combination = Vec::with_capacity(1024);
		for (combination, permutations_per_combination) in all_possible_combinations_and_permutations_for_combination_for_length
		{
			let (hash_set_static_name, hash_set_static_definition) = self.transport_protocols_hash_set_static_name_and_definition(combination);
			self.push_str(&hash_set_static_definition)?;
			
			for permutations in permutations_per_combination
			{
				permutations_with_same_combination.push((permutations, hash_set_static_name.clone()))
			}
		}
		
		Ok(permutations_with_same_combination)
	}
	
	fn transport_protocols_hash_set_static_name_and_definition<'b, Element, V: AsRef<str>>(&self, combination: Combination<'b, Element, V>) -> (Rc<HashOrIndexSetStaticName>, String)
	{
		let (hash_set_static_name, hash_set_expression) = self.hash_set_static_name_and_expression_for_combination(combination);
		let lazy_static_hash_set_expression = format!("lazy_static! {{ static ref {0}: HashSet<{1}> = {2}; }}", &hash_set_static_name, self.application_protocol_enum_name, hash_set_expression);
		
		(hash_set_static_name, lazy_static_hash_set_expression)
	}
	
	// TODO: Could this be a PHF hash set?
	fn hash_set_static_name_and_expression_for_combination<'b, Element, V: AsRef<str>>(&self, combination: Combination<'b, Element, V>) -> (Rc<HashOrIndexSetStaticName>, String)
	{
		let mut hash_set_static_name = format!("{}_combination", self.prefix);
		if combination.is_empty()
		{
			(Rc::new(hash_set_static_name), String::from("HashSet::new()"))
		}
		else
		{
			let mut hash_set_expression = String::from("hashset! { ");
			let mut after_first = false;
			for (_, application_protocol_enum_member) in combination
			{
				let application_protocol_enum_member = application_protocol_enum_member.as_ref();
				
				hash_set_static_name.push('_');
				hash_set_static_name.push_str(application_protocol_enum_member);
				
				if after_first
				{
					hash_set_expression.push_str(", ");
				}
				else
				{
					after_first = true;
				}
				hash_set_expression.push_str(self.application_protocol_enum_name);
				hash_set_expression.push_str("::");
				hash_set_expression.push_str(application_protocol_enum_member);
			}
			hash_set_expression.push_str(" };");
			(Rc::new(hash_set_static_name), hash_set_expression)
		}
	}
}
