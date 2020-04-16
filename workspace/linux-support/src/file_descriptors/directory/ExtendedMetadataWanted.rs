// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Extended metadata interest.
	///
	/// If one of these bits is chosen, extended metadata will be obtained for it.
	///
	/// Defaults to `AllAvailableInformation`.
	pub struct ExtendedMetadataWanted: u32
	{
		/// Want `mode & S_IFMT`.
		const Type = STATX_TYPE;

		/// Want `mode & !S_IFMT`.
		const Mode = STATX_MODE;

		/// Want number of hard links.
		const NumberOfHardLinks = STATX_NLINK;

		/// Want user identifier.
		const UserIdentifier = STATX_UID;

		/// Want group identifier.
		const GroupIdentifier = STATX_GID;

		/// Want time last accessed at (`atime`).
		const LastAccessedAtTime = STATX_ATIME;

		/// Want time last modified at (`mtime`).
		const LastModifiedAtTime = STATX_MTIME;

		/// Want time last changed (`ctime`).
		///
		/// This will usually be the same as `LastModifiedAtTime` unless the change was just metadata.
		const LastChangedAtTime = STATX_CTIME;

		/// Want inode.
		const Inode = STATX_INO;

		/// Want size.
		const Size = STATX_SIZE;

		/// Want size in 512-byte blocks.
		const SizeIn512ByteBlocks = STATX_BLOCKS;

		/// Want `Type`, `Mode`, `NumberOfHardLinks`, `UserIdentifier`, `GroupIdentifier`, `Inode`, `Size` and `SizeIn512ByteBlocks`.
		const BasicStatistics = STATX_BASIC_STATS;

		/// Want time created at (born at or birth time, sometimes called `btime`).
		const CreatedAtTime = STATX_BTIME;

		/// All available information.
		const AllAvailableInformation = STATX_ALL;
	}
}

impl Default for ExtendedMetadataWanted
{
	#[inline(always)]
	fn default() -> Self
	{
		ExtendedMetadataWanted::AllAvailableInformation
	}
}
