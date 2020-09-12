// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive, transmit or both.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReceiveOrTransmitOrBoth<R, T>
{
	/// Receive.
	Receive(R),
	
	/// Other.
	Transmit(T),
	
	/// Both.
	Both(R, T),
}

impl<R, T> ReceiveOrTransmitOrBoth<R, T>
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn is_receive_or_both(&self) -> bool
	{
		use self::ReceiveOrTransmitOrBoth::*;
		
		match self
		{
			&Receive(_) | &Both(_, _) => true,
			&Transmit(_) => false,
		}
	}
}

impl<R, T> ReceiveOrTransmitOrBoth<R, T>
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn use_value<MappedR, MappedT>(&self, use_receive: impl FnOnce(&R) -> MappedR, use_transmit: impl FnOnce(&T) -> MappedT) -> ReceiveOrTransmitOrBoth<MappedR, MappedT>
	{
		use self::ReceiveOrTransmitOrBoth::*;
		
		match self
		{
			Receive(ref receive) => Receive(use_receive(receive)),
			Transmit(ref transmit) => Transmit(use_transmit(transmit)),
			Both(ref receive, ref transmit) => Both(use_receive(receive), use_transmit(transmit)),
		}
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn map<MappedR, MappedT>(self, map_receive: impl FnOnce(R) -> MappedR, map_transmit: impl FnOnce(T) -> MappedT) -> ReceiveOrTransmitOrBoth<MappedR, MappedT>
	{
		use self::ReceiveOrTransmitOrBoth::*;
		
		match self
		{
			Receive(receive) => Receive(map_receive(receive)),
			Transmit(transmit) => Transmit(map_transmit(transmit)),
			Both(receive, transmit) => Both(map_receive(receive), map_transmit(transmit)),
		}
	}
}
