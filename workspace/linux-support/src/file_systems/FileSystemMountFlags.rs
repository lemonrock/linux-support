// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Mount flags.
	#[derive(Deserialize, Serialize)]
	pub struct FileSystemMountFlags: u64
	{
		#[allow(missing_docs)]
		const MandatoryLockingIsPermitted = ST_MANDLOCK;

		/// `noatime`; Access time (`atime`) is not updated.
		const AccessTimeIsNotUpdated = ST_NOATIME;

		#[allow(missing_docs)]
		const BlockDevicesAndCharacterDevicesAreInaccesible = ST_NODEV;

		/// `nodiratime`; Access time (`atime`) of directories is not updated.
		const DirectoryAccessTimeIsNotUpdated = ST_NODIRATIME;

		/// `noexec`; Binaries and scripts can be executed.
		const NoExecutables = ST_NOEXEC;

		/// `suid` and `sgid` bits are ignored.
		const SetUserIdentifierAndSetGroupIdentifierBitsIgnoredWhenExecutingBinaries = ST_NOSUID;

		#[allow(missing_docs)]
		const ReadOnly = ST_RDONLY;

		/// Update access time relative to change time (`ctime`) or modified time (`mtime`).
		const RelativeUpdatesToAccessTime = ST_RELATIME;

		/// As if every file opened has `O_SYNC`.
		const SynchronousWrites = ST_SYNCHRONOUS;

		#[doc(hidden)]
		#[deprecated(since = "0.0.0.", note = "Unused by Linux")]
		const Write = ST_WRITE;

		#[doc(hidden)]
		#[deprecated(since = "0.0.0.", note = "Unused by Linux")]
		const Append = ST_APPEND;

		#[doc(hidden)]
		#[deprecated(since = "0.0.0.", note = "Unused by Linux")]
		const Immutable = ST_IMMUTABLE;
	}
}
