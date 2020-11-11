// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


fn modern_diameter(code: &mut Code) -> io::Result<HashMap<String, String>>
{
	Ok
	(
		combine_multiple_application_services_with_protocols
		(
			"Diameter",
			
			"application_ientifier",
			
			hashmap!
			{
				"aaa" => "None",
				
				"aaa+ap1" => "Some(DiameterApplicationIdentifier::NASREQ)",
				
				"aaa+ap2" => "Some(DiameterApplicationIdentifier::MobileInternetProtocolVersion4)",
				
				"aaa+ap3" => "Some(DiameterApplicationIdentifier::BaseAccounting)",
				
				"aaa+ap4" => "Some(DiameterApplicationIdentifier::CreditControl)",
				
				"aaa+ap5" => "Some(DiameterApplicationIdentifier::ExtensibleAuthenticationProtocol)",
				
				"aaa+ap6" => "Some(DiameterApplicationIdentifier::SessionInitiationProtocol)",
				
				"aaa+ap7" => "Some(DiameterApplicationIdentifier::MobileInternetProtocolVersion6WithInternetKeyExchangeVersion4AndExtensibleAuthenticationProtocol)",
				
				"aaa+ap8" => "Some(DiameterApplicationIdentifier::MobileInternetProtocolVersion6AuthenticationProtocol)",
				
				"aaa+ap9" => "Some(DiameterApplicationIdentifier::QualityOfService)",
				
				"aaa+ap16777250" => "Some(DiameterApplicationIdentifier::_3rdGenerationPartnershipProjectSTa)",
				
				"aaa+ap16777251" => "Some(DiameterApplicationIdentifier::_3rdGenerationPartnershipProjectS6a)",
				
				"aaa+ap16777264" => "Some(DiameterApplicationIdentifier::_3rdGenerationPartnershipProjectSWm)",
				
				"aaa+ap16777267" => "Some(DiameterApplicationIdentifier::_3rdGenerationPartnershipProjectS9)",
				
				"aaa+ap16777281" => "Some(DiameterApplicationIdentifier::WNAAADA)",
				
				"aaa+ap16777282" => "Some(DiameterApplicationIdentifier::WNADA)",
				
				"aaa+ap16777283" => "Some(DiameterApplicationIdentifier::WM4DA)",
				
				"aaa+ap16777284" => "Some(DiameterApplicationIdentifier::WM6DA)",
				
				"aaa+ap16777285" => "Some(DiameterApplicationIdentifier::WDDA)",
				
				"aaa+ap16777286" => "Some(DiameterApplicationIdentifier::WLAADA)",
				
				"aaa+ap16777287" => "Some(DiameterApplicationIdentifier::W_PCC_R3_P)",
				
				"aaa+ap16777288" => "Some(DiameterApplicationIdentifier::W_PCC_R3_OFC)",
				
				"aaa+ap16777289" => "Some(DiameterApplicationIdentifier::W_PCC_R3_OFC_PRIME)",
				
				"aaa+ap16777290" => "Some(DiameterApplicationIdentifier::W_PCC_R3_OC)",
				
				"aaa+ap4294967295" => "Some(DiameterApplicationIdentifier::Relay)",
			},
			
			AllCombinationsAndPermutationsOfApplicationProtocols::process
			(
				code,
				
				"modern_diameter",
				
				"DiameterTransportProtocol",
				
				hashmap!
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
