// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegisteredBufferSetting<BufferSize: Sized>
{
	pub number_of_subdivisions_per_buffer: NonZeroU32,
	
	pub number_of_buffers: NonZeroU16,

	marker: PhantomData<BufferSize>,
}

impl<BufferSize: Sized> RegisteredBufferSetting<BufferSize>
{
	fn create_buffers(&self, buffers_count: &mut u16) -> Result<Box<[RegisteredBuffer<BufferSize>]>, RegisteredBuffersCreationError>
	{
		let number_of_buffers = self.number_of_buffers.get();
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
				memory_queue: Rc::new(Self::create_ring_queue(subdivision, setting.number_of_subdivisions_per_buffer, defaults)?),
				registered_buffer_index: RegisteredBufferIndex(count),
			})
			
			*buffers_count = count + 1;
		}
		
		Ok(buffers.into_boxed_slice())
	}
	
	#[inline(always)]
	fn create_ring_queue(subdivision_size: u64, number_of_subdivisions_per_buffer: NonZeroU32, defaults: &DefaultPageSizeAndHugePageSizes) -> Result<LargeRingQueue<BufferSize>, RegisteredBuffersCreationError>
	{
		const OneMegabyte: u64 = 1024 * 1024;
		const MaximumBufferSize: u64 = 1024 * OneMegabyte;
		
		let large_ring_queue = LargeRingQueue::new(number_of_subdivisions_per_buffer, defaults, OneMegabyte, true)?;
		if unlikely!(large_ring_queue.size_in_bytes() > MaximumBufferSize)
		{
			Err(RegisteredBuffersCreationError::BufferSizeExceeded1GbMaximumSize)
		}
		else
		{
			Ok(large_ring_queue)
		}
	}
}