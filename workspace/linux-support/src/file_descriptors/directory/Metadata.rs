// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Metadata.
#[derive(Clone)]
#[repr(transparent)]
pub struct Metadata(stat);

impl Metadata
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn filesystem_block_device(&self) -> BlockDevice
	{
		BlockDevice::from(self.0.st_dev)
	}

	/// Equivalent to `Metadata::filesystem_block_size()` and `FileSystemMetadata::filesystem_preferred_block_size()`.
	#[inline(always)]
	pub fn filesystem_block_size(&self) -> u64
	{
		self.0.st_blksize as u64
	}

	/// The number of blocks allocated to the file on the medium, in 512-byte units.
	/// (This may be smaller than `self.size() / 512` when the file has holes).
	#[inline(always)]
	pub fn size_in_512_byte_blocks(&self) -> u64
	{
		self.0.st_blocks as u64
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn user_identifier(&self) -> UserIdentifier
	{
		UserIdentifier::from(self.0.st_uid)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn group_identifier(&self) -> GroupIdentifier
	{
		GroupIdentifier::from(self.0.st_gid)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn inode(&self) -> Inode
	{
		Inode::from(self.0.st_ino)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn number_of_hard_links(&self) -> nlink_t
	{
		self.0.st_nlink
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn last_accessed_at_time(&self) -> SystemTime
	{
		Self::timestamp_value(self.0.st_atime, self.0.st_atime_nsec)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn last_modified_at_time(&self) -> SystemTime
	{
		Self::timestamp_value(self.0.st_mtime, self.0.st_mtime_nsec)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn last_changed_at_time(&self) -> SystemTime
	{
		Self::timestamp_value(self.0.st_ctime, self.0.st_ctime_nsec)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn file_type(&self) -> FileType
	{
		FileType::from(self.0.st_mode)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn access_permissions(&self) -> AccessPermissions
	{
		AccessPermissions::from(self.0.st_mode)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn file_type_and_access_permissions(&self) -> FileTypeAndAccessPermissions
	{
		FileTypeAndAccessPermissions::from(self.0.st_mode)
	}

	/// Only has meaning if `file_type()` is `RegularFile` or `SymbolicLink`.
	///
	/// Does not exceed `i64::MAX as u64`.
	#[inline(always)]
	pub fn size(&self) -> u64
	{
		self.0.st_size as u64
	}

	/// Potentially available only if `file_type() or file_type_and_access_permissions().file_type()` is `BlockDevice`.
	#[inline(always)]
	pub fn block_device(&self) -> BlockDevice
	{
		self.rdev_device_value()
	}

	/// Potentially available only if `file_type() or file_type_and_access_permissions().file_type()` is `CharacterDevice`.
	#[inline(always)]
	pub fn character_device(&self) -> CharacterDevice
	{
		self.rdev_device_value()
	}

	#[inline(always)]
	fn rdev_device_value<D: From<dev_t>>(&self) -> D
	{
		D::from(self.0.st_rdev)
	}
	
	#[cfg_attr(target_env = "musl", allow(deprecated))]
	#[inline(always)]
	fn timestamp_value(sec: time_t, nsec: c_long) -> SystemTime
	{
		UNIX_EPOCH + Duration::new(sec as u64, nsec.try_into().unwrap())
	}
}
