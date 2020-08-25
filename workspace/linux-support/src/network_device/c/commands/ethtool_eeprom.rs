// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Default, Copy, Clone)]
pub(crate) struct ethtool_eeprom
{
	/// Either `ETHTOOL_GEEPROM`, `ETHTOOL_GMODULEEEPROM` or `ETHTOOL_SEEPROM`.
	cmd: u32,
	
	pub(crate) magic: u32,
	
	pub(crate) offset: u32,
	
	pub(crate) len: u32,

	data: __IncompleteArrayField<u8>,
}

impl EthtoolCommand for ethtool_eeprom
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}

impl VariablySizedEthtoolCommand for ethtool_eeprom
{
	type ArrayElement = u8;
	
	#[inline(always)]
	fn array_length(&self) -> u32
	{
		self.len
	}
}

impl ethtool_eeprom
{
	#[inline(always)]
	pub(crate) fn get_eeprom(length: u32) -> VariablySizedEthtoolCommandWrapper<Self>
	{
		Self::new_with_initialized_header_but_uninitialized_array
		(
			Self
			{
				cmd: ETHTOOL_GEEPROM,
				magic: 0,
				offset: 0,
				len: length,
				data: __IncompleteArrayField::default(),
			}
		)
	}
	
	#[inline(always)]
	pub(crate) fn get_module_eeprom(length: u32) -> VariablySizedEthtoolCommandWrapper<Self>
	{
		Self::new_with_initialized_header_but_uninitialized_array
		(
			Self
			{
				cmd: ETHTOOL_GMODULEINFO,
				magic: 0,
				offset: 0,
				len: length,
				data: __IncompleteArrayField::default(),
			}
		)
	}
}
