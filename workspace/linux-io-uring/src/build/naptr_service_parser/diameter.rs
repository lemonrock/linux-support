// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn diameter(code: &mut Code<impl GenerateParseTreeCallback>) -> io::Result<HashMap<String, String>>
{
	Ok
	(
		combine_multiple_application_services_with_protocols
		(
			// RFC 3588, Section 11.6 NAPTR Service Fields.
			hashmap!
			{
				"AAA+D2T" => "Diameter { application_identifier: DiameterApplicationIdentifier::Unspecified { legacy: true }, transport_protocols: &diameter_combination_diameter_tcp, query_for_next: QueryForNext::parse(ServiceFieldKind::Diameter, replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag)? }",
		
				"AAA+D2S" => "Diameter { application_identifier: DiameterApplicationIdentifier::Unspecified { legacy: true }, transport_protocols: &diameter_combination_diameter_sctp, query_for_next: QueryForNext::parse(ServiceFieldKind::Diameter, replacement_domain_name_or_raw_regular_expression, mutually_exclusive_flag)? }",
			},
			
			"Diameter",
			
			"application_ientifier",
			
			"DiameterApplicationIdentifier",
			
			hashmap!
			{
				"aaa" => "Unspecified { legacy: false }",
				
				"aaa+ap1" => "NASREQ",
				
				"aaa+ap2" => "MobileInternetProtocolVersion4",
				
				"aaa+ap3" => "BaseAccounting",
				
				"aaa+ap4" => "CreditControl",
				
				"aaa+ap5" => "ExtensibleAuthenticationProtocol",
				
				"aaa+ap6" => "SessionInitiationProtocol",
				
				"aaa+ap7" => "MobileInternetProtocolVersion6WithInternetKeyExchangeVersion4AndExtensibleAuthenticationProtocol",
				
				"aaa+ap8" => "MobileInternetProtocolVersion6AuthenticationProtocol",
				
				"aaa+ap9" => "QualityOfService",
				
				"aaa+ap16777250" => "_3rdGenerationPartnershipProjectSTa",
				
				"aaa+ap16777251" => "_3rdGenerationPartnershipProjectS6a",
				
				"aaa+ap16777264" => "_3rdGenerationPartnershipProjectSWm",
				
				"aaa+ap16777267" => "_3rdGenerationPartnershipProjectS9",
				
				"aaa+ap16777281" => "WNAAADA",
				
				"aaa+ap16777282" => "WNADA",
				
				"aaa+ap16777283" => "WM4DA",
				
				"aaa+ap16777284" => "WM6DA",
				
				"aaa+ap16777285" => "WDDA",
				
				"aaa+ap16777286" => "WLAADA",
				
				"aaa+ap16777287" => "W_PCC_R3_P",
				
				"aaa+ap16777288" => "W_PCC_R3_OFC",
				
				"aaa+ap16777289" => "W_PCC_R3_OFC_PRIME",
				
				"aaa+ap16777290" => "W_PCC_R3_OC",
				
				"aaa+ap4294967295" => "Relay",
			},
			
			AllCombinationsAndPermutations::process
			(
				code,
				
				"diameter",
				
				"DiameterTransportProtocol",
				
				&indexmap!
				{
					// RFC 6733.
					"diameter.dtls.sctp" => "diameter_dtls_sctp",
					
					// RFC 6408.
					"diameter.sctp" => "diameter_sctp",
					
					// RFC 6408.
					"diameter.tcp" =>  "diameter_tcp",
					
					// RFC 6408.
					"diameter.tls.tcp" => "diameter_tls_tcp",
				}
			)?
		)
	)
}
