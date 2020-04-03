// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An error.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SendSignalError
{
	/// The calling process does not have permission to send the signal to the target process, or, pidfd doesn't refer to the calling process, and info.si_code is invalid (see `man 2 rt_sigqueueinfo`).
	PermissionDenied,

	/// The target process does not exist (ie, it has terminated and been waited on).
	ProcessHasFinished,
}

impl Display for SendSignalError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for SendSignalError
{
}
