// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Note that a `PathFileDescriptor` will cause an extra syscall after completion.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpenOnDisk<'a, Open: OnDiskFileDescriptor>
{
	open_how: open_how,
	path: Option<&'a CStr>,
	marker: PhantomData<Open>,
}

impl<'a, Open: OnDiskFileDescriptor> OpenOnDisk<'a, Open>
{
	#[inline(always)]
	fn path(&self) -> u64
	{
		let path = match self.path
		{
			None => SubmissionQueueEntry::empty_path(),
			
			Some(path) => SubmissionQueueEntry::non_empty_path(path),
		};
		SubmissionQueueEntry::to_u64_non_null(path)
	}
	
	#[inline(always)]
	fn open_character_device_internal<CharacterDevice: OnDiskFileDescriptor>(path: &'a CStr, path_resolution: PathResolution, access: Access, no_follow: bool) -> OpenOnDisk<CharacterDevice>
	{
		let mut flags = access.to_oflag() | O_CLOEXEC | O_NONBLOCK | O_NOCTTY;
		
		if no_follow
		{
			flags |= O_NOFOLLOW;
		}
		
		OpenOnDisk::<CharacterDevice>::open(flags, 0, path_resolution, Some(path))
	}
	
	#[inline(always)]
	fn open(flags: i32, mode_for_O_CREAT_and_O_TMPFILE: mode_t, path_resolution: PathResolution, path: Option<&'a CStr>) -> Self
	{
		Self
		{
			open_how: open_how
			{
				flags: flags as u64,
				mode: mode_for_O_CREAT_and_O_TMPFILE as u64,
				resolve: path_resolution.bits(),
			},
			path,
			marker: PhantomData,
		}
	}
}

impl<'a> OpenOnDisk<'a, BlockDeviceFileDescriptor>
{
	/// Open an existing block device.
	#[inline(always)]
	pub fn open_block_device(path: &'a CStr, path_resolution: PathResolution, access: Access, fail_if_path_name_is_a_block_device_and_is_in_use: bool, no_follow: bool) -> Self
	{
		let mut flags = access.to_oflag() | O_CLOEXEC | O_NONBLOCK;
		
		if fail_if_path_name_is_a_block_device_and_is_in_use
		{
			flags |= O_EXCL;
		}
		
		if no_follow
		{
			flags |= O_NOFOLLOW;
		}
		
		Self::open(flags, 0, path_resolution, Some(path))
	}
}

impl<'a> OpenOnDisk<'a, CharacterDeviceFileDescriptor>
{
	/// Open an existing character device.
	///
	/// Uses `O_NOCTTY` to prevent it taking over the terminal.
	#[inline(always)]
	pub fn open_character_device(path: &'a CStr, path_resolution: PathResolution, access: Access, no_follow: bool) -> Self
	{
		Self::open_character_device_internal::<CharacterDeviceFileDescriptor>(path, path_resolution, access, no_follow)
	}
}

impl<'a> OpenOnDisk<'a, TerminalFileDescriptor>
{
	/// Open an existing terminal.
	///
	/// Uses `O_NOCTTY` to prevent it taking over the terminal.
	#[inline(always)]
	pub fn open_terminal(path: &'a CStr, path_resolution: PathResolution, access: Access, no_follow: bool) -> Self
	{
		Self::open_character_device_internal::<TerminalFileDescriptor>(path, path_resolution, access, no_follow)
	}
}

impl<'a> OpenOnDisk<'a, DirectoryFileDescriptor>
{
	/// Open an existing directory.
	#[inline(always)]
	pub fn open_directory(path: &'a CStr, path_resolution: PathResolution) -> Self
	{
		Self::open(O_RDONLY | O_DIRECTORY | O_CLOEXEC, 0, path_resolution, Some(path))
	}
}

impl<'a> OpenOnDisk<'a, ReceivePipeFileDescriptor>
{
	/// Open an existing pipe (FIFO) to receive.
	#[inline(always)]
	pub fn open_receive_pipe(path: &'a CStr, path_resolution: PathResolution) -> Self
	{
		Self::open(O_RDONLY | O_CLOEXEC | O_NONBLOCK, 0, path_resolution, Some(path))
	}
}

impl<'a> OpenOnDisk<'a, SendPipeFileDescriptor>
{
	/// Open an existing pipe (FIFO) to send.
	///
	/// Fails if the receive pipe other end is not open by any process.
	#[inline(always)]
	pub fn open_send_pipe(path: &'a CStr, path_resolution: PathResolution) -> Self
	{
		Self::open(O_WRONLY | O_CLOEXEC | O_NONBLOCK, 0, path_resolution, Some(path))
	}
	
	/// Open an existing pipe (FIFO) to send.
	///
	/// Succeeds even if the receive pipe other end is not open by any process.
	#[inline(always)]
	pub fn open_send_pipe_irrespective_of_receive_pipe_being_open_by_any_process(path: &'a CStr, path_resolution: PathResolution) -> Self
	{
		Self::open(O_RDWR | O_CLOEXEC | O_NONBLOCK, 0, path_resolution, Some(path))
	}
}

impl<'a> OpenOnDisk<'a, PathFileDescriptor>
{
	/// Open an existing path.
	#[inline(always)]
	pub fn open_path(path: &'a CStr, path_resolution: PathResolution, is_directory: bool, no_follow: bool) -> Self
	{
		let mut flags = O_PATH | O_CLOEXEC;
		
		if is_directory
		{
			flags |= O_DIRECTORY
		}
		
		if no_follow
		{
			flags |= O_NOFOLLOW
		}
		
		Self::open(flags, 0, path_resolution, Some(path))
	}
}

impl<'a> OpenOnDisk<'a, File>
{
	/// Open an existing regular file.
	///
	/// `disable_linux_page_cache`: `O_DIRECT`.
	#[inline(always)]
	pub fn open_file_failing_if_it_does_not_exist(path: &'a CStr, path_resolution: PathResolution, disable_linux_page_cache: bool, access: Access, no_follow: bool) -> Self
	{
		let mut flags = access.to_oflag() | O_NOCTTY;
		
		if no_follow
		{
			flags |= O_NOFOLLOW;
		}
		
		if disable_linux_page_cache
		{
			flags |= O_DIRECT;
		}
		
		Self::open(flags, 0, path_resolution, Some(path))
	}
	
	/// Open an existing pipe (FIFO) to receive.
	///
	/// Fails to create if it already exists and `exclusive` is `true`.
	///
	/// `disable_linux_page_cache`: `O_DIRECT`.
	#[inline(always)]
	pub fn open_file_creating_if_it_does_not_exist(path: &'a CStr, path_resolution: PathResolution, disable_linux_page_cache: bool, access: Access, exclusive: bool, no_follow: bool, access_permissions: AccessPermissions) -> Self
	{
		let mut flags = access.to_oflag() | O_NOCTTY | O_CREAT;
		
		if exclusive
		{
			flags |= O_EXCL;
		}
		
		if no_follow
		{
			flags |= O_NOFOLLOW;
		}
		
		if disable_linux_page_cache
		{
			flags |= O_DIRECT;
		}
		
		Self::open(flags, DirectoryFileDescriptor::mask_mode(access_permissions), path_resolution, Some(path))
	}
	
	/// Open an existing pipe (FIFO) to receive.
	///
	/// Fails to create if it already exists and `exclusive` is `true`.
	///
	/// Specifying `exclusive` prevents a temporary file from being linked into the filesystem and so makes it more secure.
	///
	/// `disable_linux_page_cache`: `O_DIRECT`.
	#[inline(always)]
	pub fn open_file_temporary(path_resolution: PathResolution, disable_linux_page_cache: bool, access: Access, exclusive: bool, access_permissions: AccessPermissions) -> Self
	{
		let mut flags = access.to_oflag() | O_TMPFILE;
		
		if exclusive
		{
			flags |= O_EXCL;
		}
		
		if disable_linux_page_cache
		{
			flags |= O_DIRECT;
		}
		
		Self::open(flags, DirectoryFileDescriptor::mask_mode(access_permissions), path_resolution, None)
	}
}
