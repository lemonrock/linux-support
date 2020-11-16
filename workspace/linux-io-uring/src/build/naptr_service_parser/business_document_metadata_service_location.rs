// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn business_document_metadata_service_location() -> HashMap<&'static str, &'static str>
{
	hashmap!
	{
		"Register:CPPA" => "BusinessDocumentMetadataServiceLocation { profile: BusinessDocumentMetadataServiceLocationProfile::Register, transport_protocol: BusinessDocumentMetadataServiceLocationTransportProtocol::CPPA, domain_name_or_regular_expression: RegularExpressionResolvingToUriOrQueryUriResourceRecord::parse(ServiceFieldKind::Enum, replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag)? }",
		
		"Register:SMP" => "BusinessDocumentMetadataServiceLocation { profile: BusinessDocumentMetadataServiceLocationProfile::Register, transport_protocol: BusinessDocumentMetadataServiceLocationTransportProtocol::SMP, domain_name_or_regular_expression: RegularExpressionResolvingToUriOrQueryUriResourceRecord::parse(ServiceFieldKind::Enum, replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag)? }",
		
		"Meta:CPPA" => "BusinessDocumentMetadataServiceLocation { profile: BusinessDocumentMetadataServiceLocationProfile::Meta, transport_protocol: BusinessDocumentMetadataServiceLocationTransportProtocol::CPPA, domain_name_or_regular_expression: RegularExpressionResolvingToUriOrQueryUriResourceRecord::parse(ServiceFieldKind::Enum, replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag)? }",
		
		"Meta:SMP" => "BusinessDocumentMetadataServiceLocation { profile: BusinessDocumentMetadataServiceLocationProfile::Meta, transport_protocol: BusinessDocumentMetadataServiceLocationTransportProtocol::SMP, domain_name_or_regular_expression: RegularExpressionResolvingToUriOrQueryUriResourceRecord::parse(ServiceFieldKind::Enum, replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag)? }",
	}
}
