// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.

/// Represents a file descriptor backed by real storage.
pub trait OnDiskFileDescriptor: FileDescriptor
{
	/// Gets the metadata of the underlying file system.
	///
	/// This may work on epoll, signal and event file descriptors but Linux as usual doesn't make this clear.
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

	/// Also known as 'version number'.
	///
	/// Can legitimately fail if the file system does not support generation numbers.
	#[inline(always)]
	fn get_inode_generation_number(&self) -> io::Result<InodeGenerationNumber>
	{
		#[allow(deprecated)]
		let mut inode_generation_number = unsafe { uninitialized() };
		let result = unsafe { ioctl (self.as_raw_fd(), FS_IOC_GETVERSION, &mut inode_generation_number) };
		if likely!(result == 0)
		{
			Ok(InodeGenerationNumber::from_i32(inode_generation_number))
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("ioctl() returned unexpected result {}", result)
		}
	}

	/// Also known as 'version number'.
	///
	/// Can legitimately fail if the file system does not support generation numbers.
	#[inline(always)]
	fn set_inode_generation_number(&self, inode_generation_number: InodeGenerationNumber) -> io::Result<()>
	{
		let inode_generation_number: i32 = inode_generation_number.into();
		let result = unsafe { ioctl (self.as_raw_fd(), FS_IOC_SETVERSION, &inode_generation_number) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("ioctl() returned unexpected result {}", result)
		}
	}

	/// Also known as 'attributes'.
	///
	/// See the `lsattr` and the `chattr` programs.
	#[inline(always)]
	fn get_inode_flags(&self) -> io::Result<InodeFlags>
	{
		#[allow(deprecated)]
		let mut attributes = unsafe { uninitialized() };
		let result = unsafe { ioctl (self.as_raw_fd(), FS_IOC_GETFLAGS, &mut attributes) };
		if likely!(result == 0)
		{
			Ok(InodeFlags::from_bits(attributes).unwrap())
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("ioctl() returned unexpected result {}", result)
		}
	}

	/// Also known as 'attributes'.
	///
	/// See the `lsattr` and the `chattr` programs.
	#[inline(always)]
	fn set_inode_flags(&self, inode_flags: InodeFlags) -> io::Result<()>
	{
		let attributes = inode_flags.bits();
		let result = unsafe { ioctl (self.as_raw_fd(), FS_IOC_SETFLAGS, &attributes) };
		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("ioctl() returned unexpected result {}", result)
		}
	}

	/// Get an extended attribute.
	///
	/// Limited to 64Kb.
	///
	/// Returns `Ok(None)` if the attribute is not present.
	#[inline(always)]
	fn get_extended_attribute(&self, extended_attribute_name: ExtendedAttributeName) -> io::Result<Option<Vec<u8>>>
	{
		let SixtyFourKb = 64 * 1024;
		let mut value = Vec::with_capacity(SixtyFourKb);
		let result = unsafe { fgetxattr(self.as_raw_fd(), extended_attribute_name.as_ptr(), value.as_mut_ptr() as *mut c_void, SixtyFourKb) };
		if likely!(result >= 0)
		{
			unsafe { value.set_len(result as usize) };
			value.shrink_to_fit();
			Ok(Some(value))
		}
		else if likely!(result == 0)
		{
			match errno().0
			{
				ENODATA => Ok(None),
				_ => Err(io::Error::last_os_error()),
			}
		}
		else
		{
			unreachable!("fgetxattr() returned unexpected result {}", result)
		}
	}

	/// Set an extended attribute, creating or replacing it as necessary.
	#[inline(always)]
	fn set_extended_attribute(&self, extended_attribute_name: ExtendedAttributeName, value: &[u8]) -> io::Result<()>
	{
		self.set_extended_attribute_internal(extended_attribute_name, value, 0).map(|_| ())
	}

	/// Create an extended attribute or fail.
	///
	/// Return `Ok(false)` if could not create because it exists.
	#[inline(always)]
	fn create_extended_attribute(&self, extended_attribute_name: ExtendedAttributeName, value: &[u8]) -> io::Result<bool>
	{
		self.set_extended_attribute_internal(extended_attribute_name, value, XATTR_CREATE)
	}

	/// Replace an extended attribute or fail.
	///
	/// Return `Ok(false)` if could not replace because it does not exist.
	#[inline(always)]
	fn replace_extended_attribute(&self, extended_attribute_name: ExtendedAttributeName, value: &[u8]) -> io::Result<bool>
	{
		self.set_extended_attribute_internal(extended_attribute_name, value, XATTR_REPLACE)
	}

	#[doc(hidden)]
	#[inline(always)]
	fn set_extended_attribute_internal(&self, extended_attribute_name: ExtendedAttributeName, value: &[u8], flags: i32) -> io::Result<bool>
	{
		let result = unsafe { fsetxattr(self.as_raw_fd(), extended_attribute_name.as_ptr(), value.as_ptr() as *const c_void, value.len(), flags) };
		if likely!(result == 0)
		{
			Ok(true)
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				ENODATA if flags == XATTR_REPLACE =>
				{
					Ok(false)
				},

				EEXIST if flags == XATTR_CREATE =>
				{
					Ok(false)
				},

				_ => Err(io::Error::last_os_error()),
			}
		}
		else
		{
			unreachable!("fremovexattr() returned unexpected result {}", result)
		}
	}

	/// Remove an extended attribute.
	///
	/// Returns `Ok(true)` if removed and `Ok(false)` if the attribute wasn't present.
	#[inline(always)]
	fn remove_extended_attribute(&self, extended_attribute_name: ExtendedAttributeName) -> io::Result<bool>
	{
		let result = unsafe { fremovexattr(self.as_raw_fd(), extended_attribute_name.as_ptr()) };
		if likely!(result == 0)
		{
			Ok(true)
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				ENODATA => Ok(false),
				_ => Err(io::Error::last_os_error()),
			}
		}
		else
		{
			unreachable!("fremovexattr() returned unexpected result {}", result)
		}
	}

	/// List extended attributes.
	///
	/// Requires 64Kb of free memory.
	#[inline(always)]
	fn list_extended_attributes(&self) -> io::Result<ExtendedAttributeNames>
	{
		const SixtyFourKb: usize = 64 * 1024;
		let mut buffer = Vec::with_capacity(SixtyFourKb);
		let result = unsafe { flistxattr(self.as_raw_fd(), buffer.as_mut_ptr() as *mut c_char, SixtyFourKb) };
		if likely!(result >= 0)
		{
			unsafe { buffer.set_len(result as usize) };
			buffer.shrink_to_fit();
			Ok(ExtendedAttributeNames::new(buffer))
		}
		else if likely!(result == -1)
		{
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable!("flistxattr() returned unexpected result {}", result)
		}
	}
}
