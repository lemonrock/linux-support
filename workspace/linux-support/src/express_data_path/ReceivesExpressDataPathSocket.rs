// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receives.
pub trait ReceivesExpressDataPathSocket<ROTOB: ReceiveOrTransmitOrBoth + Receives<CommonReceiveOnly<RP>>, CA: ChunkAlignment, RP: ReceivePoll>: ExpressDataPathSocket<ROTOB, CA>
{
	/// Receive and drop frames.
	fn receive_and_drop<RFP: ReceivedFrameProcessor<ProcessingOutcome=ReceiveProcessingOutcome>>(&self, received_frame_processor: &mut RFP)
	{
		use self::ReceiveProcessingOutcome::*;
		
		self.receive_queue_peek_execute_submit
		(
			received_frame_processor,
			|available_number_of_frames, receive_queue_index, received_frame_processor|
			{
				self.fill_queue_lock_reserve_execute_submit_unlock
				(
					available_number_of_frames,
					|reserved_number_of_frames, fill_queue_index|
					{
						let mut filled_number_of_frames = 0;
						
						received_frame_processor.begin(reserved_number_of_frames.get());
						{
							for relative_frame_index in 0 .. reserved_number_of_frames
							{
								let receive_descriptor = self.receive_queue().get_receive_descriptor(receive_queue_index, relative_frame_index);
								let (headroom, received_frame) = self.user_memory().frame_from_descriptor(&receive_descriptor);
								
								match received_frame_processor.process_received_frame(relative_frame_index, received_frame)
								{
									GiftFrameBackToKernelForAnotherReceive =>
									{
										let user_memory_area_relative_address = CA::user_memory_area_relative_address(user_memory.chunk_size(), receive_descriptor);
										self.fill_queue().set_fill_user_memory_descriptor_of_frame_in_user_memory(fill_queue_index, relative_frame_index, user_memory_area_relative_address);
										filled_number_of_frames += 1;
									}
									
									ReturnFrameToUnusedFrames =>
									{
										let frame_number = AlignedFrameNumber::from_user_memory_descriptor(receive_descriptor.extract_address_if_aligned(self.user_memory().chunk_size), self.user_memory().chunk_size);
										self.user_memory().unused_frames_queue.push(frame_number)
									}
									
									RetainedFramePickAnotherOneFromUnusedFrames =>
									{
										match self.user_memory().unused_frames_queue.pop()
										{
											None => received_frame_processor.no_more_unused_frames_to_gift_to_linux_kernel(),
											
											Some(frame_nummber) =>
											{
												self.fill_queue().set_fill_user_memory_descriptor_of_frame_in_user_memory(fill_queue_index, relative_frame_index, frame_nummber.to_user_memory_descriptor(self.user_memory().chunk_size));
												filled_number_of_frames += 1;
											}
										}
									}
									
									Forward =>
									{
										// push to a pending transmit queue.
										// specialized case of `RetainedFramePickAnotherOneFromUnusedFrames`.
										XXXX;
									}
								}
							}
						}
						received_frame_processor.end();
						
						unsafe { transmute(filled_number_of_frames) }
					}
				)
			}
		);
	}
	
	/// Frames received.
	#[inline(always)]
	fn frames_received(&self) -> u64
	{
		self.common().frames_received()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn nothing_received<RFP: ReceivedFrameProcessor>(&self, received_frame_processor: &mut RFP)
	{
		self.if_fill_queue_needs_wake_up_receive_poll();
		received_frame_processor.nothing_received()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn fill_queue_lock_reserve_execute_submit_unlock(&self, requested_number_of_frames: NonZeroU32, execute: impl FnOnce(NonZeroU32, u32) -> Option<NonZeroU32>) -> Option<NonZeroU32>
	{
		let fill_queue_index = loop
		{
			self.lock_fill_queue();
			
			if let Some(fill_queue_index) = self.fill_queue().reserve(requested_number_of_frames)
			{
				break fill_queue_index
			}
			
			self.unlock_fill_queue();
			
			self.if_fill_queue_needs_wake_up_receive_poll()
		};
		
		let reserved_number_of_frames = requested_number_of_frames;
		
		match execute(reserved_number_of_frames, fill_queue_index)
		{
			None => None,
			
			Some(filled_number_of_frames) =>
			{
				self.fill_queue().submit(filled_number_of_frames);
				self.unlock_fill_queue();
				Some(filled_number_of_frames)
			}
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn if_fill_queue_needs_wake_up_receive_poll(&self)
	{
		if self.fill_queue().needs_wake_up()
		{
			self.common().receive_poll()
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn fill_queue(&self) -> &FillQueue
	{
		&self.user_memory().fill_queue
	}
	
	#[doc(hidden)]
	fn lock_fill_queue(&self);
	
	#[doc(hidden)]
	fn unlock_fill_queue(&self);
	
	#[doc(hidden)]
	#[inline(always)]
	fn receive_queue_peek_execute_submit<RFP: ReceivedFrameProcessor>(&self, received_frame_processor: &mut RFP, execute: impl FnOnce(NonZeroU32, u32, &mut RFP) -> Option<NonZeroU32>) -> Option<NonZeroU32>
	{
		if let Some((available_number_of_frames, receive_index)) = self.receive_queue().peek(received_frame_processor.maximum_number_of_frames())
		{
			match execute(available_number_of_frames, receive_index, received_frame_processor)
			{
				None => None,
				
				Some(received_number_of_frames) =>
				{
					self.receive_queue().release(received_number_of_frames);
					self.common().increase_frames_received(received_number_of_frames);
					Some(received_number_of_frames)
				}
			}
		}
		else
		{
			self.nothing_received(received_frame_processor);
			None
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn receive_queue(&self) -> &ReceiveQueue
	{
		self.common().receive_queue()
	}
}
