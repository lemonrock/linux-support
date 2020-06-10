// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Linux kernel version information.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LinuxKernelVersionNumber
{
	/// eg `5`.
	pub major: u16,

	/// eg `4`.
	pub minor: u16,

	/// eg `27`.
	#[serde(default)] pub revision: u16,
}

impl LinuxKernelVersionNumber
{
	/// Minimum for io_uring support as coded in this crate.
	pub const MinimumForIoUringSupport: Self = Self
	{
		major: 5,
		minor: 7,
		revision: 1,
	};
}
