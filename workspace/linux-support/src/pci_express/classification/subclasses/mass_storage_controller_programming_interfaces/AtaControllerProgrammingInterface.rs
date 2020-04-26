// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// ATA Controller with ADMA interface Programming Interface.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(u8)]
pub enum AtaControllerProgrammingInterface
{
	#[allow(missing_docs)]
	SingleStepping = 0x20,

	#[allow(missing_docs)]
	ContinuousOperation = 0x30,
}

impl AtaControllerProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::AtaControllerProgrammingInterface::*;

		match programming_interface
		{
			0x20 => Some(SingleStepping),
			0x30 => Some(ContinuousOperation),
			_ => None,
		}
	}
}
