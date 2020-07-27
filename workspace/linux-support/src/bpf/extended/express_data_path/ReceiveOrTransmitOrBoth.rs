// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive, transmit or both.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReceiveOrTransmitOrBoth<V>
{
	/// Receive.
	Receive(V),
	
	/// Other.
	Transmit(V),
	
	/// Both.
	Both(V, V),
}

impl<V> ReceiveOrTransmitOrBoth<V>
{
	#[inline(always)]
	pub fn use_value<MappedV>(&self, use_receive: impl FnOnce(&V) -> MappedV, use_transmit: impl FnOnce(&V) -> MappedV) -> ReceiveOrTransmitOrBoth<MappedV>
	{
		use self::ReceiveOrTransmitOrBoth::*;
		
		match self
		{
			Receive(ref receive) => Receive(use_receive(receive)),
			Transmit(ref transmit) => Transmit(use_transmit(transmit)),
			Both(ref receive, ref transmit) => Both(use_receive(receive), use_transmit(transmit)),
		}
	}
	
	#[inline(always)]
	pub fn map<MappedV>(self, map_receive: impl FnOnce(V) -> MappedV, map_transmit: impl FnOnce(V) -> MappedV) -> ReceiveOrTransmitOrBoth<MappedV>
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
