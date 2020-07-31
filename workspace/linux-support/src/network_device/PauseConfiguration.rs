// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Pause configuration.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PauseConfiguration
{
	Disabled,
	
	AutoNegotiated,
	
	TransmitOnly,
	
	ReceiveOnly,
	
	TransmitAndReceive,
}

impl Default for PauseConfiguration
{
	#[inline(always)]
	fn default() -> Self
	{
		PauseConfiguration::AutoNegotiated
	}
}

impl PauseConfiguration
{
	#[inline(always)]
	pub(crate) fn to_u32_booleans(self) -> (u32, u32, u32)
	{
		use self::PauseConfiguration::*;
		
		match self
		{
			Disabled => (0, 0, 0),
			AutoNegotiated => (1, 0, 0),
			TransmitOnly => (0, 1, 0),
			ReceiveOnly => (0, 0, 1),
			TransmitAndReceive => (0, 1, 1),
		}
	}
}
