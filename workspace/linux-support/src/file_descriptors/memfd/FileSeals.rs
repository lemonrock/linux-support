// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// File seals limit the set of allowed operations on a given file.
	///
	/// For each seal that is set on a file, a specific set of operations will fail with `EPERM` on this file from now on.
	/// The file is said to be sealed.
	///
	/// Seals are a property of an inode.
	/// Thus, all open file descriptors referring to the same inode share the same set of seals.
	///
	/// Seals can never be removed, only added.
	///
	/// Since Linux 3.17.
	#[allow(missing_docs)]
	pub struct FileSeals: i32
	{
		/// Seals the file so seals can no longer be altered.
		///
		/// If this seal is set, any further call to `fcntl()` with `Seal` fails with the error `EPERM`.
		/// Therefore, this seal prevents any modifications to the set of seals itself.
		/// If the initial set of seals of a file includes `Seal`, then this effectively causes the set of seals to be constant and locked
		const SealForever = F_SEAL_SEAL;

		/// Seal the file against shrinking.
		///
		/// If this seal is set, the file in question cannot be reduced in size.
		/// This affects `open()` with the `O_TRUNC` flag as well as `truncate()` and `ftruncate()`.
		/// Those calls fail with `EPERM` if you try to shrink the file in question.
		/// Increasing the file size is still possible.
		const DisallowShrinkin = F_SEAL_SHRINK;

		/// Seal the file against growing.
		///
		/// If this seal is set, the size of the file in question cannot be increased.
		/// This affects `write()` beyond the end of the file, `truncate()`, `ftruncate()`, and `fallocate()`.
		/// These calls fail with `EPERM` if you use them to increase the file size.
		/// If you keep the size or shrink it, those calls still work as expected.
		const DisallowGrowth = F_SEAL_GROW;

		/// Seal the file against writes.
		///
		/// If this seal is set, you cannot modify the contents of the file.
		/// Note that shrinking or growing the size of the file is still possible and allowed.
		/// Thus, this seal is normally used in combination with one of the other seals.
		/// This seal affects `write()` and `fallocate()` (only in combination with the `FALLOC_FL_PUNCH_HOLE` flag).
		/// Those calls fail with `EPERM` if this seal is set.
		/// Furthermore, trying to create new shared, writable memory-mappings via `mmap()` will also fail with `EPERM`.
		///
		/// Adding this seal fails with `EBUSY` if any writable, shared mapping exists.
		/// Such mappings must be unmapped before you can add this seal.
		/// Furthermore, if there are any asynchronous I/O operations (eg `io_submit()`) pending on the file, all outstanding writes will be discarded.
		const DisallowWrites = F_SEAL_WRITE;

		/// Seal the file against future writes.
		///
		/// The effect of this seal is similar to `Write`, but the contents of the file can still be modified via shared writable mappings that were created prior to the seal being set.
		/// Any attempt to create a new writable mapping on the file via `mmap()` will fail with `EPERM`.
		/// Likewise, an attempt to write to the file via `write()` will fail with `EPERM`.
		///
		/// Using this seal, one process can create a memory buffer that it can continue to modify while sharing that buffer on a "read-only" basis with other processes.
		///
		/// Since Linux 5.1.
		const DisallowWriteButAllowAnyCurrentWrites = F_SEAL_FUTURE_WRITE;
	}
}
