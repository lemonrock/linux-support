// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Virtual Function (VF) index.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct VirtualFunctionIndex(pub(crate) u8);

impl TryFrom<u8> for VirtualFunctionIndex
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u8) -> Result<Self, Self::Error>
	{
		if unlikely!(value > Self::InclusiveMaximum.0)
		{
			Err(ParseNumberError::TooLarge)
		}
		else
		{
			Ok(Self(value))
		}
	}
}

impl VirtualFunctionIndex
{
	/// Maximum value is capped by `u32::MAX - 1` as `u32::MAX` is used for `IFLA_NUM_VF` in route netlink's `RTM_GETLINK` message.
	///
	/// However, `ETHTOOL_RX_FLOW_SPEC_RING_VF` encodes a maximum of `255`, so the maximum value is `254`.
	pub const InclusiveMaximum: Self = Self(254);
	
	/// Exclusive maximum.
	pub const ExclusiveMaximum: Self = Self(Self::InclusiveMaximum.0 + 1);
}
