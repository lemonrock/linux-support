// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A Linux kernel module file base name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct LinuxKernelModuleFileBaseName(#[serde(with = "serde_bytes")] Box<[u8]>);

impl From<&[u8]> for LinuxKernelModuleFileBaseName
{
	#[inline(always)]
	fn from(value: &[u8]) -> Self
	{
		Self::from(value.to_vec())
	}
}

impl From<Box<[u8]>> for LinuxKernelModuleFileBaseName
{
	#[inline(always)]
	fn from(value: Box<[u8]>) -> Self
	{
		Self(value)
	}
}

impl From<Vec<u8>> for LinuxKernelModuleFileBaseName
{
	#[inline(always)]
	fn from(value: Vec<u8>) -> Self
	{
		Self(value.into())
	}
}

impl From<String> for LinuxKernelModuleFileBaseName
{
	#[inline(always)]
	fn from(value: String) -> Self
	{
		Self::from(value.into_bytes())
	}
}

impl LinuxKernelModuleFileBaseName
{
	/// Loads a Linux Kernel Module.
	///
	/// Does not use `modprobe`.
	///
	/// Returns true if loaded.
	/// Returns false if permissions error occurred (eg was not root).
	pub fn load_linux_kernel_module_from_ko_file(&self, linux_kernel_modules_path: &Path) -> Result<bool, io::Error>
	{
		let linux_kernel_module_path = PathBuf::from(linux_kernel_modules_path).append(self.to_ko_file_name());

		let file = OpenOptions::new().read(true).open(linux_kernel_module_path)?;
		let file_descriptor = file.as_raw_fd();

		lazy_static!
		{
    		static ref options: CString = CString::new("").unwrap();
    	}
		const flags: i32 = 0;

		use crate::ErrorKind::*;
		
		match SYS::finit_module.syscall3( file_descriptor as usize, options.as_ptr() as usize, flags as usize)
		{
			0 => Ok(true),

			-1 => match errno().0
			{
				EPERM => Err(io::Error::new(PermissionDenied, "permission denied")),
				unknown @ _ => Err(io::Error::new(Other, format!("Error Code was '{}'", unknown))),
			},

			illegal @ _ => panic!("syscall(SYS_finit_module) returned illegal value '{}'", illegal),
		}
	}

	#[inline(always)]
	fn to_ko_file_name(&self) -> String
	{
		format!("{}.ko", self.to_str())
	}

	#[inline(always)]
	pub(crate) fn to_str(&self) -> &str
	{
		unsafe { from_utf8_unchecked(self.0.as_ref()) }
	}
}
