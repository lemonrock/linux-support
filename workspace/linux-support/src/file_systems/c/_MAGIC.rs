// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) const AAFS_MAGIC: u32 = 0x5A3C69F0;

pub(super) const ADFS_SUPER_MAGIC: u32 = 0xADF5;

pub(super) const AFFS_SUPER_MAGIC: u32 = 0xADFF;

pub(super) const AFS_FS_MAGIC: u32 = 0x6B414653;

pub(super) const AFS_SUPER_MAGIC: u32 = 0x5346414F;

pub(super) const AIO_RING_MAGIC: u32 = 0xA10A10A1;

pub(super) const ANON_INODE_FS_MAGIC: u32 = 0x09041934;

pub(super) const AUTOFS_SUPER_MAGIC: u32 = 0x0187;

pub(super) const BALLOON_KVM_MAGIC: u32 = 0x13661366;

pub(super) const BDEVFS_MAGIC: u32 = 0x62646576;

pub(super) const BINDERFS_SUPER_MAGIC: u32 = 0x6C6F6F70;

pub(super) const BINFMTFS_MAGIC: u32 = 0x42494E4D;

pub(super) const BPF_FS_MAGIC: u32 = 0xCAFE4A11;

pub(super) const BTRFS_SUPER_MAGIC: u32 = 0x9123683E;

pub(super) const BTRFS_TEST_MAGIC: u32 = 0x73727279;

pub(super) const CGROUP2_SUPER_MAGIC: u32 = 0x63677270;

pub(super) const CGROUP_SUPER_MAGIC: u32 = 0x27E0EB;

pub(super) const CODA_SUPER_MAGIC: u32 = 0x73757245;

/// Some random number.
pub(super) const CRAMFS_MAGIC: u32 = 0x28CD3D45;

/// Magic number with the wrong endianess.
pub(super) const CRAMFS_MAGIC_WEND: u32 = 0x453DCD28;

pub(super) const DAXFS_MAGIC: u32 =  0x64646178;

pub(super) const DEBUGFS_MAGIC: u32 = 0x64626720;

pub(super) const DEVPTS_SUPER_MAGIC: u32 = 0x1CD1;

pub(super) const DMA_BUF_MAGIC: u32 = 0x444D4142;

pub(super) const ECRYPTFS_SUPER_MAGIC: u32 = 0xF15F;

pub(super) const EFIVARFS_MAGIC: u32 = 0xDE5E81E4;

pub(super) const EFS_SUPER_MAGIC: u32 = 0x414A53;

pub(super) const EROFS_SUPER_MAGIC_V1: u32 = 0xE0F5E1E2;

pub(super) const EXT2_SUPER_MAGIC: u32 = 0xEF53;

pub(super) const EXT3_SUPER_MAGIC: u32 = 0xEF53;

pub(super) const EXT4_SUPER_MAGIC: u32 = 0xEF53;

pub(super) const F2FS_SUPER_MAGIC: u32 = 0xF2F52010;

pub(super) const FUSE_CTL_SUPER_MAGIC: u32 = 0x65735543;

pub(super) const FUSE_SUPER_MAGIC: u32 = 0x65735546;

pub(super) const FUTEXFS_SUPER_MAGIC: u32 = 0xBAD1DEA;

pub(super) const HOSTFS_SUPER_MAGIC: u32 = 0x00C0FFEE;

pub(super) const HPFS_SUPER_MAGIC: u32 = 0xF995E849;

/// Some random number.
pub(super) const HUGETLBFS_MAGIC : u32 = 0x958458F6;

pub(super) const ISOFS_SUPER_MAGIC: u32 = 0x9660;

pub(super) const JFFS2_SUPER_MAGIC: u32 = 0x72B6;

/// MINIX v2, 30 character names.
pub(super) const MINIX2_SUPER_MAGIC2: u32 = 0x2478;

/// MINIX v2, 14 character names.
pub(super) const MINIX2_SUPER_MAGIC: u32 = 0x2468;

/// MINIX v3, 60 character names.
pub(super) const MINIX3_SUPER_MAGIC: u32 = 0x4D5A;

/// MINIX v1, 30 character names.
pub(super) const MINIX_SUPER_MAGIC2: u32 = 0x138F;

/// minix v1, 14 character names.
pub(super) const MINIX_SUPER_MAGIC: u32 = 0x137F;

pub(super) const MQUEUE_MAGIC: u32 = 0x19800202;

/// `MD`.
pub(super) const MSDOS_SUPER_MAGIC: u32 = 0x4D44;

pub(super) const MTD_INODE_FS_MAGIC: u32 = 0x11307854;

/// `VL`.
pub(super) const NCP_SUPER_MAGIC: u32 = 0x564C;

pub(super) const NFS_SUPER_MAGIC: u32 = 0x6969;

pub(super) const NILFS_SUPER_MAGIC: u32 = 0x3434;

pub(super) const NSFS_MAGIC: u32 = 0x6E736673;

pub(super) const OCFS2_SUPER_MAGIC: u32 = 0x7461636F;

pub(super) const OPENPROM_SUPER_MAGIC: u32 = 0x9FA1;

pub(super) const OVERLAYFS_SUPER_MAGIC: u32 = 0x794C7630;

pub(super) const PIPEFS_MAGIC: u32 = 0x50495045;

pub(super) const PPC_CMM_MAGIC: u32 = 0xC7571590;

pub(super) const PROC_SUPER_MAGIC: u32 = 0x9FA0;

pub(super) const PSTOREFS_MAGIC: u32 = 0x6165676C;

pub(super) const QNX4_SUPER_MAGIC: u32 = 0x002F;

pub(super) const QNX6_SUPER_MAGIC: u32 = 0x68191122;

/// Some random number.
pub(super) const RAMFS_MAGIC: u32 = 0x858458F6;

pub(super) const RDTGROUP_SUPER_MAGIC: u32 = 0x7655821;

/// Used by GCC.
pub(super) const REISERFS_SUPER_MAGIC: u32 = 0x52654973;

pub(super) const SECURITYFS_MAGIC: u32 = 0x73636673;

pub(super) const SELINUX_MAGIC: u32 = 0xF97CFF8C;

/// `SMAC`.
pub(super) const SMACK_MAGIC: u32 = 0x43415D53;

pub(super) const SMB_SUPER_MAGIC: u32 = 0x517B;

pub(super) const SOCKFS_MAGIC: u32 = 0x534F434B;

pub(super) const SQUASHFS_MAGIC: u32 = 0x73717368;

pub(super) const STACK_END_MAGIC: u32 = 0x57AC6E9D;

pub(super) const SYSFS_MAGIC: u32 = 0x62656572;

pub(super) const TMPFS_MAGIC: u32 = 0x01021994;

pub(super) const TRACEFS_MAGIC: u32 = 0x74726163;

pub(super) const UDF_SUPER_MAGIC: u32 = 0x15013346;

pub(super) const USBDEVICE_SUPER_MAGIC: u32 = 0x9FA2;

pub(super) const V9FS_MAGIC: u32 = 0x01021997;

pub(super) const XENFS_SUPER_MAGIC: u32 = 0xABBA1974;

/// `XFSB`.
pub(super) const XFS_SUPER_MAGIC: u32 = 0x58465342;

pub(super) const Z3FOLD_MAGIC: u32 = 0x33;

pub(super) const ZONEFS_MAGIC: u32 = 0x5A4F4653;

pub(super) const ZSMALLOC_MAGIC: u32 = 0x58295829;
