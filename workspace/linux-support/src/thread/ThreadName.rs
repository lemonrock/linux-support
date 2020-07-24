// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Thread (`comm`) name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ThreadName(CommandName);

impl Display for ThreadName
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl From<CommandName> for ThreadName
{
	#[inline(always)]
	fn from(value: CommandName) -> Self
	{
		Self(value)
	}
}

impl From<ObjectName> for ThreadName
{
	#[inline(always)]
	fn from(value: ObjectName) -> Self
	{
		Self::from(CommandName(value))
	}
}

impl Into<CommandName> for ThreadName
{
	#[inline(always)]
	fn into(self) -> CommandName
	{
		self.0
	}
}

impl Into<ObjectName> for ThreadName
{
	#[inline(always)]
	fn into(self) -> ObjectName
	{
		(self.0).into()
	}
}

impl Deref for ThreadName
{
	type Target = CommandName;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl Default for ThreadName
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(CommandName::from_bytes(b"unnamed").unwrap())
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
		let object_name = ObjectName::construct_from_c_function_call
		(
			|buffer|
			{
				let result = unsafe { prctl(PR_GET_NAME, buffer.as_mut_ptr() as c_ulong) };
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
			},
			|| io::Error::new(ErrorKind::Other, "DoesNotEndWithAsciiNulError")
		).expect("No good reason to fail");
		Self(CommandName(object_name))
	}

	/// This should not fail under ordinary circumstances.
	///
	/// Uses `prctl()`.
	#[inline(always)]
	pub fn set_current_thread_name(&self) -> io::Result<()>
	{
		let pointer = self.0.as_ptr();
		let result = unsafe { prctl(PR_SET_NAME, pointer as c_ulong, 0, 0, 0) };
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
		self.0.write_to_file_line_feed_terminated(&file_path)
	}

	/// For any process and any thread.
	#[inline(always)]
	pub fn get_thread_name(&self, process_identifier: ProcessIdentifierChoice, thread_identifier: ThreadIdentifier, proc_path: &ProcPath) -> io::Result<Self>
	{
		let file_path = proc_path.process_thread_file_path(process_identifier, thread_identifier, "comm");
		CommandName::read_from_file_line_feed_terminated(&file_path).map(|object_name| Self(object_name))
	}
}
