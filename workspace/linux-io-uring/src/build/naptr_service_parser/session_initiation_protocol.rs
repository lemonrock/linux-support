// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// See <https://www.iana.org/assignments/sip-table/sip-table.xhtml>.
fn session_initiation_protocol() -> HashMap<&'static str, &'static str>
{
	hashmap!
	{
		// RFC 3263, Section 4.1 Selecting a Transport Protocol.
		"SIP+D2T" => "SessionInitiationProtocol { resolution_service: SessionInitiationProtocolResolutionService::TCP }",
		
		// RFC 3263, Section 4.1 Selecting a Transport Protocol.
		"SIPS+D2T" => "SessionInitiationProtocol { resolution_service: SessionInitiationProtocolResolutionService::TLS_over_TCP }",
		
		// RFC 3263, Section 4.1 Selecting a Transport Protocol.
		"SIP+D2U" => "SessionInitiationProtocol { resolution_service: SessionInitiationProtocolResolutionService::UDP }",
		
		// RFC 3263, Section 4.1 Selecting a Transport Protocol.
		"SIP+D2S" => "SessionInitiationProtocol { resolution_service: SessionInitiationProtocolResolutionService::SCTP }",
		
		// RFC 3263, Section 4.1 Selecting a Transport Protocol.
		"SIPS+D2S" => "SessionInitiationProtocol { resolution_service: SipSecureLegacyResolutionService::DTLS_over_SCTP }",
		
		// RFC 7118.
		"SIP+D2W" => "SessionInitiationProtocol { resolution_service: SessionInitiationProtocolResolutionService::WebSocket }",
		
		// RFC 7118.
		"SIPS+D2W" => "SessionInitiationProtocol { resolution_service: SipSecureLegacyResolutionService::WebSocketSecure }",
	}
}
