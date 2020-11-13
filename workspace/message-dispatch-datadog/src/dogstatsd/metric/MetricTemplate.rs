// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A DogStatsD metric template, intended for use with `lazy_static!` initialization.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetricTemplate
{
	/// Name.
	pub name: Name,
	
	/// Tags.
	pub tags: DogStatsDTags,
}

impl MetricTemplate
{
	/// Creates a new metric template.
	#[inline(always)]
	pub fn new_with_common_tags(name: &str) -> Self
	{
		Self
		{
			name: Name::new(name).unwrap(),
			tags: DogStatsDTags::common_dog_stats_d_tags(),
		}
	}
	
	/// Creates a metric message.
	///
	/// The message is escaped and truncated to 4000 characters.
	#[inline(always)]
	pub fn message<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>(&self, additional_tags: AdditionalDogStatsDTags<CoroutineHeapSize, GTACSA>, metric_value: MetricValue) -> DogStatsDMessage<CoroutineHeapSize, GTACSA>
	{
		DogStatsDMessage::Metric(self.with(additional_tags, metric_value))
	}
	
	/// Creates a metric.
	#[inline(always)]
	pub fn with<CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>(&self, additional_tags: AdditionalDogStatsDTags<CoroutineHeapSize, GTACSA>, metric_value: MetricValue) -> Metric<CoroutineHeapSize, GTACSA>
	{
		Metric
		{
			template: self,
			additional_tags,
			metric_value,
		}
	}
}
