// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


type EnumServiceSubTypeType = &'static str;

type NaptrSubType = &'static str;

type EnumServiceSubTypeMember = &'static str;

type SubTypeSpec = String;

fn enum_(code: &mut Code) -> io::Result<HashMap<String, String>>
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
				
				let servicespec = format!("+{}{}", naptr_type, sub_type_spec);
				type_permutations_input.insert(servicespec, ordered_enum_services);
			}
		}
		else
		{
			let servicespec = format!("+{}", naptr_type);
			let ordered_enum_services = vec![format!("EnumService::{}", enum_service_member)];
			type_permutations_input.insert(servicespec, ordered_enum_services);
		}
	}
	
	let all_possible_combinations_and_permutations_for_combination_for_length = AllPermutationsOfASet::from_index_map(&type_permutations_input, false);
	
	let mut result = HashMap::new();
	let mut permutation_index = 0;
	for (_combination, mut permutations_per_combination) in all_possible_combinations_and_permutations_for_combination_for_length
	{
		for mut permutation in permutations_per_combination.drain(..)
		{
			let mut service_field = String::from("E2U");
			let mut enum_services = Vec::with_capacity(16);
			for (servicespec, ordered_enum_services) in permutation.drain(..)
			{
				service_field.push_str(&servicespec);
				enum_services.extend_from_slice(&ordered_enum_services[..]);
			}
			let index_set_name = index_set_definition(code, permutation_index, enum_services)?;
			permutation_index += 1;
			
			let old = result.insert(service_field, format!("Enum {{ enum_services: &{} }}", index_set_name));
			debug_assert!(old.is_none());
		}
	}
	
	Ok(result)
}

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

fn subtype_spec<'a>(permutation: &Permutation<'a, NaptrSubType, EnumServiceSubTypeMember>) -> SubTypeSpec
{
	let mut subtype_spec = String::with_capacity(MaximumServiceFieldSize);
	for &(naptr_sub_type, _) in permutation
	{
		subtype_spec.push(':');
		subtype_spec.push_str(*naptr_sub_type);
	}
	subtype_spec
}

fn index_set_definition(code: &mut Code, permutation_index: usize, enum_services: Vec<String>) -> io::Result<HashOrIndexSetStaticName>
{
	let index_set_name = format!("enum_{}", permutation_index);
	
	code.push_tab_indented_line(&format!("lazy_static! {{ static ref {}: IndexSet<EnumService> = indexset! {{ ", &index_set_name))?;
	code.push_tab_indented_line("")?;
	
	let mut after_first = false;
	for enum_service in enum_services
	{
		if after_first
		{
			code.push_str(", ")?;
		}
		else
		{
			after_first = true;
		}
		
		code.push_str(&enum_service)?;
	}
	
	code.push_tab_indented_line("} };")?;
	code.push_new_line()?;
	
	Ok(index_set_name)
}

fn sub_types() -> HashMap<EnumServiceSubTypeType, IndexMap<NaptrSubType, EnumServiceSubTypeMember>>
{
	hashmap!
	{
		"EmailEnumServiceSubType" => indexmap!
		{
			"mailto" => "mailto"
		},
	
		"EmsEnumServiceSubType" => indexmap!
		{
			"mailto" => "mailto",
			"tel" => "tel",
		},
	}
}

fn enumservices() -> HashMap<&'static str, (&'static str, Option<EnumServiceSubTypeType>)>
{
	hashmap!
	{
		"acct" =>
		(
			"acct",
			None,
		),
		
		"email" =>
		(
			"email",
			Some("EmailEnumServiceSubType"),
		),
		
		"ems" =>
		(
			"ems",
			Some("EmsEnumServiceSubType"),
		),
	}
}
