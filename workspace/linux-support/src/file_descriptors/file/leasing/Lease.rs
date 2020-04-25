// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A file lease.
///
/// A record lock is also known as a:-
///
/// * byte-range lock
/// * file-segment lock
/// * file-region lock
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum Lease
{
	/// A read lease.
	///
	/// Also known as a shared lease.
	Read = F_RDLCK,

	/// A write lease.
	///
	/// Also known as an exclusive lease.
	Write = F_WRLCK,
}
