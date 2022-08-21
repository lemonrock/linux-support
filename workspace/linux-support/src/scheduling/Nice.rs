// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represent a `nice` value.
///
/// For setting the current thread niceness, use the more modern `Scheduler`.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(i32)]
pub enum Nice
{
	/// Least priority.
	Positive_19 = 19,

	Positive_18 = 18,

	Positive_17 = 17,

	Positive_16 = 16,

	Positive_15 = 15,

	Positive_14 = 14,

	Positive_13 = 13,

	Positive_12 = 12,

	Positive_11 = 11,

	Positive_10 = 10,

	Positive_9 = 9,

	Positive_8 = 8,

	Positive_7 = 7,

	Positive_6 = 6,

	Positive_5 = 5,

	Positive_4 = 4,

	Positive_3 = 3,

	Positive_2 = 2,

	Positive_1 = 1,

	Zero = 0,

	Negative_1 = -1,

	Negative_2 = -2,

	Negative_3 = -3,

	Negative_4 = -4,

	Negative_5 = -5,

	Negative_6 = -6,

	Negative_7 = -7,

	Negative_8 = -8,

	Negative_9 = -9,

	Negative_10 = -10,

	Negative_11 = -11,

	Negative_12 = -12,

	Negative_13 = -13,

	Negative_14 = -14,

	Negative_15 = -15,

	Negative_16 = -16,

	Negative_17 = -17,

	Negative_18 = -18,

	Negative_19 = -19,

	/// Highest priority.
	Negative_20 = -20,
}

impl Display for Nice
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", *self as i32)
	}
}

impl Default for Nice
{
	#[inline(always)]
	fn default() -> Self
	{
		Nice::Default
	}
}

impl Into<i8> for Nice
{
	#[inline(always)]
	fn into(self) -> i8
	{
		self as i32 as i8
	}
}

impl Into<i16> for Nice
{
	#[inline(always)]
	fn into(self) -> i16
	{
		self as i32 as i16
	}
}

impl Into<i32> for Nice
{
	#[inline(always)]
	fn into(self) -> i32
	{
		self as i32
	}
}

impl Into<i64> for Nice
{
	#[inline(always)]
	fn into(self) -> i64
	{
		self as i32 as i64
	}
}

impl Into<isize> for Nice
{
	#[inline(always)]
	fn into(self) -> isize
	{
		self as i32 as isize
	}
}

impl ParseNumber for Nice
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		use self::ParseNumberError::*;

		let value = i32::parse_number(bytes, radix, parse_byte)?;
		if unlikely!(value < Self::InclusiveMinimum)
		{
			Err(TooSmall)
		}
		else if unlikely!(value > Self::InclusiveMaximum)
		{
			Err(TooLarge)
		}
		else
		{
			Ok(unsafe { transmute(value) })
		}
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for Nice
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		UnpaddedDecimalInteger(self as i32).into_line_feed_terminated_byte_string()
	}
}

impl Nice
{
	/// Default.
	pub const Default: Self = Nice::Negative_20;
	
	pub(super) const InclusiveMinimum: i32 = -20;

	pub(super) const InclusiveMaximum: i32 = 19;

	/// Set the autogroup for the current process.
	#[inline(always)]
	pub fn set_autogroup_for_current_process(self, proc_path: &ProcPath) -> Result<(), io::Error>
	{
		self.set_autogroup_for_process(ProcessIdentifierChoice::Current, proc_path)
	}

	/// Set the autogroup for a process.
	#[inline(always)]
	pub fn set_autogroup_for_process(self, process_identifier: ProcessIdentifierChoice, proc_path: &ProcPath) -> Result<(), io::Error>
	{
		// Reads are different to writes!
		// A read might contain the value `/autogroup-25 nice 0`.
		proc_path.process_file_path(process_identifier, "autogroup").write_value(UnpaddedDecimalInteger(self as i32))
	}
	
	/// Get the autogroup for a process.
	///
	/// Name might be `/autogroup-25`.
	#[inline(always)]
	pub fn get_autogroup_name_and_nice_value(process_identifier: ProcessIdentifierChoice, proc_path: &ProcPath) -> io::Result<(Box<[u8]>, Self)>
	{
		// Reads are different to writes!
		// A read might contain the value `/autogroup-25 nice 0`.
		let line = proc_path.process_file_path(process_identifier, "autogroup").read_raw_without_line_feed()?;
		let mut parts = line.split_bytes_n(3, b' ');
		
		let name = parts.next().unwrap();
		
		let nice_string = parts.next().ok_or(io_error_other("Missing `nice` string"))?;
		if nice_string != b"nice"
		{
			return Err(io_error_other("`nice` string was not 'nice'"))
		}
		
		let raw_nice_value = parts.next().ok_or(io_error_other("Missing `nice` value"))?;
		
		let nice = Self::parse_decimal_number(raw_nice_value).map_err(io_error_other)?;
		
		Ok((name.to_vec().into_boxed_slice(), nice))
	}

