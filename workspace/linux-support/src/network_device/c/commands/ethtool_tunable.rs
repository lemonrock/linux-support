// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug)]
pub(crate) struct ethtool_tunable<T: Tunable>
{
	/// Value is always:-
	///
	/// * either `ETHTOOL_GTUNABLE` or `ETHTOOL_STUNABLE`.
	/// * either `ETHTOOL_PHY_GTUNABLE` or `ETHTOOL_PHY_STUNABLE`.
	pub(crate) cmd: u32,
	
	pub(crate) id: TunableIdentifier,
	
	pub(crate) type_id: tunable_type_id,
	
	len: u32,
	
	// In practice is always either an u16 or an u8.
	pub(crate) data: T,
}

impl<T: Tunable> EthtoolCommand for ethtool_tunable<T>
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}

impl<T: Tunable> ethtool_tunable<T>
{
	#[inline(always)]
	pub(crate) fn new_get() -> Self
	{
		Self
		{
			cmd: T::Commands.commands().0,
			id: T::Identifier,
			type_id: T::TypeIdentifier,
			len: size_of::<T>() as u32,
			data: unsafe_zeroed(),
		}
	}
	
	#[inline(always)]
	pub(crate) fn new_set(tunable: T) -> Self
	{
		Self
		{
			cmd: T::Commands.commands().1,
			id: T::Identifier,
			type_id: T::TypeIdentifier,
			len: size_of::<T>() as u32,
			data: tunable,
		}
	}
}
