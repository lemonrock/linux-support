// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn modern_diameter(code: &mut Code) -> HashMap<String, String>
{
	let mut result = HashMap::with_capacity(1024);
	
	let application_protocol_permutations = all_combinations_and_permutations_of_modern_diameter_application_protocols(code);
	
	for (application_service, application_identifier) in modern_diameter_application_services()
	{
		for (application_protocol_permutation, combination_function_name) in application_protocol_permutations.iter()
		{
			let key = format!("{}{}", application_service, application_protocol_permutation_to_string(application_protocol_permutation));
			
			if key.len() > MaximumServiceFieldSize
			{
				continue
			}
			
			let value = format!("ModernDiameter {{ application_identifier: {}, transport_protocols: {} }}", application_identifier, combination_function_name);
			result.insert(key, value)
		}
	}
	
	result
}
