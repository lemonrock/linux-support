// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents `/proc`.
///
/// Frankly, there are files in `/proc` that really belong in `/sys`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct ProcPath(PathBuf);

impl Default for ProcPath
{
	#[inline(always)]
	fn default() -> Self
	{
		ProcPath(PathBuf::from("/proc"))
	}
}

impl ProcPath
{
	/// Get a folder path for the current process or another process.
	#[inline(always)]
	pub fn process_file_path(&self, process_identifier: ProcessIdentifierChoice, relative_path: &str) -> PathBuf
	{
		self.file_path(&process_identifier.to_file_name()).append(relative_path)
	}
	
	/// Get a folder path for the current process or another process' pressure sub folder.
	///
	/// This folder only exists if the kernel has been built with support for Pressure Stall Information (PSI).
	#[inline(always)]
	pub fn process_pressure_stall_information_file_path(&self, process_identifier: ProcessIdentifierChoice, file_name: &str) -> PathBuf
	{
		self.process_file_path(process_identifier, "pressure").append(file_name)
	}

	/// Get a file path for the current process or another process' thread.
	#[inline(always)]
	pub fn process_thread_file_path(&self, process_identifier: ProcessIdentifierChoice, thread_identifier: ThreadIdentifier, file_name: &str) -> PathBuf
	{
		self.process_thread_folder_path(process_identifier, thread_identifier).append(file_name)
	}
	
	/// Get a folder path for the current process or another process' thread.
	#[inline(always)]
	pub fn process_thread_folder_path(&self, process_identifier: ProcessIdentifierChoice, thread_identifier: ThreadIdentifier) -> PathBuf
	{
		self.process_threads_folder_path(process_identifier).append(&thread_identifier.to_file_name())
	}
	
	/// Get a folder path for the current process or another process' threads.
	#[inline(always)]
	pub fn process_threads_folder_path(&self, process_identifier: ProcessIdentifierChoice) -> PathBuf
	{
		self.process_file_path(process_identifier, "task")
	}

	/// Get a file path within the ProcPath, `/proc/sys/fs/mqueue/<file_name>`.
	#[inline(always)]
	pub fn sys_fs_mqueue_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_fs_file_path("mqueue").append(file_name)
	}

	/// Get a file path within the ProcPath, `/proc/sys/fs/inotify/<file_name>`.
	#[inline(always)]
	pub fn sys_fs_inotify_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_fs_file_path("inotify").append(file_name)
	}

	/// Get a file path within the ProcPath, `/proc/sys/fs/epoll/<file_name>`.
	#[inline(always)]
	pub fn sys_fs_epoll_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_fs_file_path("epoll").append(file_name)
	}

	/// Get a file path within the ProcPath, `/proc/sys/dev/scsi/<file_name>`.
	#[inline(always)]
	pub fn sys_dev_scsi_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_dev_file_path("scsi").append(file_name)
	}
	
	/// Get a file path within the ProcPath, `/proc/sys/dev/<file_name>`.
	#[inline(always)]
	pub fn sys_dev_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_file_path("dev").append(file_name)
	}
	
	/// Get a file path within the ProcPath, `/proc/sys/debug/<file_name>`.
	#[inline(always)]
	pub fn sys_debug_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_file_path("debug").append(file_name)
	}
	
	/// Get a file path within the ProcPath, `/proc/sys/abi/<file_name>`.
	#[inline(always)]
	pub fn sys_abi_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_file_path("abi").append(file_name)
	}
	
	/// Get a file path within the ProcPath, `/proc/sys/vm/<file_name>`.
	#[inline(always)]
	pub fn sys_vm_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_file_path("vm").append(file_name)
	}

	/// Get a file path within the ProcPath, `/proc/sys/net/unix/<file_name>`.
	#[inline(always)]
	pub fn sys_net_unix_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_net_file_path("unix").append(file_name)
	}

	/// Get a file path within the ProcPath, `/proc/sys/net/ipv4/<file_name>`.
	#[inline(always)]
	pub fn sys_net_ipv4_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_net_file_path("ipv4").append(file_name)
	}
	
	/// Get a file path within the ProcPath, `/proc/sys/net/core/<file_name>`.
	#[inline(always)]
	pub fn sys_net_core_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_net_file_path("core").append(file_name)
	}

	/// Get a file path within the ProcPath, `/proc/sys/net/<file_name>`.
	#[inline(always)]
	pub fn sys_net_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_file_path("net").append(file_name)
	}

	/// Get a file path within the ProcPath, `/proc/sys/fs/<file_name>`.
	#[inline(always)]
	pub fn sys_fs_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_file_path("fs").append(file_name)
	}

	/// Get a file path within the ProcPath, `/proc/sys/user/<file_name>`.
	#[inline(always)]
	pub fn sys_user_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_file_path("user").append(file_name)
	}

	/// Get a file path within the ProcPath, `/proc/sys/kernel/random/<file_name>`.
	#[inline(always)]
	pub fn sys_kernel_random_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_kernel_file_path("random").append(file_name)
	}
	
	/// Get a file path within the ProcPath, `/proc/sys/kernel/seccomp/<file_name>`.
	#[inline(always)]
	pub fn sys_kernel_seccomp_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_kernel_file_path("seccomp").append(file_name)
	}
	
	/// Get a file path within the ProcPath, `/proc/sys/kernel/<file_name>`.
	#[inline(always)]
	pub fn sys_kernel_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_file_path("kernel").append(file_name)
	}

	/// Get a file path within the ProcPath, `/proc/sys/<file_name>`.
	#[inline(always)]
	pub fn sys_file_path(&self, file_name: &str) -> PathBuf
	{
		self.sys_folder_path().append(file_name)
	}

	/// Get a folder path within the ProcPath, `/proc/sys`.
	#[inline(always)]
	pub fn sys_folder_path(&self) -> PathBuf
	{
		self.file_path("sys")
	}

	/// Get a file path within the ProcPath, `/proc/irq/<number>/<file_name>`.
	#[inline(always)]
	pub fn irq_number_file_path(&self, interrupt_request: InterruptRequest, file_name: &str) -> PathBuf
	{
		self.irq_file_path(&interrupt_request.file_name()).append(file_name)
	}
	
	/// Get a file path within the ProcPath, `/proc/irq/<file_name>`.
	#[inline(always)]
	pub fn irq_file_path(&self, file_name: &str) -> PathBuf
	{
		self.file_path("irq").append(file_name)
	}

	/// Get a file path within the ProcPath, `/proc`.
	#[inline(always)]
	pub fn file_path(&self, file_name: &str) -> PathBuf
	{
		self.path().append(file_name)
	}

	#[inline(always)]
	fn path(&self) -> PathBuf
	{
		self.0.to_owned()
	}
}
