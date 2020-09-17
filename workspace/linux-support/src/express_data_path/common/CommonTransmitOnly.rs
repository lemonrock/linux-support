// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Debug)]
pub struct CommonTransmitOnly
{
	transmit_queue: TransmitQueue,
	
	frames_transmitted: FramesCount,
	
	outstanding_frames_to_transmit: Cell<u32>,
}

impl Supports for CommonTransmitOnly
{
	const SupportsReceive: bool = false;
	
	const SupportsTransmit: bool = true;
}

impl ReceiveOrTransmitOrBoth for CommonTransmitOnly
{
}

impl Transmits<Self> for CommonTransmitOnly
{
	#[inline(always)]
	fn transmit(&self) -> &Self
	{
		self
	}
}

impl CommonTransmitOnly
{
	#[inline(always)]
	pub(crate) const fn new(transmit_queue: TransmitQueue) -> Self
	{
		Self
		{
			transmit_queue,
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
	pub(crate) fn increase_frames_transmitted(&self, number_of_frames: u32)
	{
		self.frames_transmitted.increment(number_of_frames)
	}
	
	#[inline(always)]
	pub(crate) fn have_no_outstanding_frames_to_transmit(&self) -> bool
	{
		self.outstanding_frames_to_transmit() == 0
	}
	
	#[inline(always)]
	pub(crate) fn increment_outstanding_frames_to_transmit(&self, number_of_frames: u32)
	{
		self.outstanding_frames_to_transmit.set(self.outstanding_frames_to_transmit() + number_of_frames)
	}
	
	#[inline(always)]
	pub(crate) fn decrement_outstanding_frames_to_transmit(&self, number_of_frames: u32)
	{
		self.outstanding_frames_to_transmit.set(self.outstanding_frames_to_transmit() - number_of_frames)
	}
	
	#[inline(always)]
	fn outstanding_frames_to_transmit(&self) -> u32
	{
		self.outstanding_frames_to_transmit.get()
	}
}
