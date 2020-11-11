// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn internet_registry_information_service(code: &mut Code) -> io::Result<HashMap<String, String>>
{
	Ok
	(
		combine_multiple_application_services_with_protocols
		(
			HashMap::default(),
			
			"InternetRegistryInformationService",
			
			"registry_type",
			
			"InternetRegistryInformationServiceRegistryType",
			
			hashmap!
			{
				"AREG1" => "Address",
				
				"DCHK1" => "DomainAvailabilityCheck",
				
				"DREG1" => "DomainRegistry",
				
				"EREG1" => "EnumRegistry",
			},
			
			AllCombinationsAndPermutationsOfApplicationProtocols::process
			(
				code,
				
				"internet_registry_information_service",
				
				"InternetRegistryInformationServiceTransportProtocol",
				
				hashmap!
				{
					// Defined by RFC 3983.
					"iris.beep" => "iris_beep",
					
					// Defined by RFC 4993.
					"iris.lwz" => "iris_lwz",
					
					// Defined by RFC 4992.
					"iris.xpc" => "iris_xpc",
					
					// Defined by RFC 4992.
					"iris.xpcs" => "iris_xpcs",
				}
			)?
		)
	)
}
