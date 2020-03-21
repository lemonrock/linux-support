/// Represents a process; implement this to do something useful.
pub trait Process
{
	/// Is Enhanced Intel SpeedStep technology used?
	const UseEnhancedIntelSpeedStepTechnology: bool = false;

	/// Are isolated CPUs required at boot?
	const IsolatedCpusRequired: bool = true;

	/// The type of error that could occur when `load_kernel_modules()` is executed.
	type LoadKernelModulesError: error::Error;

	/// Load any additional Linux kernel modules.
	fn load_kernel_modules(&self) -> Result<(), Self::LoadKernelModulesError>;

	/// The type of error that could occur when `additional_linux_kernel_command_line_validations()` is executed.
	type AdditionalLinuxKernelCommandLineValidationsError: error::Error;

	/// Perform additional Linux kernel command line validations (or other checks, eg for filesystems).
	#[allow(unused_variable)]
	fn additional_linux_kernel_command_line_validations(&self, linux_kernel_command_line_parameters: &LinuxKernelCommandLineParameters, proc_path: &ProcPath) -> Result<(), Self::AdditionalLinuxKernelCommandLineValidationsError>;

	/// The type of error that could occur when `main()` is executed.
	type MainError: error::Error;

	/// Main method, daemonized, etc.
	fn main(self, online_shared_hyper_threads_for_os: BTreeSet<HyperThread>, online_shared_hyper_threads_for_process: BTreeSet<HyperThread>, online_isolated_hyper_threads_for_process: BTreeSet<HyperThread>, master_logical_core: HyperThread, proc_path: &ProcPath)-> Result<Option<SignalNumber>, Self::MainError>;
}
