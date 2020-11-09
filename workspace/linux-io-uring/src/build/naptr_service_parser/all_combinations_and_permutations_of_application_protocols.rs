// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn all_combinations_and_permutations_of_application_protocols(code: &mut Code, prefix: &'static str, application_protocol_enum_name: &'static str, application_protocols: HashMap<&'static str, &'static str>) -> Vec<(Permutation<&'static str>, String)>
{
	let (all_combinations, all_permutations_per_combination) = AllPermutationsOfASet::from_hash_map(&application_protocols);
	
	let mut permutations_with_combination_index = Vec::with_capacity(1024);
	let mut combination_index = 0;
	for (combination, permutations_per_combination) in all_combinations.into_iter().zip(all_permutations_per_combination.into_iter())
	{
		let (combination_function_name, combination_function_definition) = combination_function_name_and_definition(prefix, application_protocol_enum_name, combination_index, combination);
		code.push_str(&combination_function_definition);
		
		for permutations in permutations_per_combination
		{
			permutations_with_combination_index.push((permutations, combination_function_name))
		}
		
		combination_index += 1;
	}
	
	permutations_with_combination_index
}

fn combination_function_name_and_definition(prefix: &'static str, application_protocol_enum_name: &'static str, combination_index: usize, combination: Vec<(&'static str, &'static str)>) -> (String, String)
{
	let combination_function_name = format!("{}_combination{}()", prefix, combination_index);
	
	let combination_function_definition = format!("\
	#[inline(always)]
	fn {} -> &'static HashSet<{}>
	{{
		use self::{}::*;

		lazy_static!
		{{
    		static ref Combination: HashSet<{}> = {};
    	}}
		
		&Combination
	}}
	
	", &combination_function_name, application_protocol_enum_name, application_protocol_enum_name, application_protocol_enum_name, hash_set_expression_for_combination(application_protocol_enum_name, combination));
	
	(combination_function_name, combination_function_definition)
}

fn hash_set_expression_for_combination(application_protocol_enum_name: &'static str, combination: Vec<(&'static str, &'static str)>) -> String
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
			expression.push_str(application_protocol_enum_member);
		}
		expression.push_str(" };");
		expression
	}
}
