// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Diameter (AAA) legacy transport protocol.
///
/// Only one of these may be present.
///
/// See RFC 3588, Section 11.6 NAPTR Service Fields.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Hash)]
pub enum DiameterLegacyResolutionService
{
	/// TCP.
	D2T,

	/// SCTP.
	D2S,
}
