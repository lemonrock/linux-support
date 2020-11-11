// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn radius(code: &mut Code) -> io::Result<HashMap<String, String>>
{
	Ok
	(
		combine_multiple_application_services_with_protocols
		(
			HashMap::default(),
			
			"Radius",
			
			"traffic_identifier",
			
			"RadiusTrafficIdentifier",
			
			hashmap!
			{
				"aaa+auth" => "Authentication",
				
				"aaa+acct" => "Accounting",
				
				"aaa+dynauth" => "DynamicAuthorization",
			},
			
			AllCombinationsAndPermutations::process
			(
				code,
				
				"radius",
				
				"RadiusTransportProtocol",
				
				hashmap!
				{
					// RFC 7585.
					"radius.dtls.udp" => "radius_dtls_udp",
					
					// RFC 7585.
					"radius.tls.tcp" => "radius_tls_tcp",
				}
			)?
		)
	)
}
