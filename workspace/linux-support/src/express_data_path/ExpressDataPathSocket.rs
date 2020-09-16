// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Access common fields.
pub trait ExpressDataPathSocket<ReceiveTransmit: ReceiveOrTransmitOrBoth<ReceiveQueue, TransmitQueue>, RP: ReceivePoll, CA: ChunkAlignment>
{
	/// `maximum_number_of_frames` is usually `64`.
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
		
		let fill_queue_index = self.empty_fill_queue_until_sufficient_space_is_available(received_number_of_frames);
		
		received_frame_processor.begin(received_number_of_frames);
		for relative_frame_index in 0 .. received_number_of_frames
		{
			let descriptor = receive_queue.receive_descriptor(receive_index + relative_frame_index);
			let (headroom, received_frame) = user_memory.frame_from_descriptor(&descriptor);
			
			received_frame_processor.process_received_frame(relative_frame_index, received_frame);
			
			fill_queue.set_fill_address(fill_queue_index, relative_frame_index, CA::user_memory_area_relative_address(user_memory.chunk_size(), descriptor));
		}
		let result = received_frame_processor.end();
		
		fill_queue.submit(received_number_of_frames);
		receive_queue.release(received_number_of_frames);
		
		common.increase_frames_received(received_number_of_frames);
		
		result
	}
	
	/// Frame data are (ethernet) frames (packets) held in user memory.
	///
	/// Frames in user memory do not include a trailing ethernet frame check sequeunce (FCS).
	fn transmit_only(&self, frames: &[FrameReference])
	{
		let user_memory = self.user_memory();
		let common = self.common();
		let transmit_queue = self.transmit_queue();
		
		let number_of_frames = user_memory.number_of_frames(frames);
		
		let mut transmit_queue_index_to_start_enqueuing_at = self.empty_transmit_queue_until_sufficient_space_is_available(number_of_frames);
		
		for frame_reference in frames
		{
			let transmit_descriptor = transmit_queue.transmit_descriptor(transmit_queue_index_to_start_enqueuing_at);
			transmit_queue_index_to_start_enqueuing_at += 1;
			
			xdp_desc::write_frame_reference(transmit_descriptor, frame_reference, user_memory);
		}
		
		transmit_queue.submit(number_of_frames);
		
		common.increment_outstanding_frames_to_transmit(number_of_frames);
		self.complete_transmit(number_of_frames)
	}
	
	/// Frames received and transmitted.
	#[inline(always)]
	fn frames_received_and_transmitted(&self) -> (u64, u64)
	{
		self.common().frames_received_and_transmitted()
	}
	
	/// Statistics.
	#[inline(always)]
	fn statistics(&self) -> xdp_statistics
	{
		self.express_data_path_socket_file_descriptor().statistics()
	}
	
	/// Options.
	#[inline(always)]
	fn options(&self) -> xdp_options
	{
		self.express_data_path_socket_file_descriptor().options()
	}
	
	#[doc(hidden)]
	fn user_memory(&self) -> &UserMemory<CA>;
	
	#[doc(hidden)]
	fn common(&self) -> &CommonSharedExpressDataPathSocket<ReceiveTransmit, RP>;
	
	#[doc(hidden)]
	fn express_data_path_socket_file_descriptor(&self) -> &ExpressDataPathSocketFileDescriptor;
	
	#[doc(hidden)]
	#[inline(always)]
	fn fill_queue(&self) -> &FillQueue
	{
		&self.user_memory().fill_queue
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn completion_queue(&self) -> &CompletionQueue
	{
		&self.user_memory().completion_queue
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn receive_queue(&self) -> &ReceiveQueue
	{
		self.common().receive_queue()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn transmit_queue(&self) -> &TransmitQueue
	{
		self.common().transmit_queue()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn empty_transmit_queue_until_sufficient_space_is_available(&self, to_transmit_number_of_frames: u32) -> u32
	{
		let transmit_queue = self.transmit_queue();
		loop
		{
			match transmit_queue.reserve(to_transmit_number_of_frames)
			{
				Some(transmit_queue_index_to_start_enqueuing_at) => return transmit_queue_index_to_start_enqueuing_at,
				
				None =>
				{
					self.complete_transmit(to_transmit_number_of_frames)
				}
			}
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn empty_fill_queue_until_sufficient_space_is_available(&self, received_number_of_frames: u32) -> u32
	{
		let fill_queue = self.fill_queue();
		loop
		{
			if let Some(fill_queue_index) = fill_queue.reserve(received_number_of_frames)
			{
				return fill_queue_index
			}
			
			if fill_queue.needs_wake_up()
			{
				common.receive_poll()
			}
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn complete_transmit(&self, number_of_frames: u32)
	{
		let common = self.common();
		let user_memory = self.user_memory();
		
		if common.have_no_outstanding_frames_to_transmit()
		{
			return
		}
		
		if !common.needs_wake_up || self.transmit_queue().needs_wake_up()
		{
			self.express_data_path_socket_file_descriptor().initiate_transmit_processing_by_kernel()
		}
		
		let completion_queue = self.completion_queue();
		
		if let Some((received, index)) = completion_queue.peek(number_of_frames)
		{
			let number_of_frames_received = received.get();
			completion_queue.release(number_of_frames_received);
			
			common.decrement_outstanding_frames_to_transmit(number_of_frames_received);
			common.increase_frames_transmitted(number_of_frames_received);
		}
	}
}
