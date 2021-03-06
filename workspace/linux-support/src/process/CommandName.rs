// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Command (process or thread) name.
///
/// Also known as `comm`.
///
/// Deref excludes trailing ASCII NUL.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct CommandName(ObjectName16);

impl Display for CommandName
{
	#[inline(always)]
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}", self.0)
	}
}

impl Into<ObjectName16> for CommandName
{
	#[inline(always)]
	fn into(self) -> ObjectName16
	{
		self.0
	}
}

impl From<ObjectName16> for CommandName
{
	#[inline(always)]
	fn from(value: ObjectName16) -> Self
	{
		Self(value)
	}
}

impl FromBytes for CommandName
{
	type Error = ObjectNameFromBytesError;
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		ObjectName16::from_bytes(bytes).map(|object_name| Self(object_name))
	}
}

impl Deref for CommandName
{
	type Target = ObjectName16;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl AsRef<[u8]> for CommandName
{
	#[inline(always)]
	fn as_ref(&self) -> &[u8]
	{
		self.0.as_ref()
	}
}

impl AsRef<CStr> for CommandName
{
	#[inline(always)]
	fn as_ref(&self) -> &CStr
	{
		self.0.as_ref()
	}
}

impl CommandName
{
	/// Reads from a Linux ProcPath or SysPath resource.
	#[inline(always)]
	pub fn read_from_file_line_feed_terminated(file_path: &Path) -> io::Result<Self>
	{
		ObjectName16::read_from_file_line_feed_terminated(file_path).map(|object_name| Self(object_name))
	}
	
	/// Writes to a Linux ProcPath or SysPath resource.
	#[inline(always)]
	pub fn write_to_file_line_feed_terminated(&self, file_path: &Path) -> io::Result<()>
	{
		self.0.write_to_file_line_feed_terminated(file_path)
	}
}
