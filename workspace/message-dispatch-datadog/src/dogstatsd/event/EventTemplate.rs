// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A DogStatsD event.
///
/// Implements `Message`.
#[derive(Debug)]
pub struct EventTemplate
{
	/// Name.
	pub title: Name,
	
	/// Tags.
	pub tags: DogStatsDTags,
	
	/// Defaults to recipient's idea of host.
	pub host_name: Option<&'static Label>,
	
	/// If omitted, defaults to `Normal`.
	pub priority: EventPriority,
	
	/// If omitted, defaults to `Information`.
	pub alert_type: EventAlertType,
	
	/// A key used for aggregating events.
	pub aggregation_key: Option<&'static ArrayString<100>>,
	
	/// Source type name.
	pub source_type_name: Option<SourceTypeName>,
}

impl EventTemplate
{
	/// A new alert with common tags.
	#[inline(always)]
	pub fn new_alert_with_common_tags(title: &str, priority: EventPriority, alert_type: EventAlertType, aggregation_key: &'static ArrayString<100>) -> Self
	{
		Self
		{
			title: Name::new(title).unwrap(),
			tags: DogStatsDTags::common_dog_stats_d_tags(),
			host_name: Some(Label::host_name()),
			priority,
			alert_type,
			aggregation_key: Some(aggregation_key),
			source_type_name: Some(SourceTypeName::ALERT),
		}
	}
	
	/// Creates an event message.
	///
	/// The message is escaped and truncated to 4000 characters.
	#[inline(always)]
	pub fn message<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>(&self, additional_tags: AdditionalDogStatsDTags<CoroutineHeapSize, GTACSA>, message: Arguments, global_allocator: &'static GTACSA) -> DogStatsDMessage<CoroutineHeapSize, GTACSA>
	{
		DogStatsDMessage::Event(self.with(additional_tags, message, global_allocator))
	}
	
	/// Creates an event.
	///
	/// The message is escaped and truncated to 4000 characters.
	#[inline(always)]
	pub fn with<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>(&self, additional_tags: AdditionalDogStatsDTags<CoroutineHeapSize, GTACSA>, message: Arguments, global_allocator: &'static GTACSA) -> Event<CoroutineHeapSize, GTACSA>
	{
		let timestamp = Some(SystemTime::now());
		
		Event
		{
			template: self,
			additional_tags,
			timestamp,
			message: Text::escape_for_event(global_allocator, message),
		}
	}
}
