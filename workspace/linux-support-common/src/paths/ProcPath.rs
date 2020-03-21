/// Represents `/proc`.
///
/// Frankly, there are files in `/proc` that really belong in `/sys`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[derive(Deserialize, Serialize)]
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
	/// Is autogroup active? (from `/proc/sys/kernel/sched_autogroup_enabled`).
	#[inline(always)]
	pub fn is_autogroup_active(&self) -> Result<bool, io::Error>
	{
		if cfg!(any(target_os = "android", target_os = "linux"))
		{
			let value = self.sched_autogroup_enabled_file_path().read_raw_without_line_feed()?;
			match &value[..]
			{
				b"0" => Ok(false),
				b"1" => Ok(true),
				_ => Err(io::Error::from(ErrorKind::InvalidData)),
			}
		}
		else
		{
			Ok(false)
		}
	}

	/// Enable the autogroup feature (requires root).
	#[inline(always)]
	pub fn enable_autogroup(&self) -> Result<(), io::Error>
	{
		if cfg!(any(target_os = "android", target_os = "linux"))
		{
			self.sched_autogroup_enabled_file_path().write_value("1")
		}
		else
		{
			Ok(())
		}
	}

	/// Disable the autogroup feature (requires Root).
	#[inline(always)]
	pub fn disable_autogroup(&self) -> Result<(), io::Error>
	{
		if cfg!(any(target_os = "android", target_os = "linux"))
		{
			self.sched_autogroup_enabled_file_path().write_value("0")
		}
		else
		{
			Ok(())
		}
	}

	/// Adjust the autogroup setting of nice for the current process.
	#[inline(always)]
	pub fn adjust_autogroup_nice_value_for_self(&self, nice_value: Nice) -> Result<(), io::Error>
	{
		if cfg!(any(target_os = "android", target_os = "linux"))
		{
			self.file_path("self/autogroup").write_value(nice_value)
		}
		else
		{
			Ok(())
		}
	}

	#[inline(always)]
	fn sched_autogroup_enabled_file_path(&self) -> PathBuf
	{
		self.file_path("sys/kernel/sched_autogroup_enabled")
	}

	/// Status information from `/proc/self/status`.
	#[inline(always)]
	pub fn self_status(&self) -> Result<ProcessStatusStatistics, ProcessStatusFileParseError>
	{
		self.file_path("self/status").parse_process_status_file()
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
