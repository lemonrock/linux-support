// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// First defined in RFC 6408.
// Subset of <https://www.iana.org/assignments/s-naptr-parameters/s-naptr-parameters.xhtml#s-naptr-parameters-2>.
fn modern_diameter_application_protocols() -> HashMap<&'static str, &'static str>
{
	hashmap!
	{
		// RFC 6733.
		"diameter.dtls.sctp" => "diameter_dtls_sctp",
		
		// RFC 6408.
		"diameter.sctp" => "diameter_sctp",
		
		// RFC 6408.
		"diameter.tcp" =>  "tcp",
		
		// RFC 6408.
		"diameter.tls.tcp" => "tls_tcp",
	}
}