	/// Returns an `Err()` if the user did not have permission to adjust the priority (eg was not privileged or had the capability `CAP_SYS_NICE`).
	#[inline(always)]
	pub fn set_process_group_priority(self, process_group_identifier: ProcessGroupIdentifierChoice) -> Result<(), ()>
	{
		let pgid: pid_t = process_group_identifier.into();
		self.set_priority(PRIO_PGRP, pgid as u32)
	}

	/// Returns an `Err()` if the user did not have permission to adjust the priority (eg was not privileged or had the capability `CAP_SYS_NICE`).
	#[inline(always)]
	pub fn set_process_priority(self, process_identifier: ProcessIdentifierChoice) -> Result<(), ()>
	{
		let pid: pid_t = process_identifier.into();
		self.set_priority(PRIO_PROCESS, pid as u32)
	}

	/// Returns an `Err()` if the user did not have permission to adjust the priority (eg was not privileged or had the capability `CAP_SYS_NICE`).
	#[inline(always)]
	pub fn set_thread_priority(self, thread_identifier: ThreadIdentifier) -> Result<(), ()>
	{
		let tid: pid_t = thread_identifier.into();
		self.set_priority(PRIO_PROCESS, tid as u32)
	}

	/// Returns an `Err()` if the user did not have permission to adjust the priority (eg was not privileged or had the capability `CAP_SYS_NICE`).
	#[inline(always)]
	pub fn set_real_user_priority(self, user_identifier: UserIdentifier) -> Result<(), ()>
	{
		let uid: uid_t = user_identifier.into();
		self.set_priority(PRIO_USER, uid)
	}

	#[inline(always)]
	fn set_priority(self, which: i32, who: u32) -> Result<(), ()>
	{
		let result = unsafe { setpriority(which, who, self as i32) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno_panic()
			{
				ESRCH => Err(()),
				
				EACCES | EPERM => panic!("Permission denied"),
				
				EINVAL => panic!("`which` was not one of `PRIO_PROCESS`, `PRIO_PGRP`, or `PRIO_USER`"),

				unexpected_error @ _ => unexpected_error!(setpriority, unexpected_error),
			}
		}
		else
		{
			unexpected_result!(setpriority, result)
		}
	}
	
	/// Returns an `Err()` if the process group does not exist.
	#[inline(always)]
	pub fn get_process_group_priority(process_group_identifier: ProcessGroupIdentifierChoice) -> Result<Self, ()>
	{
		let pgid: pid_t = process_group_identifier.into();
		Self::get_priority(PRIO_PGRP, pgid as u32)
	}
	
	/// Returns an `Err()` if the process does not exist.
	#[inline(always)]
	pub fn get_process_priority(process_identifier: ProcessIdentifierChoice) -> Result<Self, ()>
	{
		let pid: pid_t = process_identifier.into();
		Self::get_priority(PRIO_PROCESS, pid as u32)
	}
	
	/// Returns an `Err()` if the thread does not exist.
	#[inline(always)]
	pub fn get_thread_priority(thread_identifier: ThreadIdentifier) -> Result<Self, ()>
	{
		let tid: pid_t = thread_identifier.into();
		Self::get_priority(PRIO_PROCESS, tid as u32)
	}
	
	/// Returns an `Err()` if the user does not exist.
	#[inline(always)]
	pub fn get_real_user_priority(user_identifier: UserIdentifier) -> Result<Self, ()>
	{
		let uid: uid_t = user_identifier.into();
		Self::get_priority(PRIO_USER, uid)
	}
	
	#[inline(always)]
	fn get_priority(which: i32, who: u32) -> Result<Self, ()>
	{
		SystemCallErrorNumber::reset_errno_to_zero();
		
		let result = unsafe { getpriority(which, who) };
		
		match SystemCallErrorNumber::from_errno()
		{
			None => match result
			{
				-20 ..= 19 => Ok(unsafe { transmute(result) }),
				
				invalid @ _ => panic!("Unknown priority value `{}` from getpriority", invalid),
			}
			
			Some(EINVAL) => panic!("`which` was not one of `PRIO_PROCESS`, `PRIO_PGRP`, or `PRIO_USER`"),
			
			Some(ESRCH) => Err(()),
			
			Some(unexpected_error) => unexpected_error!(getpriority, unexpected_error),
		}
	}
}
