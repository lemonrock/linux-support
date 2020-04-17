// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents an inotify instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InotifyFileDescriptor(RawFd);

impl Drop for InotifyFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for InotifyFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for InotifyFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for InotifyFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for InotifyFileDescriptor
{
}

impl InotifyFileDescriptor
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new() -> Result<Rc<Self>, CreationError>
	{
		let result = unsafe { inotify_init1(IN_NONBLOCK | IN_CLOEXEC) };
		if likely!(result != -1)
		{
			Ok(Rc::new(InotifyFileDescriptor(result)))
		}
		else
		{
			use self::CreationError::*;

			Err
			(
				match errno().0
				{
					EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,
					ENFILE => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,
					ENOMEM => KernelWouldBeOutOfMemory,
					EINVAL => panic!("Invalid arguments"),
					_ => unreachable!(),
				}
			)
		}
	}

	/// Calls `add_watch()` after first converting the path.
	///
	/// Inefficient as makes a copy of the bytes in path and appends a trailing ASCII NUL (blame Rust for a poor design for Path and OsString).
	///
	/// Panics if `path` can not be so converted.
	pub fn add_watch_inefficient(this: &Rc<Self>, path: impl AsRef<Path>, flags: InotifyAddWatchFlags, adjust: bool) -> Result<InotifyWatchDescriptor, InotifyAddError>
	{
		let path = CString::new(path_bytes_without_trailing_nul(&path)).unwrap();
		Self::add_watch(this, &path, flags, adjust)
	}

	/// `adjust`, if true, assumes a watch already exists and adds the specified `flags` to it.
	///
	/// The maximum number of open (added) watch descriptors is specified in `/proc/sys/fs/inotify/max_user_watches`.
	pub fn add_watch(this: &Rc<Self>, path: &CStr, flags: InotifyAddWatchFlags, adjust: bool) -> Result<InotifyWatchDescriptor, InotifyAddError>
	{
		debug_assert_ne!(flags, InotifyAddWatchFlags::empty(), "flags must not be empty");

		let flags = if unlikely!(adjust)
		{
			flags.add_bitmask()
		}
		else
		{
			flags.set_bitmask()
		};

		let result = unsafe { inotify_add_watch(this.0, path.as_ptr(), flags) };

		if likely!(result >= 0)
		{
			Ok
			(
				InotifyWatchDescriptor
				{
					parent: Rc::downgrade(this),
					watch_descriptor: result,
				}
			)
		}
		else if likely!(result == -1)
		{
			use self::InotifyAddError::*;

			Err
			(
				match errno().0
				{
					EACCES => PermissionDenied,
					ENOMEM => KernelWouldBeOutOfMemory,
					ENOENT => FilePathInvalid,
					ENOSPC => MaximumNumberOfWatchesWouldBeExceeded,

					EBADF => panic!("`fd` is not a valid file descriptor"),
					EFAULT => panic!("`pathname` points outside of the process's accessible address space"),
					EINVAL => panic!("The given event `mask` contains no valid events; or `fd` is not an inotify file descriptor"),

					_ => unreachable!(),
				}
			)
		}
		else
		{
			unreachable!()
		}
	}

	/// Reads an inotify event.
	///
	/// Only one-at-a-time can be (straightforwardly) read as the underlying structure is variable in size.
	///
	/// Use this only after a read-ready event notification is received (using edge-triggered events).
	#[inline(always)]
	pub fn read(&self) -> Result<inotify_event, StructReadError>
	{
		use self::StructReadError::*;

		let mut value: inotify_event = inotify_event::unpopulated();

		const SizeOfRead: usize = size_of::<inotify_event>();

		let result = unsafe { read(self.0, &mut value as *mut _ as *mut _, SizeOfRead) };

		if likely!(result == SizeOfRead as isize)
		{
			Ok(value)
		}
		else
		{
			match result
			{
				-1 =>
				{
					let error_number = errno();
					match error_number.0
					{
						EAGAIN => Err(WouldBlock),
						ECANCELED => Err(Cancelled),
						EINTR => Err(Interrupted),
						EIO => Err(Cancelled),
						EBADF => panic!("`fd` is not a valid file descriptor or is not open for reading"),
						EFAULT => panic!("`buf` is outside your accessible address space"),
						EINVAL => panic!("`fd` is attached to an object which is unsuitable for reading OR was created via a call to `timerfd_create()` and the wrong size buffer was given to `read()`"),
						EISDIR => panic!("`fd` refers to a directory"),

						_ => panic!("Unexpected error `{}`", error_number),
					}
				}

				0 => panic!("End of file but we haven't closed the file descriptor"),

				_ => unreachable!(),
			}
		}
	}
}
