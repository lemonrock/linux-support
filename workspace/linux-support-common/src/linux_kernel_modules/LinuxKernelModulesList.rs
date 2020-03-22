// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A list of Linux kernel modules loaded on the system.
///
/// Is *not* updated if a module is loaded or unloaded.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct LinuxKernelModulesList(HashSet<LinuxKernelModuleName>);

impl LinuxKernelModulesList
{
	/// Unloads a Linux kernel module.
	///
	/// Does not use `modprobe`.
	///
	/// true if unloaded.
	/// false if does not exist.
	pub fn unload_linux_kernel_module(linux_kernel_module_name: &LinuxKernelModuleName) -> Result<bool, io::Error>
	{
		use self::ErrorKind::*;

		let name: CString = linux_kernel_module_name.into();
		const flags: c_long = O_NONBLOCK as c_long;

		match unsafe { syscall(SYS_delete_module as i64, name.as_ptr(), flags) }
		{
			0 => Ok(true),
			-1 => match errno().0
			{
				EPERM => Err(io::Error::new(PermissionDenied, "permission denied")),
				EBUSY => Err(io::Error::new(PermissionDenied, "busy")),
				ENOENT => Ok(false),
				EWOULDBLOCK => Err(io::Error::new(PermissionDenied, "In use")),

				EFAULT => panic!("EFAULT should not occur"),

				unknown @ _ => panic!("syscall(SYS_delete_module) failed with illegal error code '{}'", unknown),
			},
			illegal @ _ => panic!("syscall(SYS_delete_module) returned illegal value '{}'", illegal),
		}
	}

	/// Loads a Linux Kernel Module.
	///
	/// `module_file_base_name` excludes the `.ko` file extension.
	///
	/// Does not use `modprobe`.
	///
	/// Returns true if loaded.
	/// Returns false if already loaded.
	///
	/// Updates the list of loaded modules.
	pub fn load_linux_kernel_module_if_absent_from_ko_file(&mut self, linux_kernel_module: &LinuxKernelModule, linux_kernel_modules_path: &Path) -> Result<bool, io::Error>
	{
		let linux_kernel_module_name = linux_kernel_module.linux_kernel_module_name();

		if self.is_linux_kernel_module_is_loaded(linux_kernel_module_name)
		{
			return Ok(false)
		}

		linux_kernel_module.load_linux_kernel_module_from_ko_file(linux_kernel_modules_path)?;
		self.0.insert(linux_kernel_module_name.clone());
		Ok(true)
	}

	/// Loads a module if absent from the Kernel.
	///
	/// Uses `modprobe`.
	///
	/// Updates the list of loaded modules.
	pub fn load_linux_kernel_module_if_absent_using_modprobe(&mut self, linux_kernel_module: &LinuxKernelModule) -> Result<bool, ModProbeError>
	{
		let linux_kernel_module_name = linux_kernel_module.linux_kernel_module_name();

		if self.is_linux_kernel_module_is_loaded(linux_kernel_module_name)
		{
			return Ok(false)
		}

		linux_kernel_module.load_linux_kernel_module_using_modprobe()?;
		self.0.insert(linux_kernel_module_name.clone());
		Ok(true)
	}

	/// Is the Linux kernel module loaded?
	#[inline(always)]
	pub fn is_linux_kernel_module_is_loaded(&self, linux_kernel_module_name: &LinuxKernelModuleName) -> bool
	{
		self.0.contains(linux_kernel_module_name)
	}

	/// Current loaded Linux kernel modules (from `/proc/modules`).
	#[inline(always)]
	pub fn parse(proc_path: &ProcPath) -> Result<Self, LinuxKernelModulesListParseError>
	{
		let file_path = proc_path.file_path("modules");

		let reader = BufReader::with_capacity(4096, File::open(file_path)?);

		let mut modules_list = HashSet::new();
		let mut zero_based_line_number = 0;
		use self::LinuxKernelModulesListParseError::*;
		for line in reader.split(b'\n')
		{
			{
				let line = line?;
				let mut split = splitn(&line, 2, b' ');

				let linux_kernel_module_name_bytes = split.next().unwrap();

				if linux_kernel_module_name_bytes.is_empty()
				{
					return Err(CouldNotParseEmptyModuleName { zero_based_line_number })
				}

				let linux_kernel_module_name = linux_kernel_module_name_bytes.into();

				if let Some(duplicated) = modules_list.replace(linux_kernel_module_name)
				{
					return Err(DuplicateModuleName { zero_based_line_number, linux_kernel_module_name: duplicated });
				}
			}

			zero_based_line_number += 1;
		}

		Ok(LinuxKernelModulesList(modules_list))
	}
}
