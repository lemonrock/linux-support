// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// File system types.
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum FileSystemType
{
	bdev,
	cgroup,
	cpuset,
	devpts,
	devtmpfs,
	ext2,
	ext3,
	ext4,
	hugetlbfs,
	mqueue,
	pipefs,
	pstore,
	ramfs,
	rootfs,
	security,
	sockfs,
	sysfs,
	tmpfs,
	_proc,

	anon_inodefs,
	binfmt_misc,
	debugfs,
	ecryptfs,
	fuse,
	fuseblk,
	fusectl,
	prl_fs,
	securityfs,
	vfat,
	dax,
	cgroup2,

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
	/// To `CString`.
	#[inline(always)]
	pub fn to_c_string(&self) -> CString
	{
		use self::FileSystemType::*;

		let ref_value = match *self
		{
			sysfs => "sysfs",
			rootfs => "rootfs",
			ramfs => "ramfs",
			bdev => "bdev",
			_proc => "proc",
			cpuset => "cpuset",
			cgroup => "cgroup",
			tmpfs => "tmpfs",
			devtmpfs => "devtmpfs",
			security => "security",
			sockfs => "sockfs",
			pipefs => "pipefs",
			devpts => "devpts",
			hugetlbfs => "hugetlbfs",
			pstore => "pstore",
			mqueue => "mqueue",
			ext2 => "ext2",
			ext3 => "ext3",
			ext4 => "ext4",

			anon_inodefs => "anon_inodefs",
			binfmt_misc => "binfmt_misc",
			debugfs => "debugfs",
			ecryptfs => "ecryptfs",
			fuse => "fuse",
			fuseblk => "fuseblk",
			fusectl => "fusectl",
			prl_fs => "prl_fs",
			securityfs => "securityfs",
			vfat => "vfat",
			dax => "dax",
			cgroup2 => "cgroup2",

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
			sysfs => b"sysfs" as &[u8],
			rootfs => b"rootfs" as &[u8],
			ramfs => b"ramfs" as &[u8],
			bdev => b"bdev" as &[u8],
			_proc => b"proc" as &[u8],
			cpuset => b"cpuset" as &[u8],
			cgroup => b"cgroup" as &[u8],
			tmpfs => b"tmpfs" as &[u8],
			devtmpfs => b"devtmpfs" as &[u8],
			security => b"security" as &[u8],
			sockfs => b"sockfs" as &[u8],
			pipefs => b"pipefs" as &[u8],
			devpts => b"devpts" as &[u8],
			hugetlbfs => b"hugetlbfs" as &[u8],
			pstore => b"pstore" as &[u8],
			mqueue => b"mqueue" as &[u8],
			ext2 => b"ext2" as &[u8],
			ext3 => b"ext3" as &[u8],
			ext4 => b"ext4" as &[u8],
			
			anon_inodefs => b"anon_inodefs" as &[u8],
			binfmt_misc => b"binfmt_misc" as &[u8],
			debugfs => b"debugfs" as &[u8],
			ecryptfs => b"ecryptfs" as &[u8],
			fuse => b"fuse" as &[u8],
			fuseblk => b"fuseblk" as &[u8],
			fusectl => b"fusectl" as &[u8],
			prl_fs => b"prl_fs" as &[u8],
			securityfs => b"securityfs" as &[u8],
			vfat => b"vfat" as &[u8],
			dax => b"dax" as &[u8],
			cgroup2 => b"cgroup2" as &[u8],
			
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
			b"sysfs" => sysfs,
			b"rootfs" => rootfs,
			b"ramfs" => ramfs,
			b"bdev" => bdev,
			b"proc" => _proc,
			b"cpuset" => cpuset,
			b"cgroup" => cgroup,
			b"tmpfs" => tmpfs,
			b"devtmpfs" => devtmpfs,
			b"security" => security,
			b"sockfs" => sockfs,
			b"pipefs" => pipefs,
			b"devpts" => devpts,
			b"hugetlbfs" => hugetlbfs,
			b"pstore" => pstore,
			b"mqueue" => mqueue,
			b"ext2" => ext2,
			b"ext3" => ext3,
			b"ext4" => ext4,

			b"anon_inodefs" => anon_inodefs,
			b"binfmt_misc" => binfmt_misc,
			b"debugfs" => debugfs,
			b"ecryptfs" => ecryptfs,
			b"fuse" => fuse,
			b"fuseblk" => fuseblk,
			b"fusectl" => fusectl,
			b"prl_fs" => prl_fs,
			b"securityfs" => securityfs,
			b"vfat" => vfat,
			b"dax" => dax,
			b"cgroup2" => cgroup2,

			_ => Unrecognised(value.to_vec().into_boxed_slice())
		}
	}
}
