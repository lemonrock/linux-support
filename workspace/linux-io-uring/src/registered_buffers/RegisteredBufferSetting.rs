// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Registered buffer settings.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegisteredBufferSetting<BufferSize: MemorySize>
{
	/// How many buffers per registered buffer?
	pub number_of_buffers_per_registered_buffer: NonZeroU32,
	
	/// How many registered buffers?
	///
	/// Up to 1024 in total across all sizes are permitted.
	pub number_of_registered_buffers: u16,

	#[serde(skip)] marker: PhantomData<BufferSize>,
}

impl<BufferSize: MemorySize> RegisteredBufferSetting<BufferSize>
{
	const OneMegabyte: u64 = 1024 * 1024;
	
	const MaximumRegisteredBufferSize: u64 = 1024 * Self::OneMegabyte;
	
	const MaximumRegisteredBufferSizeNonZeroU32: NonZeroU32 = new_non_zero_u32(Self::MaximumRegisteredBufferSize as u32);
	
	/// Assumes a 1Gb maximum sized buffer.
	#[inline(always)]
	pub fn maximum_number_of_buffers_per_registered_buffer() -> NonZeroU32
	{
		Self::maximum_number_of_buffers_per_registered_buffer_of_size(Self::MaximumRegisteredBufferSizeNonZeroU32)
	}
	
	/// How many subdivisions?
	#[inline(always)]
	pub fn maximum_number_of_buffers_per_registered_buffer_of_size(maximum_registered_buffer_size: NonZeroU32) -> NonZeroU32
	{
		debug_assert!(maximum_registered_buffer_size <= Self::MaximumRegisteredBufferSizeNonZeroU32, "buffer_size is too large");
		
		let buffer_size: u32 = size_of::<BufferSize>().try_into().expect("too many subdivisions for this buffer_size");
		new_non_zero_u32(maximum_registered_buffer_size.get() / buffer_size)
	}
	
	/// No registered buffers.
	#[inline(always)]
	pub fn none() -> Self
	{
		Self
		{
			number_of_buffers_per_registered_buffer: new_non_zero_u32(1),
			number_of_registered_buffers: 0,
			marker: PhantomData,
		}
	}
	
	/// New instance.
	#[inline(always)]
	pub fn maximum_registered_buffer_size(maximum: NonZeroU32) -> Self
	{
		Self::new(Self::maximum_number_of_buffers_per_registered_buffer_of_size(maximum), new_non_zero_u16(1))
	}
	
	/// New instance.
	#[inline(always)]
	pub fn new(number_of_buffers_per_registered_buffer: NonZeroU32, number_of_registered_buffers: NonZeroU16) -> Self
	{
		let number_of_registered_buffers = number_of_registered_buffers.get();
		debug_assert!(number_of_registered_buffers <= 1024, "The maximum number of registered buffers is 1024; even close to this number prevents other buffer size combinations being registered");
		
		Self
		{
			number_of_buffers_per_registered_buffer,
			number_of_registered_buffers,
			marker: PhantomData,
		}
	}
	
	fn create_buffers(&self, buffers_count: &mut u16, defaults: &DefaultHugePageSizes) -> Result<Box<[RegisteredBuffer<BufferSize>]>, RegisteredBuffersCreationError>
	{
		let number_of_buffers = self.number_of_registered_buffers;
		let mut buffers = Vec::with_capacity(number_of_buffers as usize);
		for _index in 0 .. number_of_buffers
		{
			let count = *buffers_count;
			if count == RegisteredBufferIndex::ExclusiveMaximum.get()
			{
				return Err(RegisteredBuffersCreationError::TooManyBuffersNeedToBeCreated)
			}
			
			buffers.push(RegisteredBuffer
			{
				memory_queue: self.create_ring_queue(defaults)?,
				registered_buffer_index: RegisteredBufferIndex(count),
			});
			
			*buffers_count = count + 1;
		}
		
		Ok(buffers.into_boxed_slice())
	}
	
	#[inline(always)]
	fn create_ring_queue(&self, defaults: &DefaultHugePageSizes) -> Result<ReferenceCountedLargeRingQueue<BufferSize>, RegisteredBuffersCreationError>
	{
		let large_ring_queue = ReferenceCountedLargeRingQueue::new(self.ideal_maximum_number_of_elements(), defaults, Self::OneMegabyte, true)?;
		if unlikely!(large_ring_queue.size_in_bytes() > Self::MaximumRegisteredBufferSize)
		{
			Err(RegisteredBuffersCreationError::BufferSizeExceeded1GbMaximumSize)
		}
		else
		{
			Ok(large_ring_queue)
		}
	}
	
	#[inline(always)]
	fn ideal_maximum_number_of_elements(&self) -> NonZeroU64
	{
		new_non_zero_u64(self.number_of_buffers_per_registered_buffer.get() as u64)
	}
}
