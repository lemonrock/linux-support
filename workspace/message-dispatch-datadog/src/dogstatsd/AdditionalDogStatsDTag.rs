// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An atomically-allocated tag for tags not known statically but which are often long-lived.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AdditionalDogStatsDTag<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>(GloballyAllocated<Arc<DogStatsDTag>, CoroutineHeapSize, GTACSA>);

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> Deref for AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>
{
	type Target = DogStatsDTag;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> Clone for AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self(self.0.clone_arc())
	}
}

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> AdditionalDogStatsDTag<CoroutineHeapSize, GTACSA>
{
	/// From.
	#[inline(always)]
	pub fn from(tag: DogStatsDTag, global_allocator: &'static GTACSA) -> Self
	{
		Self
		(
			GloballyAllocated::allocate(global_allocator, ||
			{
				Arc::new(tag)
			})
		)
	}
}
