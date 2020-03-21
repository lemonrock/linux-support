// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Cause of error when trying to parse list of Linux linux_kernel_modules.
#[derive(Debug)]
pub enum LinuxKernelModulesListParseError
{
	/// Could not open file list of Linux kernel linux_kernel_modules.
	CouldNotOpenFile(io::Error),

	/// A module name was empty.
	CouldNotParseEmptyModuleName
	{
		/// Zero-based line number in the file the error occurred at.
		zero_based_line_number: usize,
	},

	/// A module name was duplicated.
	DuplicateModuleName
	{
		/// Zero-based line number in the file the error occurred at.
		zero_based_line_number: usize,

		/// The Linux kernel module name (not necessarily UTF-8).
		linux_kernel_module_name: Box<[u8]>,
	},
}

impl Display for LinuxKernelModulesListParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for LinuxKernelModulesListParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use self::LinuxKernelModulesListParseError::*;

		match self
		{
			&CouldNotOpenFile(ref error) => Some(error),

			&CouldNotParseEmptyModuleName { .. } => None,

			&DuplicateModuleName { .. } => None,
		}
	}
}

impl From<io::Error> for LinuxKernelModulesListParseError
{
	#[inline(always)]
	fn from(error: io::Error) -> Self
	{
		LinuxKernelModulesListParseError::CouldNotOpenFile(error)
	}
}
