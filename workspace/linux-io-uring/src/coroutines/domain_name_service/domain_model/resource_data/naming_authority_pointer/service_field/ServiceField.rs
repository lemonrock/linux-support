// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Service field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceField
{
	/// Modern diameter using S-NAPTR.
	///
	/// This is a subset of <https://www.iana.org/assignments/s-naptr-parameters/s-naptr-parameters.xhtml#s-naptr-parameters-1>.
	///
	/// Format defined by RFC 3958, Section 6.5 Service Parameters.
	ModernDiameter
	{
		/// An IANA-registered application identifier.
		application_identifier: Option<u32>,
	
		/// Transport protocols.
		///
		/// Can legitimately be empty.
		transport_protocols: HashSet<ModernDiameterTransportProtocol>,
	},
	
	/// Legacy diameter.
	LegacyDiameter(DiameterLegacyResolutionService),
	
	/// Legacy SIPS.
	LegacySipSecure(SipSecureLegacyResolutionService),
	
	/// Legacy SIP.
	LegacySip(SipLegacyResolutionService),
}
