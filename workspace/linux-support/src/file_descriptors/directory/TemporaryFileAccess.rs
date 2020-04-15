// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Write or read-write.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(i32)]
pub enum TemporaryFileAccess
{
	/// Write only.
	WriteOnly(WriteSynchronization),

	/// Append only.
	AppendOnly(WriteSynchronization),

	/// Read and write.
	ReadWrite(ReadAccessTimeUpdating, WriteSynchronization),
}

impl TemporaryFileAccess
{
	#[inline(always)]
	fn to_oflag(self) -> i32
	{
		use self::TemporaryFileAccess::*;

		match self
		{
			WriteOnly(write_synchronization) => O_WRONLY | write_synchronization as i32,
			AppendOnly(write_synchronization) => O_WRONLY | O_APPEND | write_synchronization as i32,
			ReadWrite(read_access_time_updating, write_synchronization) => O_RDWR | read_access_time_updating as i32 | write_synchronization as i32,
		}
	}
}
