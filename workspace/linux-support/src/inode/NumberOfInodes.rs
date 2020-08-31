// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Number of inodes.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct NumberOfInodes(pub usize);

impl From<usize> for NumberOfInodes
{
	#[inline(always)]
	fn from(value: usize) -> Self
	{
		Self(value)
	}
}

impl Into<usize> for NumberOfInodes
{
	#[inline(always)]
	fn into(self) -> usize
	{
		self.0
	}
}

impl NumberOfInodes
{
	/// Allocated and free as reported by `/proc/sys/fs/inode-nr`.
	///
	/// Allocated + free will be the maximum.
	#[inline(always)]
	pub fn allocated_and_free(proc_path: &ProcPath) -> io::Result<(Self, Self)>
	{
		let bytes = proc_path.sys_fs_file_path("inode-nr").read_raw_without_line_feed()?;
		let mut fields = bytes.split_bytes_n(2, b'\t');

		#[inline(always)]
		fn next<'a>(fields: &mut impl Iterator<Item=&'a [u8]>) -> NumberOfInodes
		{
			NumberOfInodes(usize::parse_decimal_number(fields.next().unwrap()).unwrap())
		}
		Ok((next(&mut fields), next(&mut fields)))
	}
}
