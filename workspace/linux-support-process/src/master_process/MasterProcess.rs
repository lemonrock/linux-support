/// Master process.
#[derive(Debug)]
pub struct MasterProcess
{
	/// Maximum children.
	pub maximum_children: u64,
}

impl Process for MasterProcess
{
	type LoadKernelModulesError = NoPossibleError;

	fn load_kernel_modules(&self) -> Result<(), Self::LoadKernelModulesError>
	{
		Ok(())
	}

	type AdditionalLinuxKernelCommandLineValidationsError = MasterProcessAdditionalLinuxKernelCommandLineValidationsError;

	fn additional_linux_kernel_command_line_validations(&self, _linux_kernel_command_line_parameters: &LinuxKernelCommandLineParameters, proc_path: &ProcPath) -> Result<(), Self::AdditionalLinuxKernelCommandLineValidationsError>
	{
		use self::MasterProcessAdditionalLinuxKernelCommandLineValidationsError::*;

		let file_systems = proc_path.file_systems().map_err(|io_error| ReadingFileSystems(io_error))?;
		file_systems.verify_file_system_is_supported(FileSystemType::cgroup2)?;
		Ok(())
	}

	type MainError = MasterProcessError;

	fn main(self, _online_shared_hyper_threads_for_os: BTreeSet<HyperThread>, _online_shared_hyper_threads_for_process: BTreeSet<HyperThread>, _online_isolated_hyper_threads_for_process: BTreeSet<HyperThread>, _master_logical_core: HyperThread, proc_path: &ProcPath)-> Result<Option<SignalNumber>, Self::MainError>
	{
		let maximum_namespaces = self.maximum_children + 1;

		let namespace_proc_path = NamespacesProcPath(proc_path);
		namespace_proc_path.write_maximum_cgroup_namespaces(maximum_namespaces)?;
		namespace_proc_path.write_maximum_inter_process_communication_namespaces(maximum_namespaces)?;
		namespace_proc_path.write_maximum_mount_namespaces(maximum_namespaces)?;
		namespace_proc_path.write_maximum_net_namespaces(maximum_namespaces)?;
		namespace_proc_path.write_maximum_process_identifier_namespaces(maximum_namespaces)?;
		namespace_proc_path.write_maximum_user_namespaces(maximum_namespaces)?;
		namespace_proc_path.write_maximum_uts_namespaces(maximum_namespaces)?;

		struct CPF;

		impl ChildProcessFunction for CPF
		{
			type ChildProcessArgument = ();

			#[inline(always)]
			fn child_process(_argument: Self::ChildProcessArgument) -> io::Result<()>
			{
				Ok(())
			}
		}

		let mut child_stack_allocator = System;
		let child_process_stack_size = 1024 * 1024;
		let child_process_argument = ();
		let new_root = PathBuf::from("/new_roots/uid");
		let _child_process_identifier = clone_child_process_in_new_namespace::<CPF, _>(proc_path, &mut child_stack_allocator, child_process_stack_size, child_process_argument, new_root)?;

		Ok(None)
	}
}
