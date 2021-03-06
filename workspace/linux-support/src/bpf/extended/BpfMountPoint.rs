// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `mount -t bpf none /sys/fs/bpf`.
///
/// Used for pinned objects.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct BpfMountPoint(PathBuf);

impl Default for BpfMountPoint
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::default_sys_fs_path(&SysPath::default())
	}
}

impl VirtualFileSystemMountPoint for BpfMountPoint
{
	const FileSystemType: FileSystemType = FileSystemType::bpf;
	
	#[inline(always)]
	fn to_path(&self) -> &Path
	{
		&self.0
	}
	
	#[inline(always)]
	fn from_path(path: PathBuf) -> Self
	{
		Self(path)
	}
}

impl BpfMountPoint
{
	/// Passes `pinned_object_fully_qualified_file_path` to `pinned_object_file_path_user` as they are encountered.
	pub fn all_pinned_object_file_paths(&self, pinned_object_fully_qualified_file_path_user: &mut impl FnMut(PathBuf))
	{
		let mut folder_paths = vec![Cow::Borrowed(&self.0)];
		
		while let Some(folder_path) = folder_paths.pop()
		{
			if let Ok(read_dir) = (&folder_path).read_dir()
			{
				for dir_entry in read_dir
				{
					if let Ok(dir_entry) = dir_entry
					{
						if let Ok(metadata) = dir_entry.metadata()
						{
							if metadata.is_dir()
							{
								folder_paths.push(Cow::Owned(dir_entry.path()))
							}
							else if metadata.is_file()
							{
								pinned_object_fully_qualified_file_path_user(dir_entry.path())
							}
						}
					}
				}
			}
		}
	}
}
