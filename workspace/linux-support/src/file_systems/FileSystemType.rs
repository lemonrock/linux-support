// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// File system types.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum FileSystemType
{
	anon_inodefs,
	
	bdev,
	
	/// BPF File System, `BPFFS`.
	///
	/// Usually mounted at `/sys/fs/bpf`.
	bpf,
	
	binfmt_misc,
	
	cgroup,
	
	/// Cgroup version 2 file system.
	///
	/// Usually mounted at `/sys/fs/cgroup`.
	cgroup2,
	
	cpuset,

	dax,

	debugfs,

	devpts,

	devtmpfs,

	ecryptfs,

	ext2,

	ext3,

	ext4,

	fuse,

	fuseblk,

	fusectl,

	hugetlbfs,

	mqueue,

	pipefs,

	prl_fs,

	proc,

	pstore,

	ramfs,

	rootfs,

	security,

	securityfs,

	sockfs,

	sysfs,

	tmpfs,

	tracefs,

	vfat,

	Unrecognised(Box<[u8]>)
}

impl AsRef<OsStr> for FileSystemType
{
	#[inline(always)]
	fn as_ref(&self) -> &OsStr
	{
		OsStr::from_bytes(self.to_bytes())
	}
}

impl AsRef<Path> for FileSystemType
{
	#[inline(always)]
	fn as_ref(&self) -> &Path
	{
		let this: &OsStr = self.as_ref();
		Path::new(this)
	}
}

impl FileSystemType
{
	/// Note that some older filesystems such as MS-DOS only have a 16-bit magic (ie cast the returned value to `u16`).
	///
	/// Not complete; `*_MAGIC` constants are scattered all over the Linux code base.
	#[inline(always)]
	pub fn magic(self) -> Option<u32>
	{
		use self::FileSystemType::*;
		
		let magic = match self
		{
			anon_inodefs => ANON_INODE_FS_MAGIC,
			bdev => BDEVFS_MAGIC,
			bpf => BPF_FS_MAGIC,
			binfmt_misc => BINFMTFS_MAGIC,
			cgroup => CGROUP_SUPER_MAGIC,
			cgroup2 => CGROUP2_SUPER_MAGIC,
			cpuset => return None,
			dax => DAXFS_MAGIC,
			debugfs => DEBUGFS_MAGIC,
			devpts => DEVPTS_SUPER_MAGIC,
			devtmpfs => return None,
			ecryptfs => ECRYPTFS_SUPER_MAGIC,
			ext2 => EXT2_SUPER_MAGIC,
			ext3 => EXT3_SUPER_MAGIC,
			ext4 => EXT4_SUPER_MAGIC,
			fuse => FUSE_SUPER_MAGIC,
			fuseblk => return None,
			fusectl => FUSE_CTL_SUPER_MAGIC,
			hugetlbfs => HUGETLBFS_MAGIC,
			mqueue => MQUEUE_MAGIC,
			pipefs => PIPEFS_MAGIC,
			prl_fs => return None,
			proc => PROC_SUPER_MAGIC,
			pstore => PSTOREFS_MAGIC,
			ramfs => RAMFS_MAGIC,
			rootfs => return None,
			security => return None,
			securityfs => SECURITYFS_MAGIC,
			sockfs => SOCKFS_MAGIC,
			sysfs => SYSFS_MAGIC,
			tmpfs => TMPFS_MAGIC,
			tracefs => TRACEFS_MAGIC,
			vfat => return None,
			
			Unrecognised(_) => return None,
		};
		
		Some(magic)
	}
	
	/// To `CString`.
	#[inline(always)]
	pub fn to_c_string(&self) -> CString
	{
		use self::FileSystemType::*;

		let ref_value = match *self
		{
			anon_inodefs => "anon_inodefs",
			bdev => "bdev",
			bpf => "bpf",
			binfmt_misc => "binfmt_misc",
			cgroup => "cgroup",
			cgroup2 => "cgroup2",
			cpuset => "cpuset",
			dax => "dax",
			debugfs => "debugfs",
			devpts => "devpts",
			devtmpfs => "devtmpfs",
			ecryptfs => "ecryptfs",
			ext2 => "ext2",
			ext3 => "ext3",
			ext4 => "ext4",
			fuse => "fuse",
			fuseblk => "fuseblk",
			fusectl => "fusectl",
			hugetlbfs => "hugetlbfs",
			mqueue => "mqueue",
			pipefs => "pipefs",
			prl_fs => "prl_fs",
			proc => "proc",
			pstore => "pstore",
			ramfs => "ramfs",
			rootfs => "rootfs",
			security => "security",
			securityfs => "securityfs",
			sockfs => "sockfs",
			sysfs => "sysfs",
			tmpfs => "tmpfs",
			tracefs => "tracefs",
			vfat => "vfat",

			Unrecognised(ref value) => return CString::new(value.clone()).unwrap(),
		};

		CString::new(ref_value.to_owned()).expect("file system type should not contain interior ASCII NULs")
	}
	
