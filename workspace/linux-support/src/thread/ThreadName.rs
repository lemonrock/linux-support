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

impl From<ObjectName16> for ThreadName
{
	#[inline(always)]
	fn from(value: ObjectName16) -> Self
	{
		Self::from(CommandName::from(value))
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

impl Into<ObjectName16> for ThreadName
{
	#[inline(always)]
	fn into(self) -> ObjectName16
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
		Self(CommandName::from(ObjectName16::from_bytes(b"unnamed").unwrap()))
	}
}

impl ThreadName
{
	/// This should not fail under ordinary circumstances.
	#[inline(always)]
	pub fn get_current_thread_name() -> Self
	{
		let object_name = ObjectName16::construct_from_c_function_call
		(
			|buffer|
			{
				process_control_wrapper2
				(
					PR_GET_NAME,
					buffer.as_mut_ptr() as usize,
					result_must_be_zero,
					|error_number| Err(error_number.into())
				)
			},
			|| io::Error::new(ErrorKind::Other, "DoesNotEndWithAsciiNulError")
		).expect("No good reason to fail");
		Self(CommandName::from(object_name))
	}

	/// This should not fail under ordinary circumstances.
	#[inline(always)]
	pub fn set_current_thread_name(&self) -> Result<(), Errno>
	{
		let pointer = self.0.as_ptr();
		process_control_wrapper2
		(
			PR_SET_NAME,
			pointer as usize,
			result_must_be_zero,
			Err,
		)
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
	pub fn get_thread_name(process_identifier: ProcessIdentifierChoice, thread_identifier: ThreadIdentifier, proc_path: &ProcPath) -> io::Result<Self>
	{
		let file_path = proc_path.process_thread_file_path(process_identifier, thread_identifier, "comm");
		CommandName::read_from_file_line_feed_terminated(&file_path).map(|object_name| Self(object_name))
	}
}
