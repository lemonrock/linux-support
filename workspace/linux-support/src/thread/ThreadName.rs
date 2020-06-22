// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Thread (`comm`) name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ThreadName(CommandName);

impl Deref for ThreadName
{
	type Target = CommandName;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl AsRef<CStr> for ThreadName
{
	#[inline(always)]
	fn as_ref(&self) -> &CStr
	{
		self.0.as_ref()
	}
}

impl AsRef<[u8]> for ThreadName
{
	#[inline(always)]
	fn as_ref(&self) -> &[u8]
	{
		self.0.deref()
	}
}

impl ToString for ThreadName
{
	#[inline(always)]
	fn to_string(&self) -> String
	{
		self.0.to_string()
	}
}

impl Default for ThreadName
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(CommandName::new_from_bytes_excluding_ascii_nul(b"unnamed").unwrap())
	}
}

impl ThreadName
{
	/// This should not fail under ordinary circumstances.
	///
	/// Uses `prctl()`.
	#[inline(always)]
	pub fn get_current_thread_name() -> Self
	{
		let mut buffer = ArrayVec::<[u8; CommandName::MaximumCommandNameLengthIncludingAsciiNul]>::new();
		let result = unsafe { prctl(PR_GET_NAME, buffer.as_mut_ptr()) };
		if likely!(result == 0)
		{
			let haystack = unsafe { from_raw_parts(buffer.as_ptr(), CommandName::MaximumCommandNameLengthIncludingAsciiNul) };
			let index = memchr(b'\0', haystack).expect("final element was not ASCII null");
			unsafe { buffer.set_len(index + 1) };
			Self(CommandName(buffer))
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error()).expect("No good reason to fail")
		}
		else
		{
			unreachable!("prctl() returned unexpected result {}", result)
		}
	}

	/// This should not fail under ordinary circumstances.
	///
	/// Uses `prctl()`.
	#[inline(always)]
	pub fn set_current_thread_name(&self) -> io::Result<()>
	{
		let pointer = (self.0).0.as_ptr() as *const c_char;
		let result = unsafe { prctl(PR_SET_NAME, pointer, 0, 0, 0) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error()).expect("No good reason to fail")
		}
		else
		{
			unreachable!("prctl() returned unexpected result {}", result)
		}
	}

	/// For any process and any thread.
	#[inline(always)]
	pub fn set_thread_name(&self, process_identifier: ProcessIdentifierChoice, thread_identifier: ThreadIdentifier, proc_path: &ProcPath) -> io::Result<()>
	{
		let file_path = proc_path.process_thread_file_path(process_identifier, thread_identifier, "comm");
		file_path.write_value(&self.0[..])
	}

	/// For any process and any thread.
	#[inline(always)]
	pub fn get_thread_name(&self, process_identifier: ProcessIdentifierChoice, thread_identifier: ThreadIdentifier, proc_path: &ProcPath) -> io::Result<Self>
	{
		let file_path = proc_path.process_thread_file_path(process_identifier, thread_identifier, "comm");
		let command_name = file_path.read_value()?;
		Ok(Self(command_name))
	}
}
