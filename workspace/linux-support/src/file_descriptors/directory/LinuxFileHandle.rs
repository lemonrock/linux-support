// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a (Linux) file handle.
///
/// Can go stale (`ESTALE`) if a file is deleted or moved.
/// If a file is deleted and replaced by one with the same path, this file handle will not let the subsequent file be opened (they are considered distinct).
///
/// Major use case is to pass to another process something more 'robust' than a path string.
pub struct LinuxFileHandle
{
	file_system_mount_identifier: FileSystemMountIdentifier,
	file_handle: file_handle,
}

impl LinuxFileHandle
{
	/// Use this to find a file descriptor for a file system on which the file to be mounted is present.
	#[inline(always)]
	pub fn file_system_mount_identifier(&self) -> FileSystemMountIdentifier
	{
		self.file_system_mount_identifier
	}

	/// Difficult to use safely.
	///
	/// Caller needs the `CAP_DAC_READ_SEARCH` capability.
	#[inline(always)]
	pub unsafe fn open<MountFileDescriptor: FileDescriptor, OpenedFileDescriptor: FileDescriptor>(&mut self, file_descriptor_on_file_system_indicated_by_file_system_mount_identifier: MountFileDescriptor, o_flags: i32) -> io::Result<OpenedFileDescriptor>
	{
		let fd = file_descriptor_on_file_system_indicated_by_file_system_mount_identifier.as_raw_fd();
		let result = open_by_handle_at(fd, &mut self.file_handle, o_flags);
		if likely!(result == 0)
		{
			Ok(OpenedFileDescriptor::from_raw_fd(result))
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable_code(format_args!("result {} was unexpected for open_by_handle_at()", result))
		}
	}
}
