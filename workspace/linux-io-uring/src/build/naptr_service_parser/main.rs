// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub fn main(manifest_dir: &OsString, out_dir: &OsString) -> io::Result<()>
{
	let mut code = Code
	{
		writer: new_buf_writer(out_dir, "naptr_service_field_parse.naptr_service_parser.rs")?
	};
	
	let mut all = NaiveTrie::new();
	all.add(application_layer_traffic_optimization());
	all.add(centralized_conferencing());
	all.add(diameter(&mut code)?);
	all.add(enum_(&mut code)?);
	all.add(internet_registry_information_service(&mut code)?);
	all.add(local_location_information_server());
	all.add(location_to_service_translation_protocol()?);
	all.add(radius(&mut code)?);
	all.add(session_initiation_protocol());
	all.add(session_initiation_protocol_user_agent_configuration());
	all.add(traversal_using_relays_around_network_address_translation(&mut code)?);
	
	let generate_parse_tree = GenerateParseTree::new(&mut code);
	generate_parse_tree.generate(&all)?;
	
	
	// TODO: https://tools.ietf.org/html/rfc2169 THTTP
	// TODO: E2U enum services.
	
	
	
	Ok(())
}
