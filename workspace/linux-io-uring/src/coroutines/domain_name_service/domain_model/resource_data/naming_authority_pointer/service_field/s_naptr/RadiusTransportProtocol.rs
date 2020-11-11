// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is a subset of IANA-registered application service tags at <https://www.iana.org/assignments/s-naptr-parameters/s-naptr-parameters.xhtml#s-naptr-parameters-2>.
///
/// Format defined by RFC 3958, Section 6.5 Service Parameters.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RadiusTransportProtocol
{
	/// `radius.tls.tcp`.
	///
	/// Defined by RFC 7585.
	radius_tls_tcp,
	
	/// `radius.dtls.udp`.
	///
	/// Defined by RFC 7585.
	radius_dtls_udp,
}

impl ToNamingAuthorityCommonTransportProtocol for RadiusTransportProtocol
{
	#[inline(always)]
	fn to_naming_authority_common_transport_protocol(self) -> NamingAuthorityCommonTransportProtocol
	{
		use self::RadiusTransportProtocol::*;
		use self::NamingAuthorityCommonTransportProtocol::*;
		
		match self
		{
			radius_tls_tcp => TLS_over_TCP,
			
			radius_dtls_udp => DTLS_over_UDP,
		}
	}
}
