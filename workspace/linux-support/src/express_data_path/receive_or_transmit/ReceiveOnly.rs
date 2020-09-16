// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receive-only.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ReceiveOnly<Receive, Transmit>(pub Receive, PhantomData<Transmit>);

impl<Receive, Transmit> ReceiveOrTransmitOrBoth<Receive, Transmit> for ReceiveOnly<Receive, Transmit>
{
	const IsReceiveOrBoth: bool = true;
	
	#[inline(always)]
	fn receive(&self) -> &Receive
	{
		&self.0
	}
	
	#[inline(always)]
	fn transmit(&self) -> &Transmit
	{
		panic!("Unsupported")
	}
	
	#[inline(always)]
	fn use_value(&self, use_receive: impl FnOnce(&Receive), _use_transmit: impl FnOnce(&Transmit))
	{
		use_receive(self.receive())
	}
}

impl MapReceiveOrTransmitOrBoth for ReceiveOnly<RingQueueDepth, RingQueueDepth>
{
	type To = ReceiveOnly<ReceiveQueue, TransmitQueue>;
	
	#[allow(missing_docs)]
	fn map(self, map_receive: impl FnOnce(RingQueue) -> ReceiveQueue, _map_transmit: impl FnOnce(RingQueue) -> TransmitQueue) -> Self::To
	{
		ReceiveOnly::new(map_receive(self.0))
	}
}

impl<Receive, Transmit> ReceiveOnly<Receive, Transmit>
{
	/// New instance.
	#[inline(always)]
	pub const fn new(receive: Receive) -> Self
	{
		Self(receive, PhantomData)
	}
}
