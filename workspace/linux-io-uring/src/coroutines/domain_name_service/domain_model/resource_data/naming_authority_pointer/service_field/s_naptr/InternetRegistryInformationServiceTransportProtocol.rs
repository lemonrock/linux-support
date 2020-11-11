// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// This is a subset of IANA-registered application service tags at <https://www.iana.org/assignments/s-naptr-parameters/s-naptr-parameters.xhtml#s-naptr-parameters-2>.
///
/// Format defined by RFC 3958, Section 6.5 Service Parameters.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum InternetRegistryInformationServiceTransportProtocol
{
	/// `iris.beep`.
	///
	/// Defined by RFC 3983.
	///
	/// Blocks Extensible Exchange Protocol.
	///
	/// Formerly the default protocol but as of RFC 4992 XPC is the default protocol.
	///
	/// In principle transport neutral but in reality used over TCP.
	iris_beep,
	
	/// `iris.lwz`.
	///
	/// Defined by RFC 4993.
	///
	/// Lightweight using Compression.
	///
	/// Runs optionally DEFALTE compressed XML over UDP port 715.
	iris_lwz,
	
	/// `iris.xpc`.
	///
	/// Defined by RFC 4992.
	///
	/// XML Pipelining with Chunks.
	///
	/// Runs over TCP on port 713.
	///
	/// Default protocol.
	iris_xpc,
	
	/// `iris.xpcs`.
	///
	/// Defined by RFC 4992.
	///
	/// XML Pipelining with Chunks Secure.
	///
	///  Runs over TLS over TCP on port 714.
	iris_xpcs,
}

impl ToNamingAuthorityCommonTransportProtocol for InternetRegistryInformationServiceTransportProtocol
{
	#[inline(always)]
	fn to_naming_authority_common_transport_protocol(self) -> NamingAuthorityCommonTransportProtocol
	{
		use self::InternetRegistryInformationServiceTransportProtocol::*;
		use self::NamingAuthorityCommonTransportProtocol::*;
		
		match self
		{
			iris_beep => TCP,
			
			iris_lwz => UDP,
			
			iris_xpc => TCP,
			
			iris_xpcs => TLS_over_TCP,
		}
	}
}
