// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn location_to_service_translation_protocol() -> HashMap<&'static str, &'static str>
{
	hashmap!
	{
		"LoST:http" => "LocationToServiceTranslationProtocol { profile: LocationToServiceTranslationProtocolProfile::Generic, transport_protocol: Some(http), ), uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord::parse_LocationToServiceTranslationProtocol(replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag, Some(http))? }",
		
		"LoST:https" => "LocationToServiceTranslationProtocol { profile: LocationToServiceTranslationProtocolProfile::Generic, transport_protocol: Some(https), uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord::parse_LocationToServiceTranslationProtocol(replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag, Some(https))? }",
		
		"LoST" => "LocationToServiceTranslationProtocol { profile: LocationToServiceTranslationProtocolProfile::Generic, transport_protocol: None, uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord::parse_LocationToServiceTranslationProtocol(replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag, None)? }",
		
		"LoST-Validation:http" => "LocationToServiceTranslationProtocol { profile: LocationToServiceTranslationProtocolProfile::ValidationOnly, transport_protocol: Some(http), uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord::parse_LocationToServiceTranslationProtocol(replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag, Some(http))? }",
		
		"LoST-Validation:https" => "LocationToServiceTranslationProtocol { profile: LocationToServiceTranslationProtocolProfile::ValidationOnly, transport_protocol: Some(https), uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord::parse_LocationToServiceTranslationProtocol(replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag, Some(https))? }",
		
		"LoST-Validation" => "LocationToServiceTranslationProtocol { profile: LocationToServiceTranslationProtocolProfile::ValidationOnly, transport_protocol: None, uri_or_query_for_uri_resource_record_next: UriOrQueryUriResourceRecord::parse_LocationToServiceTranslationProtocol(replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag, None)? }",
	}
}
