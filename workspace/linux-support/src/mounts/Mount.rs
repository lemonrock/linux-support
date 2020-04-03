// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a mount.
#[derive(Debug)]
pub struct Mount
{
	/// eg eg `/dev/sda1`, `proc`, etc; not really that useful.
	pub source: CString,

	/// Mount point.
	pub mount_point: PathBuf,

	/// File system type.
	///
	/// eg `proc`, `sysfs`, `hugetlbs`, `ext4`; listed in second column of `/proc/filesystems`/
	pub file_system_type: FileSystemType,

	/// Mount options.
	///
	/// eg `nodev mode=0177`
	pub mount_options: HashMap<Box<[u8]>, Option<Box<[u8]>>>,

	/// Typically `0` (zero).
	dump_frequency_in_days: i32,

	/// Typically `0` (zero).
	pass_number_on_parallel_filesystem_type: i32,
}

impl Mount
{
	/// Unmounts.
	pub fn unmount(mount_point: &Path, unmount_flags: UnmountFlags) -> io::Result<()>
	{
		use self::ErrorKind::*;

		let target = mount_point.to_c_string();
		match unsafe { umount2(target.as_ptr(), unmount_flags.bits()) }
		{
			0 => Ok(()),

			-1 => match errno().0
			{
				EAGAIN =>
				{
					if unmount_flags.contains(UnmountFlags::Expire)
					{
						Ok(())
					}
					else
					{
						panic!("umount() set an illegal errno of EAGAIN when unmount flags did not contain MNT_EXPIRE");
					}
				},
				EBUSY => Err(io::Error::new(TimedOut, "Busy")),
				EPERM => Err(io::Error::new(PermissionDenied, "permission denied")),

				ENOENT => Err(io::Error::new(NotFound, "Mount path had an empty or non-existent component")),
				EINVAL => Err(io::Error::new(InvalidData, "One of many possible failures (EINVAL)")),
				ENOMEM => panic!("Out of memory (ENOMEM)"),
				ENAMETOOLONG => panic!("mount_point path name is too long"),
				EFAULT => panic!("Invalid data"),

				illegal @ _ => panic!("umount() set an illegal errno '{}'", illegal),
			},

			illegal @ _ => panic!("umount() returned an illegal result '{}'", illegal),
		}
	}

	/// Does this mount have this file system type?
	#[inline(always)]
	pub fn has_file_system_type(&self, file_system_type: &FileSystemType) -> bool
	{
		&self.file_system_type == file_system_type
	}

	fn from_mntent(raw: *mut mntent) -> Self
	{
		debug_assert!(!unsafe {(*raw).mnt_fsname }.is_null(), "null");
		let source = unsafe { CStr::from_ptr((*raw).mnt_fsname) }.to_owned();
		let mount_point = (unsafe { c_string_pointer_to_path_buf((*raw).mnt_dir) }).expect("mnt_dir was empty").expect("mnt_dir was null");
		let file_system_type = FileSystemType::from_c_str(unsafe { CStr::from_ptr((*raw).mnt_type) });

		let mount_options_string = unsafe { CStr::from_ptr((*raw).mnt_opts) };

		let mut mount_options = HashMap::with_capacity(16);
		for mount_option_string in mount_options_string.to_bytes().split(|byte| *byte == b',')
		{
			let mut split = mount_option_string.splitn(2, |byte| *byte == b'=');
			let name = split.next().unwrap().to_vec().into_boxed_slice();
			let value_if_any = split.next().map(|value| value.to_vec().into_boxed_slice());
			assert!(mount_options.insert(name, value_if_any).is_none(), "Duplicate key in mount options for mount_point '{:?}'", mount_point);
		}

		Self
		{
			source,
			mount_point,
			file_system_type,
			mount_options,
			dump_frequency_in_days: unsafe { (*raw).mnt_freq },
			pass_number_on_parallel_filesystem_type: unsafe { (*raw).mnt_passno },
		}
	}

	/// New instance for file systems which do not have a source (eg `hugetlbs`).
	pub fn new_where_source_is_file_system_type(mount_point: PathBuf, file_system_type: FileSystemType, mount_options: HashMap<Box<[u8]>, Option<Box<[u8]>>>) -> Self
	{
		Self
		{
			source: file_system_type.to_c_string(),
			mount_point,
			file_system_type,
			mount_options,
			dump_frequency_in_days: 0,
			pass_number_on_parallel_filesystem_type: 0,
		}
	}

	/// Mount.
	pub fn mount(&self, mount_flags: MountFlags) -> Result<(), io::Error>
	{
		mount_wrapper(&self.source, &self.mount_point, &self.file_system_type, &self.mount_options, mount_flags)
	}
}