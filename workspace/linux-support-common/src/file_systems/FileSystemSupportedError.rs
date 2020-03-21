/// A File System Supported error.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum FileSystemSupportedError
{
	/// Linux kernel does not support file system.
	Unsupported(FileSystemType),

	/// File system has associated devices (ie is not 'nodev').
	HasAssociatedDevices(FileSystemType),
}

impl Display for FileSystemSupportedError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for FileSystemSupportedError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::FileSystemSupportedError::*;

		match self
		{
			&Unsupported(..) => None,
			&HasAssociatedDevices(..) => None,
		}
	}
}
