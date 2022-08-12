// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Global configuration.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct GlobalConfiguration
{
	/// Requires root.
	pub global_scheduling: GlobalSchedulingConfiguration,

	/// Requires root.
	pub pipe: GlobalPipeConfiguration,

	/// Requires root.
	pub file_leasing: GlobalFileLeasingConfiguration,

	/// Requires root.
	pub posix_message_queue: GlobalPosixMessageQueueConfiguration,

	/// Requires root.
	pub system_v_memory_segment: GlobalSystemVMemorySegmentConfiguration,

	/// Requires root.
	pub system_v_message_queue: GlobalSystemVMessageQueueConfiguration,
	
	/// Requires root.
	pub system_v_semaphore: Option<SemaphoresConfiguration>,
	
	/// Requires root.
	pub inotify: GlobalInotifyConfiguration,

	/// Requires root.
	pub epoll: GlobalEPollConfiguration,

	/// Requires root.
	pub linux_kernel_asynchronous_io: GlobalLinuxKernelAsynchronousIoConfiguration,

	/// Requires root.
	pub file_handle: GlobalFileHandleConfiguration,

	/// Requires root.
	pub file_descriptor: GlobalFileDescriptorConfiguration,

	/// Requires root.
	pub linux_module: GlobalLinuxModuleConfiguration,

	/// Requires root.
	pub kernel_panic: GlobalKernelPanicConfiguration,
	
	/// Requires root.
	pub memory: GlobalMemoryConfiguration,
	
	/// Requires root.
	pub network: GlobalNetworkConfiguration,
	
	/// Requires root.
	pub bpf: GlobalBpfConfiguration,
	
	/// Requires root.
	pub cgroup: GlobalCgroupConfiguration,
	
	/// Validation-only checks.
	pub linux_kernel_command_line: GlobalLinuxKernelCommandLineConfiguration,
	
	/// Requires root.
	///
	/// Done last as changing, say, system integrity can have a huge impact on the ability to read and write files.
	pub security: GlobalSecurityConfiguration,
}

impl GlobalConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, sys_path: &SysPath, proc_path: &ProcPath, defaults: &DefaultHugePageSizes) -> Result<(), GlobalConfigurationError>
	{
		use self::GlobalConfigurationError::*;
		
		self.global_scheduling.configure(sys_path, proc_path)?;

		self.pipe.configure(proc_path)?;

		self.file_leasing.configure(proc_path)?;

		self.posix_message_queue.configure(proc_path)?;

		self.system_v_memory_segment.configure(proc_path)?;

		self.system_v_message_queue.configure(proc_path)?;

		if let Some(ref system_v_semaphore) = self.system_v_semaphore
		{
			system_v_semaphore.write(proc_path).map_err(GlobalSystemVSemaphoreConfiguration)?
		}
		
		self.inotify.configure(proc_path)?;

		self.epoll.configure(proc_path)?;

		self.linux_kernel_asynchronous_io.configure(proc_path)?;

		self.file_handle.configure(proc_path)?;

		self.file_descriptor.configure(proc_path)?;

		self.linux_module.configure(sys_path, proc_path)?;

		self.kernel_panic.configure(proc_path)?;

		self.memory.configure(sys_path, proc_path)?;

		self.network.configure(sys_path, proc_path)?;
		
		self.bpf.configure(sys_path, proc_path)?;
		
		self.cgroup.configure(sys_path, proc_path, defaults)?;
		
		self.linux_kernel_command_line.configure(proc_path)?;
		
		self.security.configure(sys_path, proc_path)?;

		Ok(())
	}
}
