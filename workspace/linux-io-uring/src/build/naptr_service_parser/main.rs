// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub fn main(_manifest_dir: &OsString, out_dir: &OsString) -> io::Result<()>
{
	let mut naming_authority_pointer_service_field_parse = Code::<StringOrE164Value>::new(out_dir, "naming_authority_pointer_service_field_parse")?;
	naming_authority_pointer_service_field_parse.push_function_start()?;
	{
		let mut all = NaiveTrie::new();
		
		all.add_e164_for_enum();
		
		all.add(application_layer_traffic_optimization());
		all.add(business_document_metadata_service_location());
		all.add(centralized_conferencing());
		all.add(diameter(&mut naming_authority_pointer_service_field_parse)?);
		all.add(internet_registry_information_service(&mut naming_authority_pointer_service_field_parse)?);
		all.add(local_location_information_server());
		all.add(location_to_service_translation_protocol());
		all.add(no_solicit());
		all.add(radius(&mut naming_authority_pointer_service_field_parse)?);
		all.add(session_initiation_protocol());
		all.add(session_initiation_protocol_user_agent_configuration());
		all.add(traversal_using_relays_around_network_address_translation(&mut naming_authority_pointer_service_field_parse)?);
		
		let generate_parse_tree = GenerateParseTree::new(&mut naming_authority_pointer_service_field_parse);
		generate_parse_tree.generate(&all)?;
	}
	naming_authority_pointer_service_field_parse.push_function_end()?;
	
	let mut e164_naming_authority_pointer_service_field_parse_servicespec = Code::<VecEnumServices>::new(out_dir, "e164_naming_authority_pointer_service_field_parse_servicespec")?;
	e164_naming_authority_pointer_service_field_parse_servicespec.push_function_start()?;
	{
		let all_enum = enum_::enum_();
		
		let generate_parse_tree = GenerateParseTree::new(&mut e164_naming_authority_pointer_service_field_parse_servicespec);
		generate_parse_tree.generate(&all_enum)?;
	}
	e164_naming_authority_pointer_service_field_parse_servicespec.push_function_end()?;
	
	
	Ok(())
}
