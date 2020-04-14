// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Allocation mode bitflags.
	pub struct AllocationMode: i32
	{
		/// Default is to match `posix_fallocate()`.
		const Posix = 0;

		/// Similar to `Posix` but the file size will not be changed even if `offset + length` is greater than the file size.
		///
		/// Preallocating zeroed blocks beyond the end of the file in this manner is useful for optimizing append workloads.
		///
		/// Equivalent to `FALLOC_FL_KEEP_SIZE`.
		const KeepSize = 0x01;

		/// Deallocates space (ie creates a hole) in the byte range starting at offset and continuing for `length` bytes.
		///
		/// Within the specified range, partial filesystem blocks are zeroed, and whole filesystem blocks are removed from the file.
		/// After a successful call, subsequent reads from this range will return zeros.
		///
		/// The file's size will not change.
		///
		/// Equivalent to `FALLOC_FL_PUNCH_HOLE | FALLOC_FL_KEEP_SIZE`.
		///
		/// Since Linux 2.6.38.
		const PunchHoleWithKeepSize = 0x02 | 0x01;

		// Equivalent to `FALLOC_FL_NO_HIDE_STALE`.
		#[doc(hidden)]
		#[deprecated]
		const NoHideStale = 0x04;

		/// Used to remove a range of a file without leaving a hole in the file.
		///
		/// The contents of the file beyond the range being removed is appended to the start offset of the range being removed (i.e. the hole that was punched is "collapsed"),  resulting in a file layout that looks like the range that was removed never existed.
		/// As such collapsing a range of a file changes the size of the file, reducing it by the same length of the range that has been removed by the operation.
		///
		/// Different filesystems may implement different limitations on the granularity of the operation.
		/// Most will limit operations to filesystem block size boundaries, but this boundary may be larger or smaller depending on the filesystem and/or the configuration of the filesystem or file.
		///
		/// Attempting to collapse a range that crosses the end of the file is  considered an illegal operation - just use `ftruncate()` if you need to collapse a range that crosses EOF.
		///
		/// Equivalent to `FALLOC_FL_COLLAPSE_RANGE`.
		const CollapseRange = 0x08;

		/// Used to convert a range of file to zeros preferably without issuing data IO.
		///
		/// Blocks should be preallocated for the regions that span holes in the file, and the entire range is preferable converted to unwritten extents - even though file system may choose to zero out the extent or do whatever which will result in reading zeros from the range while the range remains allocated for the file.
		///
		/// This can be also used to preallocate blocks past EOF in the same way as with `fallocate()`.
		///
		/// If use with `KeepSize` then the inode size should remain the same.
		///
		/// Equivalent to `FALLOC_FL_ZERO_RANGE`.
		const ZeroRange = 0x10;

		/// Used to insert space within the file size without overwriting any existing data.
		///
		/// The contents of the file beyond offset are shifted towards right by `length` bytes to create a hole.
		/// As such, this operation will increase the size of the file by `length` bytes.
		///
		/// Different filesystems may implement different limitations on the granularity of the operation.
		/// Most will limit operations to filesystem block size boundaries, but this boundary may be larger or smaller depending on / the filesystem and/or the configuration of the filesystem or file.
		///
		/// Attempting to insert space using this flag at *or beyond* the end of the file is considered an illegal operation - just use `ftruncate()` or `allocate(Posix)` for such type of operations.
		///
		/// Equivalent to `FALLOC_FL_INSERT_RANGE`.
		const InsertRange = 0x20;

		/// Used to unshare shared blocks within the file size without overwriting any existing data.
		///
		/// The purpose of this call is to preemptively reallocate any blocks that are subject to copy-on-write.
		///
		/// Different filesystems may implement different limitations on the granularity of the operation.
		/// Most will limit operations to filesystem block size boundaries, but this boundary may be larger or smaller depending on the filesystem and/or the configuration of the filesystem or file.
		///
		/// This flag can only be used with allocate-mode fallocate, which is to say that it cannot be used with `PunchHoleWithKeepSize`, `CollapseRange`, `ZeroRange` or `InsertRange`.
		///
		/// Equivalent to `FALLOC_FL_UNSHARE_RANGE`.
		const UnshareRange = 0x40;
	}
}
