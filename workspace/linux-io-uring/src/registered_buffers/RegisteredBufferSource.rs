// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A registered buffer suitable for use.
#[derive(Debug)]
pub struct RegisteredBufferSource<T>
{
	memory: NonNull<T>,
	memory_queue: Rc<LargeRingQueue<T>>,
	registered_buffer_index: RegisteredBufferIndex,
}

impl<T> Drop for RegisteredBufferSource<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.memory_queue.relinquish(self.memory)
	}
}

impl<T> Deref for RegisteredBufferSource<T>
{
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		unsafe { & * self.memory.as_ptr() }
	}
}

impl<T> DerefMut for RegisteredBufferSource<T>
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		unsafe { &mut * self.memory.as_ptr() }
	}
}

impl<T> RegisteredBufferSource<T>
{
	#[inline(always)]
	pub fn new(memory_queue: &Rc<LargeRingQueue<T>>, registered_buffer_index: RegisteredBufferIndex) -> Option<Self>
	{
		match memory_queue.obtain()
		{
			None => None,
			
			Some(memory) =>
			{
				Some
				(
					Self
					{
						memory,
						memory_queue: memory_queue.clone(),
						registered_buffer_index,
					}
				)
			}
		}
	}
}
