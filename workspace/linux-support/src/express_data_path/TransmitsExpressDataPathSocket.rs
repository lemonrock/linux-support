// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Transmits.
pub trait TransmitsExpressDataPathSocket<ROTOB: ReceiveOrTransmitOrBoth + Transmits<CommonTransmitOnly>, CA: ChunkAlignment>: ExpressDataPathSocket<ROTOB, CA>
{
	/// Frame data are (ethernet) frames (packets) held in user memory.
	fn transmit_only(&self, frames_to_transmit: &[AlignedFrameReference])
	{
		const peek_release_completion_queue: bool = true;
		
		if unlikely!(frames_to_transmit.is_empty())
		{
			return
		}
		
		let number_of_frames_to_transmit = self.user_memory().number_of_frames(frames_to_transmit);
		
		self.transmit_queue_reserve_execute_submit(number_of_frames_to_transmit, peek_release_completion_queue, |number_of_frames_to_transmit, transmit_queue_index|
		{
			for relative_frame_index in 0 .. number_of_frames_to_transmit.get()
			{
				let aligned_frame_reference = unsafe { frames_to_transmit.get_unchecked(relative_frame_index as usize) };
				self.transmit_queue().set_transmit_descriptor_from_aligned_frame_reference(transmit_queue_index, relative_frame_index, aligned_frame_reference, self.user_memory().chunk_size, self.user_memory().frame_headroom)
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
	fn transmit_queue_reserve_execute_submit(&self, expect_to_transmit_number_of_frames: NonZeroU32, peek_release_completion_queue: bool, execute: impl FnOnce(NonZeroU32, u32) -> Option<NonZeroU32>) -> Option<NonZeroU32>
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
					if CA::IsUnaligned
					{
						let relative_addresss_and_offsets = RelativeAddressesAndOffsets::from_completed_frame_descriptor_if_unaligned(completed_frame_descriptor_bitfield, self.user_memory().frame_headroom);
						
						let absolute_frame_index = relative_addresss_and_offsets.orig_addr / self.user_memory().chunk_size.to_u64();
						AlignedFrameNumber::from(absolute_frame_index as u32);
						
						self.user_memory().unused_frames_queue.push(aligned_frame_number);
					}
					else
					{
						RelativeAddressesAndOffsets::from_completed_frame_descriptor_if_aligned(completed_frame_descriptor_bitfield, self.user_memory().frame_headroom);
						
						// TODO push frames back onto the free queue.
						xxxx;
						
					};
					
				}
				
				Some(available_number_of_frames)
			});
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn completion_queue_lock_peek_execute_release_unlock(&self, requested_number_of_frames: NonZeroU32, execute: impl FnOnce(NonZeroU32, u32) -> Option<NonZeroU32>) -> Option<NonZeroU32>
	{
		self.lock_completion_queue();
		
		if let Some((available_number_of_frames, completion_queue_index)) = self.completion_queue().peek(requested_number_of_frames)
		{
			match execute(available_number_of_frames, completion_queue_index)
			{
				None =>
				{
					Self.unlock_completion_queue();
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
