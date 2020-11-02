// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A trait for virtual file systems with fixed mount points.
pub trait VirtualFileSystemMountPoint: Sized
{
	/// Virtual file system type.
	const FileSystemType: FileSystemType;
	
	/// Default path below `/sys/fs`.
	#[inline(always)]
	fn default_sys_fs_path(sys_path: &SysPath) -> Self
	{
		Self::from_path(sys_path.file_system_fs_folder_path(Self::FileSystemType))
	}
	
	/// To a Path.
	fn to_path(&self) -> &Path;
	
	/// From path; path's existence and status of being mounted or not (with the correct file system type or not) is not checked.
	fn from_path(path: PathBuf) -> Self;
	
	/// With a relative path.
	#[inline(always)]
	fn with_relative_path(&self, relative_path: &OsStr) -> PathBuf
	{
		self.to_path().to_path_buf().append(relative_path)
	}
	
	/// Mount options.
	#[inline(always)]
	fn mount_options() -> HashMap<Cow<'static, [u8]>, Option<Cow<'static, [u8]>>>
	{
		fast_secure_hash_map!
		{
			Cow::Borrowed(b"uid" as &[u8]) => Some(Cow::Borrowed(b"0" as &[u8])),
			Cow::Borrowed(b"gid" as &[u8]) => Some(Cow::Borrowed(b"0" as &[u8])),
			Cow::Borrowed(b"mode" as &[u8]) => Some(Cow::Borrowed(b"0755" as &[u8])),
		}
	}
	
	/// Mount flags.
	#[inline(always)]
	fn mount_flags() -> MountFlags
	{
		MountFlags::DoNotUpdateAccessTimes | MountFlags::DoNotAllowDeviceFiles | MountFlags::DoNotAllowProgramsToBeExecuted | MountFlags::DoNotHonourSetUidAndSetGidPermissions
	}
	
	/// Creates, including parent folders, if does not already exist; then mounts.
	///
	/// Short-circuits creation if already exists (to avoid permission failures).
	#[inline(always)]
	fn create_and_mount(&self) -> io::Result<()>
	{
		let folder_path = self.to_path();
		
		if !folder_path.exists()
		{
			create_dir_all(&folder_path)?;
		}
		
		// cgroup2 specific mount options are in here: <https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v2.html> - but `memory_recursiveprot` does not exist in code.
		let mount_options = Self::mount_options();
		let mount_flags = Self::mount_flags();
		
		let mount = Mount::new_where_source_is_file_system_type(folder_path.to_path_buf(), FileSystemType::cgroup2, mount_options);
		mount.mount(mount_flags)
	}
}
