// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2017 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Process (`comm`) name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ProcessName(CommandName);

impl Deref for ProcessName
{
	type Target = CommandName;

	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl AsRef<CStr> for ProcessName
{
	#[inline(always)]
	fn as_ref(&self) -> &CStr
	{
		self.0.as_ref()
	}
}

impl ToString for ProcessName
{
	#[inline(always)]
	fn to_string(&self) -> String
	{
		self.0.to_string()
	}
}

impl Default for ProcessName
{
	#[inline(always)]
	fn default() -> Self
	{
		let length = unsafe { strnlen(program_invocation_short_name, CommandName::MaximumCommandNameLengthExcludingAsciiNul) };

		Self(CommandName::new_from_bytes_excluding_ascii_nul(unsafe { from_raw_parts(program_invocation_short_name as *const u8, length) }))
	}
}

impl ProcessName
{
	/// For any process.
	#[inline(always)]
	pub fn set_process_name(&self, process_identifier: ProcessIdentifierChoice, proc_path: &ProcPath) -> io::Result<()>
	{
		let file_path = proc_path.process_file_path(process_identifier, "comm");
		file_path.write_value(&self.0[..])
	}

	/// For any process.
	#[inline(always)]
	pub fn get_process_name(process_identifier: ProcessIdentifierChoice, proc_path: &ProcPath) -> io::Result<Self>
	{
		let file_path = proc_path.process_file_path(process_identifier, "comm");
		let command_name = file_path.read_value()?;
		Ok(Self(command_name))
	}
}
