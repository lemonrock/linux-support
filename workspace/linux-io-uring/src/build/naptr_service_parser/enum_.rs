// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn enum_(code: &mut Code) -> io::Result<HashMap<String, String>>
{
	for (enumservice_name, (enumservice_enum_member, subtype_enum,, subtypes)) in enumservices_and_subtypes()
	{
	
	}
	
	Ok(())
}

fn enumservices_and_subtypes() -> HashMap<&'static str, (&'static str, HashMap<&'static str, &'static str>)>
{
	/// Allowed zero or more subtypes - it's a perms of perms!
	
	hashmap!
	{
		"acct" =>
		(
			"EnumService::acct",
			"",
			hashset!
			{
			}
		),
		
		"email" =>
		(
			"EnumService::email",
			"EmailEnumServiceSubType",
			hashmap!
			{
				"mailto" => "EmailEnumServiceSubType::mailto"
			}
		),
		
		"ems" =>
		(
			"EnumService::ems",
			"EmsEnumServiceSubType",
			hashmap!
			{
				"mailto" => "EmsEnumServiceSubType::mailto",
				"tel" => "EmsEnumServiceSubType::tel",
			}
		),
	}
}

pub(super) type EnumserviceSubtypesHashSetStaticName = String;

fn enumservices_subtypes_permutations(code: &mut Code) -> io::Result<Vec<(Permutation<&'static str>, EnumserviceSubtypesHashSetStaticName)>>
{
	AllCombinationsAndPermutationsOfApplicationProtocols::new(code, "internet_registry_information_service", "InternetRegistryInformationServiceTransportProtocol").all_combinations_and_permutations_of_application_protocols(internet_registry_information_service_application_protocols())
}
