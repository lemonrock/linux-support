// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Command (process or thread) name.
///
/// Also known as `comm`.
///
/// Deref excludes trailing ASCII NUL.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct CommandName(ObjectName);

impl Display for CommandName
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl Into<ObjectName> for CommandName
{
	#[inline(always)]
	fn into(self) -> ObjectName
	{
		self.0
	}
}

impl Deref for CommandName
{
	type Target = ObjectName;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl CommandName
{
	/// Reads from a Linux ProcPath or SysPath resource.
	#[inline(always)]
	pub fn read_from_file_line_feed_terminated(file_path: &Path) -> io::Result<Self>
	{
		ObjectName::read_from_file_line_feed_terminated(file_path).map(|object_name| Self(object_name))
	}
	
	/// Writes to a Linux ProcPath or SysPath resource.
	#[inline(always)]
	pub fn write_to_file_line_feed_terminated(&self, file_path: &Path) -> io::Result<()>
	{
		self.0.write_to_file_line_feed_terminated(file_path)
	}
}
