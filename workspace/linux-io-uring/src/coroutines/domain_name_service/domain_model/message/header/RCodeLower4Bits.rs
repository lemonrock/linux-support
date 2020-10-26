// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// With `EDNS(0)` (see RFC 6891) and the not-quite unified error code space clarified in RFC 6895, Section 2.1 RCODE Assignment, error codes have become a complicated mess with overlapping assignments.
///
/// It is also no longer possible to short-circuit processing of a message until after most or all of the Additional Section has been processed, in case an `EDNS(0)` `OPT` record is present (or not present when it should be present).
/// This, of course, makes any potential Denial-of-Service attack more effective as it forces nearly complete processing of each reply, rather than fail-fast after parsing the message header.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct RCodeLower4Bits(u8);

impl RCodeLower4Bits
{
	#[inline(always)]
	pub(crate) const fn into_u16(self) -> u16
	{
		self.0 as u16
	}
}
