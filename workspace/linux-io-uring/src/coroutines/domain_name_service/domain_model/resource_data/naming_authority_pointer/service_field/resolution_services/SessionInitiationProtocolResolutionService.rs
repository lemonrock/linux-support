// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// SIP (insecure) legacy transport protocol.
///
/// Only one of these may be present.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SessionInitiationProtocolResolutionService
{
	/// TCP.
	///
	/// RFC 3263.
	TCP,

	/// UDP.
	///
	/// RFC 3263.
	UDP,

	/// SCTP.
	///
	/// RFC 3263.
	SCTP,

	/// Web Socket (WS).
	///
	/// RFC 7118.
	WebSocket,
	
	/// TLS over TCP.
	///
	/// RFC 3263.
	TLS_over_TCP,
	
	/// DTLS over SCTP.
	///
	/// RFC 4168.
	DTLS_over_SCTP,
	
	/// Web Socket (WS) over HTTPS.
	///
	/// RFC 7118.
	WebSocketSecure,
}

impl ToNamingAuthorityCommonTransportProtocol for SessionInitiationProtocolResolutionService
{
	#[inline(always)]
	fn to_naming_authority_common_transport_protocol(self) -> NamingAuthorityCommonTransportProtocol
	{
		use self::NamingAuthorityCommonTransportProtocol::*;
		
		match self
		{
			SessionInitiationProtocolResolutionService::TCP => TCP,
			
			SessionInitiationProtocolResolutionService::UDP => UDP,
			
			SessionInitiationProtocolResolutionService::SCTP => SCTP,
			
			SessionInitiationProtocolResolutionService::WebSocket => WebSocket,
			
			SessionInitiationProtocolResolutionService::TLS_over_TCP => TLS_over_TCP,
			
			SessionInitiationProtocolResolutionService::DTLS_over_SCTP => DTLS_over_SCTP,
			
			SessionInitiationProtocolResolutionService::WebSocketSecure => WebSocketSecure,
		}
	}
}
