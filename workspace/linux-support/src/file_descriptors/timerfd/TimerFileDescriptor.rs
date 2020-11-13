// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Represents a timer instance.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimerFileDescriptor(RawFd);

impl Drop for TimerFileDescriptor
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.close()
	}
}

impl AsRawFd for TimerFileDescriptor
{
	#[inline(always)]
	fn as_raw_fd(&self) -> RawFd
	{
		self.0
	}
}

impl IntoRawFd for TimerFileDescriptor
{
	#[inline(always)]
	fn into_raw_fd(self) -> RawFd
	{
		self.0
	}
}

impl FromRawFd for TimerFileDescriptor
{
	#[inline(always)]
	unsafe fn from_raw_fd(fd: RawFd) -> Self
	{
		Self(fd)
	}
}

impl FileDescriptor for TimerFileDescriptor
{
}

impl TimerFileDescriptor
{
	/// Creates a new instance.
	#[inline(always)]
	pub fn new(clock: TimerClock) -> Result<Self, CreationError>
	{
		let result = unsafe { timerfd_create(clock as i32, TFD_NONBLOCK | TFD_CLOEXEC) };
		if likely!(result != -1)
		{
			Ok(TimerFileDescriptor(result))
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
					EINVAL => panic!("Invalid clockid or flags"),
					ENODEV => panic!("Could not mount (internal) anonymous inode device"),
					_ => unreachable_code(format_args!("")),
				}
			)
		}
	}

	/// Reads the time from the timer.
	///
	/// Use this only after a read-ready event notification is received (using edge-triggered events).
	#[inline(always)]
	pub fn read(&self) -> Result<u64, StructReadError>
	{
		use self::StructReadError::*;

		let mut value: u64 = unsafe_uninitialized();

		const SizeOfRead: usize = size_of::<u64>();

		let result = unsafe { libc::read(self.as_raw_fd(), &mut value as *mut _ as *mut _, SizeOfRead) };

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

				_ => unreachable_code(format_args!("")),
			}
		}
	}

	/// Get the value of the timer.
	#[inline(always)]
	pub fn get(&self) -> itimerspec
	{
		let mut current_value = unsafe_uninitialized();
		let result = unsafe { timerfd_gettime(self.0, &mut current_value) };
		if likely!(result == 0)
		{
			return current_value
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF => panic!("`fd` is not a valid file descriptor"),
				EFAULT => panic!("curr_value` is not a valid pointer"),
				EINVAL => panic!("`fd` is not a valid timerfd file descriptor"),
				_ => unreachable_code(format_args!("")),
			}
		}
		else
		{
			unreachable_code(format_args!(""))
		}
	}

	/// Disarms the timer and returns the old value of it.
	#[inline(always)]
	pub fn disarm(&self) -> itimerspec
	{
		static Disarm: itimerspec = itimerspec
		{
			it_interval: timespec
			{
				tv_sec: 0,
				tv_nsec: 0,
			},
			it_value: timespec
			{
				tv_sec: 0,
				tv_nsec: 0,
			},
		};
		self.set(&Disarm, TimerSetChoices::Relative)
	}

	/// Arms the timer to go off once and returns the old value of it.
	#[inline(always)]
	pub fn arm_as_one_off(&self, alarm_goes_off_at: &timespec, interpretation_of_new_value: TimerSetChoices) -> itimerspec
	{
		debug_assert_ne!((alarm_goes_off_at.tv_sec == 0 && alarm_goes_off_at.tv_nsec == 0), true, "alarm_goes_off_at.tv_sec and alarm_goes_off_at.tv_nsec can not both be zero");

		self.set
		(
			&itimerspec
			{
				it_interval: timespec
				{
					tv_sec: 0,
					tv_nsec: 0,
				},
				it_value: timespec
				{
					tv_sec: alarm_goes_off_at.tv_sec,
					tv_nsec: alarm_goes_off_at.tv_nsec,
				},
			},
			interpretation_of_new_value
		)
	}

	/// Arms the timer to go off once and returns the old value of it.
	#[inline(always)]
	pub fn arm_to_go_off_repeatedly(&self, alarm_goes_off_at_repeatedly: &timespec, interpretation_of_new_value: TimerSetChoices) -> itimerspec
	{
		debug_assert_ne!((alarm_goes_off_at_repeatedly.tv_sec == 0 && alarm_goes_off_at_repeatedly.tv_nsec == 0), true, "alarm_goes_off_at_repeatedly.tv_sec and alarm_goes_off_at_repeatedly.tv_nsec can not both be zero");

		self.set
		(
			&itimerspec
			{
				it_interval: timespec
				{
					tv_sec: alarm_goes_off_at_repeatedly.tv_sec,
					tv_nsec: alarm_goes_off_at_repeatedly.tv_nsec,
				},
				it_value: timespec
				{
					tv_sec: alarm_goes_off_at_repeatedly.tv_sec,
					tv_nsec: alarm_goes_off_at_repeatedly.tv_nsec,
				},
			},
			interpretation_of_new_value
		)
	}

	/// Once the number is set, any waiter on the timer is woken up.
	///
	/// The only purpose of this command is to restore the expirations for the purpose of checkpoint-restore.
	///
	/// This operation is available only if the	kernel was built with `CONFIG_CHECKPOINT_RESTORE`.
	///
	/// Since Linux 3.17
	pub fn set_ticks(&self, number_of_expirations: u64) -> io::Result<()>
	{
		let result = unsafe { ioctl(self.0, TFD_IOC_SET_TICKS, &number_of_expirations) };
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
			unreachable_code(format_args!("ioctl() returned unexpected result {}", result))
		}
	}

	/// Arms or disarms the timer.
	///
	/// Set both fields of `new_value.it_value to disarm the timer`.
	///
	/// If both fields of `new_value.it_interval` are zero, the timer expires just once, at the time specified by `new_value.it_value`.
	///
	/// Returns the previous value of the timer.
	#[inline(always)]
	fn set(&self, new_value: &itimerspec, interpretation_of_new_value: TimerSetChoices) -> itimerspec
	{
		debug_assert!(new_value.it_interval.tv_nsec <= 999_999_999, "new_value.it_interval must not exceed 999,999,999");
		debug_assert!(new_value.it_value.tv_nsec <= 999_999_999, "new_value.it_value must not exceed 999,999,999");

		let mut old_value = unsafe_uninitialized();
		let result = unsafe { timerfd_settime(self.0, interpretation_of_new_value as i32, new_value, &mut old_value) };
		if likely!(result == 0)
		{
			return old_value
		}
		else if likely!(result == -1)
		{
			match errno().0
			{
				EBADF => panic!("`fd` is not a valid file descriptor"),
				EFAULT => panic!("`new_value` or `old_value` is not a valid pointer"),
				EINVAL => panic!("arguments were invalid"),
				_ => unreachable_code(format_args!("")),
			}
		}
		else
		{
			unreachable_code(format_args!(""))
		}
	}
}
