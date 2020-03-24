// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A Linux kernel module name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinuxKernelModuleName(Box<[u8]>);

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
	/// Loads a Linux Kernel Module.
	///
	/// Uses the `modprobe` binary.
	pub fn load_linux_kernel_module_using_modprobe(&self) -> Result<(), ModProbeError>
	{
		assert!(!self.0.starts_with(b"-"), "{:?} starts with a hyphen. This confuses some modprobe implementations (and some don't support `--` parsing it seems)", self);

		assert_effective_user_id_is_root(&format!("modprobe of {:?}", self));

		#[inline(always)]
		fn new_command_in_clean_environment(command: &str) -> Command
		{
			let mut command = Command::new(command);
			command.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).env_clear();
			if let Some(path) = var_os("PATH")
			{
				command.env("PATH", path);
			}
			command
		}

		let os_str = OsStr::from_bytes(&self.0[..]);
		let exit_code = new_command_in_clean_environment("modprobe").arg("-s").arg("-b").arg(os_str).status()?;

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
}
