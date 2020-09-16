// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CommonSharedExpressDataPathSocket<ReceiveTransmit: ReceiveOrTransmitOrBoth<ReceiveQueue, TransmitQueue>, RP: ReceivePoll>
{
	queue_identifier: QueueIdentifier,
	
	/// receive is `xsk_ring_cons`.
	/// transmit is `xsk_ring_prod`.
	receive_transmit: ReceiveTransmit,
	
	receive_poll: RefCell<RP>,
	
	outstanding_frames_to_transmit: Cell<u32>,

	frames_received: Cell<u64>,
	
	frames_transmitted: Cell<u64>,
}

impl<ReceiveTransmit: ReceiveOrTransmitOrBoth<ReceiveQueue, TransmitQueue>, RP: ReceivePoll> CommonSharedExpressDataPathSocket<ReceiveTransmit, RP>
{
	#[inline(always)]
	fn new(receive_transmit: ReceiveTransmit, queue_identifier: QueueIdentifier, receive_poll: RP) -> Self
	{
		Self
		{
			receive_transmit,
			receive_poll: RefCell::new(receive_poll),
			queue_identifier,
			outstanding_frames_to_transmit: Cell::new(0),
			frames_received: Cell::new(0),
			frames_transmitted: Cell::new(0),
		}
	}
	
	#[inline(always)]
	fn receive_poll(&self)
	{
		self.receive_poll.borrow_mut().blocking_poll()
	}
	
	#[inline(always)]
	fn receive_queue(&self) -> &ReceiveQueue
	{
		self.receive_transmit.receive()
	}
	
	#[inline(always)]
	fn transmit_queue(&self) -> &TransmitQueue
	{
		self.receive_transmit.transmit()
	}
	
	#[inline(always)]
	fn remove_receive_map_queue_identifier(&mut self, express_data_path_extended_bpf_program: &RedirectMapAndAttachedProgram)
	{
		express_data_path_extended_bpf_program.remove_receive_map_queue_identifier::<ReceiveTransmit>(self.queue_identifier)
	}
	
	#[inline(always)]
	fn frames_received_and_transmitted(&self) -> (u64, u64)
	{
		(self.frames_received.get(), self.frames_transmitted.get())
	}
	
	#[inline(always)]
	fn increase_frames_received(&self, number_of_frames: u32)
	{
		Self::increase_frames_count(&self.frames_received, number_of_frames)
	}
	
	#[inline(always)]
	fn increase_frames_transmitted(&self, number_of_frames: u32)
	{
		Self::increase_frames_count(&self.frames_transmitted, number_of_frames)
	}
	
	fn have_no_outstanding_frames_to_transmit(&self) -> bool
	{
		self.outstanding_frames_to_transmit() == 0
	}
	
	#[inline(always)]
	fn increment_outstanding_frames_to_transmit(&self, number_of_frames: u32)
	{
		self.outstanding_frames_to_transmit.set(self.outstanding_frames_to_transmit() + number_of_frames)
	}
	
	#[inline(always)]
	fn decrement_outstanding_frames_to_transmit(&self, number_of_frames: u32)
	{
		self.outstanding_frames_to_transmit.set(self.outstanding_frames_to_transmit() - number_of_frames)
	}
	
	#[inline(always)]
	fn outstanding_frames_to_transmit(&self) -> u32
	{
		self.outstanding_frames_to_transmit.get()
	}
	
	#[inline(always)]
	fn increase_frames_count(frames_counter: &Cell<u64>, number_of_frames: u32)
	{
		frames_counter.set(frames_counter.get() + number_of_frames as u64)
	}
}
