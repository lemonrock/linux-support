// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) fn enum_() -> NaiveTrie<VecEnumServices>
{
	let sub_types = sub_types();
	let sub_type_permutations = sub_type_permutations(&sub_types);
	
	let mut type_permutations_input = IndexMap::new();
	for (naptr_type, (enum_service_member, enum_service_sub_type_type_option)) in enumservices()
	{
		if let Some(enum_service_sub_type_type) = enum_service_sub_type_type_option
		{
			let permutations = sub_type_permutations.get(enum_service_sub_type_type).unwrap();
			for (ref sub_type_spec, ref permutation) in permutations
			{
				let mut ordered_enum_services = Vec::with_capacity(4);
				for &(_naptr_sub_type, enum_service_sub_type_member) in permutation
				{
					ordered_enum_services.push(format!("EnumService::{}({}::{})", enum_service_member, enum_service_sub_type_type, *enum_service_sub_type_member));
				}
				
				let servicespec = format!("{}{}", naptr_type, sub_type_spec);
				type_permutations_input.insert(servicespec, VecEnumServices(ordered_enum_services));
			}
		}
		else
		{
			let servicespec = format!("{}", naptr_type);
			let ordered_enum_services = vec![format!("EnumService::{}", enum_service_member)];
			type_permutations_input.insert(servicespec, VecEnumServices(ordered_enum_services));
		}
	}
	
	let mut all_servicespec = NaiveTrie::new();
	all_servicespec.add_e164_servicespec(type_permutations_input);
	all_servicespec
	
	// // Here: There are just too many permuations.
	// let all_possible_combinations_and_permutations_for_combination_for_length = AllPermutationsOfASet::from_index_map(&type_permutations_input, false);
	//
	// let mut result = HashMap::new();
	// let mut permutation_index = 0;
	// for (_combination, mut permutations_per_combination) in all_possible_combinations_and_permutations_for_combination_for_length
	// {
	// 	for mut permutation in permutations_per_combination.drain(..)
	// 	{
	// 		let mut service_field = String::from("E2U");
	// 		let mut enum_services = Vec::with_capacity(16);
	// 		for (servicespec, ordered_enum_services) in permutation.drain(..)
	// 		{
	// 			service_field.push_str(&servicespec);
	// 			enum_services.extend_from_slice(&ordered_enum_services[..]);
	// 		}
	// 		let index_set_name = index_set_definition(code, permutation_index, enum_services)?;
	// 		permutation_index += 1;
	//
	// 		// service_field_kind: ServiceFieldKind,
	// 		let old = result.insert(service_field, format!("Enum {{ enum_services: &{}, domain_name_or_regular_expression: RegularExpressionResolvingToUriOrQueryUriResourceRecord::parse(ServiceFieldKind::Enum, replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag)? }}", index_set_name));
	// 		debug_assert!(old.is_none());
	// 	}
	// }
	//
	// Ok(result)
}
