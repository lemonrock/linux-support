// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Status.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HyperThreadingStatus
{
	/// Enabled.
	On,

	/// Disabled.
	Off,

	/// Forcibly disabled; status can not be changed.
	ForceOff,

	/// Not supported by the CPU.
	NotSupported,

	/// Not supported by the CPU architecture.
	NotImplemented,
}

impl FromStr for HyperThreadingStatus
{
	type Err = io::Error;

	#[inline(always)]
	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		use self::HyperThreadingStatus::*;

		let value = match s
		{
			"on" => On,
			"off" => Off,
			"forceoff" => ForceOff,
			"notsupported" => ForceOff,
			"notimplemented" => ForceOff,
			_ => return Err(io::Error::new(ErrorKind::Other, "Unknown value")),
		};
		Ok(value)
	}
}
