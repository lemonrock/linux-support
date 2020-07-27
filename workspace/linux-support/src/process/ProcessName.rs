// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process (`comm`) name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ProcessName(CommandName);

impl Display for ProcessName
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl From<CommandName> for ProcessName
{
	#[inline(always)]
	fn from(value: CommandName) -> Self
	{
		Self(value)
	}
}

impl From<ObjectName16> for ProcessName
{
	#[inline(always)]
	fn from(value: ObjectName16) -> Self
	{
		Self::from(CommandName(value))
	}
}

impl Into<CommandName> for ProcessName
{
	#[inline(always)]
	fn into(self) -> CommandName
	{
		self.0
	}
}

impl Into<ObjectName16> for ProcessName
{
	#[inline(always)]
	fn into(self) -> ObjectName16
	{
		(self.0).into()
	}
}

impl Deref for ProcessName
{
	type Target = CommandName;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl Default for ProcessName
{
	#[inline(always)]
	fn default() -> Self
	{
		let length = unsafe { strnlen(program_invocation_short_name, ObjectName16::MaximumLengthExcludingAsciiNull) };

		Self(CommandName::from_bytes(unsafe { from_raw_parts(program_invocation_short_name as *const u8, length) }).unwrap())
	}
}

impl ProcessName
{
	/// For any process.
	#[inline(always)]
	pub fn set_process_name(&self, process_identifier: ProcessIdentifierChoice, proc_path: &ProcPath) -> io::Result<()>
	{
		let file_path = proc_path.process_file_path(process_identifier, "comm");
		self.0.write_to_file_line_feed_terminated(&file_path)
	}

	/// For any process.
	#[inline(always)]
	pub fn get_process_name(process_identifier: ProcessIdentifierChoice, proc_path: &ProcPath) -> io::Result<Self>
	{
		let file_path = proc_path.process_file_path(process_identifier, "comm");
		CommandName::read_from_file_line_feed_terminated(&file_path).map(|object_name| Self(object_name))
	}
}
