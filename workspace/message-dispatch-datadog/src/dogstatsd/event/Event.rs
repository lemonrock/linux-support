// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A DogStatsD event.
#[derive(Debug)]
pub struct Event<'a, HeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>>
{
	template: &'a EventTemplate<HeapSize, GTACSA>,
	
	/// Defaults to UNIX epoch at recipient.
	timestamp: Option<SystemTime>,
	
	/// Limited to 4000 bytes.
	message: Text<HeapSize, GTACSA>,
}

impl<'a, HeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>> Event<'a, HeapSize, GTACSA>
{
	/// Write to a buffer.
	///
	// `_e{<TITLE>.length,<TEXT>.length}:<TITLE>|<TEXT>|d:<TIMESTAMP>|h:<HOSTNAME>|k:<AGGREGATION_KEY>|p:<PRIORITY>|s:<SOURCE_TYPE_NAME>|t:<ALERT_TYPE>|#<TAG_KEY_1>:<TAG_VALUE_1>,<TAG_2>`.
	#[inline(always)]
	pub fn dog_stats_d_write(&self, buffer: &mut [u8]) -> Result<usize, ()>
	{
		let mut dog_stats_d_writer = DogStatsDWriter::new(buffer);
		
		dog_stats_d_writer.write_bytes(b"_e|{")?;
		
		dog_stats_d_writer.write_usize(self.template.title.0.len())?;
		dog_stats_d_writer.write_comma()?;
		dog_stats_d_writer.write_usize(self.message.len())?;
		dog_stats_d_writer.write_bytes(b"}:")?;
		
		self.template.title.dog_stats_d_write(&mut dog_stats_d_writer)?;
		dog_stats_d_writer.write_pipe()?;
		dog_stats_d_writer.write_bytes(&self.message)?;
		
		if let Some(ref timestamp) = self.timestamp
		{
			dog_stats_d_writer.write_bytes(b"|d:")?;
			dog_stats_d_writer.write_system_time(timestamp)?;
		}
		
		if let Some(ref host_name) = self.template.host_name
		{
			dog_stats_d_writer.write_bytes(b"|h:")?;
			host_name.dog_stats_d_write(&mut dog_stats_d_writer)?;
		}
		
		if let Some(ref aggregation_key) = self.template.aggregation_key
		{
			dog_stats_d_writer.write_bytes(b"|k:")?;
			dog_stats_d_writer.write_array_string(aggregation_key)?;
		}
		
		if self.template.priority != EventPriority::default()
		{
			dog_stats_d_writer.write_bytes(b"|p:")?;
			dog_stats_d_writer.write_bytes(self.template.priority.to_bytes())?;
		}
		
		if let Some(source_type_name) = self.template.source_type_name
		{
			dog_stats_d_writer.write_bytes(b"|s:")?;
			dog_stats_d_writer.write_bytes(source_type_name.to_bytes())?;
		}
		
		if self.template.alert_type != EventAlertType::default()
		{
			dog_stats_d_writer.write_bytes(b"|t:")?;
			dog_stats_d_writer.write_bytes(self.template.alert_type.to_bytes())?;
		}
		
		self.template.tags.dog_stats_d_write(&mut dog_stats_d_writer)?;
		
		dog_stats_d_writer.write_line_feed()?;
		Ok(dog_stats_d_writer.written_length())
	}
}
