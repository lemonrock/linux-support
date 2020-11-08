// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is a subset of IANA-registered application service tags at <https://www.iana.org/assignments/s-naptr-parameters/s-naptr-parameters.xhtml#s-naptr-parameters-2>.
///
/// Format defined by RFC 3958, Section 6.5 Service Parameters.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModernDiameterTransportProtocol
{
	/// `diameter.dtls.sctp`.
	///
	/// Defined by RFC 6733.
	diameter_dtls_sctp,
	
	/// `diameter.sctp`.
	///
	/// Defined by RFC 6408.
	diameter_sctp,
	
	/// `diameter.tcp`.
	///
	/// Defined by RFC 6408.
	diameter_tcp,
	
	/// `diameter.tls.tcp`.
	///
	/// Defined by RFC 6408.
	diameter_tls_tcp,
}

impl ToNamingAuthorityCommonTransportProtocol for ModernDiameterTransportProtocol
{
	#[inline(always)]
	fn to_naming_authority_common_transport_protocol(self) -> NamingAuthorityCommonTransportProtocol
	{
		use self::ModernDiameterTransportProtocol::*;
		use self::NamingAuthorityCommonTransportProtocol::*;
		
		match self
		{
			diameter_dtls_sctp => DTLS_over_SCTP,
			
			diameter_sctp => SCTP,
			
			diameter_tcp => TCP,
			
			diameter_tls_tcp => TLS_over_TCP,
		}
	}
}
