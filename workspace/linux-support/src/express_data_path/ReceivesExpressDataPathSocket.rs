// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receives.
pub trait ReceivesExpressDataPathSocket<ROTOB: ReceiveOrTransmitOrBoth + Receives<CommonReceiveOnly<RP>>, CA: ChunkAlignment>: ExpressDataPathSocket<ROTOB, CA>
{
	/// Frames received.
	#[inline(always)]
	fn frames_received(&self) -> u64
	{
		self.common().frames_received()
	}
	
	/// `maximum_number_of_frames` is usually `64` but can also be `16` or `32`.
	fn receive_and_drop<F: FnOnce(&ExpressDataPathFileDescriptor) + Copy, RFP: ReceivedFrameProcessor>(&self, maximum_number_of_frames: u32, received_frame_processor: &mut RFP) -> RFP::R
	{
		let receive_queue = self.receive_queue();
		let user_memory = self.user_memory();
		let common = self.common();
		let fill_queue = self.fill_queue();
		
		let (received_number_of_frames, receive_index) = match receive_queue.peek(maximum_number_of_frames)
		{
			None =>
				{
					if fill_queue.needs_wake_up()
					{
						common.receive_poll()
					}
					return received_frame_processor.nothing_received()
				}
			
			Some((received_number_of_frames, receive_index)) => (received.get(), receive_index)
		};
		
		let fill_queue_index = self.empty_fill_queue_until_sufficient_space_is_available_returning_it_locked(received_number_of_frames);
		
		received_frame_processor.begin(received_number_of_frames);
		for relative_frame_index in 0..received_number_of_frames
		{
			let descriptor = receive_queue.receive_descriptor(receive_index + relative_frame_index);
			let (headroom, received_frame) = user_memory.frame_from_descriptor(&descriptor);
			
			received_frame_processor.process_received_frame(relative_frame_index, received_frame);
			
			fill_queue.set_fill_address(fill_queue_index, relative_frame_index, CA::user_memory_area_relative_address(user_memory.chunk_size(), descriptor));
		}
		let result = received_frame_processor.end();
		
		fill_queue.submit(received_number_of_frames);
		self.unlock_fill_queue();
		
		receive_queue.release(received_number_of_frames);
		
		common.increase_frames_received(received_number_of_frames);
		
		result
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn fill_queue(&self) -> &FillQueue
	{
		&self.user_memory().fill_queue
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn receive_queue(&self) -> &ReceiveQueue
	{
		self.common().receive_queue()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn empty_fill_queue_until_sufficient_space_is_available_returning_it_locked(&self, received_number_of_frames: u32) -> u32
	{
		let fill_queue = self.fill_queue();
		loop
		{
			self.lock_fill_queue();
			if let Some(fill_queue_index) = fill_queue.reserve(received_number_of_frames)
			{
				return fill_queue_index
			}
			self.unlock_fill_queue();
			
			if fill_queue.needs_wake_up()
			{
				common.receive_poll()
			}
		}
	}
	
	#[doc(hidden)]
	fn lock_fill_queue(&self);
	
	#[doc(hidden)]
	fn unlock_fill_queue(&self);
}
