// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct statx
{
	pub(super) stx_mask: ExtendedMetadataWanted,
	pub(super) stx_blksize: u32,
	pub(super) stx_attributes: FileAttributesSubset,
	pub(super) stx_nlink: u32,
	pub(super) stx_uid: u32,
	pub(super) stx_gid: u32,
	pub(super) stx_mode: u16,
	__statx_pad1: u16,
	pub(super) stx_ino: u64,
	pub(super) stx_size: u64,
	pub(super) stx_blocks: u64,
	pub(super) stx_attributes_mask: FileAttributesSubset,
	pub(super) stx_atime: statx_timestamp,
	pub(super) stx_btime: statx_timestamp,
	pub(super) stx_ctime: statx_timestamp,
	pub(super) stx_mtime: statx_timestamp,
	pub(super) stx_rdev_major: u32,
	pub(super) stx_rdev_minor: u32,
	pub(super) stx_dev_major: u32,
	pub(super) stx_dev_minor: u32,
	__statx_pad2: [u64; 14],
}

impl statx
{
	#[inline(always)]
	pub(super) fn zero_padding(&mut self)
	{
		self.__statx_pad1 = 0;
		self.stx_atime.zero_padding();
		self.stx_btime.zero_padding();
		self.stx_ctime.zero_padding();
		self.stx_mtime.zero_padding();
		self.__statx_pad2 = unsafe_zeroed();
	}
}
