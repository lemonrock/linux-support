// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// An error that can occur during creation of a socket instance.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SocketAcceptError
{
	/// The per-process limit on the number of open file descriptors would be exceeded.
	PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,

	/// The system-wide limit on the total number of open files would be exceeded.
	SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,

	/// Kernel would be out of memory.
	KernelWouldBeOutOfMemory,

	/// Interrupted.
	Interrupted,

	/// No more connections to accept for now.
	Again,

	/// Connection establishment failed in some way.
	ConnectionFailed(ConnectionFailedReason),
}

impl Display for SocketAcceptError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<SocketAcceptError as Debug>::fmt(self, f)
	}
}

impl error::Error for SocketAcceptError
{
}
