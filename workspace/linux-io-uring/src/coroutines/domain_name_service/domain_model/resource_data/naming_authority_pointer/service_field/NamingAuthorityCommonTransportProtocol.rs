// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An attempt to synthesize the various ways a transport protocol is represent by NAPTR records!
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum NamingAuthorityCommonTransportProtocol
{
	/// TCP.
	TCP,
	
	/// UDP.
	UDP,

	/// SCTP.
	SCTP,

	/// TLS over TCP.
	TLS_over_TCP,
	
	/// DTLS over SCTP.
	DTLS_over_SCTP,
	
	/// Web Socket (WS)
	WebSocket,
	
	/// Web Socket (WS) over HTTPS.
	WebSocketSecure,
}
