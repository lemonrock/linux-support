// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A process group (or session) identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ProcessGroupIdentifier(NonZeroI32);

impl Default for ProcessGroupIdentifier
{
	#[inline(always)]
	fn default() -> Self
	{
		let pid = unsafe { getpgid(0) };
		debug_assert!(pid > 0);
		Self(new_non_zero_i32(pid))
	}
}

impl TryFrom<pid_t> for ProcessGroupIdentifier
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn try_from(value: pid_t) -> Result<Self, Self::Error>
	{
		if likely!(value > 0)
		{
			Ok(Self(new_non_zero_i32(value)))
		}
		else
		{
			Err(ParseNumberError::TooSmall)
		}
	}
}

impl From<NonZeroI32> for ProcessGroupIdentifier
{
	#[inline(always)]
	fn from(value: NonZeroI32) -> Self
	{
		Self(value)
	}
}

impl Into<NonZeroI32> for ProcessGroupIdentifier
{
	#[inline(always)]
	fn into(self) -> NonZeroI32
	{
		self.0
	}
}

impl Into<pid_t> for ProcessGroupIdentifier
{
	#[inline(always)]
	fn into(self) -> pid_t
	{
		self.0.get()
	}
}

impl ParseNumber for ProcessGroupIdentifier
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let pid = pid_t::parse_number(bytes, radix, parse_byte)?;
		if unlikely!(pid < 0)
		{
			Err(ParseNumberError::TooShort)
		}
		else if unlikely!(pid == 0)
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Ok(Self(new_non_zero_i32(pid)))
		}
	}
}

impl ParseNumberOption for ProcessGroupIdentifier
{
	#[inline(always)]
	fn parse_number_option(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Option<Self>, ParseNumberError>
	{
		let pid = pid_t::parse_number(bytes, radix, parse_byte)?;
		if unlikely!(pid < -1)
		{
			Err(ParseNumberError::TooShort)
		}
		// eg as in `/proc/<N>/stat`.
		else if unlikely!(pid == -1)
		{
			Ok(None)
		}
		else if unlikely!(pid == 0)
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Ok(Some(ProcessGroupIdentifier(new_non_zero_i32(pid))))
		}
	}
}

impl ProcessGroupIdentifier
{
	/// Get the process group identifier (pgid) for a process identifier.
	#[inline(always)]
	pub fn process_group_identifier(process_identifier: ProcessIdentifierChoice) -> Result<Self, ()>
	{
		let result = unsafe { getpgid(process_identifier.into()) };
		if likely!(result == 0)
		{
			Ok(Self(new_non_zero_i32(result)))
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno_panic()
			{
				ESRCH | EPERM => Err(()),
				
				unexpected_error @ _ => unexpected_error!(getpgid, unexpected_error),
			}
		}
		else
		{
			unexpected_result!(getpgid, result)
		}
	}

	/// Get session identifier (sid) for a process identifier.
	///
	/// The session identifier of a process is the process group identifier of the session leader.
	#[inline(always)]
	pub fn session_identifier(process_identifier: ProcessIdentifierChoice) -> Result<Self, ()>
	{
		let result = unsafe { getsid(process_identifier.into()) };
		if likely!(result == 0)
		{
			Ok(Self(new_non_zero_i32(result)))
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno_panic()
			{
				ESRCH | EPERM => Err(()),
				
				unexpected_error @ _ => unexpected_error!(getsid, unexpected_error),
			}
		}
		else
		{
			unexpected_result!(getsid, result)
		}
	}
	
	/// ?May be `None` if audit is not running?
	#[inline(always)]
	pub fn audit_session_identifier(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> io::Result<Option<Self>>
	{
		let value: u32 = proc_path.process_file_path(process_identifier, "sessionid").read_value()?;
		if value == u32::MAX
		{
			Ok(None)
		}
		else if value > 0 && value <= (i32::MAX as u32)
		{
			Ok(Some(Self(new_non_zero_i32(value as i32))))
		}
		else
		{
			unreachable_code(format_args!("Invalid value `{}` in /proc/{:?}/sessionid", value, process_identifier))
		}
	}
}
