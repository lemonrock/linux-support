// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// SIPS (SIP Secure) legacy transport protocol.
///
/// Only one of these may be present.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SipSecureLegacyResolutionService
{
	/// TCP.
	D2T,

	/// SCTP.
	D2S,

	/// Web Socket (WS).
	D2W,
}

impl ToNamingAuthorityCommonTransportProtocol for SipSecureLegacyResolutionService
{
	#[inline(always)]
	fn to_naming_authority_common_transport_protocol(self) -> NamingAuthorityCommonTransportProtocol
	{
		use self::SipSecureLegacyResolutionService::*;
		use self::NamingAuthorityCommonTransportProtocol::*;
		
		match self
		{
			D2T => TLS_over_TCP,
			
			D2S => DTLS_over_SCTP,
			
			D2W => WebSocketSecure,
		}
	}
}
