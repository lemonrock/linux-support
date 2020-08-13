// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A version 2 root cgroup.
///
/// See <https://www.kernel.org/doc/Documentation/cgroup-v2.txt>.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RootCgroup;

impl<'name> Cgroup<'name> for RootCgroup
{
	#[inline(always)]
	fn to_path<'b>(&self, mount_point: &'b CgroupMountPoint) -> Cow<'b, Path>
	{
		Cow::Borrowed(mount_point.to_path())
	}
	
	/// Does not check if the child exists.
	#[inline(always)]
	fn child(self: Rc<Self>, name: impl Into<Cow<'name, CgroupName>>) -> Rc<NonRootCgroup<'name>>
	{
		Rc::new(NonRootCgroup::ChildOfRoot { name: name.into() })
	}
}

impl RootCgroup
{
	/// Extant children.
	#[inline(always)]
	pub fn extant_children(mount_point: &CgroupMountPoint) -> io::Result<impl Iterator<Item=NonRootCgroup>>
	{
		child_cgroup_names(mount_point.to_path()).map(Self::child)
	}
	
	/// Creates, including parent folders, if does not already exist; then mounts.
	///
	/// Short-circuits creation if already exists (to avoid permission failures).
	#[inline(always)]
	pub fn create_and_mount(mount_point: &CgroupMountPoint) -> io::Result<()>
	{
		let folder_path = mount_point.to_path();
		
		if !folder_path.exists()
		{
			create_dir_all(&folder_path)?;
		}
		
		RootCgroup::create_and_mount(&mount_point)?;
		
		// cgroup2 specific mount options are in here: <https://www.kernel.org/doc/html/latest/admin-guide/cgroup-v2.html> - but `memory_recursiveprot` does not exist in code.
		let mount_options = hashmap!
		{
			Cow::Borrowed(b"uid" as &[u8]) => Some(Cow::Borrowed(b"0" as &[u8])),
			Cow::Borrowed(b"gid" as &[u8]) => Some(Cow::Borrowed(b"0" as &[u8])),
			Cow::Borrowed(b"mode" as &[u8]) => Some(Cow::Borrowed(b"0755" as &[u8])),
		};
		
		let mount = Mount::new_where_source_is_file_system_type(folder_path.to_path_buf(), FileSystemType::cgroup2, mount_options);
		mount.mount(MountFlags::DoNotUpdateAccessTimes | MountFlags::DoNotAllowDeviceFiles | MountFlags::DoNotAllowProgramsToBeExecuted | MountFlags::DoNotHonourSetUidAndSetGidPermissions)
	}
}
