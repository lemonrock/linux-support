// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Default, Copy, Clone)]
pub(crate) struct ethtool_gstats
{
	/// either `ETHTOOL_GSTATS` or `ETHTOOL_GPHYSTATS`.
	cmd: u32,
	
	n_stats: u32,

	data: __IncompleteArrayField<u64>,
}

impl EthtoolCommand for ethtool_gstats
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}

impl VariablySizedEthtoolCommand for ethtool_gstats
{
	type ArrayElement = u64;
	
	#[inline(always)]
	fn array_length(&self) -> u32
	{
		self.n_stats
	}
}

impl ethtool_gstats
{
	#[inline(always)]
	pub(crate) fn get(command: u32, number_of_statistics: u32) -> VariablySizedEthtoolCommandWrapper<Self>
	{
		VariablySizedEthtoolCommand::new_with_initialized_header_but_uninitialized_array
		(
			Self
			{
				cmd: command,
				n_stats: number_of_statistics,
				data: __IncompleteArrayField::default(),
			}
		)
	}
}
