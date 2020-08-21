// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A Linux kernel module name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct LinuxKernelModuleName(#[serde(with = "serde_bytes")] Box<[u8]>);

impl From<&[u8]> for LinuxKernelModuleName
{
	#[inline(always)]
	fn from(value: &[u8]) -> Self
	{
		Self::from(value.to_vec())
	}
}

impl From<Box<[u8]>> for LinuxKernelModuleName
{
	#[inline(always)]
	fn from(value: Box<[u8]>) -> Self
	{
		Self(value)
	}
}

impl From<Vec<u8>> for LinuxKernelModuleName
{
	#[inline(always)]
	fn from(value: Vec<u8>) -> Self
	{
		Self(value.into())
	}
}

impl From<String> for LinuxKernelModuleName
{
	#[inline(always)]
	fn from(value: String) -> Self
	{
		Self::from(value.into_bytes())
	}
}

impl Into<CString> for LinuxKernelModuleName
{
	#[inline(always)]
	fn into(self) -> CString
	{
		CString::new(&self.0[..]).unwrap()
	}
}

impl<'a> Into<CString> for &'a LinuxKernelModuleName
{
	#[inline(always)]
	fn into(self) -> CString
	{
		CString::new(&self.0[..]).unwrap()
	}
}

impl Into<OsString> for LinuxKernelModuleName
{
	#[inline(always)]
	fn into(self) -> OsString
	{
		OsString::from_vec(self.0.to_vec())
	}
}

impl<'a> Into<OsString> for &'a LinuxKernelModuleName
{
	#[inline(always)]
	fn into(self) -> OsString
	{
		OsString::from_vec(self.0.clone().to_vec())
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for &'a LinuxKernelModuleName
{
	/// Converts data to a byte string terminated with a new line (`\n`).
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		let mut value = self.0.clone().into_vec();
		value.push(b'\n');
		Cow::from(value)
	}
}

impl LinuxKernelModuleName
{
	/// If module loading is supported then the default is `/sbin/modprobe`.
	///
	/// Reads `/proc/sys/kernel/modprobe`.
	#[inline(always)]
	pub fn modprobe_executable_path(proc_path: &ProcPath) -> Option<PathBuf>
	{
		let file_path = Self::modprobe_file_path(proc_path);
		if file_path.exists()
		{
			let bytes = file_path.read_raw_without_line_feed().unwrap();
			Some(PathBuf::from(OsString::from_vec(bytes.into_vec())))
		}
		else
		{
			None
		}
	}

	/// If module loading is supported then the default is `/sbin/modprobe`.
	///
	/// Does not write (and does not error) if module loading is not supported.
	///
	/// Verifies `modprobe_executable_path` is valid (extant, readable, executable, owned by root and a regular file) before setting it.
	#[inline(always)]
	pub fn set_modprobe_executable_path(proc_path: &ProcPath, modprobe_executable_path: &impl AsRef<Path>) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write /proc/sys/kernel/modprobe");

		let file_path = Self::modprobe_file_path(proc_path);
		if file_path.exists()
		{
			fn validate_modprobe_path(modprobe_executable_path: &impl AsRef<Path>) -> io::Result<&Path>
			{
				let modprobe_executable_path = modprobe_executable_path.as_ref();

				let path_file_descriptor = PathFileDescriptor::open(&path_to_cstring(modprobe_executable_path), false, false)?;
				let metadata = path_file_descriptor.metadata_of_self()?;
				if unlikely!(!metadata.user_identifier().is_root())
				{
					return Err(io_error_other("Not owned by root"))
				}
				if unlikely!(!metadata.file_type().is_regular_file())
				{
					return Err(io_error_other("Not a regular file"))
				}
				if unlikely!(!metadata.access_permissions().user().is_readable_and_executable())
				{
					return Err(io_error_other("Not a readable, executable file"))
				}
				Ok(modprobe_executable_path)
			}

			file_path.write_value(validate_modprobe_path(modprobe_executable_path)?)
		}
		else
		{
			Ok(())
		}
	}

	/// Usually `false` but not necessarily so.
	#[inline(always)]
	pub fn is_module_loading_and_unloading_disabled(proc_path: &ProcPath) -> bool
	{
		let file_path = Self::modules_disabled_file_path(proc_path);
		if file_path.exists()
		{
			file_path.read_zero_or_one_bool().unwrap()
		}
		else
		{
			true
		}
	}

	/// Disables if not already disabled.
	///
	/// Does not error if already disabled.
	#[inline(always)]
	pub fn disable_module_loading_and_unloading_until_reboot(proc_path: &ProcPath) -> io::Result<()>
	{
		assert_effective_user_id_is_root("write /proc/sys/kernel/modules_disabled");

		if Self::is_module_loading_and_unloading_disabled(proc_path)
		{
			return Ok(())
		}
		Self::modules_disabled_file_path(proc_path).write_value(true)
	}

	#[inline(always)]
	fn modprobe_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_kernel_file_path("modprobe")
	}

	#[inline(always)]
	fn modules_disabled_file_path(proc_path: &ProcPath) -> PathBuf
	{
		proc_path.sys_kernel_file_path("modules_disabled")
	}

	/// Loads a Linux Kernel Module.
	///
	/// Uses the `modprobe` binary.
	pub fn load_linux_kernel_module_using_modprobe(&self, modprobe_path: &Path) -> Result<(), ModProbeError>
	{
		assert!(!self.0.starts_with(b"-"), "{:?} starts with a hyphen. This confuses some modprobe implementations (and some don't support `--` parsing it seems)", self);

		assert_effective_user_id_is_root(&format!("modprobe of {:?}", self));

		#[inline(always)]
		fn new_command_in_clean_environment(modprobe_path: &Path) -> Command
		{
			let mut command = Command::new(modprobe_path);
			command.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).env_clear();
			if let Some(path) = var_os("PATH")
			{
				command.env("PATH", path);
			}
			command
		}

		let os_str = OsStr::from_bytes(&self.0[..]);
		let exit_code = new_command_in_clean_environment(modprobe_path).arg("-s").arg("-b").arg(os_str).status()?;

		use self::ModProbeError::*;
		match exit_code.code()
		{
			None => Err(SignalTerminatedExitCode(self.clone())),

			Some(exit_code) => match exit_code
			{
				0 => Ok(()),

				_ => Err(NonZeroExitCode { linux_kernel_module_name: self.clone(), exit_code }),
			}
		}
	}

	/// Unloads a Linux kernel module.
	///
	/// Does not use `modprobe`.
	///
	/// true if unloaded.
	/// false if does not exist.
	pub fn unload_linux_kernel_module(&self) -> Result<bool, io::Error>
	{
		let name: CString = self.into();
		const flags: c_long = O_NONBLOCK as c_long;

		match SYS::delete_module.syscall2(name.as_ptr() as usize, flags as usize)
		{
			0 => Ok(true),
			-1 => match errno().0
			{
				EPERM => Err(io_error_permission_denied("permission denied")),
				EBUSY => Err(io_error_permission_denied("busy")),
				ENOENT => Ok(false),
				EWOULDBLOCK => Err(io_error_permission_denied("In use")),

				EFAULT => panic!("EFAULT should not occur"),

				unknown @ _ => panic!("syscall(SYS_delete_module) failed with illegal error code '{}'", unknown),
			},
			illegal @ _ => panic!("syscall(SYS_delete_module) returned illegal value '{}'", illegal),
		}
	}
}
