// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// On entry, `services_field` will have been validated to contain, case-insensitively, at least `E2U+`.
fn e164_naming_authority_pointer_service_field_parse<'message>(services_field: &[u8], replacement_domain_name_or_raw_regular_expression: Either<ParsedName<'message>, ParsedCharacterString<'message>>, mutually_exclusive_flag: Option<NamingAuthorityMutuallyExclusiveFlag>) -> Result<NamingAuthorityPointer, IgnoredServiceFieldReason>
{
	// Remove leading 'E2U+'.
	let servicespecs = &services_field[4 ..];
	
	let mut enum_services = IndexMap::with_capacity(1);
	
	for servicespec in servicespecs.split_bytes(b'+')
	{
		e164_naming_authority_pointer_service_field_parse_servicespec(&mut enum_services, servicespec)?;
	}
}

#[inline(always)]
fn e164_naming_authority_pointer_service_field_parse_add_enum_service(enum_services: &mut IndexSet<EnumService>, enum_service: EnumService) -> Result<(), IgnoredServiceFieldReason>
{
	let added = enum_services.insert(enum_service);
	if unlikely!(!added)
	{
		Err(IgnoredServiceFieldReason::DuplicateEnumService(enum_service))
	}
	else
	{
		Ok(())
	}
}
