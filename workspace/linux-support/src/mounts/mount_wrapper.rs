// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Mount wrapper function.
pub fn mount_wrapper<'a>(source: &CStr, mount_point: &Path, file_system_type: &FileSystemType, mount_options: &HashMap<Cow<'a, [u8]>, Option<Cow<'a, [u8]>>>, mount_flags: MountFlags) -> io::Result<()>
{
	fn to_mount_options_c_string<'a>(mount_options: &HashMap<Cow<'a, [u8]>, Option<Cow<'a, [u8]>>>) -> CString
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
			mount_options_cstring.extend_from_slice(&name[..]);
			if let Some(ref value) = *value_if_any
			{
				mount_options_cstring.push(b'=');
				mount_options_cstring.extend_from_slice(&value[..]);
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

		-1 => match SystemCallErrorNumber::from_errno_panic()
		{
			EACCES => Err(io_error_not_found("Component of mount path to mount does not exist")),
			ENOENT => Err(io_error_not_found("Mount path had an empty or non-existent component")),
			ENOTDIR => Err(io_error_not_found("target or source is not a directory")),
			ELOOP => Err(io_error_not_found("Loops - target is a descendant of source, or too many links in mount path")),
			EPERM => Err(io_error_permission_denied("permission denied")),
			EBUSY => Err(io_error_timed_out("Busy")),
			EINVAL => Err(io_error_invalid_data("One of many possible failures (EINVAL)")),

			EMFILE => panic!("Out of memory (EMFILE)"),
			ENOMEM => panic!("Out of memory (ENOMEM)"),
			ENODEV => panic!("File system type not supported by Linux Kernel (check /proc/filesystem first)"),
			ENOTBLK => panic!("Specified block device wasn't"),
			ENXIO => panic!("Block device major number is out of range"),
			ENAMETOOLONG => panic!("Mount path name is too long"),
			EFAULT => panic!("Invalid data"),
			
			unexpected_error @ _ => unexpected_error!(mount, unexpected_error),
		},
		
		unexpected @ _ => unexpected_result!(mount, unexpected),
	}
}
