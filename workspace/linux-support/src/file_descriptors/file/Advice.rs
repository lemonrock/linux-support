// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Access pattern advice.
///
/// Default is `Normal`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum Advice
{
	/// Indicates that the application has no advice to give about its access pattern for the specified data.
	///
	/// If no advice is given for an open file, this is the default assumption.
	Normal = POSIX_FADV_NORMAL,

	/// The application expects to access the specified data sequentially (with lower offsets read before higher ones).
	Sequential = POSIX_FADV_SEQUENTIAL,

	/// The specified data will be accessed in random order.
	Random = POSIX_FADV_RANDOM,

	/// The specified data will be accessed only once.
	NoReuse = POSIX_FADV_NOREUSE,

	/// The specified data will be accessed in the near future.
	WillNeed = POSIX_FADV_WILLNEED,

	/// The specified data will not be accessed in the near future.
	WillNotNeed = POSIX_FADV_DONTNEED,
}

impl Default for Advice
{
	#[inline(always)]
	fn default() -> Self
	{
		Advice::Normal
	}
}
