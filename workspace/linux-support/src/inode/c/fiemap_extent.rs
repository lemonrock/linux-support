// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A file extent (range of bytes that is not sparse).
///
/// File extents should not overlap.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct fiemap_extent
{
	fe_logical: u64,

	fe_physical: u64,

	fe_length: u64,

	fe_reserved64: [u64; 2],

	fe_flags: FileExtentFlags,

	fe_reserved: [u32; 3],
}

impl fiemap_extent
{
	/// Logical range in bytes of the extent.
	///
	/// This is relative to the start of the file.
	///
	/// It is valid for an extents logical offset to start before the range requested or its logical length to extend past that requested.
	#[inline(always)]
	pub const fn logical_range_in_bytes(&self) -> Range<u64>
	{
		self.fe_logical .. (self.fe_logical + self.fe_length)
	}

	/// Physical range in bytes of the extent.
	///
	/// This is relative to the start of the file.
	#[inline(always)]
	pub const fn physical_range_in_bytes(&self) -> Range<u64>
	{
		self.fe_physical .. (self.fe_physical + self.fe_length)
	}

	/// Physical range in bytes of the extent.
	#[inline(always)]
	pub const fn flags(&self) -> FileExtentFlags
	{
		self.fe_flags
	}
}
