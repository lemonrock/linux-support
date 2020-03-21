/// Master process error.
#[derive(Debug)]
pub enum MasterProcessAdditionalLinuxKernelCommandLineValidationsError
{
	/// Reading file systems.
	ReadingFileSystems(io::Error),

	/// Unsupported file system.
	UnsupportedFileSystem(FileSystemSupportedError),
}

impl Display for MasterProcessAdditionalLinuxKernelCommandLineValidationsError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for MasterProcessAdditionalLinuxKernelCommandLineValidationsError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::MasterProcessAdditionalLinuxKernelCommandLineValidationsError::*;

		match self
		{
			&ReadingFileSystems(ref error) => Some(error),

			&UnsupportedFileSystem(ref error) => Some(error),
		}
	}
}

impl From<FileSystemSupportedError> for MasterProcessAdditionalLinuxKernelCommandLineValidationsError
{
	#[inline(always)]
	fn from(error: FileSystemSupportedError) -> Self
	{
		MasterProcessAdditionalLinuxKernelCommandLineValidationsError::UnsupportedFileSystem(error)
	}
}
