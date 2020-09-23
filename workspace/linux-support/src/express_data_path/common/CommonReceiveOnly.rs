// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Debug)]
pub struct CommonReceiveOnly<RP: ReceivePoll>
{
	receive_queue: ReceiveQueue,
	
	frames_received: FramesCount,
	
	receive_poll: RefCell<RP>,
	
	receive_queue_identifier: QueueIdentifier,
}

impl<RP: ReceivePoll> Supports for CommonReceiveOnly<RP>
{
	const SupportsReceive: bool = true;
	
	const SupportsTransmit: bool = false;
}

impl<RP: ReceivePoll> ReceiveOrTransmitOrBoth for CommonReceiveOnly<RP>
{
	type RP = RP;
}

impl<RP: ReceivePoll> Receives<Self> for CommonReceiveOnly<RP>
{
	#[inline(always)]
	fn receive(&self) -> &Self
	{
		self
	}
}

impl<RP: ReceivePoll> CommonReceiveOnly<RP>
{
	#[inline(always)]
	pub(crate) const fn new(receive_queue: ReceiveQueue, receive_poll: RP, receive_queue_identifier: QueueIdentifier) -> Self
	{
		Self
		{
			receive_queue,
			frames_received: FramesCount::new(),
			receive_poll: RefCell::new(receive_poll),
			receive_queue_identifier,
		}
	}
	
	#[inline(always)]
	pub(crate) fn receive_queue(&self) -> &ReceiveQueue
	{
		&self.receive_queue
	}
	
	#[inline(always)]
	pub(crate) fn frames_received(&self) -> u64
	{
		self.frames_received.current()
	}
	
	#[inline(always)]
	pub(crate) fn increase_frames_received(&self, number_of_frames: NonZeroU32)
	{
		self.frames_received.increment(number_of_frames)
	}
	
	#[inline(always)]
	pub(crate) fn receive_poll(&self)
	{
		self.receive_poll.borrow_mut().poll()
	}
	
	#[inline(always)]
	pub(crate) fn remove_receive_map_queue_identifier(&mut self, redirect_map_and_attached_program: &RedirectMapAndAttachedProgram) -> Result<bool, Errno>
	{
		redirect_map_and_attached_program.remove_receive_map_receive_queue_identifier(self.receive_queue_identifier)
	}
}
