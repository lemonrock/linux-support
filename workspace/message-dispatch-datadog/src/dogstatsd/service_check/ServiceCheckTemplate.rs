// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A DogStatsD service check template, intending for use with `lazy_static!` initialization.
#[derive(Debug)]
pub struct ServiceCheckTemplate<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>
{
	/// Name.
	pub name: Name,
	
	/// Tags.
	pub tags: DogStatsDTags,
	
	/// Defaults to recipient's idea of host.
	pub host_name: Option<Label>,
	
	/// Global allocator.
	pub global_allocator: &'static GTACSA,
	
	/// Pointless.
	pub global_allocator_marker: PhantomData<CoroutineHeapSize>,
}

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> ServiceCheckTemplate<CoroutineHeapSize, GTACSA>
{
	/// Creates a service check.
	///
	/// The message is escaped and truncated to 500 characters.
	#[inline(always)]
	pub fn with(&self, status: ServiceCheckStatus, message: Arguments) -> ServiceCheck<CoroutineHeapSize, GTACSA>
	{
		let timestamp = Some(SystemTime::now());
		
		ServiceCheck
		{
			template: self,
			status,
			timestamp,
			message: Text::escape_for_service_check(self.global_allocator, message),
		}
	}
}
