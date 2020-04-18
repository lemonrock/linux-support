// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
fn pivot_root_wrapper(new_root: &Path, put_old: &Path)
{
	let new_root = path_to_cstring(new_root);
	let put_old = path_to_cstring(put_old);

	let result = unsafe { pivot_root(new_root.as_ptr() as *const c_char, put_old.as_ptr() as *const c_char) };
	if likely!(result == 0)
	{
		return
	}
	else if likely!(result == -1)
	{
		match errno().0
		{
			EBUSY => panic!("`new_root` or `put_old` are on the current root file system, or a file system is already mounted on `put_old`"),
			EINVAL => panic!("`put_old` is not underneath `new_root`"),
			ENOTDIR => panic!("`new_root` or `put_old` is not a directory"),
			EPERM => panic!("The calling process does not have the `CAP_SYS_ADMIN` capability"),

			EACCES => panic!("Search permission is denied for one of the directories in the path prefix of path"),
			EBADF => panic!("fd is bad (?how as a FD is not used)?"),
			EFAULT => panic!("Bad address"),
			ELOOP => panic!("Too many symbolic links encountered while traversing the path"),
			ENAMETOOLONG => panic!("path is too long"),
			ENOENT => panic!("A component of path does not exist, or path is an empty string"),
			ENOMEM => panic!("Out of (kernel) memory"),
			EOVERFLOW => panic!("path or fd refers to a file whose size, inode number, or number of blocks cannot be represented in, respectively, the types off_t, ino_t, or u64. This error can occur when, for example, an application compiled on a 32-bit platform without -D_FILE_OFFSET_BITS=64 calls stat() on a file whose size exceeds (1<<31)-1 bytes"),

			unknown @ _ => panic!("Unexpected error `{}` from `pivot_root()`", unknown),
		}
	}
	else
	{
		panic!("Invalid result from pivot_root of `{}`", result)
	}
}
