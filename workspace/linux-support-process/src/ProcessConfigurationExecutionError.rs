/// A process configuration execution error.
#[derive(Debug)]
pub enum ProcessConfigurationExecutionError<LoadKernelModulesError: error::Error, AdditionalLinuxKernelCommandLineValidationsError: error::Error, MainError: error::Error>
{
	/// Could not load kernel modules (explanation in tuple argument).
	CouldNotLoadKernelModules(LoadKernelModulesError),

	/// Could not write system control values.
	CouldNotWriteSystemControlValues(io::Error),

	/// Rescan of all PCI buses and devices failed.
	RescanOfAllPciBusesAndDevices(io::Error),

	/// CPU features failed validation (explanation in tuple argument).
	CpuFeaturesValidationFailed(String),

	/// Linux kernel command line failed validation (explanation in tuple argument).
	LinuxKernelCommandLineValidationFailed(LinuxKernelCommandLineValidationError<AdditionalLinuxKernelCommandLineValidationsError>),

	/// Could not set work queue hyper thread affinity to online shared hyper threads.
	///
	/// Shared hyper threads are those shared with the operating system and other processes (ie not isolated).
	CouldNotSetWorkQueueHyperThreadAffinityToOnlineSharedHyperThreads(io::Error),

	/// Could not force watchdog hyper thread affinity to online shared hyper threads.
	///
	/// Shared hyper threads are those shared with the operating system and other processes (ie not isolated).
	CouldNotForceWatchdogHyperThreadAffinityToOnlineSharedHyperThreads(io::Error),

	/// Could not disable Transparent Huge Pages (THP).
	CouldNotDisableTransparentHugePages(DisableTransparentHugePagesError),

	/// Could not configure.
	CouldNotConfigure(DetailedProcessConfigurationError),

	/// Execution failed (with description of reason).
	ExecutionFailed(MainError),

	/// Execution panicked (with panic info data).
	ExecutionPanicked(Box<dyn Any + Send + 'static>),
}

impl<LoadKernelModulesError: error::Error, AdditionalLinuxKernelCommandLineValidationsError: error::Error, MainError: error::Error> Display for ProcessConfigurationExecutionError<LoadKernelModulesError, AdditionalLinuxKernelCommandLineValidationsError, MainError>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<LoadKernelModulesError: 'static + error::Error, AdditionalLinuxKernelCommandLineValidationsError: 'static + error::Error, MainError: 'static + error::Error> error::Error for ProcessConfigurationExecutionError<LoadKernelModulesError, AdditionalLinuxKernelCommandLineValidationsError, MainError>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::ProcessConfigurationExecutionError::*;

		match self
		{
			&CouldNotLoadKernelModules(ref error) => Some(error),

			&CouldNotWriteSystemControlValues(ref error) => Some(error),

			&RescanOfAllPciBusesAndDevices(ref error) => Some(error),

			&CpuFeaturesValidationFailed(_) => None,

			&LinuxKernelCommandLineValidationFailed(ref error) => Some(error),

			&CouldNotSetWorkQueueHyperThreadAffinityToOnlineSharedHyperThreads(ref error) => Some(error),

			&CouldNotForceWatchdogHyperThreadAffinityToOnlineSharedHyperThreads(ref error) => Some(error),

			&CouldNotDisableTransparentHugePages(ref error) => Some(error),

			&CouldNotConfigure(ref error) => Some(error),

			&ExecutionFailed(ref error) => Some(error),

			&ExecutionPanicked(..) => None,
		}
	}
}

impl<LoadKernelModulesError: error::Error, AdditionalLinuxKernelCommandLineValidationsError: error::Error, MainError: error::Error> From<DisableTransparentHugePagesError> for ProcessConfigurationExecutionError<LoadKernelModulesError, AdditionalLinuxKernelCommandLineValidationsError, MainError>
{
	#[inline(always)]
	fn from(error: DisableTransparentHugePagesError) -> Self
	{
		ProcessConfigurationExecutionError::CouldNotDisableTransparentHugePages(error)
	}
}

impl<LoadKernelModulesError: error::Error, AdditionalLinuxKernelCommandLineValidationsError: error::Error, MainError: error::Error> From<DetailedProcessConfigurationError> for ProcessConfigurationExecutionError<LoadKernelModulesError, AdditionalLinuxKernelCommandLineValidationsError, MainError>
{
	#[inline(always)]
	fn from(error: DetailedProcessConfigurationError) -> Self
	{
		ProcessConfigurationExecutionError::CouldNotConfigure(error)
	}
}

impl<LoadKernelModulesError: error::Error, AdditionalLinuxKernelCommandLineValidationsError: error::Error, MainError: error::Error> From<Box<dyn Any + Send + 'static>> for ProcessConfigurationExecutionError<LoadKernelModulesError, AdditionalLinuxKernelCommandLineValidationsError, MainError>
{
	#[inline(always)]
	fn from(panic_information: Box<dyn Any + Send + 'static>) -> Self
	{
		ProcessConfigurationExecutionError::ExecutionPanicked(panic_information)
	}
}
