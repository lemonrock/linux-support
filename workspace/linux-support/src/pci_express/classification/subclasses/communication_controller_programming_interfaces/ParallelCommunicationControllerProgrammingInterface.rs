// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Parallel.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum ParallelCommunicationControllerProgrammingInterface
{
	/// Parallel port.
	ParallelPort = 0x00,

	/// Bi-directional parallel port.
	BiDirectionalParallelPort = 0x01,

	/// ECP 1.X compliant parallel port.
	Ecp1DotXCompliantParallelPort = 0x02,

	/// IEEE1284 controller.
	Ieee1284Controller = 0x03,

	/// IEEE1284 target device (not a controller).
	Ieee1284TargetDevice = 0xFE,
}

impl ParallelCommunicationControllerProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::ParallelCommunicationControllerProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(ParallelPort),
			0x01 => Some(BiDirectionalParallelPort),
			0x02 => Some(Ecp1DotXCompliantParallelPort),
			0x03 => Some(Ieee1284Controller),
			0xFE => Some(Ieee1284TargetDevice),
			_ => None,
		}
	}
}
