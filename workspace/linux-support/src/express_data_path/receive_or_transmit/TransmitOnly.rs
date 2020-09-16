// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Transmit only.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TransmitOnly<Receive, Transmit>(PhantomData<Receive>, Transmit);

impl<Receive, Transmit> ReceiveOrTransmitOrBoth<Receive, Transmit> for TransmitOnly<T>
{
	const IsReceiveOrBoth: bool = false;
	
	#[inline(always)]
	fn receive(&self) -> &Receive
	{
		panic!("Unsupported")
	}
	
	#[inline(always)]
	fn transmit(&self) -> &Transmit
	{
		&self.0
	}
	
	#[inline(always)]
	fn use_value(&self, _use_receive: impl FnOnce(&Receive), use_transmit: impl FnOnce(&Transmit))
	{
		use_transmit(self.transmit())
	}
}

impl MapReceiveOrTransmitOrBoth for TransmitOnly<RingQueueDepth, RingQueueDepth>
{
	type To = TransmitOnly<ReceiveQueue, TransmitQueue>;
	
	#[allow(missing_docs)]
	fn map(self, _map_receive: impl FnOnce(RingQueue) -> ReceiveQueue, map_transmit: impl FnOnce(RingQueue) -> TransmitQueue) -> Self::To
	{
		TransmitOnly::new(map_transmit(self.1))
	}
}

impl<Receive, Transmit> TransmitOnly<Receive, Transmit>
{
	/// New instance.
	#[inline(always)]
	pub const fn new(transmit: Transmit) -> Self
	{
		Self(PhantomData, transmit)
	}
}
