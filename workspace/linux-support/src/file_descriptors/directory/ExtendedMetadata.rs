// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Extended metadata.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ExtendedMetadata(statx);

macro_rules! extended_metadata_field
{
	($self: ident, $field: ident, $bit: ident) =>
	{
		extended_metadata_field!($self, $field, $bit, |v| v);
	};

	($self: ident, $field: ident, $bit: ident, $transform: expr) =>
	{
		$self.value(&($self.0).$field, ExtendedMetadataWanted::$bit, $transform)
	}
}

macro_rules! timestamp_extended_metadata_field
{
	($self: ident, $field: ident, $bit: ident) =>
	{
		$self.timestamp_value(&($self.0).$field, ExtendedMetadataWanted::$bit)
	}
}

impl ExtendedMetadata
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn filesystem_block_device(&self) -> BlockDevice
	{
		BlockDevice::from((self.0.stx_dev_major, self.0.stx_dev_minor))
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn filesystem_block_size(&self) -> blksize_t
	{
		self.0.stx_blksize as blksize_t
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn file_attributes(&self) -> FileAttributesSubset
	{
		self.0.stx_attributes
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn supported_file_attributes(&self) -> FileAttributesSubset
	{
		self.0.stx_attributes_mask
	}

	/// The number of blocks allocated to the file on the medium, in 512-byte units.
	/// (This may be smaller than `self.size() / 512` when the file has holes).
	#[inline(always)]
	pub fn size_in_512_byte_blocks(&self) -> Option<blkcnt_t>
	{
		extended_metadata_field!(self, stx_blocks, SizeIn512ByteBlocks, |n| n as blkcnt_t)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn number_of_hard_links(&self) -> Option<nlink_t>
	{
		extended_metadata_field!(self, stx_nlink, NumberOfHardLinks, |n| n as nlink_t)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn user_identifier(&self) -> Option<UserIdentifier>
	{
		extended_metadata_field!(self, stx_uid, UserIdentifier, UserIdentifier::from)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn group_identifier(&self) -> Option<GroupIdentifier>
	{
		extended_metadata_field!(self, stx_gid, GroupIdentifier, GroupIdentifier::from)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn inode(&self) -> Option<Inode>
	{
		extended_metadata_field!(self, stx_ino, Inode, Inode::from)
	}

	/// Non-zero and potentially available only if `file_type() or file_type_and_access_permissions().file_type()` is `RegularFile` or `SymbolicLink`.
	#[inline(always)]
	pub fn size(&self) -> Option<u64>
	{
		extended_metadata_field!(self, stx_size, Size)
	}

	/// Potentially available only if `file_type() or file_type_and_access_permissions().file_type()` is `BlockDevice`.
	#[inline(always)]
	pub fn block_device(&self) -> Option<BlockDevice>
	{
		self.rdev_device_value()
	}

	/// Potentially available only if `file_type() or file_type_and_access_permissions().file_type()` is `CharacterDevice`.
	#[inline(always)]
	pub fn character_device(&self) -> Option<CharacterDevice>
	{
		self.rdev_device_value()
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn last_accessed_at_time(&self) -> Option<SystemTime>
	{
		timestamp_extended_metadata_field!(self, stx_atime, LastAccessedAtTime)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn last_modified_at_time(&self) -> Option<SystemTime>
	{
		timestamp_extended_metadata_field!(self, stx_mtime, LastModifiedAtTime)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn last_changed_at_time(&self) -> Option<SystemTime>
	{
		timestamp_extended_metadata_field!(self, stx_ctime, LastChangedAtTime)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn created_at_time(&self) -> Option<SystemTime>
	{
		timestamp_extended_metadata_field!(self, stx_btime, CreatedAtTime)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn file_type(&self) -> Option<FileType>
	{
		extended_metadata_field!(self, stx_mode, Type, FileType::from)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn access_permissions(&self) -> Option<AccessPermissions>
	{
		extended_metadata_field!(self, stx_mode, Type, AccessPermissions::from)
	}

	#[allow(missing_docs)]
	#[inline(always)]
	pub fn file_type_and_access_permissions(&self) -> Option<FileTypeAndAccessPermissions>
	{
		if likely!(self.0.stx_mask.contains(ExtendedMetadataWanted::Type | ExtendedMetadataWanted::Mode))
		{
			Some(FileTypeAndAccessPermissions::from(self.0.stx_mode))
		}
		else
		{
			None
		}
	}

	#[inline(always)]
	fn value<V: Copy, T>(&self, field: &V, bit: ExtendedMetadataWanted, transform: impl FnOnce(V) -> T) -> Option<T>
	{
		if likely!(self.0.stx_mask.contains(bit))
		{
			Some(transform(*field))
		}
		else
		{
			None
		}
	}

	#[inline(always)]
	fn timestamp_value(&self, field: &statx_timestamp, bit: ExtendedMetadataWanted) -> Option<SystemTime>
	{
		if likely!(self.0.stx_mask.contains(bit))
		{
			Some(UNIX_EPOCH + Duration::new(field.tv_sec as u64, field.tv_nsec))
		}
		else
		{
			None
		}
	}

	#[inline(always)]
	fn rdev_device_value<D: From<(u32, u32)>>(&self) -> Option<D>
	{
		if likely!(self.0.stx_mask.contains(ExtendedMetadataWanted::Type))
		{
			Some(D::from((self.0.stx_rdev_major, self.0.stx_rdev_minor)))
		}
		else
		{
			None
		}
	}
}
