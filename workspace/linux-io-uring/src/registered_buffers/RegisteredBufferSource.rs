// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A registered buffer suitable for use.
#[derive(Debug)]
pub struct RegisteredBufferSource<BufferSize: MemorySize>
{
	element: ReferenceCountedLargeRingQueueElement<BufferSize>,
	registered_buffer_index: RegisteredBufferIndex,
}

impl<BufferSize: MemorySize> Deref for RegisteredBufferSource<BufferSize>
{
	type Target = [u8];
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		unsafe
		{
			let element = self.element.element();
			from_raw_parts(element.as_ptr() as *const BufferSize as *const u8, size_of::<BufferSize>())
		}
	}
}

impl<BufferSize: MemorySize> DerefMut for RegisteredBufferSource<BufferSize>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe
		{
			let element = self.element.element();
			from_raw_parts_mut(element.as_ptr() as *mut u8, size_of::<BufferSize>())
		}
	}
}
