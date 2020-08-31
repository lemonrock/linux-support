// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// File system metadata.
#[derive(Debug, Clone)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct FileSystemMetadata(pub(crate) statvfs);

impl FileSystemMetadata
{
	/// Block size.
	///
	/// Except for NFS, the same as `filesystem_fragment_block_size()`.
	/// Equivalent to `Metadata::filesystem_block_size()` and `ExtendedMetadata::filesystem_block_size()`.
	#[inline(always)]
	pub fn filesystem_preferred_block_size(&self) -> u64
	{
		self.0.f_bsize
	}

	/// Block size.
	///
	/// Except for NFS, the same as `filesystem_preferred_block_size()`.
	#[inline(always)]
	pub fn filesystem_fragment_block_size(&self) -> u64
	{
		self.0.f_frsize
	}

	/// Multiply by `filesystem_fragment_block_size()` for number of bytes.
	///
	/// Should be equal to or greater than `number_of_free_blocks()` and `number_of_free_blocks_for_unprivileged_users()`.
	#[inline(always)]
	pub fn number_of_blocks(&self) -> u64
	{
		self.0.f_blocks
	}

	/// Multiply by `filesystem_fragment_block_size()` for number of bytes.
	///
	/// Should be equal to or less than `number_of_blocks()`.
	/// Should be equal to or greater than `number_of_free_blocks_for_unprivileged_users()`.
	#[inline(always)]
	pub fn number_of_free_blocks(&self) -> u64
	{
		self.0.f_bfree
	}

	/// Multiply by `filesystem_fragment_block_size()` for number of bytes.
	///
	/// Should be equal to or less than `number_of_inodes()`.
	/// Should be equal to or less than `number_of_free_blocks()`.
	#[inline(always)]
	pub fn number_of_free_blocks_for_unprivileged_users(&self) -> u64
	{
		self.0.f_bavail
	}

	/// Should be equal to or greater than `number_of_free_inodes()` and `number_of_free_inodes_for_unprivileged_users()`.
	#[inline(always)]
	pub fn number_of_inodes(&self) -> NumberOfInodes
	{
		NumberOfInodes(self.0.f_files as usize)
	}

	/// Should be equal to or less than `number_of_inodes()`.
	/// Should be equal to or greater than `number_of_free_inodes()`.
	#[inline(always)]
	pub fn number_of_free_inodes(&self) -> NumberOfInodes
	{
		NumberOfInodes(self.0.f_ffree)
	}

	/// Should be equal to or less than `number_of_inodes()`.
	/// Should be equal to or less than `number_of_free_inodes()`.
	#[inline(always)]
	pub fn number_of_free_inodes_for_unprivileged_users(&self) -> NumberOfInodes
	{
		NumberOfInodes(self.0.f_favail)
	}

	/// ?
	#[inline(always)]
	pub fn file_system_identifier(&self) -> u64
	{
		self.0.f_fsid
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn maximum_file_name_length(&self) -> u64
	{
		self.0.f_namemax
	}

	/// Does not return invalid flags.
	#[inline(always)]
	pub fn mount_flags(&self) -> Option<FileSystemMountFlags>
	{
		let flags = self.0.f_flag;

		if likely!(flags & ST_VALID != 0)
		{
			Some(FileSystemMountFlags::from_bits_truncate(flags))
		}
		else
		{
			None
		}
	}
}
