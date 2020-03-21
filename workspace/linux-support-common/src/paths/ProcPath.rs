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
	/// Autogroup setting of nice for the current process.
	#[inline(always)]
	pub(crate) fn self_autogroup_filepath(&self) -> PathBuf
	{
		self.file_path("self/autogroup")
	}

	#[inline(always)]
	pub(crate) fn sched_autogroup_enabled_file_path(&self) -> PathBuf
	{
		self.file_path("sys/kernel/sched_autogroup_enabled")
	}

	/// Status information from `/proc/self/status`.
	#[inline(always)]
	pub fn self_status(&self) -> Result<ProcessStatusStatistics, ProcessStatusFileParseError>
	{
		self.file_path("self/status").parse_process_status_file()
	}

	/// Status information from `/proc/<IDENTIFIER>/status` where `<IDENTIFIER>` is `identifier`.
	#[inline(always)]
	pub fn process_status(&self, identifier: pid_t) -> Result<ProcessStatusStatistics, ProcessStatusFileParseError>
	{
		self.file_path(&format!("{}/status", identifier)).parse_process_status_file()
	}

	/// Memory statistics (from `/proc/vmstat`).
	///
	/// Interpret this by multiplying counts by page size.
	#[inline(always)]
	pub fn global_zoned_virtual_memory_statistics(&self) -> io::Result<HashMap<VirtualMemoryStatisticName, u64>>
	{
		self.file_path("vmstat").parse_virtual_memory_statistics_file()
	}

	/// Memory information (from `/proc/meminfo`).
	#[inline(always)]
	pub fn memory_information(&self, memory_information_name_prefix: &[u8]) -> Result<MemoryInformation, MemoryInformationParseError>
	{
		self.file_path("meminfo").parse_memory_information_file(memory_information_name_prefix)
	}

	/// Only execute this after any kernel modules have loaded.
	///
	/// We ignore failures.
	#[inline(always)]
	pub fn write_system_control_values(&self, settings: &HashMap<String, u64>) -> io::Result<()>
	{
		if cfg!(any(target_os = "android", target_os = "linux"))
		{
			for (setting_name, setting_value) in settings.iter()
			{
				let file_path = self.file_path(&format!("sys/{}", setting_name));
				file_path.write_value(setting_value)?;
			}
		}
		Ok(())
	}

	/// Get the maximum number of open file descriptors.
	#[inline(always)]
	pub fn maximum_number_of_open_file_descriptors(&self) -> io::Result<u64>
	{
		self.file_path("sys/fs/nr_open").read_value()
	}

	/// File systems (from `/proc/filesystems`).
	#[inline(always)]
	pub fn file_systems(&self) -> Result<FileSystemTypeList, io::Error>
	{
		if cfg!(any(target_os = "android", target_os = "linux"))
		{
			let file_path = self.file_path("filesystems");
			FileSystemTypeList::parse(&file_path)
		}
		else
		{
			Ok(FileSystemTypeList::default())
		}
	}

	/// Current mounts (from `/proc/self/mounts`).
	#[inline(always)]
	pub fn mounts(&self) -> Result<Mounts, io::Error>
	{
		if cfg!(any(target_os = "android", target_os = "linux"))
		{
			let file_path = self.file_path("self/mounts");
			Mounts::parse(&file_path)
		}
		else
		{
			Ok(Mounts::default())
		}
	}

	/// Current loaded Linux kernel modules (from `/proc/modules`).
	#[inline(always)]
	pub fn linux_kernel_modules(&self) -> Result<LinuxKernelModulesList, LinuxKernelModulesListParseError>
	{
		if cfg!(any(target_os = "android", target_os = "linux"))
		{
			let file_path = self.file_path("modules");
			LinuxKernelModulesList::parse(&file_path)
		}
		else
		{
			Ok(LinuxKernelModulesList::default())
		}
	}

	/// Command line parameters used to start Linux.
	#[inline(always)]
	pub fn linux_command_line_parameters(&self) -> Result<LinuxKernelCommandLineParameters, io::Error>
	{
		if cfg!(any(target_os = "android", target_os = "linux"))
		{
			let file_path = self.file_path("cmdline");
			LinuxKernelCommandLineParameters::parse(&file_path)
		}
		else
		{
			Ok(LinuxKernelCommandLineParameters::default())
		}
	}

	/// Get a folder path for the current process (`process` is `0`) or another process.
	#[inline(always)]
	pub fn proces_folder_path(&self, process: pid_t) -> PathBuf
	{
		if process == 0
		{
			self.file_path("self")
		}
		else
		{
			self.file_path(&format!("{}", process))
		}
	}

	/// Get a file path within the ProcPath, `/proc`.
	#[inline(always)]
	pub fn file_path(&self, file_name: &str) -> PathBuf
	{
		let mut path = self.path();
		path.push(file_name);
		path
	}

	#[inline(always)]
	fn path(&self) -> PathBuf
	{
		self.0.to_owned()
	}
}
