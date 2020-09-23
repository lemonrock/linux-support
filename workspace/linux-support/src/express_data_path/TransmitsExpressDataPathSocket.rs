// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Transmits.
pub trait TransmitsExpressDataPathSocket<ROTOB: ReceiveOrTransmitOrBoth + Transmits<CommonTransmitOnly>, FFQ: FreeFrameQueue>: ExpressDataPathSocket<ROTOB, FFQ>
{
	/// Send as part of a burst of frames with `transmit_only()`.
	/// `populate` takes `our_frame_headroom, space_for_ethernet_packet` and returns the actual size of the ethernet packet (which must be `<= space_for_ethernet_packet.len()`.
	fn populate_new_frame_to_transmit(&self, populate: impl FnOnce(&mut [u8], &mut [u8]) -> usize) -> Option<FrameReference<FFQ::CS>>
	{
		let frame_identifier = self.user_memory().pop_free_frame();
		frame_identifier.map(|frame_identifier|
		{
			let (our_frame_headroom, ethernet_packet) = self.user_memory().frame_to_transmit_our_frame_headroom_ethernet_packet(frame_identifier);
			let length_of_packet = populate(our_frame_headroom, ethernet_packet);
			debug_assert!(length_of_packet <= ethernet_packet.len());
			FrameReference
			{
				frame_identifier,
				length_of_packet
			}
		})
	}
	
	/// Frame data are (ethernet) frames (packets) held in user memory.
	///
	/// Do not submit more frames than the capacity of the `TransmitQueue`.
	fn transmit_only(&self, frames_to_transmit: &[FrameReference<FFQ::CS>])
	{
		const peek_release_completion_queue: bool = true;
		
		if unlikely!(frames_to_transmit.is_empty())
		{
			return
		}
		
		let number_of_frames_to_transmit = self.user_memory().number_of_frames(frames_to_transmit);
		
		debug_assert!(self.transmit_queue().number_of_frames_to_transmit_is_within_or_at_capacity(number_of_frames_to_transmit));
		
		self.transmit_queue_reserve_execute_submit(number_of_frames_to_transmit, peek_release_completion_queue, |number_of_frames_to_transmit, transmit_queue_index|
		{
			for relative_frame_index in 0 .. number_of_frames_to_transmit.get()
			{
				let frame_reference = unsafe { frames_to_transmit.get_unchecked(relative_frame_index as usize) };
				let transmit_frame_descriptor_bitfield = frame_reference.transmit_frame_descriptor_bitfield(self.user_memory());
				self.transmit_queue().set_transmit_descriptor_from_frame(transmit_queue_index, relative_frame_index, transmit_frame_descriptor_bitfield, frame_reference.length_of_packet)
			}
			Some(number_of_frames_to_transmit)
		});
		
		self.complete_transmit(peek_release_completion_queue)
	}
	
	/// Frames transmitted.
	#[inline(always)]
	fn frames_transmitted(&self) -> u64
	{
		self.common().frames_transmitted()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn transmit_queue_reserve_execute_submit(&self, expect_to_transmit_number_of_frames: NonZeroU32, peek_release_completion_queue: bool, execute: impl FnOnce(NonZeroU32, RingQueueIndex) -> Option<NonZeroU32>) -> Option<NonZeroU32>
	{
		let transmit_queue_index = loop
		{
			match self.transmit_queue().reserve(expect_to_transmit_number_of_frames)
			{
				Some(transmit_queue_index) => break transmit_queue_index,
				
				None =>
				{
					self.complete_transmit(peek_release_completion_queue);
				}
			}
		};
		
		match execute(expect_to_transmit_number_of_frames, transmit_queue_index)
		{
			None => None,
			
			Some(transmitted_number_of_frames) =>
			{
				self.transmit_queue().submit(transmitted_number_of_frames);
				self.common().increment_outstanding_frames_to_transmit(transmitted_number_of_frames);
				Some(transmitted_number_of_frames)
			}
		}
	}
	
	// On return, the best attempt possible to transmit all outstanding frames has been done.
	#[doc(hidden)]
	#[inline(always)]
	fn complete_transmit(&self, peek_release_completion_queue: bool)
	{
		let requested_number_of_frames = match self.common().outstanding_frames_to_transmit()
		{
			0 => return,
			non_zero => unsafe { NonZeroU32::new_unchecked(non_zero) }
		};
		
		self.initiate_transmit_processing_by_kernel_if_transmit_queue_needs_wake_up();
		
		if peek_release_completion_queue
		{
			self.completion_queue_lock_peek_execute_release_unlock(requested_number_of_frames, |available_number_of_frames, completion_queue_index|
			{
				for relative_frame_index in 0 .. available_number_of_frames.get()
				{
					let completed_frame_descriptor_bitfield = self.completion_queue().get_completed_frame_descriptor_bitfield(completion_queue_index, relative_frame_index);
					self.user_memory().push_free_frame_from_completion(completed_frame_descriptor_bitfield);
				}
				
				Some(available_number_of_frames)
			});
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn completion_queue_lock_peek_execute_release_unlock(&self, requested_number_of_frames: NonZeroU32, execute: impl FnOnce(NonZeroU32, RingQueueIndex) -> Option<NonZeroU32>) -> Option<NonZeroU32>
	{
		self.lock_completion_queue();
		
		if let Some((available_number_of_frames, completion_queue_index)) = self.completion_queue().peek(requested_number_of_frames)
		{
			match execute(available_number_of_frames, completion_queue_index)
			{
				None =>
				{
					self.unlock_completion_queue();
					None
				},
				
				Some(completed_number_of_frames) =>
				{
					self.completion_queue().release(completed_number_of_frames);
					self.unlock_completion_queue();
					self.common().change_frames_transmitted(completed_number_of_frames);
					Some(completed_number_of_frames)
				}
			}
		}
		else
		{
			self.unlock_completion_queue();
			None
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn completion_queue(&self) -> &CompletionQueue
	{
		&self.user_memory().completion_queue
	}
	
	#[doc(hidden)]
	fn lock_completion_queue(&self);
	
	#[doc(hidden)]
	fn unlock_completion_queue(&self);
	
	#[doc(hidden)]
	#[inline(always)]
	fn initiate_transmit_processing_by_kernel_if_transmit_queue_needs_wake_up(&self)
	{
		if self.transmit_queue().needs_wake_up()
		{
			self.express_data_path_socket_file_descriptor().initiate_transmit_processing_by_kernel()
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn transmit_queue(&self) -> &TransmitQueue
	{
		self.common().transmit_queue()
	}
}
