// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Command to get or set receive flow hash indirection.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct ethtool_rxfh_indir
{
	/// Either `ETHTOOL_GRXFHINDIR` or `ETHTOOL_SRXFHINDIR`.
	pub(crate) cmd: u32,
	
	/// For `ETHTOOL_GRXFHINDIR`:-
	///
	/// If on entry zero, then, on return, the array size of the hardware indirection table (RETA).
	/// if on entry non-zero, then, on return the array size of the hardware indirection table (RETA) and `ring_index` contains a valid hardware indirection table (RETA) of ring queue indices.
	///
	/// For `ETHTOOL_SRXFHINDIR`:-
	///
	/// If on entry zero, then reset the table to default values.
	/// If on entry non-zero, then the hardware indirection table (RETA) is set to the ring queue indices in `ring_index`.
	/// Note that reset is not necessarily supported by all implementations.
	pub(crate) size: u32,
	
	/// Receive ring queue index for each hash value.
	pub(crate) ring_index: __IncompleteArrayField<QueueIdentifier>,
}

impl EthtoolCommand for ethtool_rxfh_indir
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}

impl VariablySizedEthtoolCommand for ethtool_rxfh_indir
{
	type ArrayElement = QueueIdentifier;
	
	#[inline(always)]
	fn array_length(&self) -> u32
	{
		self.size
	}
}
