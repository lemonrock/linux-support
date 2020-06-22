// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A drop guard.
#[repr(transparent)]
pub struct SimplePerThreadMemoryAllocatorInstantiatorDropGuard<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>(&'static GTACSA, PhantomData<CoroutineHeapSize>);

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> Drop for SimplePerThreadMemoryAllocatorInstantiatorDropGuard<CoroutineHeapSize, GTACSA>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.drop_thread_local_allocator()
	}
}

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> SimplePerThreadMemoryAllocatorInstantiatorDropGuard<CoroutineHeapSize, GTACSA>
{
	#[inline(always)]
	fn new(global_allocator: &'static GTACSA) -> Self
	{
		Self(global_allocator, PhantomData)
	}
}
