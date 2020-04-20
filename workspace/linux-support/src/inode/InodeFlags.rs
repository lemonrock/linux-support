// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Inode flags.
	///
	/// Also known as 'attributes'.
	///
	/// See the `lsattr` and the `chattr` programs and <http://man7.org/linux/man-pages/man2/ioctl_iflags.2.html>.
	pub struct InodeFlags: i32
	{
		/// Allow the file to be undeleted if it is deleted.
		///
		/// When a file with this attribute set is deleted, its contents are saved.
		/// This allows the user to ask for its undeletion.
		///
		/// Inherited by files and subdirectories if set on a directory.
		///
		/// `chattr` represents this using the letter `u`.
		#[deprecated(since = "0.0.0", note = "unsupported by any filesystem on Linux")]
		const AllowUndelete = FS_UNRM_FL;

		/// Mark the file for secure deletion.
		///
		/// When the file is deleted, its blocks are zeroed and written back out to the disk.
		///
		/// Inherited by files and subdirectories if set on a directory.
		///
		/// `chattr` represents this using the letter `s`.
		#[deprecated(since = "0.0.0", note = "unsupported by any filesystem on Linux")]
		const MarkForSecureDeletion = FS_SECRM_FL;

		/// Store the file in a compressed format on disk.
		///
		/// Inherited by files and subdirectories if set on a directory.
		///
		/// Ignored by ext2, ext3 and ext4.
		///
		/// `chattr` represents this using the letter `c`.
		const CompressionRequested = FS_COMPR_FL;

		/// Write changes synchronously to disk.
		///
		/// This flag provides semantics equivalent to:-
		///
		/// * the `mount()` `MS_SYNC` option (but for a specific file)
		/// * `WriteSynchronization::MetaDataAndData`
		/// * Calling `Synchronize::synchronize_data_and_metadata()` after every `write()`.
		/// * `open()` flag `O_SYNC`.
		///
		/// This flag can be used by regular files and directories.
		///
		/// Inherited by files and subdirectories if set on a directory.
		///
		/// `chattr` represents this using the letter `S`.
		const ChangesSynchronous = FS_SYNC_FL;

		/// The file is immutable: no changes are permitted to the file contents or metadata (permissions, timestamps, ownership, link count and so on).
		/// (This restriction applies even to the superuser).
		/// Additonally, it cannot be deleted or renamed, no link can be created to this file, most of the file's metadata can not be modified, and the file can not be opened in write mode.
		///
		/// Only a process with the `CAP_LINUX_IMMUTABLE` capability can set or clear this attribute.
		///
		/// If a file is already open, and this flag is set, the already open file is unaffected.
		///
		/// Inherited by files and subdirectories if set on a directory.
		///
		/// `chattr` represents this using the letter `i`.
		const Immutable = FS_IMMUTABLE_FL;

		/// The file can be opened only with the `O_APPEND` flag.
		/// (This restriction applies even to the superuser).
		///
		/// Only a process with the `CAP_LINUX_IMMUTABLE` capability can set or clear this attribute.
		///
		/// If a file is already open, and this flag is set, the already open file is unaffected.
		///
		/// Inherited by files and subdirectories if set on a directory.
		///
		/// `chattr` represents this using the letter `a`.
		const AppendOnly = FS_APPEND_FL;

		/// Don't include this file in backups made using `dump`.
		///
		/// Inherited by files and subdirectories if set on a directory.
		///
		/// `chattr` represents this using the letter `d`.
		const NoDump = FS_NODUMP_FL;

		/// `noatime`; Access time (`atime`) is not updated.
		///
		/// This flag provides semantics equivalent to the `mount()` `MS_NOATIME` option but on a per-file basis.
		///
		/// Inherited by files and subdirectories if set on a directory.
		///
		/// `chattr` represents this using the letter `A`.
		const AccessTimeIsNotUpdated = FS_NOATIME_FL;

		/// A compressed file is dirty.
		///
		/// Formerly used by `ext2` experimental patches that were not merged into the mainline Linux kernel.
		///
		/// It is not possible to modify this.
		///
		/// `chattr` previously represented this using the letter `Z`.
		#[deprecated(since = "0.0.0", note = "Was experimental for ext2, now deprecated")]
		const CompressedDirty = FS_DIRTY_FL;

		/// Linux source: "One or more compressed clusters".
		///
		/// Formerly used by `ext2` experimental patches that were not merged into the mainline Linux kernel.
		///
		/// `chattr` previously represented this using the letter `B`.
		#[deprecated(since = "0.0.0", note = "Was experimental for ext2, now deprecated")]
		const HasCompressedClusters = FS_COMPRBLK_FL;

		/// Indicates that the raw contents of a compressed file can be accessed directly.
		///
		/// Formerly used by `ext2` experimental patches that were not merged into the mainline Linux kernel.
		///
		/// It is not possible to modify this.
		///
		/// `chattr` previously represented this using the letter `X`.
		#[deprecated(since = "0.0.0", note = "Was experimental for ext2, now deprecated")]
		const CompressionRawAccess = FS_NOCOMP_FL;

		/// A file, directory, or symlink with the 'E' attribute set is encrypted by the filesystem.
		///
		/// It is not possible to modify this.
		///
		/// `chattr` represents this using the letter `E`.
		/// 
		/// This value was also previously used for `ext2` compression errors.
		const Encrypt = FS_ENCRYPT_FL;

		/// Indicates that a directory is being indexed using hashed trees (or, historically, B-Trees).
		///
		/// It is not possible to modify this.
		///
		/// The definition using B-Trees (`FS_BTREE_FL`) is referenced only by `ext2` and `ocfs2`.
		///
		/// `chattr` represents this using the letter `I`.
		const DirectoryIndexedUsingHashedTreesOrBTrees = FS_INDEX_FL;

		/// AFS (?Andrew File System) format directory.
		///
		/// Referenced by `ext2` and `ocfs2`.
		#[deprecated(since = "0.0.0", note = "Seems to be unused")]
		const DirectoryIndexedLikeAndrewFileSystem = FS_IMAGIC_FL;

		/// Enable journaling of file data on ext3 and ext4 filesystems.
		///
		/// Only a procss with the `CAP_SYS_RESOURCE` capability can set this flag.
		///
		/// On a filesystem that is journaling in ordered mode (mount option: `data=ordered`) or writeback mode (mount option: `data=writeback`), setting this flag enables journaling of data updates on a per-file basis (ie as if the mount option `data=journal` was specified).
		///
		/// If the file system is journalling in journal mode (sic) (mount option: `data=journal`) then this flag has no effect.
		///
		/// Setting this flag forces all written data to go to the filesystems journal before being transferred to the file.
		///
		/// Inherited by files and subdirectories if set on a directory.
		///
		/// `chattr` represents this using the letter `j`.
		const JournalData = FS_JOURNAL_DATA_FL;

		/// Disable the reiserfs tail-packing feature, which tries to pack small files (and the final fragment of larger files) into the same disk block as the file metadata.
		///
		/// Also known as 'merging of file tail'.
		///
		/// A file with this attribute set will not have a partial block fragment at the end of the file merged with other files.
		///
		/// This is necessary for applications such as LILO which read the filesystem directly, and which don't understand tail-merged files.
		///
		/// Inherited by files and subdirectories if set on a directory.
		///
		/// `chattr` represents this using the letter `t`.
		#[deprecated(since = "0.0.0", note = "reiserfs is obsolete")]
		const DisableReiserfsTailPacking = FS_NOTAIL_FL;

		/// Write directory changes synchronously to disk.
		///
		/// This flag provides semantics equivalent to the `mount()` `MS_DIRSYNC` option but on a per-directory basis.
		///
		/// This flag can be applied only to directories.
		///
		/// Inherited by files and subdirectories.
		///
		/// `chattr` represents this using the letter `D`.
		const DirectoryChangesSynchronized = FS_DIRSYNC_FL;

		/// Mark a directory for special treatment under the Orlov block-allocation strategy on ext2, ext3 and ext4 filesystems.
		///
		/// This is a hint to the Orlov block allocator used by ext2, ext3 and ext4 that the subdirectories under this directory are not related, and thus should be spread apart for allocation purposes.
		/// For example it is a very good idea to set this attribute on the `/home` directory, so that `/home/john` and `/home/mary` are placed into separate block groups.
		/// For directories where this attribute is not set, the Orlov block allocator will try to group subdirectories closer together where possible.
		///
		/// This flag can be applied only to directories.
		///
		/// Inherited by only subdirectories.
		///
		/// `chattr` represents this using the letter `T`.
		const TopDirectory = FS_TOPDIR_FL;

		/// This attribute indicates the file is storing its blocks in units of the filesystem blocksize instead of in units of sectors, and means that the file is (or at one time was) larger than 2TB.
		///
		/// It is not possible to modify this.
		///
		/// `chattr` formerly represented this using the letter `h`.
		#[deprecated(since = "0.0.0", note = "Believed to be no longer in use")]
		const HugeFile = FS_HUGE_FILE_FL;

		/// The file is using extents for mapping the blocks on disk.
		///
		/// It is not possible to modify this.
		///
		/// `chattr` represents this using the letter `e`.
		const UsesExtents = FS_EXTENT_FL;

		/// A file with this attribute set has fs-verity enabled.
		///
		/// It cannot be written to, and the filesystem will automatically verify all data read from it against a cryptographic hash that covers the entire file's contents, eg via a Merkle tree.
		/// This makes it possible to efficiently authenticate the file.
		///
		/// It is not possible to modify this.
		///
		/// `chattr` represents this using the letter `V`.
		const FsVerity = FS_VERITY_FL;

		/// Large "EA"
		///
		/// Unused by any filesystem.
		#[deprecated(since = "0.0.0", note = "Seems to be unused")]
		const LargeEA = FS_EA_INODE_FL;

		/// Reserved for `ext4`.
		///
		/// Unused by any filesystem.
		#[deprecated(since = "0.0.0", note = "Seems to be unused")]
		const EndOfFileBlocks = FS_EOFBLOCKS_FL;

		/// This file will not be subject to Copy-On-Write (COW) for updates.
		///
		/// Works on `btrfs`, where it should only be set on new files or empty files.
		///
		/// Inherited by files and subdirectories if set on a directory.
		///
		/// `chattr` represents this using the letter `C`.
		const DisableCopyOnWriteForUpdates = FS_NOCOW_FL;

		/// File has data stored inline.
		///
		/// Seems to only be supported by `ext4`.
		///
		/// It is not possible to modify this.
		///
		/// `chattr` represents this using the letter `N`.
		const InlineData = FS_INLINE_DATA_FL;

		/// Files and subdirectories will inherit the project quota identifier of the directory this applies to.
		///
		/// This flag can be applied only to directories.
		///
		/// Inherited by files and subdirectories.
		///
		/// This means that files and directories created in the directory will inherit the project id of the directory, rename operations are constrained so when a file or directory is moved into another directory, that the project ids must match.
		/// In addition, a hard link to file can only be created when the project id for the file and the destination directory match.
		///
		/// `chattr` represents this using the letter `P`.
		///
		/// Since Linux 4.5.
		const InheritProjectQuotaIdentifier = FS_PROJINHERIT_FL;

		/// `chattr` represents this using the letter `F`.
		///
		/// This flag can be applied only to directories.
		///
		/// Inherited by files and subdirectories.
		///
		/// All the path lookups inside this directory are made in a case-insensitive fashion.
		///
		/// This attribute can only be changed in empty directories on file systems with the casefold feature enabled.
		const CaseInsensitive = FS_CASEFOLD_FL;

		/// Reserved.
		///
		/// Referenced by `ext2` and `ocfs2`.
		#[deprecated(since = "0.0.0", note = "Seems to be unused")]
		const Reserved = FS_RESERVED_FL;

		#[doc(hidden)]
		const AllValidAsFileAttributesSubset = FS_COMPR_FL | FS_IMMUTABLE_FL | FS_APPEND_FL | FS_NODUMP_FL | FS_ENCRYPT_FL | FS_VERITY_FL;
	}
}

impl TryInto<FileAttributesSubset> for InodeFlags
{
	type Error = FileAttributesSubset;

	#[inline(always)]
	fn try_into(self) -> Result<FileAttributesSubset, Self::Error>
	{
		// All the values stating `STATX_ATTR_` are the same as their companions starting `FS_`, with the exception of `STATX_ATTR_AUTOMOUNT`.
		let bits = self.bits;
		if bits & !(InodeFlags::AllValidAsFileAttributesSubset.bits) != 0
		{
			Err(FileAttributesSubset::from_bits((bits & InodeFlags::AllValidAsFileAttributesSubset.bits) as u64).unwrap())
		}
		else
		{
			Ok(FileAttributesSubset::from_bits(bits as u64).unwrap())
		}
	}
}
