/// Cause of error when modprobe fails.
#[derive(Debug)]
pub enum ModProbeError
{
	/// Could not open file list of Linux kernel linux_kernel_modules.
	InputOutputError(io::Error),

	/// A module name was empty.
	SignalTerminatedExitCode
	{
		/// The Linux kernel module name (not necessarily UTF-8).
		linux_kernel_module_name: Box<[u8]>,
	},

	/// A module name was duplicated.
	NonZeroExitCode
	{
		/// The Linux kernel module name (not necessarily UTF-8).
		linux_kernel_module_name: Box<[u8]>,

		/// Exit code.
		exit_code: i32,
	},
}

impl Display for ModProbeError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for ModProbeError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ModProbeError::*;

		match self
		{
			&InputOutputError(ref error) => Some(error),

			&SignalTerminatedExitCode { .. } => None,

			&NonZeroExitCode { .. } => None,
		}
	}
}

impl From<io::Error> for ModProbeError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		ModProbeError::InputOutputError(error)
	}
}
