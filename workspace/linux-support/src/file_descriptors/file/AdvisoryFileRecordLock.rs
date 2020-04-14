// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An advisory record lock kind.
///
/// A record lock is also known as a:-
///
/// * byte-range lock
/// * file-segment lock
/// * file-region lock
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum AdvisoryFileRecordLock
{
	/// A read lock.
	///
	/// Also known as a shared lock.
	///
	/// The file so-locked must be open for reading.
	Read = F_RDLCK,

	/// A write lock.
	///
	/// Also known as an exclusive lock.
	Write = F_WRLCK,
}
