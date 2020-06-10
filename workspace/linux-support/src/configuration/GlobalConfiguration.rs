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
	pub system_v_message_queue: GlobalSystemVMessageQueueConfiguration,

	/// Requires root.
	pub inotify: GlobalInotifyConfiguration,

	/// Requires root.
	pub epoll: GlobalEPollConfiguration,

	/// Requires root.
	pub same_page_merging: GlobalLinuxKernelSamePageMergingConfiguration,

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
	pub security: GlobalSecurityConfiguration,

	/// Requires root.
	pub transparent_huge_pages: Option<GlobalTransparentHugePagesConfiguration>,
	
	/// Requires root.
	pub network: GlobalNetworkConfiguration,
	
	/// Validation-only checks.
	pub linux_kernel_command_line: GlobalLinuxKernelCommandLineConfiguration,
}

impl GlobalConfiguration
{
	/// Configures.
	#[inline(always)]
	pub fn configure(&self, sys_path: &SysPath, proc_path: &ProcPath) -> Result<(), GlobalConfigurationError>
	{
		self.global_scheduling.configure(sys_path, proc_path)?;

		self.pipe.configure(proc_path)?;

		self.file_leasing.configure(proc_path)?;

		self.posix_message_queue.configure(proc_path)?;

		self.system_v_message_queue.configure(proc_path)?;

		self.inotify.configure(proc_path)?;

		self.epoll.configure(proc_path)?;

		self.same_page_merging.configure(sys_path)?;

		self.linux_kernel_asynchronous_io.configure(proc_path)?;

		self.file_handle.configure(proc_path)?;

		self.file_descriptor.configure(proc_path)?;

		self.linux_module.configure(proc_path)?;

		self.kernel_panic.configure(proc_path)?;

		self.security.configure(proc_path)?;

		if let Some(ref transparent_huge_pages) = self.transparent_huge_pages
		{
			transparent_huge_pages.configure(sys_path)?;
		}

		self.network.configure(proc_path)?;
		
		self.linux_kernel_command_line.configure(proc_path)?;

		Ok(())
	}
}
