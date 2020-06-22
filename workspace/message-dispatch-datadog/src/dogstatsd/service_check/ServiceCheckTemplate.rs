// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A DogStatsD service check template, intending for use with `lazy_static!` initialization.
#[derive(Debug)]
pub struct ServiceCheckTemplate
{
	/// Name.
	pub name: Name,
	
	/// Tags.
	pub tags: DogStatsDTags,
	
	/// Defaults to recipient's idea of host.
	pub host_name: Option<&'static Label>,
}

impl ServiceCheckTemplate
{
	/// Creates a service check message.
	///
	/// The message is escaped and truncated to 4000 characters.
	#[inline(always)]
	pub fn message<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>(&self, additional_tags: AdditionalDogStatsDTags<CoroutineHeapSize, GTACSA>, status: ServiceCheckStatus, message: Arguments, global_allocator: &'static GTACSA) -> DogStatsDMessage<CoroutineHeapSize, GTACSA>
	{
		DogStatsDMessage::ServiceCheck(self.with(additional_tags, status, message, global_allocator))
	}
	
	/// Creates a service check.
	///
	/// The message is escaped and truncated to 500 characters.
	#[inline(always)]
	pub fn with<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>(&self, additional_tags: AdditionalDogStatsDTags<CoroutineHeapSize, GTACSA>, status: ServiceCheckStatus, message: Arguments, global_allocator: &'static GTACSA) -> ServiceCheck<CoroutineHeapSize, GTACSA>
	{
		let timestamp = Some(SystemTime::now());
		
		ServiceCheck
		{
			template: self,
			additional_tags,
			status,
			timestamp,
			message: Text::escape_for_service_check(global_allocator, message),
		}
	}
}
