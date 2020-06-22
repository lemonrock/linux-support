// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A DogStatsD metric.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Metric<'a, CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>>
{
	template: &'a MetricTemplate,
	
	additional_tags: AdditionalDogStatsDTags<CoroutineHeapSize, GTACSA>,

	metric_value: MetricValue,
}

impl<'a, CoroutineHeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<CoroutineHeapSize>> Metric<'a, CoroutineHeapSize, GTACSA>
{
	/// Write to a buffer.
	///
	/// `<METRIC_NAME>:<VALUE>|<TYPE>|@<SAMPLE_RATE>|#<TAG_KEY_1>:<TAG_VALUE_1>,<TAG_2>`.
	#[inline(always)]
	pub fn dog_stats_d_write(&self, buffer: &mut [u8]) -> Result<usize, ()>
	{
		let mut dog_stats_d_writer = DogStatsDWriter::new(buffer);
		
		self.template.name.dog_stats_d_write(&mut dog_stats_d_writer)?;
		dog_stats_d_writer.write_colon()?;
		self.metric_value.dog_stats_d_write(&mut dog_stats_d_writer)?;
		
		self.additional_tags.dog_stats_d_write(&mut dog_stats_d_writer, &self.template.tags)?;
		
		dog_stats_d_writer.write_line_feed()?;
		Ok(dog_stats_d_writer.written_length())
	}
}
