// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Extends `SeekFrom`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExtendedSeekFrom
{
	/// Sets the offset to the provided number of bytes.
	Start(u64),

	/// Sets the offset to the size of this object plus the specified number of bytes.
	///
	/// It is possible to seek beyond the end of an object, but it's an error to seek before byte 0.
	End(i64),

	/// Sets the offset to the current position plus the specified number of bytes.
	///
	/// It is possible to seek beyond the end of an object, but it's an error to seek before byte 0.
	Current(i64),

	/// Adjust the file offset to the next location in the file greater than or equal to offset containing data.
	///
	/// If offset points to data, then the file offset is set to offset.
	///
	/// Since Linux 3.1.
	Hole(i64),

	/// Adjust the file offset to the next hole in the file greater than or equal to offset.
	///
	/// If offset points into the middle of a hole, then the file offset is set to offset.
	/// If there is no hole past offset, then the file offset is adjusted to the end of the file (ie, there is an implicit hole at the end of any file).
	///
	/// Since Linux 3.1.
	Data(i64),
}

impl From<SeekFrom> for ExtendedSeekFrom
{
	#[inline(always)]
	fn from(value: SeekFrom) -> Self
	{
		match value
		{
			SeekFrom::Start(value) => ExtendedSeekFrom::Start(value),
			SeekFrom::End(value) => ExtendedSeekFrom::End(value),
			SeekFrom::Current(value) => ExtendedSeekFrom::Current(value),
		}
	}
}

impl ExtendedSeekFrom
{
	#[inline(always)]
	fn to_whence_and_start(self) -> (i16, i64)
	{
		use self::ExtendedSeekFrom::*;

		match self
		{
			Start(start) => (SEEK_SET as i16, start.try_into().expect("We do not support extremely large offsets that do not fit in an i64")),
			End(start) => (SEEK_END as i16, start),
			Current(start) => (SEEK_CUR as i16, start),
			Hole(start) => (SEEK_HOLE as i16, start),
			Data(start) => (SEEK_DATA as i16, start),
		}
	}

	#[inline(always)]
	fn from_whence_and_start(l: &flock) -> Self
	{
		use self::ExtendedSeekFrom::*;

		match l.l_whence as i32
		{
			SEEK_SET => Start(l.l_start.try_into().expect("We do not support negative offsets that do not fit in an u64")),
			SEEK_END => End(l.l_start),
			SEEK_CUR => Current(l.l_start),
			SEEK_HOLE => Hole(l.l_start),
			SEEK_DATA => Data(l.l_start),
			unknown @ _ => panic!("Unknown l_whence {}", unknown),
		}
	}
}
