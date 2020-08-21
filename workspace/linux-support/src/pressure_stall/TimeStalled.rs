// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Time stalled.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeStalled
{
	/// Average time stalled over 10 seconds.
	pub average_over_10_seconds: TimeStalledPercentage,
	
	/// Average time stalled over 60 seconds.
	pub average_over_60_seconds: TimeStalledPercentage,
	
	/// Average time stalled over 300 seconds.
	pub average_over_300_seconds: TimeStalledPercentage,
	
	/// Total absolute stall time.
	pub total_absolute_stall_time: U64Microseconds,
}

impl FromBytes for TimeStalled
{
	type Error = io::Error;
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		#[inline(always)]
		fn parse_field<T: FromBytes<Error=ParseNumberError>>(field_value: &[u8], field: &mut Option<T>) -> io::Result<()>
		{
			if field.is_some()
			{
				Err(io_error_invalid_data("duplicate field"))
			}
			else
			{
				*field = Some(T::from_bytes(field_value).map_err(io_error_invalid_data)?);
				Ok(())
			}
		}
		
		#[inline(always)]
		fn unwrap_field<T: FromBytes>(field: Option<T>) -> io::Result<T>
		{
			field.ok_or(io_error_invalid_data("Missing field"))
		}
		
		let mut iterator = bytes.split_bytes(b' ');
		
		let mut average_over_10_seconds = None;
		let mut average_over_60_seconds = None;
		let mut average_over_300_seconds = None;
		let mut total_absolute_stall_time = None;
		
		for field in iterator
		{
			let index = memchr(b'=', field).ok_or(io_error_invalid_data("Missing `=`"))?;
			let field_name = &field[ .. index];
			let field_value = &field[(index + 1) .. ];
			match field_name
			{
				b"avg10" => parse_field(field_value, &mut average_over_10_seconds)?,
				
				b"avg60" => parse_field(field_value, &mut average_over_60_seconds)?,
				
				b"avg300" => parse_field(field_value, &mut average_over_300_seconds)?,
				
				b"total" => parse_field(field_value, &mut total_absolute_stall_time)?,
				
				_ => continue,
			}
		}
		
		Ok
		(
			Self
			{
				average_over_10_seconds: unwrap_field(average_over_10_seconds)?,
				average_over_60_seconds: unwrap_field(average_over_60_seconds)?,
				average_over_300_seconds: unwrap_field(average_over_300_seconds)?,
				total_absolute_stall_time: unwrap_field(total_absolute_stall_time)?,
			}
		)
	}
}


impl TimeStalled
{
	pub(crate) fn parse_line(line_without_line_feed: &[u8]) -> io::Result<(&[u8], Self)>
	{
		let index = memchr(b' ', line_without_line_feed).ok_or(io_error_invalid_data("Missing data"))?;
		let name = &line_without_line_feed[ .. index];
		let data = &line_without_line_feed[(index + 1) .. ];
		let time_stalled = Self::from_bytes(data)?;
		Ok((name, time_stalled))
	}
}
