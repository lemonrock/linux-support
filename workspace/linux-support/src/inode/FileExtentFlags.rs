// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Flags for a file extent.
	pub struct FileExtentFlags: u32
	{
		#[doc(hidden)]
		const Last = FIEMAP_EXTENT_LAST;

		/// The location of this extent is currently unknown.
		///
		/// This may indicate the data is stored on an inaccessible volume or that no storage has been allocated for the file yet.
		const Unknown = FIEMAP_EXTENT_UNKNOWN;

		/// Delayed allocation.
		///
		/// While there is data for this extent, its physical location has not been allocated yet.
		///
		/// If this is set, so will `Unknown`.
		const DelayedAllocation = FIEMAP_EXTENT_DELALLOC;

		/// Encoded.
		///
		/// This extent does not consist of plain filesystem blocks but is encoded (eg encrypted or compressed).
		/// Reading the data in this extent via I/O to the block device will have undefined results.
		///
		/// Note that it is *always* undefined to try to update the data in-place by writing to the indicated location without the assistance of the filesystem, or to access the data using the information returned by the using file extents while the filesystem is mounted.
		/// In other words, user applications may only read the extent data via I/O to the block device while the filesystem is unmounted, and then only if this flag is *unset* is ; user applications must not try reading or writing to the filesystem via the block device under any other circumstances.
		const Encoded = FIEMAP_EXTENT_ENCODED;

		/// Data encrypted.
		///
		/// If this is set, so will `Encoded`.
		const DataEncrypted = FIEMAP_EXTENT_DATA_ENCRYPTED;

		/// The values of `fiemap_extent.logical_range_in_bytes()` and `fiemap_extent.physical_range_in_bytes()` are not aligned to the file system block size.
		const NotAligned = FIEMAP_EXTENT_NOT_ALIGNED;

		/// Data inline.
		///
		/// If this is set, so will `NotAligned`.
		const DataInline = FIEMAP_EXTENT_DATA_INLINE;

		/// Data tail-packed (only for the obsolete `reiserfs` file system).
		///
		/// If this is set, so will `NotAligned`.
		#[deprecated(since = "0.0.0", note = "only for the obsolete reiserfs file system")]
		const DataTailPacked = FIEMAP_EXTENT_DATA_TAIL;

		/// Unwritten.
		///
		/// The extent is allocated but its data has not been initialized.
		/// This indicates the extent's data will be all zero if read through the filesystem but the contents are undefined if read directly from the device.
		const Unwritten = FIEMAP_EXTENT_UNWRITTEN;

		/// Adjacent file extents have been returned as a merged extent.
		///
		/// There may be subsequent extents which immediately follow this one and have not been 'merged'; this needs to be taken into consideration when copying files.
		///
		/// This will be set when a file does not support extents, ie, it uses a block based addressing scheme.
		/// Since returning an extent for each block back to userspace would be highly inefficient, the kernel will try to merge most adjacent blocks into 'extents'
		const Merged = FIEMAP_EXTENT_MERGED;

		/// Shared.
		///
		/// ?
		const Shared = FIEMAP_EXTENT_SHARED;
	}
}
