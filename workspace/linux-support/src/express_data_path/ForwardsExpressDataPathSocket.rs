// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Forwards.
pub trait ForwardsExpressDataPathSocket<ROTOB: ReceiveOrTransmitOrBoth + Receives<CommonReceiveOnly<RP>> + Transmits<CommonTransmitOnly>, CA: ChunkAlignment, RP: ReceivePoll>: ReceivesExpressDataPathSocket<ROTOB, CA> + TransmitsExpressDataPathSocket<ROTOB, CA>
{
	fn forward<RFP: ReceivedFrameProcessor<ProcessingOutcome=()>>(&self, received_frame_processor: &mut RFP)
	{
		const DoNotPeekReleaseCompletionQueueAsNeedCompletionAddressesIn_complete_forward: bool = false;
		
		self.receive_queue_peek_execute_submit
		(
			received_frame_processor,
			|received_number_of_frames, receive_queue_index, received_frame_processor|
			{
				self.transmit_queue_reserve_execute_submit
				(
					received_number_of_frames,
					DoNotPeekReleaseCompletionQueueAsNeedCompletionAddressesIn_complete_forward,
					|expect_to_transmit_number_of_frames, transmit_queue_index|
					{
						received_frame_processor.begin(received_number_of_frames.get());
						{
							for relative_frame_index in 0..received_number_of_frames
							{
								let receive_descriptor = self.receive_queue().get_receive_descriptor(receive_queue_index, relative_frame_index);
								
								let (headroom, received_frame) = self.user_memory().frame_from_descriptor(&receive_descriptor);
								received_frame_processor.process_received_frame(relative_frame_index, received_frame);
								
								self.transmit_queue().set_transmit_descriptor_from_receive_descriptor(transmit_queue_index, relative_frame_index, receive_descriptor);
							}
						}
						received_frame_processor.end();
						Some(expect_to_transmit_number_of_frames)
					}
				)
			}
		);
		
		self.complete_forward()
	}
	
	fn complete_forward(&self)
	{
		let requested_number_of_frames = match self.common().outstanding_frames_to_transmit()
		{
			0 => return,
			
			non_zero => unsafe { NonZeroU32::new_unchecked(non_zero) }
		};
		
		self.initiate_transmit_processing_by_kernel_if_transmit_queue_needs_wake_up();
		
		self.completion_queue_lock_peek_execute_release_unlock(requested_number_of_frames, |available_number_of_frames, completion_queue_index|
		{
			self.fill_queue_lock_reserve_execute_submit_unlock
			(
				available_number_of_frames,
				|reserved_number_of_frames, fill_queue_index|
				{
					for relative_frame_index in 0 .. reserved_number_of_frames
					{
						let relative_address_of_frame_in_user_memory = self.completion_queue().get_completed_frame_descriptor_bitfield(completion_queue_index, relative_frame_index);
						
						// Gift the frame back to the kernel.
						self.fill_queue().set_fill_user_memory_descriptor_of_frame_in_user_memory(fill_queue_index, relative_frame_index, relative_address_of_frame_in_user_memory);
					}
					Some(reserved_number_of_frames)
				}
			)
		});
	}
}
