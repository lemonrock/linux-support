// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn combine_solitary_application_service_with_protocols<'a>(service_field_enum_member_name: &'static str, application_service: ApplicationServiceTag, application_protocol_permutations: Vec<(Permutation<'a, &'static str, &'static str>, Rc<HashOrIndexSetStaticName>)>) -> HashMap<String, String>
{
	let mut result = HashMap::with_capacity(1024);
	
	for (application_protocol_permutation, transport_protocols_hash_set_static_name) in application_protocol_permutations
	{
		let key = format!("{}{}", application_service, protocol_permutation_to_delimited_string(':', &application_protocol_permutation));
		
		if key.len() > MaximumServiceFieldSize
		{
			continue
		}
		
		let value = format!("{0} {{ transport_protocols: &{1}, query_for_next: QueryForNext::parse(ServiceFieldKind::{0}, replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag)? }}", service_field_enum_member_name, transport_protocols_hash_set_static_name);
		let old = result.insert(key, value);
		debug_assert!(old.is_none());
	}
	
	result
}
