// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Module parameter value.
///
/// From Linux source `params.c`  definitions, the standard types are:-
///
/// * byte, short, ushort, int, uint, long, ulong, ullong.
/// * charp: a character pointer
/// * bool: a bool, values 0/1, y/n, Y/N.
/// * invbool: the above, only sense-reversed (N = true).
///
/// For how these are formatted in sysfs, see functions such as `param_get_invbool()` in `kernel/params.c`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[derive(EnumDiscriminants)]
#[strum_discriminants(name(ModuleParameterValueKind))]
#[strum_discriminants(derive(PartialOrd))]
#[strum_discriminants(derive(Ord))]
#[strum_discriminants(derive(Hash))]
#[strum_discriminants(derive(Deserialize))]
#[strum_discriminants(derive(Serialize))]
pub enum ModuleParameterValueChoice
{
	/// `byte`.
	byte(u8),
	
	/// `short`.
	short(i16),
	
	/// `ushort`.
	ushort(u16),
	
	/// `int`.
	int(i32),
	
	/// `uint`.
	uint(u32),
	
	/// `long`.
	long(isize),
	
	/// `ulong`.
	ulong(usize),
	
	/// `ullong`.
	ullong(u64),
	
	/// `charp`.
	charp(StringLinuxKernelModuleParameterValue),
	
	/// `bool`.
	bool(bool),
	
	/// `invbool`.
	invbool(InverseBooleanModuleParameterValue),
	
	/// `int` but restricted to values `0` to `16` inclusive.
	intel_ixgbevf_debug_level(IntelIxgbevfDebugLevelModuleParameterValue),
}

impl ModuleParameterValueChoice
{
	/// Write.
	#[inline(always)]
	pub fn write(&self, sys_path: &SysPath, module_name: &LinuxKernelModuleName, parameter_name: &CStr) -> io::Result<()>
	{
		use self::ModuleParameterValueChoice::*;
		
		match self
		{
			byte(module_parameter_value) => module_parameter_value.write_linux_module_parameter(sys_path, module_name, parameter_name),
			
			short(module_parameter_value) => module_parameter_value.write_linux_module_parameter(sys_path, module_name, parameter_name),
			
			ushort(module_parameter_value) => module_parameter_value.write_linux_module_parameter(sys_path, module_name, parameter_name),
			
			int(module_parameter_value) => module_parameter_value.write_linux_module_parameter(sys_path, module_name, parameter_name),
			
			uint(module_parameter_value) => module_parameter_value.write_linux_module_parameter(sys_path, module_name, parameter_name),
			
			long(module_parameter_value) => module_parameter_value.write_linux_module_parameter(sys_path, module_name, parameter_name),
			
			ulong(module_parameter_value) => module_parameter_value.write_linux_module_parameter(sys_path, module_name, parameter_name),
			
			ullong(module_parameter_value) => module_parameter_value.write_linux_module_parameter(sys_path, module_name, parameter_name),
			
			charp(module_parameter_value) => module_parameter_value.write_linux_module_parameter(sys_path, module_name, parameter_name),
			
			bool(module_parameter_value) => module_parameter_value.write_linux_module_parameter(sys_path, module_name, parameter_name),
			
			invbool(module_parameter_value) => module_parameter_value.write_linux_module_parameter(sys_path, module_name, parameter_name),
			
			intel_ixgbevf_debug_level(module_parameter_value) => module_parameter_value.write_linux_module_parameter(sys_path, module_name, parameter_name),
		}
	}
}
