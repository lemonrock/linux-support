// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn traversal_using_relays_around_network_address_translation(code: &mut Code) -> io::Result<HashMap<String, String>>
{
	Ok
	(
		combine_solitary_application_service_with_protocols
		(
			"TraversalUsingRelaysAroundNetworkAddressTranslation",
			
			"RELAY",
			
			AllCombinationsAndPermutationsOfApplicationProtocols::process
			(
				code,
				
				"traversal_using_relays_around_network_address_translation",
				
				"TraversalUsingRelaysAroundNetworkAddressTranslationTransportProtocol",
				
				hashmap!
				{
					"turn.dtls" => "turn_dtls",
					
					"turn.tcp" => "turn_tcp",
					
					"turn.tls" => "turn_tls",
					
					"turn.udp" => "turn_udp",
				}
			)?
		)
	)
}
