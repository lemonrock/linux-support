// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A thread identifier (`tid`).
///
/// In a single-threaded process, the thread identifier (`tid`) is equal to the process identifier (`pid`) as returned by `ProcessIdentifier::default()`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ThreadIdentifier(NonZeroI32);

impl Default for ThreadIdentifier
{
	#[inline(always)]
	fn default() -> Self
	{
		extern "C"
		{
			fn gettid() -> pid_t;
		}
		
		let tid = unsafe { gettid() };
		debug_assert!(tid > 0);
		Self(new_non_zero_i32(tid))
	}
}

impl TryFrom<pid_t> for ThreadIdentifier
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

impl From<NonZeroI32> for ThreadIdentifier
{
	#[inline(always)]
	fn from(value: NonZeroI32) -> Self
	{
		Self(value)
	}
}

impl From<ProcessIdentifier> for ThreadIdentifier
{
	#[inline(always)]
	fn from(value: ProcessIdentifier) -> Self
	{
		Self(value.into())
	}
}

impl Into<NonZeroI32> for ThreadIdentifier
{
	#[inline(always)]
	fn into(self) -> NonZeroI32
	{
		self.0
	}
}

impl Into<pid_t> for ThreadIdentifier
{
	#[inline(always)]
	fn into(self) -> pid_t
	{
		self.0.get()
	}
}

impl Into<ProcessIdentifier> for ThreadIdentifier
{
	#[inline(always)]
	fn into(self) -> ProcessIdentifier
	{
		ProcessIdentifier::from(self.0)
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for ThreadIdentifier
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		UnpaddedDecimalInteger(self.0).into_line_feed_terminated_byte_string()
	}
}

impl ParseNumber for ThreadIdentifier
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let tid = pid_t::parse_number(bytes, radix, parse_byte)?;
		if unlikely!(tid < 0)
		{
			Err(ParseNumberError::TooShort)
		}
		else if unlikely!(tid == 0)
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Ok(Self(new_non_zero_i32(tid)))
		}
	}
}

impl ParseNumberOption for ThreadIdentifier
{
	#[inline(always)]
	fn parse_number_option(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Option<Self>, ParseNumberError>
	{
		let tid = pid_t::parse_number(bytes, radix, parse_byte)?;
		if unlikely!(tid < 0)
		{
			Err(ParseNumberError::TooShort)
		}
		else if tid == 0
		{
			Ok(None)
		}
		else
		{
			Ok(Some(ThreadIdentifier(new_non_zero_i32(tid))))
		}
	}
}

impl ThreadIdentifier
{
	#[inline(always)]
	pub(crate) fn to_file_name(self) -> String
	{
		format!("{}", self.0)
	}
	
	/// Current threads for a process.
	pub fn for_process(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> io::Result<impl Iterator<Item=ThreadIdentifier>>
	{
		let folder_path = proc_path.process_threads_folder_path(process_identifier);
		
		fn process_dir_entry(dir_entry: io::Result<DirEntry>) -> Option<ThreadIdentifier>
		{
			let dir_entry = match dir_entry
			{
				Err(_) => return None,
				Ok(dir_entry) => dir_entry
			};
			
			match dir_entry.file_type()
			{
				Err(_) => return None,
				Ok(file_type) => if !file_type.is_dir()
				{
					return None
				}
			};
			
			let file_name = dir_entry.file_name().into_vec();
			NonZeroI32::from_bytes(&file_name[..]).ok().map(|tid| ThreadIdentifier(tid))
		}
		
		Ok(folder_path.read_dir()?.filter_map(process_dir_entry))
	}
}
