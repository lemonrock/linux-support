// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Debug)]
pub struct CommonTransmitOnly<TS: TransmitSend>
{
	transmit_queue: TransmitQueue,
	
	transmit_send: RefCell<TS>,
	
	frames_transmitted: FramesCount,
	
	outstanding_frames_to_transmit: Cell<u32>,
}

impl<TS: TransmitSend> Supports for CommonTransmitOnly<TS>
{
	const SupportsReceive: bool = false;
	
	const SupportsTransmit: bool = true;
}

impl<TS: TransmitSend> ReceiveOrTransmitOrBoth for CommonTransmitOnly<TS>
{
	type RP = ();
	
	type TS = TS;
}

impl<TS: TransmitSend> Transmits<Self> for CommonTransmitOnly<TS>
{
	#[inline(always)]
	fn transmit(&self) -> &Self
	{
		self
	}
}

impl<TS: TransmitSend> CommonTransmitOnly<TS>
{
	#[inline(always)]
	pub(crate) const fn new(transmit_queue: TransmitQueue, transmit_send: TS) -> Self
	{
		Self
		{
			transmit_queue,
			transmit_send: RefCell::new(transmit_send),
			frames_transmitted: FramesCount::new(),
			outstanding_frames_to_transmit: Cell::new(0),
		}
	}
	
	#[inline(always)]
	pub(crate) fn transmit_queue(&self) -> &TransmitQueue
	{
		&self.transmit_queue
	}
	
	#[inline(always)]
	pub(crate) fn frames_transmitted(&self) -> u64
	{
		self.frames_transmitted.current()
	}
	
	#[inline(always)]
	pub(crate) fn increase_frames_transmitted(&self, number_of_frames: NonZeroU32)
	{
		self.frames_transmitted.increment(number_of_frames)
	}
	
	#[inline(always)]
	pub(crate) fn increment_outstanding_frames_to_transmit(&self, number_of_frames: NonZeroU32)
	{
		self.outstanding_frames_to_transmit.set(self.outstanding_frames_to_transmit() + number_of_frames.get())
	}
	
	#[inline(always)]
	pub(crate) fn decrement_outstanding_frames_to_transmit(&self, number_of_frames: NonZeroU32)
	{
		self.outstanding_frames_to_transmit.set(self.outstanding_frames_to_transmit() - number_of_frames.get())
	}
	
	#[inline(always)]
	pub(crate) fn outstanding_frames_to_transmit(&self) -> u32
	{
		self.outstanding_frames_to_transmit.get()
	}
	
	#[inline(always)]
	pub(crate) fn transmit_send(&self)
	{
		self.transmit_send.borrow_mut().send()
	}
}
