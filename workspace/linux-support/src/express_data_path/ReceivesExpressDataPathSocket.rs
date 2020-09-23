// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Receives.
pub trait ReceivesExpressDataPathSocket<'a, ROTOB: 'a + ReceiveOrTransmitOrBoth<RP=RP> + Receives<CommonReceiveOnly<RP>>, FFQ: 'a + FreeFrameQueue, RP: 'a + ReceivePoll>: ExpressDataPathSocket<'a, ROTOB, FFQ>
{
	/// Receive and drop frames.
	fn receive_and_drop<RFP: ReceivedFrameProcessor<ProcessingOutcome=ReceiveProcessingOutcome>>(&'a self, received_frame_processor: &mut RFP)
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
							let mut fill_relative_frame_index = RelativeFrameIndex::Zero;
							for relative_frame_index in RelativeFrameIndex::relative_frame_indices(reserved_number_of_frames)
							{
								let received_descriptor = self.receive_queue().get_receive_descriptor(receive_queue_index, relative_frame_index);
								let (fill_frame_descriptor_bitfield, xdp_headroom, our_frame_headroom, ethernet_packet, minimum_tailroom_length) = self.user_memory().received_xdp_headroom_our_frame_headroom_ethernet_packet_minimum_tailroom_length(received_descriptor);
								match received_frame_processor.process_received_frame(relative_frame_index, xdp_headroom, our_frame_headroom, ethernet_packet, minimum_tailroom_length)
								{
									GiftFrameBackToKernelForAnotherReceive => self.fill_queue().set_fill_address_receive(fill_queue_index, &mut fill_relative_frame_index, fill_frame_descriptor_bitfield, &mut filled_number_of_frames),
									
									ReturnFrameToUnusedFrames => self.user_memory().push_free_frame_from_receive(received_descriptor.frame_descriptor_bitfield()),
									
									RetainedFramePickAnotherOneFromUnusedFrames => match self.user_memory().pop_free_frame()
									{
										None => received_frame_processor.no_more_unused_frames_to_gift_to_linux_kernel(),
										
										Some(frame_identifier) =>
										{
											let fill_frame_descriptor_bitfield = self.user_memory().frame_identifier_to_fill_frame_descriptor_bitfield(frame_identifier);
											
											self.fill_queue().set_fill_address_receive(fill_queue_index, &mut fill_relative_frame_index, fill_frame_descriptor_bitfield, &mut filled_number_of_frames);
										}
									},
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
	fn frames_received(&'a self) -> u64
	{
		self.common().frames_received()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn nothing_received<RFP: ReceivedFrameProcessor>(&'a self, received_frame_processor: &mut RFP)
	{
		self.if_fill_queue_needs_wake_up_receive_poll();
		received_frame_processor.nothing_received()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn fill_queue_lock_reserve_execute_submit_unlock(&'a self, requested_number_of_frames: NonZeroU32, execute: impl FnOnce(NonZeroU32, RingQueueIndex) -> Option<NonZeroU32>) -> Option<NonZeroU32>
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
	fn if_fill_queue_needs_wake_up_receive_poll(&'a self)
	{
		if self.fill_queue().needs_wake_up()
		{
			self.common().receive_poll()
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn fill_queue(&'a self) -> &'a FillQueue
	{
		&self.user_memory().fill_queue
	}
	
	#[doc(hidden)]
	fn lock_fill_queue(&self);
	
	#[doc(hidden)]
	fn unlock_fill_queue(&self);
	
	#[doc(hidden)]
	#[inline(always)]
	fn receive_queue_peek_execute_submit<RFP: ReceivedFrameProcessor>(&'a self, received_frame_processor: &mut RFP, execute: impl FnOnce(NonZeroU32, RingQueueIndex, &mut RFP) -> Option<NonZeroU32>) -> Option<NonZeroU32>
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
	fn receive_queue(&'a self) -> &'a ReceiveQueue
	{
		self.common().receive_queue()
	}
}
