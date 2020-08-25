// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Default, Copy, Clone)]
pub(crate) struct ethtool_dump
{
	/// Either `ETHTOOL_GET_DUMP_FLAG`, `ETHTOOL_GET_DUMP_DATA` or `ETHTOOL_SET_DUMP`.
	cmd: u32,
	
	pub(crate) version: u32,
	
	pub(crate) flag: u32,
	
	len: u32,

	data: __IncompleteArrayField<u8>
}

impl EthtoolCommand for ethtool_dump
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}

impl VariablySizedEthtoolCommand for ethtool_dump
{
	type ArrayElement = u8;
	
	#[inline(always)]
	fn array_length(&self) -> u32
	{
		self.len
	}
}

impl ethtool_dump
{
	#[inline(always)]
	pub(crate) fn get_flag() -> Self
	{
		Self
		{
			cmd: ETHTOOL_GET_DUMP_FLAG,
			version: 0,
			flag: 0,
			len: 0,
			data: __IncompleteArrayField::default(),
		}
	}
	
	#[inline(always)]
	pub(crate) fn get_data(length: u32) -> VariablySizedEthtoolCommandWrapper<Self>
	{
		Self::new_with_initialized_header_but_uninitialized_array
		(
			Self
			{
				cmd: ETHTOOL_GET_DUMP_DATA,
				version: 0,
				flag: 0,
				len: length,
				data: __IncompleteArrayField::default(),
			}
		)
	}
}
