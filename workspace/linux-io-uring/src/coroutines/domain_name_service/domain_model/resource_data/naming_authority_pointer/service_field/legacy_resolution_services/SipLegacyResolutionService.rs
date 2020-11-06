// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// SIP (insecure) legacy transport protocol.
///
/// Only one of these may be present.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Hash)]
pub enum SipLegacyResolutionService
{
	/// TCP.
	D2T,

	/// UDP.
	D2U,

	/// SCTP.
	D2S,

	/// Web Socket (WS).
	D2W,
}
