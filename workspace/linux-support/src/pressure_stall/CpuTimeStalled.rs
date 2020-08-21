// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// CPU time stalled.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CpuTimeStalled
{
	/// Indicates the share of time in which at least some tasks are stalled on a given resource.
	pub some: TimeStalled,
}

impl CpuTimeStalled
{
	/// The resultant file can be passed to `poll()`, `epoll()` or `select()`; it will be triggered if `maximum_total_stall_time_in_window` is exceeded.
	///
	/// `maximum_total_stall_time_in_window` must be less than or equal to `window`.
	pub fn monitor_some(file_path: &Path, maximum_total_stall_time_in_window: U64Microseconds, window: U64Microseconds) -> io::Result<File>
	{
		MemoryOrInputOutputTimeStalled::monitor_some(file_path, maximum_total_stall_time_in_window, window)
	}
	
	/// For current process.
	///
	/// Will not exist if the kernel is not configured for Pressure Stall Information (`CONFIG_PSI`).
	#[inline(always)]
	pub fn for_self(proc_path: &ProcPath) -> io::Result<Self>
	{
		Self::for_process(proc_path, ProcessIdentifierChoice::Current)
	}
	
	/// For process.
	///
	/// Will not exist if the kernel is not configured for Pressure Stall Information (`CONFIG_PSI`).
	#[inline(always)]
	pub fn for_process(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> io::Result<Self>
	{
		let file_path = proc_path.process_pressure_stall_information_file_path(process_identifier, "cpu");
		Self::from_file(&file_path)
	}
	
	/// Will not exist if the kernel is not configured for Pressure Stall Information (`CONFIG_PSI`).
	#[inline(always)]
	pub(crate) fn from_file(file_path: &Path) -> io::Result<Self>
	{
		#[inline(always)]
		fn store_field(time_stalled: TimeStalled, field: &mut Option<TimeStalled>) -> io::Result<()>
		{
			if field.is_some()
			{
				Err(io_error_invalid_data("duplicate field"))
			}
			else
			{
				*field = Some(time_stalled);
				Ok(())
			}
		}
		
		#[inline(always)]
		fn unwrap_field<T: FromBytes>(field: Option<T>) -> io::Result<T>
		{
			field.ok_or(io_error_invalid_data("Missing field"))
		}
		
		let bytes = file_path.read_raw()?;
		
		let mut some = None;
		for line in bytes.split_bytes(b'\n')
		{
			let (name, time_stalled) = TimeStalled::parse_line(line)?;
			match name
			{
				b"some" => store_field(time_stalled, &mut some)?,
				
				_ => continue,
			}
		}
		
		Ok
		(
			Self
			{
				some: unwrap_field(some)?,
			}
		)
	}
}
