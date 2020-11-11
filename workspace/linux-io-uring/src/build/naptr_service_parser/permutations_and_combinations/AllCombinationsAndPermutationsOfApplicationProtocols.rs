// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) struct AllCombinationsAndPermutationsOfApplicationProtocols<'a>
{
	code: &'a mut Code,
	
	prefix: &'static str,
	
	application_protocol_enum_name: &'static str
}

impl<'a> Deref for AllCombinationsAndPermutationsOfApplicationProtocols<'a>
{
	type Target = Code;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.code
	}
}

impl<'a> DerefMut for AllCombinationsAndPermutationsOfApplicationProtocols<'a>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		self.code
	}
}

impl<'a> AllCombinationsAndPermutationsOfApplicationProtocols<'a>
{
	pub(super) fn process(code: &'a mut Code, prefix: &'static str, application_protocol_enum_name: &'static str, application_protocols: HashMap<&'static str, &'static str>) -> io::Result<Vec<(Permutation<&'static str>, HashSetStaticName)>>
	{
		Self::new(code, prefix, application_protocol_enum_name).all_combinations_and_permutations_of_application_protocols(application_protocols)
	}
	
	fn new(code: &'a mut Code, prefix: &'static str, application_protocol_enum_name: &'static str) -> Self
	{
		Self
		{
			code,
			prefix,
			application_protocol_enum_name
		}
	}
	
	fn all_combinations_and_permutations_of_application_protocols(mut self, application_protocols: HashMap<&'static str, &'static str>) -> io::Result<Vec<(Permutation<&'static str>, HashSetStaticName)>>
	{
		let (all_combinations, all_permutations_per_combination) = AllPermutationsOfASet::from_hash_map(&application_protocols);
		
		let mut permutations_with_combination_index = Vec::with_capacity(1024);
		let mut combination_index = 0;
		for (combination, permutations_per_combination) in all_combinations.into_iter().zip(all_permutations_per_combination.into_iter())
		{
			let (hash_set_static_name, hash_set_static_definition) = self.transport_protocols_hash_set_static_name_and_definition(combination_index, combination);
			self.push_str(&hash_set_static_definition)?;
			
			for permutations in permutations_per_combination
			{
				permutations_with_combination_index.push((permutations, hash_set_static_name))
			}
			
			combination_index += 1;
		}
		
		Ok(permutations_with_combination_index)
	}
	
	fn transport_protocols_hash_set_static_name_and_definition(&self, combination_index: usize, combination: Combination<&'static str, &'static str>) -> (HashSetStaticName, String)
	{
		let transport_protocols_hash_set_static_name = format!("{}_combination_{}", self.prefix, combination_index);
		
		let combination_function_definition = format!("lazy_static! {{ static ref {0}: HashSet<{1}> = {2}; }}", &transport_protocols_hash_set_static_name, self.application_protocol_enum_name, self.hash_set_expression_for_combination(combination));
		
		(transport_protocols_hash_set_static_name, combination_function_definition)
	}
	
	// TODO: Could this be a PHF hash set?
	fn hash_set_expression_for_combination(&self, combination: Combination<&'static str, &'static str>) -> String
	{
		if combination.is_empty()
		{
			String::from("HashSet::new()")
		}
		else
		{
			let mut expression = String::from("hashset! { ");
			let mut after_first = false;
			for (_, application_protocol_enum_member) in combination
			{
				if after_first
				{
					expression.push_str(", ");
				}
				else
				{
					after_first = true;
				}
				expression.push_str(self.application_protocol_enum_name);
				expression.push_str("::");
				expression.push_str(application_protocol_enum_member);
			}
			expression.push_str(" };");
			expression
		}
	}
}
