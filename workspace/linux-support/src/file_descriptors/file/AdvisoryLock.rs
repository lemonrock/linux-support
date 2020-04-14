// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Advisory lock types.
///
/// Converting a lock (shared to exclusive, or vice versa) is not guaranteed to be atomic: the existing lock is first removed, and then a new lock is established.
/// Between these two steps, a pending lock request by another process may be granted, with the result that the conversion either blocks, or fails if using a non-blocking variant.
///
/// These *DO NOT WORK* on files on NFS shares.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum AdvisoryLock
{
	/// Shared.
	///
	/// More than one process may hold a shared lock for a given file descriptor and its duplicates at a given time.
	Shared = LOCK_SH,

	/// Exclusive.
	///
	/// Only one process may hold an exclusive lock for a given file descriptor and its duplicates at a given time.
	Exclusive = LOCK_EX,
}
