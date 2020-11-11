// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


type EnumServiceSubTypeType = &'static str;

type NaptrSubType = &'static str;

type EnumServiceSubTypeMember = &'static str;

type SubTypeSpec = String;

fn enum_(code: &mut Code) -> io::Result<HashMap<String, String>>
{
	let sub_type_permutations = sub_type_permutations(code)?;
	
	let mut type_permutations_input = HashMap::new();
	for (naptr_type, (enum_service_member, enum_service_sub_type_type_option)) in enumservices()
	{
		if let Some(enum_service_sub_type_type) = enum_service_sub_type_type_option
		{
			let permutations = sub_type_permutations.get(enum_service_sub_type_type).unwrap();
			for (permutation_index, &(ref naptr_sub_type_to_enum_service_sub_type_member, ref sub_type_spec, ref permutation)) in permutations.into_iter().enumerate()
			{
				let mut ordered_enum_services = Vec::with_capacity(4);
				let mut after_first = false;
				for naptr_sub_type in permutation
				{
					if after_first
					{
						ordered_enum_services.push_str(", ");
					}
					else
					{
						after_first = true;
					}
					
					let enum_service_sub_type_member = naptr_sub_type_to_enum_service_sub_type_member.get(naptr_sub_type).unwrap();
					ordered_enum_services.push(format!("EnumService::{}({}::{})", enum_service_member, enum_service_sub_type_type, enum_service_sub_type_member))?;
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
	
	let (_, all_permutations) = AllPermutationsOfASet::from_hash_map(&type_permutations_input, false);
	
	let mut result = HashMap::new();
	let mut permutation_index = 0;
	for some_permutations in all_permutations
	{
		for permutation in some_permutations
		{
			let mut service_field = String::from("E2U");
			let mut enum_services = Vec::with_capacity(16);
			for servicespec in permutation
			{
				service_field.push_str(&servicespec);
				let mut ordered_enum_services = type_permutations_input.remove(&servicespec).unwrap();
				enum_services.append(&mut ordered_enum_services);
			}
			let index_set_name = index_set_definition(code, permutation_index, enum_services)?;
			permutation_index += 1;
			
			result.insert(service_field, format!("Enum {{ enum_services: &{} }}", index_set_name))
		}
	}
	
	Ok(result)
}

fn sub_type_permutations(code: &mut Code) -> io::Result<HashMap<EnumServiceSubTypeType, Vec<(HashMap<NaptrSubType, EnumServiceSubTypeMember>, SubTypeSpec, Permutation<&'static str>)>>>
{
	let sub_types = sub_types();
	let mut sub_type_permutations = HashMap::with_capacity(sub_types.len());
	
	for (enum_service_sub_type_type, naptr_sub_type_to_enum_service_sub_type_member) in sub_types
	{
		let (_, all_permutations) = AllPermutationsOfASet::from_hash_map(&naptr_sub_type_to_enum_service_sub_type_member, false);
		
		let mut permutations = Vec::with_capacity(128);
		for some_permutations in all_permutations
		{
			for permutation in some_permutations
			{
				let subtype_spec = subtype_spec(enum_service_sub_type_type, &permutation)?;
				permutations.push((naptr_sub_type_to_enum_service_sub_type_member, subtype_spec, permutation));
			}
		}
		
		sub_type_permutations.insert(enum_service_sub_type_type, permutations)
	}
	
	Ok(sub_type_permutations)
}

fn subtype_spec(enum_service_sub_type_type: EnumServiceSubTypeType, permutation: &Permutation<NaptrSubType>) -> SubTypeSpec
{
	let mut subtype_spec = String::with_capacity(MaximumServiceFieldSize);
	for naptr_sub_type in permutation
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
	for enum_services in enum_services
	{
		if after_first
		{
			code.push_str(", ");
		}
		else
		{
			after_first = true;
		}
		
		code.push_str(enum_service)?;
	}
	
	code.push_tab_indented_line("} };")?;
	code.push_new_line()?;
	
	Ok(index_set_name)
}

fn sub_types() -> HashMap<EnumServiceSubTypeType, HashMap<NaptrSubType, EnumServiceSubTypeMember>>
{
	hashmap!
	{
		"EmailEnumServiceSubType" => hashmap!
		{
			"mailto" => "mailto"
		},
	
		"EmsEnumServiceSubType" => hashmap!
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
