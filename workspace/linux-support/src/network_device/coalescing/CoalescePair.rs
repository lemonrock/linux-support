// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// It is illegal to set both microseconds and maximum_frames to zero as this would cause interrupts to never be generated.
/// To disable coalescing, set `microseconds = 0` and `maximum_frames = 1`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct CoalescePair
{
	/// How many microseconds to delay a receive interrupt after a packet arrives or delay a transmit interrupt after a packet is sent.
	///
	/// `None` can mean unsupported; it can also mean zero for Amazon ENA.
	pub microseconds: Option<NonZeroU32>,
	
	/// Maximum number of packets to receive before an interrupt.
	pub maximum_frames: Option<NonZeroU32>,
}

impl Default for CoalescePair
{
	#[inline(always)]
	fn default() -> Self
	{
		CoalescePair::Disabled
	}
}

impl CoalescePair
{
	/// Disabled.
	pub const DisabledWhereMaximumFramesUnsupported: Self = Self
	{
		microseconds: None,
		
		maximum_frames: None
	};
	
	/// Disabled.
	pub const Disabled: Self = Self
	{
		microseconds: None,
		
		maximum_frames: Some(unsafe { NonZeroU32::new_unchecked(1) }),
	};
}
