// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// RFC 6503, Section 12.4 DNS Registrations.
///
/// `LIS` stands for 'Local Location Information Server'.
///
/// `HELD` is an acronym for HTTP-Enabled Location Delivery.
fn local_location_information_server() -> HashMap<&'static str, &'static str>
{
	hashmap!
	{
		"LIS:HELD" => "LocalLocationInformationServer { transport_protocol: Some(LocalLocationInformationServerTransportProtocol::HELD), uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord::parse_LocalLocationInformationServer(replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag)? }",
		
		// This is probably not intended by the authors of RFC 6503 but is permitted by RFC 3958.
		"LIS" =>  "LocalLocationInformationServer { transport_protocol: None, uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord::parse_LocalLocationInformationServer(replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag)? }",
	}
}
