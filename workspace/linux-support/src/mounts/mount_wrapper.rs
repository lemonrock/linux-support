// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Mount wrapper function.
pub fn mount_wrapper(source: &CStr, mount_point: &Path, file_system_type: &FileSystemType, mount_options: &HashMap<Box<[u8]>, Option<Box<[u8]>>>, mount_flags: MountFlags) -> io::Result<()>
{
	use self::ErrorKind::*;

	fn to_mount_options_c_string(mount_options: &HashMap<Box<[u8]>, Option<Box<[u8]>>>) -> CString
	{
		let mut mount_options_cstring: Vec<u8> = Vec::with_capacity(64);
		let mut after_first = false;
		for (name, value_if_any) in mount_options
		{
			if after_first
			{
				mount_options_cstring.push(b',');
			}
			else
			{
				after_first = true;
			}
			mount_options_cstring.extend_from_slice(name);
			if let Some(ref value) = *value_if_any
			{
				mount_options_cstring.push(b'=');
				mount_options_cstring.extend_from_slice(value);
			}
		}
		CString::new(mount_options_cstring).expect("mount_options should not contain interior ASCII NULs")
	}

	let target = mount_point.to_c_string();
	let file_system_type = file_system_type.to_c_string();
	let data = to_mount_options_c_string(mount_options);

	match unsafe { mount(source.as_ptr(), target.as_ptr(), file_system_type.as_ptr(), mount_flags.bits(), data.as_ptr() as *mut c_void) }
	{
		0 => Ok(()),

		-1 => match errno().0
		{
			EACCES => Err(io::Error::new(NotFound, "Component of mount path to mount does not exist")),
			ENOENT => Err(io::Error::new(NotFound, "Mount path had an empty or non-existent component")),
			ENOTDIR => Err(io::Error::new(NotFound, "target or source is not a directory")),
			ELOOP => Err(io::Error::new(NotFound, "Loops - target is a descendant of source, or too many links in mount path")),
			EPERM => Err(io::Error::new(PermissionDenied, "permission denied")),
			EBUSY => Err(io::Error::new(TimedOut, "Busy")),
			EINVAL => Err(io::Error::new(InvalidData, "One of many possible failures (EINVAL)")),

			EMFILE => panic!("Out of memory (EMFILE)"),
			ENOMEM => panic!("Out of memory (ENOMEM)"),
			ENODEV => panic!("File system type not supported by Linux Kernel (check /proc/filesystem first)"),
			ENOTBLK => panic!("Specified block device wasn't"),
			ENXIO => panic!("Block device major number is out of range"),
			ENAMETOOLONG => panic!("Mount path name is too long"),
			EFAULT => panic!("Invalid data"),

			illegal @ _ => panic!("mount() set an illegal errno '{}'", illegal),
		},

		illegal @ _ => panic!("mount() returned an illegal result '{}'", illegal),
	}
}
