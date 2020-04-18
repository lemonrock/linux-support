// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.

/// Represents a file descriptor backed by real storage.
pub trait OnDiskFileDescriptor: FileDescriptor
{
	/// Gets the metadata of the underlying file system.
	#[inline(always)]
	fn underlying_file_system_metadata(&self) -> io::Result<FileSystemMetadata>
	{
		#[allow(deprecated)]
		let mut statvfs: statvfs = unsafe { uninitialized() };
		let result = unsafe { fstatvfs(self.as_raw_fd(), &mut statvfs) };
		if likely!(result == 0)
		{
			Ok(FileSystemMetadata(statvfs))
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("Unexpected result {} from fstatvfs()", result)
		}
	}
}

/*
*  (files, directories, fifos, character/block, path file descriptors but not unix socket file descriptors and not memory file descriptors)
    * fstatvfs
    * fgetxattr
    * fsetxattr
    * flistxattr
    * fremovexattr
        * Extended attributes are name:value pairs; can be zero length; can find length but can change between calls
        * Names can be namespaced (`security.`, `system.`, `trusted.`, and `user.`)
        * The VFS imposes limitations that an attribute names is limited to 255
                 bytes and an attribute value is limited to 64 kB.  The list of
                 attribute names that can be returned is also limited to 64 kB (see
                 BUGS in listxattr(2)).
    * Inode Flags, sometimes known as attributes.
        * http://man7.org/linux/man-pages/man2/ioctl_iflags.2.html (eg immutable, append only, etc)
*/
