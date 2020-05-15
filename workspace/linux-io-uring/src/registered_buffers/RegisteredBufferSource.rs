// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A registered buffer suitable for use.
#[derive(Debug)]
pub struct RegisteredBufferSource<BufferSize: Sized>
{
	element: ReferenceCountedLargeRingQueueElement<BufferSize>,
	registered_buffer_index: RegisteredBufferIndex,
}

impl<BufferSize: Sized> Deref for RegisteredBufferSource<BufferSize>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		unsafe { & * self.memory.as_ptr() }
	}
}

impl<BufferSize: Sized> DerefMut for RegisteredBufferSource<BufferSize>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { &mut * self.memory.as_ptr() }
	}
}

impl<BufferSize: Sized> RegisteredBufferSource<BufferSize>
{
}
