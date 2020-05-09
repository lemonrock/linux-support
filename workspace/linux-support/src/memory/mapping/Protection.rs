// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Memory protection.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(i32)]
pub enum Protection
{
	/// No access allowed.
	Unaccessible = PROT_NONE,

	/// Readable.
	Read = PROT_READ,

	/// Readable and Writable.
	ReadWrite = PROT_READ | PROT_WRITE,

	/// Readable and Executable (eg for binaries).
	ReadExecutable = PROT_READ | PROT_EXEC,

	/// Readable, Writable and Executable (eg for generated machine code).
	ReadWriteExecutable = PROT_READ | PROT_WRITE | PROT_EXEC,
}

impl Default for Protection
{
	#[inline(always)]
	fn default() -> Self
	{
		Protection::ReadWrite
	}
}

impl Protection
{
	/// Adjust open options to match.
	#[inline(always)]
	pub fn adjust_open_options_to_match(self, open_options: &mut OpenOptions) -> &mut OpenOptions
	{
		use self::Protection::*;

		let open_options = open_options.truncate(false).append(false);

		match self
		{
			Unaccessible => open_options,
			Read | ReadExecutable => open_options.read(true),
			ReadWrite | ReadWriteExecutable=> open_options.read(true).write(true),
		}
	}
}
