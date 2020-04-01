// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// VGA or 8514 compatible.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum VgaOr8514DisplayControllerProgrammingInterface
{
	/// VGA-compatible controller.
	///
	/// Memory addresses 0A 0000h through 0B FFFFh.
	/// I/O addresses 3B0h to 3BBh and 3C0h to 3DFh and all aliases of these addresses.
	VgaCompatible = 0x00,

	/// 8514-compatible controller.
	///
	/// 2E8h and its aliases, 2EAh-2EFh.
	_8514Compatible = 0x01,
}

impl VgaOr8514DisplayControllerProgrammingInterface
{
	#[inline(always)]
	pub(crate) fn parse(programming_interface: u8) -> Option<Self>
	{
		use self::VgaOr8514DisplayControllerProgrammingInterface::*;

		match programming_interface
		{
			0x00 => Some(VgaCompatible),
			0x01 => Some(_8514Compatible),
			_ => None,
		}
	}
}
