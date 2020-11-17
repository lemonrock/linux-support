// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn sub_type_permutations<'a>(sub_types: &'a HashMap<EnumServiceSubTypeType, IndexMap<NaptrSubType, EnumServiceSubTypeMember>>) -> HashMap<EnumServiceSubTypeType, Vec<(SubTypeSpec, Permutation<'a, &'static str, &'static str>)>>
{
	let mut sub_type_permutations = HashMap::with_capacity(sub_types.len());
	
	for (enum_service_sub_type_type, naptr_sub_type_to_enum_service_sub_type_member) in sub_types
	{
		let all_possible_combinations_and_permutations_for_combination_for_length = AllPermutationsOfASet::from_index_map(&naptr_sub_type_to_enum_service_sub_type_member, false);
		
		let mut permutations = Vec::with_capacity(128);
		for (_combination, permutations_per_combination) in all_possible_combinations_and_permutations_for_combination_for_length
		{
			for permutation in permutations_per_combination
			{
				let subtype_spec = subtype_spec(&permutation);
				permutations.push((subtype_spec, permutation));
			}
		}
		
		let old = sub_type_permutations.insert(*enum_service_sub_type_type, permutations);
		debug_assert!(old.is_none());
	}
	
	sub_type_permutations
}
