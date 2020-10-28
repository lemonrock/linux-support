// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Text.
#[derive(Debug)]
pub struct Text<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>(GloballyAllocated<Vec<u8>, CoroutineHeapSize, GTACSA>);

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> Deref for Text<CoroutineHeapSize, GTACSA>
{
	type Target = Vec<u8>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> Text<CoroutineHeapSize, GTACSA>
{
	/// The message is escaped and truncated to 500 characters.
	#[inline(always)]
	fn escape_for_service_check(global_allocator: &'static GTACSA, message: Arguments) -> Self
	{
		Self::new(global_allocator, message, 4000, |mut message|
		{
			loop
			{
				let mut start_next_slice_index = 0;
				
				const LineFeed: u8 = b'\n';
				const Colon: u8 = b':';
				const m: u8 = b'm';
				let bytes = &message[start_next_slice_index..];
				match memchr2(LineFeed, Colon, bytes)
				{
					None => break message,
					
					Some(index) => match unsafe { * bytes.get_unchecked(index) }
					{
						LineFeed => Self::replace(&mut message, &mut start_next_slice_index, index, b'n'),
						
						Colon => if index == 0
						{
							Self::no_replacement(&mut start_next_slice_index, index)
						}
						else
						{
							let previous_byte = unsafe { * bytes.get_unchecked(index - 1) };
							if unlikely!(previous_byte == m)
							{
								Self::replace(&mut message, &mut start_next_slice_index, index, Colon)
							}
							else
							{
								Self::no_replacement(&mut start_next_slice_index, index)
							}
						}
						
						_ => unreachable_code(format_args!("")),
					}
				}
			}
		})
	}
	
	/// The message is escaped and truncated to 4000 characters.
	#[inline(always)]
	fn escape_for_event(global_allocator: &'static GTACSA, message: Arguments) -> Self
	{
		Self::new(global_allocator, message, 4000, |mut message|
		{
			loop
			{
				let mut start_next_slice_index = 0;
				
				const LineFeed: u8 = b'\n';
				let bytes = &message[start_next_slice_index..];
				match memchr(LineFeed, bytes)
				{
					None => break message,
					
					Some(index) => match unsafe { * bytes.get_unchecked(index) }
					{
						LineFeed => Self::replace(&mut message, &mut start_next_slice_index, index, b'n'),
						
						_ => unreachable_code(format_args!("")),
					}
				}
			}
		})
	}
	
	#[inline(always)]
	fn new<F: FnOnce(Vec<u8>) -> Vec<u8> + UnwindSafe>(global_allocator: &'static GTACSA, message: Arguments, maximum_size: usize, escape: F) -> Self
	{
		Self
		(
			GloballyAllocated::allocate
			(
				global_allocator, AssertUnwindSafe(||
				{
					let message = Self::unescaped_length_capped_message(message, maximum_size);
					let escaped_message = escape(message);
					Self::escaped_truncated_message(escaped_message, maximum_size)
				})
			)
		)
	}
	
	#[inline(always)]
	fn escaped_truncated_message(mut message: Vec<u8>, maximum_size: usize) -> Vec<u8>
	{
		// Second truncation to actual size.
		message.truncate(maximum_size);
		message
	}
	
	#[inline(always)]
	fn unescaped_length_capped_message(message: Arguments, maximum_size: usize) -> Vec<u8>
	{
		let mut message = format(message).into_bytes();
		
		// initial truncation to cap loop time.
		message.truncate(maximum_size);
		
		message
	}

	#[inline(always)]
	fn no_replacement(start_next_slice_index: &mut usize, index: usize)
	{
		*start_next_slice_index = index + 1
	}
	
	#[inline(always)]
	fn escaped(start_next_slice_index: &mut usize, index: usize)
	{
		*start_next_slice_index = index + 2
	}
	
	#[inline(always)]
	fn replace(message: &mut Vec<u8>, start_next_slice_index: &mut usize, index: usize, byte: u8)
	{
		unsafe { *message.get_unchecked_mut(index) = b'\\' };
		message.insert(index + 1, byte);
		Self::escaped(start_next_slice_index, index)
	}
}
