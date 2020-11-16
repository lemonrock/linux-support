// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub fn main(_manifest_dir: &OsString, out_dir: &OsString) -> io::Result<()>
{
	let mut code = Code
	{
		writer: new_buf_writer(out_dir, "naming_authority_pointer_service_field_parse.naptr_service_parser.rs")?,
		stack_depth: 0,
	};
	
	code.push_function_start()?;
	
	let mut all = NaiveTrie::new();
	all.add(application_layer_traffic_optimization());
	all.add(business_document_metadata_service_location());
	all.add(centralized_conferencing());
	all.add(diameter(&mut code)?);
	all.add(internet_registry_information_service(&mut code)?);
	all.add(local_location_information_server());
	all.add(location_to_service_translation_protocol());
	all.add(no_solicit());
	all.add(radius(&mut code)?);
	all.add(session_initiation_protocol());
	all.add(session_initiation_protocol_user_agent_configuration());
	all.add(traversal_using_relays_around_network_address_translation(&mut code)?);
	all.add(enum_(&mut code)?);
	
	let generate_parse_tree = GenerateParseTree::new(&mut code);
	generate_parse_tree.generate(&all)?;
	
	code.push_function_end()?;
	
	Ok(())
}
