// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// File attributes subset; overlaps with inode flags `FS_IOC_FLAGS` (or iflags) except for `Automount`.
	///
	/// See <http://man7.org/linux/man-pages/man2/ioctl_iflags.2.html>.
	///
	/// See also the `chattr` and `lsattr` programs.
	pub struct FileAttributesSubset: u64
	{
		/// Overlaps with `FS_IOC_FLAGS`.
		///
		/// Equivalent to `STATX_ATTR_COMPRESSED`.
		const Compressed = STATX_ATTR_COMPRESSED;

		/// Overlaps with `FS_IOC_FLAGS`.
		///
		/// Equivalent to `STATX_ATTR_IMMUTABLE`.
		const Immutable = STATX_ATTR_IMMUTABLE;

		/// Overlaps with `FS_IOC_FLAGS`.
		///
		/// Equivalent to `STATX_ATTR_APPEND`.
		const Append = STATX_ATTR_APPEND;

		/// Overlaps with `FS_IOC_FLAGS`.
		///
		/// Equivalent to `STATX_ATTR_NODUMP`.
		const NoDump = STATX_ATTR_NODUMP;

		/// Overlaps with `FS_IOC_FLAGS`.
		///
		/// Equivalent to `STATX_ATTR_ENCRYPTED`.
		const Encrypted = STATX_ATTR_ENCRYPTED;

		/// *DOES NOT OVERLAP* with `FS_IOC_FLAGS`.
		///
		/// Equivalent to `STATX_ATTR_AUTOMOUNT`.
		const Automount = STATX_ATTR_AUTOMOUNT;

		/// Overlaps with `FS_IOC_FLAGS`.
		///
		/// 'fs-verity' is enabled on the file.
		///
		/// This is a powerful security setting to complement immutable files.
		///
		/// Equivalent to `STATX_ATTR_VERITY`.
		const FsVerify = STATX_ATTR_VERITY;

		#[doc(hidden)]
		const AllValidAsInodeFlags = STATX_ATTR_COMPRESSED | STATX_ATTR_IMMUTABLE | STATX_ATTR_APPEND | STATX_ATTR_NODUMP | STATX_ATTR_ENCRYPTED | STATX_ATTR_VERITY;
	}
}

/// Conversion fails if flags contains `Automount`.
///
/// Returns `Err(inode_flags)` if flags contains `Automount`; `Err(inode_flags)` contains all representable flags without `Automount`.
impl TryInto<InodeFlags> for FileAttributesSubset
{
	type Error = InodeFlags;

	#[inline(always)]
	fn try_into(self) -> Result<InodeFlags, Self::Error>
	{
		// All the values stating `STATX_ATTR_` are the same as their companions starting `FS_`, with the exception of `STATX_ATTR_AUTOMOUNT`.
		let bits = self.bits;
		if unlikely!(bits & STATX_ATTR_AUTOMOUNT != 0)
		{
			Err(InodeFlags::from_bits((bits & !(STATX_ATTR_AUTOMOUNT)) as i32).unwrap())
		}
		else
		{
			Ok(InodeFlags::from_bits(bits as i32).unwrap())
		}
	}
}
