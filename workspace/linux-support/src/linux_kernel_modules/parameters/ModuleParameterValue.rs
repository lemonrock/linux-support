// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// From Linux source `include/linux/moduleparam.h`, the standard types are:-
///
/// * byte, short, ushort, int, uint, long, ulong
/// * charp: a character pointer
/// * bool: a bool, values 0/1, y/n, Y/N.
/// * invbool: the above, only sense-reversed (N = true).
///
/// For how these are formatted in sysfs, see functions such as `param_get_invbool()` in `kernel/params.c`.
pub trait ModuleParameterValue: Sized
{
	#[doc(hidden)]
	#[inline(always)]
	fn read_linux_module_parameter(sys_path: &SysPath, module_name: &LinuxKernelModuleName, parameter_name: &CStr) -> io::Result<Option<Self>>
	{
		if let Some(parameter_file_path) = Self::parameter_file_name(sys_path, module_name, parameter_name)
		{
			let value = parameter_file_path.read_raw_without_line_feed()?;
			Self::parse_bytes(value).map(|value| Some(value))
		}
		else
		{
			Ok(None)
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn write_linux_module_parameter(&self, sys_path: &SysPath, module_name: &LinuxKernelModuleName, parameter_name: &CStr) -> io::Result<()>
	{
		if let Some(parameter_file_path) = Self::parameter_file_name(sys_path, module_name, parameter_name)
		{
			self.write_value(parameter_file_path)
		}
		else
		{
			Ok(())
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn parameter_file_name(sys_path: &SysPath, module_name: &LinuxKernelModuleName, parameter_name: &CStr) -> Option<PathBuf>
	{
		let parameter_file_path = sys_path.module_file_or_folder_path(module_name, "parameters").append(OsStr::from_bytes(parameter_name.to_bytes()));
		if parameter_file_path.exists()
		{
			Some(parameter_file_path)
		}
		else
		{
			None
		}
	}
	
	fn parse_bytes(bytes: Box<[u8]>) -> io::Result<Self>;
	
	fn write_value(&self, extant_parameter_file_path: PathBuf) -> io::Result<()>;
}
