// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Linux kernel version information.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LinuxKernelVersion
{
	/// Contents of `/proc/sys/kernel/osrelease`.
	///
	/// Equivalent to `uname -r`.
	/// Equivalent to `UTS_RELEASE` macro.
	///
	/// Contains something like `5.4.27-0-virt`.
	pub release: Box<[u8]>,

	/// Contents of `/proc/sys/kernel/version`.
	///
	/// Equivalent to `UTS_VERSION` macro.
	///
	/// A string such as `#5 Wed Feb 25 21:49:24 MET 1998`.
	/// The `#5` means that this is the fifth kernel built from this source base and the date following it indicates the time the kernel was built.
	pub timestamp: Box<[u8]>,

	/// Contents of `/proc/version`.
	///
	/// Similar to, but not identical to, `uname -a`.
	pub full: Box<[u8]>,
}

impl LinuxKernelVersion
{
	/// Version.
	#[inline(always)]
	pub fn major_minor_revision(&self) -> LinuxKernelVersionNumber
	{
		// eg `5.4.27`.
		let left = self.release.split_bytes_n(2, b'-').next().unwrap();

		let mut parts = left.split_bytes_n(3, b'.');
		let major = u16::parse_decimal_number(parts.next().unwrap()).unwrap();
		let minor = u16::parse_decimal_number(parts.next().unwrap()).unwrap();
		let revision = u16::parse_decimal_number(parts.next().unwrap()).unwrap();

		LinuxKernelVersionNumber
		{
			major,
			minor,
			revision,
		}
	}
	
	/// Equivalent to `uname`, `uname -s` or `uname -o`.
	/// Equivalent to `UTS_SYSNAME`.
	#[inline(always)]
	fn verify_ostype(proc_path: &ProcPath) -> io::Result<()>
	{
		let type_ = proc_path.sys_kernel_file_path("ostype").read_raw_without_line_feed()?;
		if &type_[..] == b"Linux"
		{
			Ok(())
		}
		else
		{
			Err(io::Error::new(ErrorKind::InvalidData, format!("This is not Linux but {:?}", type_)))
		}
	}
	
	/// Parse.
	///
	/// Panics if not Linux.
	#[inline(always)]
	pub fn parse(proc_path: &ProcPath) -> io::Result<Self>
	{
		Self::verify_ostype(proc_path);

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
