// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A DogStatsD event.
///
/// Implements `Message`.
#[derive(Debug)]
pub struct EventTemplate<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>
{
	/// Name.
	pub title: Name,
	
	/// Tags.
	pub tags: DogStatsDTags,
	
	/// Defaults to recipient's idea of host.
	pub host_name: Option<Label>,
	
	/// Global allocator.
	pub global_allocator: &'static GTACSA,
	
	/// Pointless.
	pub global_allocator_marker: PhantomData<CoroutineHeapSize>,
	
	/// If omitted, defaults to `Normal`.
	pub priority: EventPriority,
	
	/// If omitted, defaults to `Information`.
	pub alert_type: EventAlertType,
	
	/// A key used for aggregating events.
	pub aggregation_key: Option<ArrayString<[u8; 100]>>,
	
	/// Source type name.
	pub source_type_name: Option<SourceTypeName>,
}

impl<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> EventTemplate<CoroutineHeapSize, GTACSA>
{
	/// Creates an event.
	///
	/// The message is escaped and truncated to 4000 characters.
	#[inline(always)]
	pub fn with(&self, message: Arguments) -> Event<CoroutineHeapSize, GTACSA>
	{
		let timestamp = Some(SystemTime::now());
		
		Event
		{
			template: self,
			timestamp,
			message: Text::escape_for_event(self.global_allocator, message),
		}
	}
}
