// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Linux kernel version information.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LinuxKernelVersion
{
	/// Contents of `/proc/sys/kernel/osrelease`.
	///
	/// Equivalent to `uname -r`.
	pub release: Box<[u8]>,

	/// Contents of `/proc/sys/kernel/version`.
	pub timestamp: Box<[u8]>,

	/// Contents of `/proc/version`.
	///
	/// Similar to, but not identical to, `uname -a`.
	pub full: Box<[u8]>,
}

impl LinuxKernelVersion
{
	/// Parse.
	///
	/// Panics if not Linux.
	#[inline(always)]
	pub fn parse(proc_path: &ProcPath) -> io::Result<Self>
	{
		{
			// Equivalent to `uname`, `uname -s` or `uname -o`.
			let _type = proc_path.sys_kernel_file_path("osversion").read_raw_without_line_feed()?;
			if &_type[..] != b"Linux"
			{
				panic!("This is not Linux");
			}
		}

		Ok
		(
			Self
			{
				release: proc_path.sys_kernel_file_path("osrelease").read_raw_without_line_feed()?,
				timestamp: proc_path.sys_kernel_file_path("version").read_raw_without_line_feed()?,
				full: proc_path.file_path("version").read_raw_without_line_feed()?
			}
		)
	}
}
