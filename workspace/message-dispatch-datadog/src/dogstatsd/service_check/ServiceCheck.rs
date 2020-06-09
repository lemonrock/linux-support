// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A DogStatsD service check.
pub struct ServiceCheck<'a, HeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>>
{
	template: &'a ServiceCheckTemplate<HeapSize, GTACSA>,
	
	status: ServiceCheckStatus,
	
	/// Defaults to UNIX epoch at recipient.
	timestamp: Option<SystemTime>,

	/// Additional information or a description of why the status occurred.
	///
	/// Empty messages are omitted and not transmitted.
	message: Text<HeapSize, GTACSA>,
}

impl<'a, HeapSize: MemorySize, GTACSA: 'static + GlobalThreadAndCoroutineSwitchableAllocator<HeapSize>> ServiceCheck<'a, HeapSize, GTACSA>
{
	/// Write to a buffer.
	///
	// `_sc|<NAME>|<STATUS>|d:<TIMESTAMP>|h:<HOSTNAME>|#<TAG_KEY_1>:<TAG_VALUE_1>,<TAG_2>|m:<SERVICE_CHECK_MESSAGE>`.
	#[inline(always)]
	pub fn dog_stats_d_write(&self, buffer: &mut [u8]) -> Result<usize, ()>
	{
		let mut dog_stats_d_writer = DogStatsDWriter::new(buffer);
		
		dog_stats_d_writer.write_bytes(b"_sc|")?;
		self.template.name.dog_stats_d_write(&mut dog_stats_d_writer)?;
		self.status.dog_stats_d_write(&mut dog_stats_d_writer)?;
		
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
		
		self.template.tags.dog_stats_d_write(&mut dog_stats_d_writer)?;
		
		if !self.message.is_empty()
		{
			dog_stats_d_writer.write_bytes(b"|m:")?;
			dog_stats_d_writer.write_bytes(&self.message)?;
		}
		
		dog_stats_d_writer.write_line_feed()?;
		Ok(dog_stats_d_writer.written_length())
	}
}
