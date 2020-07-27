// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a RFC 3164 message template, which just needs a timestamp and message.
pub struct Rfc3164MessageTemplate
{
	before_timestamp: ArrayVec<[u8; 5]>,
	after_timestamp: Vec<u8>,
}

impl MessageTemplate for Rfc3164MessageTemplate
{
	#[inline(always)]
	fn format(&self, buffer: &mut [u8], timestamp: DateTime<Utc>, message: &str) -> (usize, bool)
	{
		let timestamp_month = match timestamp.month()
		{
			1 => "Jan",
			2 => "Feb",
			3 => "Mar",
			4 => "Apr",
			5 => "May",
			6 => "Jun",
			7 => "Jul",
			8 => "Aug",
			9 => "Sep",
			10 => "Oct",
			11 => "Nov",
			12 => "Dec",
			_ => unreachable!(),
		};
		
		// eg `Feb  5 17:32:18`.
		let timestamp_string = format!("{} {:2} {:02}:{:02}:{:02}", timestamp_month, timestamp.day(), timestamp.hour(), timestamp.minute(), timestamp.second());
		let timestamp_bytes = timestamp_string.as_bytes();
		
		let start_pointer = buffer.as_mut_ptr();
		let mut write_to = start_pointer;
		unsafe
		{
			let end_pointer = write_to.add(buffer.len());
			write_to = write_slice_unchecked(write_to, &self.before_timestamp[..], end_pointer);
			write_to = write_slice_unchecked(write_to, &timestamp_bytes[..], end_pointer);
			write_to = write_slice_unchecked(write_to, &self.after_timestamp[..], end_pointer);
			let (write_to, truncated) = write_message_with_line_feed_escaped_truncated(write_to, message, end_pointer);
			let written_length = (write_to as usize) - (start_pointer as usize);
			(written_length, truncated)
		}
	}
}

impl Rfc3164MessageTemplate
{
	/// Panics if `host_name` or `process_name` contains a space.
	pub fn new(facility: KnownFacility, severity: Severity, host_name: Option<&LinuxKernelHostName>, process_name: &ProcessName) -> Self
	{
		let before_timestamp =
		{
			let mut before_timestamp = ArrayVec::new();
			let string = format!("<{}>", PriorityValue::from_facility_and_severity(facility, severity).as_u8());
			before_timestamp.try_extend_from_slice(string.as_bytes()).unwrap();
			before_timestamp
		};
		
		let mut after_timestamp =
		{
			let mut after_timestamp = Vec::with_capacity(128);
			
			after_timestamp.push(b' ');
			
			if let Some(host_name) = host_name
			{
				assert!(!(&host_name[..]).contains(&b' '), "host_name contains a space");
				
				after_timestamp.extend_from_slice(&host_name[..]);
			}
			else
			{
				after_timestamp.extend_from_slice(b"unknown");
			}
			after_timestamp.push(b' ');
			
			let process_name: &[u8] = process_name.as_ref();
			
			assert!(!process_name.contains(&b' '), "process_name contains a space");
			after_timestamp.extend_from_slice(process_name);
			after_timestamp.push(b'[');
			after_timestamp.extend_from_slice(b"]: ");
			
			after_timestamp
		};
		
		after_timestamp.shrink_to_fit();
		
		Self
		{
			before_timestamp,
			after_timestamp,
		}
	}
}
