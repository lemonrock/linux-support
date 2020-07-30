// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct ethtool_gstrings
{
	/// Always `ETHTOOL_GSTRINGS`.
	cmd: u32,
	
	/// String set identifier.
	pub(crate) string_set: ethtool_stringset,
	
	/// Number of strings the buffer can hold.
	pub(crate) len: u32,
	
	/// A variably-sized buffer suitable for holding all known strings.
	///
	/// Each string is an ASCII NULL padded array of size `ETH_GSTRING_LEN`.
	///
	/// The number of strings is in places such as ethdrv_info and can also be found using the command `ETHTOOL_GSSET_INFO`.
	data: __IncompleteArrayField<u8>,
}

impl EthtoolCommand for ethtool_gstrings
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}

impl VariablySizedEthtoolCommand for ethtool_gstrings
{
	type ArrayElement = [u8; ETH_GSTRING_LEN];
	
	#[inline(always)]
	fn array_length(&self) -> u32
	{
		self.len
	}
}

impl ethtool_gstrings
{
	#[inline(always)]
	pub(crate) fn new(string_set: ethtool_stringset, array_size: u32) -> VariablySizedEthtoolCommandWrapper<Self>
	{
		Self::new_with_initialized_header_but_uninitialized_array
		(
			Self
			{
				cmd: ETHTOOL_GSTRINGS,
				string_set,
				len,
				data: __IncompleteArrayField::new(),
			}
		)
	}
}
