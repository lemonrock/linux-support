// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Linux source: "Secure deletion".
pub(super) const FS_SECRM_FL: i32 = 0x00000001;

/// Linux source: "Undelete".
pub(super) const FS_UNRM_FL: i32 = 0x00000002;

/// Linux source: "Compress file".
pub(super) const FS_COMPR_FL: i32 = 0x00000004;

/// Linux source: "Synchronous updates".
pub(super) const FS_SYNC_FL: i32 = 0x00000008;

/// Linux source: "Immutable file".
pub(super) const FS_IMMUTABLE_FL: i32 = 0x00000010;

/// Linux source: "writes to file may only append".
pub(super) const FS_APPEND_FL: i32 = 0x00000020;

/// Linux source: "do not dump file".
pub(super) const FS_NODUMP_FL: i32 = 0x00000040;

/// Linux source: "do not update atime".
pub(super) const FS_NOATIME_FL: i32 = 0x00000080;

/// Linux source: "Dirty compression".
pub(super) const FS_DIRTY_FL: i32 = 0x00000100;

/// Linux source: "One or more compressed clusters".
pub(super) const FS_COMPRBLK_FL: i32 = 0x00000200;

/// Linux source: "Don't compress".
pub(super) const FS_NOCOMP_FL: i32 = 0x00000400;

/// Linux source: "Encrypted file".
pub(super) const FS_ENCRYPT_FL: i32 = 0x00000800;

/// Linux source: "btree format dir".
///
/// Note this is the same value as `FS_BTREE_FL`.
#[allow(dead_code)]
pub(super) const FS_BTREE_FL: i32 = FS_INDEX_FL;

/// Linux source: "hash-indexed directory".
///
/// Note this is the same value as `FS_INDEX_FL`.
pub(super) const FS_INDEX_FL: i32 = 0x00001000;

/// Linux source: "AFS directory".
pub(super) const FS_IMAGIC_FL: i32 = 0x00002000;

/// Linux source: "Reserved for ext3".
pub(super) const FS_JOURNAL_DATA_FL: i32 = 0x00004000;

/// Linux source: "file tail should not be merged".
pub(super) const FS_NOTAIL_FL: i32 = 0x00008000;

/// Linux source: "dirsync behaviour (directories only)".
pub(super) const FS_DIRSYNC_FL: i32 = 0x00010000;

/// Linux source: "Top of directory hierarchies".
pub(super) const FS_TOPDIR_FL: i32 = 0x00020000;

/// Linux source: "Reserved for ext4".
pub(super) const FS_HUGE_FILE_FL: i32 = 0x00040000;

/// Linux source: "Extents".
pub(super) const FS_EXTENT_FL: i32 = 0x00080000;

/// Linux source: "fs-verity protected inode".
pub(super) const FS_VERITY_FL: i32 = 0x00100000;

/// Linux source: "Inode used for large EA".
pub(super) const FS_EA_INODE_FL: i32 = 0x00200000;

/// Linux source: "Reserved for ext4".
pub(super) const FS_EOFBLOCKS_FL: i32 = 0x00400000;

/// Linux source: "Do not cow file".
pub(super) const FS_NOCOW_FL: i32 = 0x00800000;

/// Linux source: "Reserved for ext4".
pub(super) const FS_INLINE_DATA_FL: i32 = 0x10000000;

/// Linux source: "Create with parents projid".
pub(super) const FS_PROJINHERIT_FL: i32 = 0x20000000;

/// Folder is case insensitive
pub(super) const FS_CASEFOLD_FL: i32 = 0x40000000;

/// Linux source: "reserved for ext2 lib".
pub(super) const FS_RESERVED_FL: i32 = 0x80000000u32 as i32;
