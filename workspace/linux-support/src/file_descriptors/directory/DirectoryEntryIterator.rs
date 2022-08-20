// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Iterates over directory entries.
///
/// Note that the returned triplet of (Inode, FileType, File Name) only has a lifetime as long as the next call to `next()`.
pub struct DirectoryEntryIterator<'a>
{
	directory_file_descriptor: &'a DirectoryFileDescriptor,
	buffer_offset: usize,
	buffer_end: usize,
	last_item: Option<NonNull<dirent>>,
	buffer: [u8; BufferCapacity],
}

const BufferCapacity: usize = size_of::<dirent>() * 16;

impl<'a> StreamingIterator for DirectoryEntryIterator<'a>
{
	type Item = DirectoryEntry<'a>;

	#[inline(always)]
	fn advance(&mut self)
	{
		if unlikely!(self.has_finished())
		{
			return
		}

		// NOTE: Strictly, `self.buffer_offset != self.buffer_end`, but `>=` is defensive.
		if unlikely!(self.buffer_offset >= self.buffer_end)
		{
			const LowestInvalidLength: usize = BufferCapacity + 1;
			match system_call_getdents(self.directory_file_descriptor.as_raw_fd(), self.buffer.as_mut_ptr() as *mut c_void, BufferCapacity).as_usize()
			{
				0 => self.finished(),
				
				length @ 1 ..= BufferCapacity =>
				{
					debug_assert!(length <= BufferCapacity);
					self.buffer_offset = 0;
					self.buffer_end = length;
				}
				
				invalid_length @ LowestInvalidLength ..= SystemCallResult::InclusiveOkRangeEndsAt_usize => panic!("Length {} exceeded BufferCapacity of {}", invalid_length, BufferCapacity),
				
				SystemCallResult::ENOENT_usize => return self.finished(),
				SystemCallResult::EBADF_usize => panic!("Invalid file descriptor fd"),
				SystemCallResult::EFAULT_usize => panic!("Argument points outside the calling process's address space"),
				SystemCallResult::EINVAL_usize => panic!("Result buffer is too small"),
				SystemCallResult::ENOTDIR_usize => panic!("File descriptor does not refer to a directory"),
				unexpected_error @ SystemCallResult::InclusiveErrorRangeStartsFrom_usize..= SystemCallResult::InclusiveErrorRangeEndsAt_usize => panic!("Unexpected error {} from from getdents()", unexpected_error)
			}
		}

		let current = unsafe { new_non_null(self.buffer.as_mut_ptr().add(self.buffer_offset) as *mut dirent) };

		self.buffer_offset += unsafe { current.as_ref().d_reclen as usize };
		self.last_item = Some(current);
	}

	#[inline(always)]
	fn get(&self) -> Option<&Self::Item>
	{
		self.last_item.map(|dirent| unsafe { & * dirent.cast::<DirectoryEntry>().as_ptr() })
	}
}

impl<'a> DirectoryEntryIterator<'a>
{
	/// Rewinds this iterator to the beginning.
	#[inline(always)]
	pub fn rewind(&mut self) -> io::Result<()>
	{
		self.seek(0)
	}
	
	/// Rewinds this iterator to a specific entry.
	#[inline(always)]
	pub fn rewind_to(&mut self, directory_entry_rewind_position: DirectoryEntryRewindPosition<'a>) -> io::Result<()>
	{
		self.seek(directory_entry_rewind_position.0)
	}

	#[inline(always)]
	fn new(directory_file_descriptor: &'a DirectoryFileDescriptor) -> Self
	{
		Self
		{
			directory_file_descriptor,
			buffer_offset: 0,
			buffer_end: 0,
			last_item: None,
			buffer: unsafe_zeroed(),
		}
	}

	#[inline(always)]
	fn seek(&mut self, offset: off_t) -> io::Result<()>
	{
		let result = unsafe { lseek(self.directory_file_descriptor.as_raw_fd(), offset, SEEK_SET) };
		if likely!(result == offset)
		{
			self.buffer_offset = 0;
			self.buffer_end = 0;
			self.last_item = None;

			Ok(())
		}
		else if likely!(result == -1)
		{
			self.finished();
			Err(io::Error::last_os_error())
		}
		else
		{
			unreachable_code(format_args!("Invalid result {} from lseek()", result))
		}
	}

	#[inline(always)]
	fn finished(&mut self)
	{
		self.buffer_end = BufferCapacity + 1;
		self.last_item = None;
	}

	#[inline(always)]
	fn has_finished(&self) -> bool
	{
		self.buffer_end == BufferCapacity + 1
	}
}