	/// To bytes.
	#[inline(always)]
	pub fn to_bytes(&self) -> &[u8]
	{
		use self::FileSystemType::*;

		match *self
		{
			anon_inodefs => b"anon_inodefs" as &[u8],
			bdev => b"bdev" as &[u8],
			bpf => b"bpf" as &[u8],
			binfmt_misc => b"binfmt_misc" as &[u8],
			cgroup => b"cgroup" as &[u8],
			cgroup2 => b"cgroup2" as &[u8],
			cpuset => b"cpuset" as &[u8],
			dax => b"dax" as &[u8],
			debugfs => b"debugfs" as &[u8],
			devpts => b"devpts" as &[u8],
			devtmpfs => b"devtmpfs" as &[u8],
			ecryptfs => b"ecryptfs" as &[u8],
			ext2 => b"ext2" as &[u8],
			ext3 => b"ext3" as &[u8],
			ext4 => b"ext4" as &[u8],
			fuse => b"fuse" as &[u8],
			fuseblk => b"fuseblk" as &[u8],
			fusectl => b"fusectl" as &[u8],
			hugetlbfs => b"hugetlbfs" as &[u8],
			mqueue => b"mqueue" as &[u8],
			pipefs => b"pipefs" as &[u8],
			prl_fs => b"prl_fs" as &[u8],
			proc => b"proc" as &[u8],
			pstore => b"pstore" as &[u8],
			ramfs => b"ramfs" as &[u8],
			rootfs => b"rootfs" as &[u8],
			security => b"security" as &[u8],
			securityfs => b"securityfs" as &[u8],
			sockfs => b"sockfs" as &[u8],
			sysfs => b"sysfs" as &[u8],
			tmpfs => b"tmpfs" as &[u8],
			tracefs => b"tracefs" as &[u8],
			vfat => b"vfat" as &[u8],
			
			Unrecognised(ref value) => &value[..]
		}
	}

	/// From string.
	#[inline(always)]
	pub fn from_c_str(value: &CStr) -> Self
	{
		Self::from_byte_slice(value.to_bytes())
	}

	/// From str.
	#[inline(always)]
	pub fn from_byte_slice(value: &[u8]) -> FileSystemType
	{
		use self::FileSystemType::*;

		match value
		{
			b"anon_inodefs" => anon_inodefs,
			b"bdev" => bdev,
			b"bpf" => bpf,
			b"binfmt_misc" => binfmt_misc,
			b"cgroup" => cgroup,
			b"cgroup2" => cgroup2,
			b"cpuset" => cpuset,
			b"dax" => dax,
			b"debugfs" => debugfs,
			b"devpts" => devpts,
			b"devtmpfs" => devtmpfs,
			b"ecryptfs" => ecryptfs,
			b"ext2" => ext2,
			b"ext3" => ext3,
			b"ext4" => ext4,
			b"fuse" => fuse,
			b"fuseblk" => fuseblk,
			b"fusectl" => fusectl,
			b"hugetlbfs" => hugetlbfs,
			b"mqueue" => mqueue,
			b"pipefs" => pipefs,
			b"prl_fs" => prl_fs,
			b"proc" => proc,
			b"pstore" => pstore,
			b"ramfs" => ramfs,
			b"rootfs" => rootfs,
			b"security" => security,
			b"securityfs" => securityfs,
			b"sockfs" => sockfs,
			b"sysfs" => sysfs,
			b"tmpfs" => tmpfs,
			b"tracefs" => tracefs,
			b"vfat" => vfat,

			_ => Unrecognised(value.to_vec().into_boxed_slice())
		}
	}
}
