// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct ReceiveTransmitCoalescing
{
	/// Receive.
	pub receive: CoalescePair,
	
	/// Transmit.
	pub transmit: CoalescePair,
}

impl ReceiveTransmitCoalescing
{
	#[inline(always)]
	fn destructure(&self) -> (Option<NonZeroU32>, Option<NonZeroU32>, Option<NonZeroU32>, Option<NonZeroU32>)
	{
		(self.receive.microseconds, self.receive.maximum_frames, self.transmit.microseconds, self.transmit.maximum_frames)
	}
}
