// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents an epoll instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EPollFileDescriptor(RawFd);

impl Drop for EPollFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for EPollFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for EPollFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for EPollFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for EPollFileDescriptor
{
}

impl EPollFileDescriptor
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new() -> Result<Self, CreationError>
	{
		use self::CreationError::*;

		let result = unsafe { epoll_create1(EPOLL_CLOEXEC) };
		if likely!(result >= 0)
		{
			Ok(EPollFileDescriptor(result))
		}
		else if likely!(result == -1)
		{
			Err
			(
				match SystemCallErrorNumber::from_errno_panic()
				{
					EMFILE => PerProcessLimitOnNumberOfFileDescriptorsWouldBeExceeded,

					ENFILE => SystemWideLimitOnTotalNumberOfFileDescriptorsWouldBeExceeded,

					ENOMEM => KernelWouldBeOutOfMemory,

					EINVAL => panic!("Invalid value specified in flags"),
					
					unexpected_error @ _ => unexpected_error!(epoll_create1, unexpected_error),
				}
			)
		}
		else
		{
			unexpected_result!(epoll_create1, result)
		}
	}

	/// Waits for events.
	///
	/// Fills `events` as much as possible and returns it as a slice.
	///
	/// Returns an error if interrupted.
	///
	/// No error occurs if a time out occurred.
	#[inline(always)]
	pub fn wait<'a>(&self, events: &'a mut [epoll_event], time_out: EPollTimeOut) -> Result<&'a [epoll_event], EPollWaitError>
	{
		let length = events.len();

		debug_assert_ne!(length, 0, "events.len() can not be zero");
		debug_assert!(length <= i32::MAX as usize, "events.len() can not exceed i32::MAX");

		let result = unsafe { epoll_wait(self.0, events.as_mut_ptr(), length as i32, time_out.into()) };
		if likely!(result >= 0)
		{
			Ok(&events[0 .. result as usize])
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno_panic()
			{
				EINTR => Err(EPollWaitError::Interrupted),

				EBADF => panic!("`epfd` is not a valid file descriptor"),
				EFAULT => panic!("Memory for events was not writable"),
				EINVAL => panic!("`epfd` is not an epoll file descriptor"),
				
				unexpected_error @ _ => unexpected_error!(epoll_wait, unexpected_error),
			}
		}
		else
		{
			unexpected_result!(epoll_wait, result)
		}
	}

	/// Similar to `wait()` but atomically changes the signal mask to `signal_mask` for the duration of the call.
	#[inline(always)]
	pub fn wait_signalled<'a>(&self, events: &'a mut [epoll_event], time_out: EPollTimeOut, signal_mask: &sigset_t) -> Result<&'a [epoll_event], EPollWaitError>
	{
		let length = events.len();

		debug_assert_ne!(length, 0, "events.len() can not be zero");
		debug_assert!(length <= i32::MAX as usize, "events.len() can not exceed i32::MAX");

		let result = unsafe { epoll_pwait(self.0, events.as_mut_ptr(), length as i32, time_out.into(), signal_mask) };
		if likely!(result >= 0)
		{
			Ok(&events[0 .. result as usize])
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno_panic()
			{
				EINTR => Err(EPollWaitError::Interrupted),

				EBADF => panic!("`epfd` is not a valid file descriptor"),
				EFAULT => panic!("Memory for events was not writable"),
				EINVAL => panic!("`epfd` is not an epoll file descriptor"),
				
				unexpected_error @ _ => unexpected_error!(epoll_pwait, unexpected_error),
			}
		}
		else
		{
			unexpected_result!(epoll_pwait, result)
		}
	}

	/// Adds a file descriptor to an Event Poll (epoll) instance.
	#[inline(always)]
	pub fn add(&self, fd: RawFd, flags: EPollAddFlags, token: u64) -> Result<(), EPollAddError>
	{
		let mut event = epoll_event
		{
			events: flags.bits,
			data: epoll_data_t
			{
				u64: token,
			},
		};

		use self::EPollAddError::*;

		let result = unsafe { epoll_ctl(self.0, EPOLL_CTL_ADD, fd, &mut event) };

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			Err
			(
				match SystemCallErrorNumber::from_errno_panic()
				{
					ENOMEM => ThereWasInsufficientKernelMemory,

					ENOSPC => LimitOnWatchesWouldBeExceeded,

					EBADF => panic!("The supplied file descriptor was not a valid file descriptor"),
					EEXIST => panic!("The supplied file descriptor was already registered with this epoll instance"),
					EINVAL => panic!("Can not add epoll file descriptor to its self, or can not make wait on an epoll file descriptor `EPOLLEXCLUSIVE`"),
					ELOOP => panic!("The supplied file descriptor is for an epoll instance and this operation would result in a circular loop of epoll instances monitoring one another"),
					EPERM => panic!("The supplied file descriptor does not support epoll (perhaps it is an open regular file or the like)"),
					
					unexpected_error @ _ => unexpected_error!(epoll_ctl, EPOLL_CTL_ADD, unexpected_error),
				}
			)
		}
		else
		{
			unexpected_result!(epoll_ctl, EPOLL_CTL_ADD, result)
		}
	}

	/// Modifies a file descriptor in an Event Poll (epoll) instance.
	#[inline(always)]
	pub fn modify(&self, fd: RawFd, flags: EPollModifyFlags, token: u64) -> Result<(), EPollModifyError>
	{
		let mut event = epoll_event
		{
			events: flags.bits,
			data: epoll_data_t
			{
				u64: token,
			},
		};

		let result = unsafe { epoll_ctl(self.0, EPOLL_CTL_MOD, fd, &mut event) };

		if likely!(result == 0)
		{
			Ok(())
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno_panic()
			{
				ENOMEM => Err(EPollModifyError::ThereWasInsufficientKernelMemory),

				EBADF => panic!("The supplied file descriptor was not a valid file descriptor"),
				EINVAL => panic!("Supplied file descriptor was not usable or there was the presence or absence of `Exclusive` when required"),
				ENOENT => panic!("The supplied file descriptor is not registered with this epoll instance"),
				EPERM => panic!("The supplied file descriptor does not support epoll (perhaps it is an open regular file or the like)"),
				
				unexpected_error @ _ => unexpected_error!(epoll_ctl, EPOLL_CTL_MOD, unexpected_error),
			}
		}
		else
		{
			unexpected_result!(epoll_ctl, EPOLL_CTL_MOD, result)
		}
	}

	/// Deletes a file descriptor in an Event Poll (epoll) instance.
	#[inline(always)]
	pub fn delete(&self, fd: RawFd)
	{
		let result = unsafe { epoll_ctl(self.0, EPOLL_CTL_DEL, fd, null_mut()) };
		if likely!(result == 0)
		{
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno_panic()
			{
				ENOMEM => panic!("Examination of the Linux source code fs/eventpoll.c suggests `ENOMEM` should not occur for `EPOLL_CTL_DEL`"),

				EBADF => panic!("The supplied file descriptor was not a valid file descriptor"),
				EINVAL => panic!("Supplied file descriptor was not usable"),
				ENOENT => panic!("The supplied file descriptor is not registered with this epoll instance"),
				EPERM => panic!("The supplied file descriptor does not support epoll (perhaps it is an open regular file or the like)"),
				
				unexpected_error @ _ => unexpected_error!(epoll_ctl, EPOLL_CTL_DEL, unexpected_error),
			}
		}
		else
		{
			unexpected_result!(epoll_ctl, EPOLL_CTL_DEL, result)
		}
	}

	/// Obtains information for this file descriptor from the `/proc` file system.
	///
	/// This involves many system calls and should not be part of the critical path of an application.
	///
	/// Additionally, most `/proc` files are only atomic for a particular read.
	///
	/// Linux version 3.8 or later is assumed.
	pub fn information(&self) -> io::Result<(FileDescriptorInformationHeader, impl Iterator<Item=EPollInformationItem>)>
	{
		struct InformationIterator
		{
			buffered_reader: Option<BufReader<File>>,
			bytes_read: ArrayVec<u8, MaximumBytesPerLine>,
		}

		impl Iterator for InformationIterator
		{
			type Item = EPollInformationItem;

			#[inline(always)]
			fn next(&mut self) -> Option<Self::Item>
			{
				if self.buffered_reader.is_none()
				{
					return None
				}

				let result =
				{
					let buffered_reader = self.buffered_reader.as_mut().unwrap();
					self.bytes_read.read_until_line_feed(buffered_reader)
				};

				match result
				{
					Err(_) =>
					{
						self.buffered_reader = None;
						None
					}

					Ok(0) =>
					{
						self.buffered_reader = None;
						None
					}

					Ok(_) => match self.parse_line()
					{
						Err(_) =>
						{
							self.buffered_reader = None;
							None
						}

						Ok(epoll_information_item) =>
						{
							self.bytes_read.clear();
							Some(epoll_information_item)
						}
					}
				}
			}
		}

		impl InformationIterator
		{
			fn parse_line(&mut self) -> io::Result<EPollInformationItem>
			{
				let remaining_bytes = &self.bytes_read[..];

				let (target_file_descriptor, remaining_bytes) = extract_fixed_width_value_from_slice(remaining_bytes, b"tfd: ", 8, |string| RawFd::from_str_radix(string.trim_start(), 10))?;
				let (event_flags, remaining_bytes) = extract_fixed_width_value_from_slice(remaining_bytes, b" events: ", 8, |string| u32::from_str_radix(string.trim_start(), 16))?;
				let (token, remaining_bytes) = extract_fixed_width_value_from_slice(remaining_bytes, b" data: ", 16, |string| u64::from_str_radix(string.trim_start(), 16))?;

				// NOTE: The double space is correct.
				let (position, remaining_bytes) = extract_space_terminated_value_from_slice(remaining_bytes, b"  pos:", |string| i64::from_str_radix(string, 10))?;
				let (inode, remaining_bytes) = extract_space_terminated_value_from_slice(remaining_bytes, b"ino:", |string| isize::from_str_radix(string, 16))?;
				let (sdevice, remaining_bytes) = extract_space_terminated_value_from_slice(remaining_bytes, b"sdev:", |string| u32::from_str_radix(string, 16))?;

				if unlikely!(remaining_bytes.len() != 0)
				{
					return Err(invalid_data())
				}

				Ok
				(
					EPollInformationItem
					{
						target_file_descriptor,
						event_flags,
						token,
						position,
						inode,
						sdevice,
					}
				)
			}
		}

		let process_identifier = unsafe { getpid() };

		let mut buffered_reader = BufReader::new(File::open(&format!("/proc/{}/fdinfo/{}", process_identifier as u32, self.0 as u32))?);

		const MaximumBytesPerLine: usize = 256;
		let mut bytes_read: ArrayVec<u8, MaximumBytesPerLine> = ArrayVec::new();

		let header = FileDescriptorInformationHeader
		{
			file_offset: bytes_read.parse_header_line(&mut buffered_reader, b"pos:\t", |string| isize::from_str_radix(string, 10))?,
			file_flags: bytes_read.parse_header_line(&mut buffered_reader, b"flags:\t", |string| u32::from_str_radix(string, 8))?,
			mount_identifier: bytes_read.parse_header_line(&mut buffered_reader, b"mnt_id:\t", |string| usize::from_str_radix(string, 16))?,
		};

		Ok((header, InformationIterator { buffered_reader: Some(buffered_reader), bytes_read }))
	}
}
