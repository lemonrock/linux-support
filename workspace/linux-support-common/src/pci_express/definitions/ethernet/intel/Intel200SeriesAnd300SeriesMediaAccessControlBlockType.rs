// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Intel® Ethernet Controller 200 and 300 Series Media Access Control (MAC) block type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Intel200SeriesAnd300SeriesMediaAccessControlBlockType
{
	/// Intel® 82575 Gigabit Ethernet Controller (Zoar) Physical Function (PF) (Discontinued).
	_82575,

	/// Intel® 82576 Gigabit Ethernet Controller (Kawela) Physical Function (PF) (Discontinued).
	_82576,

	/// Intel® 82580 Gigabit Ethernet Controller (Barton Hills) Physical Function (PF).
	_82580,

	/// Intel® Ethernet Controller I210 (Springville) Physical Function (PF).
	I210,

	/// Intel® Ethernet Controller I211 (Pearsonville) Physical Function (PF).
	I211,

	/// Intel® Ethernet Controller I350 (Powerville) Physical Function (PF).
	I350,

	/// Intel® Ethernet Controller I354 (?codename) Physical Function (PF).
	I354,

	/// All Virtual Function (VF) implementations apart from Intel® Ethernet Controller I350 (Powerville).
	MostVirtualFunction,

	/// Intel® Ethernet Controller I350 (Powerville) Virtual Function (VF).
	I350VirtualFunction,
}

impl Intel200SeriesAnd300SeriesMediaAccessControlBlockType
{
	/// Is this a Physical Function (PF) or a Virtual Function (VF)?
	#[inline(always)]
	pub fn device_function(self) -> PciDevicePhysicalOrVirtualFunction
	{
		use self::PciDevicePhysicalOrVirtualFunction::*;
		use self::Intel200SeriesAnd300SeriesMediaAccessControlBlockType::*;

		match self
		{
			MostVirtualFunction | I350VirtualFunction => VirtualFunction,
			_ => PhysicalFunction,
		}
	}
}
