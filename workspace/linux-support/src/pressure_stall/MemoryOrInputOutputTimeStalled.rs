// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Memory or Input-Output (IO) time stalled.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct MemoryOrInputOutputTimeStalled
{
	/// Indicates the share of time in which at least some tasks are stalled on a given resource.
	pub some: TimeStalled,
	
	/// Indicates the share of time in which all non-idle tasks are stalled on a given resource simultaneously.
	pub full: TimeStalled,
}

impl MemoryOrInputOutputTimeStalled
{
	/// The resultant file can be passed to `poll()`, `epoll()` or `select()`; it will be triggered if `maximum_total_stall_time_in_window` is exceeded.
	///
	/// `maximum_total_stall_time_in_window` must be less than or equal to `window`.
	pub fn monitor_some(file_path: &Path, maximum_total_stall_time_in_window: U64Microseconds, window: U64Microseconds) -> io::Result<File>
	{
		Self::monitor(file_path, "some", maximum_total_stall_time_in_window, window)
	}
	
	/// The resultant file can be passed to `poll()`, `epoll()` or `select()`; it will be triggered if `maximum_total_stall_time_in_window` is exceeded.
	///
	/// `maximum_total_stall_time_in_window` must be less than or equal to `window`.
	pub fn monitor_all(file_path: &Path, maximum_total_stall_time_in_window: U64Microseconds, window: U64Microseconds) -> io::Result<File>
	{
		Self::monitor(file_path, "all", maximum_total_stall_time_in_window, window)
	}
	
	/// The resultant file can be passed to `poll()`, `epoll()` or `select()`; it will be triggered if `maximum_total_stall_time_in_window` is exceeded.
	///
	/// `maximum_total_stall_time_in_window` must be less than or equal to `window`.
	fn monitor(file_path: &Path, name: &str, maximum_total_stall_time_in_window: U64Microseconds, window: U64Microseconds) -> io::Result<File>
	{
		assert!(maximum_total_stall_time_in_window <= window);
		
		let mut file = OpenOptions::new().read(true).write(true).open(file_path)?;
		file.write_all(format!("{} {} {}\n", name, maximum_total_stall_time_in_window.0, window.0).as_ref())?;
		Ok(file)
	}
	
	/// For current process.
	///
	/// Will not exist if the kernel is not configured for Pressure Stall Information (`CONFIG_PSI`).
	#[inline(always)]
	pub fn memory_for_self(proc_path: &ProcPath) -> io::Result<Self>
	{
		Self::memory_for_process(proc_path, ProcessIdentifierChoice::Current)
	}
	
	/// For process.
	///
	/// Will not exist if the kernel is not configured for Pressure Stall Information (`CONFIG_PSI`).
	#[inline(always)]
	pub fn memory_for_process(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> io::Result<Self>
	{
		let file_path = proc_path.process_pressure_stall_information_file_path(process_identifier, "memory");
		Self::from_file(&file_path)
	}
	
	/// For current process.
	///
	/// Will not exist if the kernel is not configured for Pressure Stall Information (`CONFIG_PSI`).
	#[inline(always)]
	pub fn input_output_for_self(proc_path: &ProcPath) -> io::Result<Self>
	{
		Self::input_output_for_process(proc_path, ProcessIdentifierChoice::Current)
	}
	
	/// For process.
	///
	/// Will not exist if the kernel is not configured for Pressure Stall Information (`CONFIG_PSI`).
	#[inline(always)]
	pub fn input_output_for_process(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> io::Result<Self>
	{
		let file_path = proc_path.process_pressure_stall_information_file_path(process_identifier, "io");
		Self::from_file(&file_path)
	}
	
	#[inline(always)]
	fn for_process(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice, file_name: &str) -> io::Result<Self>
	{
		let file_path = proc_path.process_pressure_stall_information_file_path(process_identifier, file_name);
		Self::from_file(&file_path)
	}
	
	/// Will not exist if the kernel is not configured for Pressure Stall Information (`CONFIG_PSI`).
	#[inline(always)]
	pub(crate) fn from_file(file_path: &impl AsRef<Path>) -> io::Result<Self>
	{
		#[inline(always)]
		fn store_field(time_stalled: TimeStalled, field: &mut Option<TimeStalled>) -> io::Result<()>
		{
			if field.is_some()
			{
				Err(io::Error::new(ErrorKind::InvalidData, "duplicate field"))
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
			field.ok_or(io::Error::new(ErrorKind::InvalidData, "Missing field"))
		}
		
		let bytes = file_path.as_ref().read_raw()?;
		
		let mut some = None;
		let mut full = None;
		for line in bytes.split_bytes(b'\n')
		{
			let (name, time_stalled) = TimeStalled::parse_line(line)?;
			match name
			{
				b"some" => store_field(time_stalled, &mut some)?,
				
				b"full" => store_field(time_stalled, &mut full)?,
				
				_ => continue,
			}
		}
		
		Ok
		(
			Self
			{
				some: unwrap_field(some)?,
				full: unwrap_field(full)?,
			}
		)
	}
}
