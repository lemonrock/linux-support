// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Read, write or read-write.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Access
{
	/// Read only.
	ReadOnly(ReadAccessTimeUpdating),

	/// Write only.
	WriteOnly(WriteSynchronization),

	/// Append only.
	AppendOnly(WriteSynchronization),

	/// Write only; truncate length to zero if the file exists.
	///
	/// Only effective for a regular file.
	/// Ignored for FIFOs and terminal character devices.
	/// Unspecified for other file types but may cause an error.
	WriteOnlyTruncate(WriteSynchronization),

	/// Append only; truncate length to zero if the file exists.
	///
	/// Only effective for a regular file.
	/// Ignored for FIFOs and terminal character devices.
	/// Unspecified for other file types but may cause an error.
	AppendOnlyTruncate(WriteSynchronization),

	/// Read and write.
	ReadWrite(ReadAccessTimeUpdating, WriteSynchronization),

	/// Read and write; truncate length to zero if the file exists.
	///
	/// Only effective for a regular file.
	/// Ignored for FIFOs and terminal character devices.
	/// Unspecified for other file types but may cause an error.
	ReadWriteTruncate(ReadAccessTimeUpdating, WriteSynchronization),

	/// Read and append.
	ReadAppend(ReadAccessTimeUpdating, WriteSynchronization),

	/// Read and append; truncate length to zero if the file exists.
	///
	/// Only effective for a regular file.
	/// Ignored for FIFOs and terminal character devices.
	/// Unspecified for other file types but may cause an error.
	ReadAppendTruncate(ReadAccessTimeUpdating, WriteSynchronization),
}

impl Access
{
	#[inline(always)]
	fn to_oflag(self) -> i32
	{
		use self::Access::*;

		match self
		{
			ReadOnly(read_access_time_updating) => O_RDONLY | read_access_time_updating as i32,
			WriteOnly(write_synchronization) => O_WRONLY | write_synchronization as i32,
			AppendOnly(write_synchronization) => O_WRONLY | O_APPEND | write_synchronization as i32,
			WriteOnlyTruncate(write_synchronization) => O_WRONLY | O_TRUNC | write_synchronization as i32,
			AppendOnlyTruncate(write_synchronization) => O_WRONLY | O_APPEND | O_TRUNC | write_synchronization as i32,
			ReadWrite(read_access_time_updating, write_synchronization) => O_RDWR | read_access_time_updating as i32 | write_synchronization as i32,
			ReadWriteTruncate(read_access_time_updating, write_synchronization) => O_RDWR | O_TRUNC | read_access_time_updating as i32 | write_synchronization as i32,
			ReadAppend(read_access_time_updating, write_synchronization) => O_RDWR | O_APPEND | read_access_time_updating as i32 | write_synchronization as i32,
			ReadAppendTruncate(read_access_time_updating, write_synchronization) => O_RDWR | O_APPEND | O_TRUNC | read_access_time_updating as i32 | write_synchronization as i32,
		}
	}
}
