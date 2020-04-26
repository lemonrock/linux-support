// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A process identifier (`pid`).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ProcessIdentifier(NonZeroI32);

impl Default for ProcessIdentifier
{
	#[inline(always)]
	fn default() -> Self
	{
		let pid = unsafe { getpid() };
		debug_assert!(pid > 0);
		Self(unsafe { NonZeroI32::new_unchecked(pid)})
	}
}

impl TryFrom<pid_t> for ProcessIdentifier
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn try_from(value: pid_t) -> Result<Self, Self::Error>
	{
		if likely!(value > 0)
		{
			Ok(Self(unsafe { NonZeroI32::new_unchecked(value)}))
		}
		else
		{
			Err(ParseNumberError::TooSmall)
		}
	}
}

impl From<NonZeroI32> for ProcessIdentifier
{
	#[inline(always)]
	fn from(value: NonZeroI32) -> Self
	{
		Self(value)
	}
}

impl Into<NonZeroI32> for ProcessIdentifier
{
	#[inline(always)]
	fn into(self) -> NonZeroI32
	{
		self.0
	}
}

impl Into<pid_t> for ProcessIdentifier
{
	#[inline(always)]
	fn into(self) -> pid_t
	{
		self.0.get()
	}
}

impl<'a> IntoLineFeedTerminatedByteString<'a> for ProcessIdentifier
{
	#[inline(always)]
	fn into_line_feed_terminated_byte_string(self) -> Cow<'a, [u8]>
	{
		self.0.into_line_feed_terminated_byte_string()
	}
}

impl ParseNumber for ProcessIdentifier
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let pid = pid_t::parse_number(bytes, radix, parse_byte)?;
		if unlikely!(pid < 0)
		{
			Err(ParseNumberError::TooShort)
		}
		else if unlikely!(pid == 0)
		{
			Err(ParseNumberError::WasZero)
		}
		else
		{
			Ok(Self(unsafe { NonZeroI32::new_unchecked(pid) }))
		}
	}
}

impl ParseNumber for Option<ProcessIdentifier>
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let pid = pid_t::parse_number(bytes, radix, parse_byte)?;
		if unlikely!(pid < 0)
		{
			Err(ParseNumberError::TooShort)
		}
		else if pid == 0
		{
			Ok(None)
		}
		else
		{
			Ok(Some(ProcessIdentifier(unsafe { NonZeroI32::new_unchecked(pid) })))
		}
	}
}

impl ProcessIdentifier
{
	/// Init process.
	pub const Init: Self = Self(unsafe { NonZeroI32::new_unchecked(1) });

	/// Should have a parent process?
	#[inline(always)]
	pub fn should_have_parent(self) -> bool
	{
		self != Self::Init
	}

	/// Opens a process identifier file descriptor.
	#[inline(always)]
	pub fn open_file_descriptor(self) -> Result<ProcessIdentifierFileDescriptor, CreationError>
	{
		ProcessIdentifierChoice::Other(self).open_file_descriptor()
	}

	/// Gets the process group identifier (pgid) for this process.
	#[inline(always)]
	pub fn process_group_identifier(self) -> Result<ProcessGroupIdentifier, ()>
	{
		ProcessGroupIdentifier::process_group_identifier(ProcessIdentifierChoice::Other(self))
	}

	/// Gets the session identifier (sid) for this process.
	///
	/// The session identifier of a process is the process group identifier of the session leader.
	#[inline(always)]
	pub fn session_identifer(self) -> Result<ProcessGroupIdentifier, ()>
	{
		ProcessGroupIdentifier::session_identifier(ProcessIdentifierChoice::Other(self))
	}

	/// Gets the thread identifier (tid) for the first thread created in this process.
	///
	/// This thread may no longer exist.
	#[inline(always)]
	pub fn thread_identifier(self) -> ThreadIdentifier
	{
		ThreadIdentifier::from(self)
	}

	/// Process round robin scheduler interval.
	///
	/// This process may not exist.
	#[inline(always)]
	pub fn round_robin_scheduler_interval(self) -> Option<RoundRobinInterval>
	{
		RoundRobinInterval::for_process(ProcessIdentifierChoice::Other(self))
	}

	/// Process name.
	///
	/// This process may not exist.
	#[inline(always)]
	pub fn process_name(self, proc_path: &ProcPath) -> io::Result<ProcessName>
	{
		ProcessName::get_process_name(ProcessIdentifierChoice::Other(self), proc_path)
	}

	/// Set process name.
	///
	/// This process may not exist.
	#[inline(always)]
	pub fn set_process_name(self, process_name: ProcessName, proc_path: &ProcPath) -> io::Result<()>
	{
		process_name.set_process_name(ProcessIdentifierChoice::Other(self), proc_path)
	}

	/// Use this wherever a specific process identifier is read from repeatedly.
	#[inline(always)]
	pub fn to_vectored_read<'a>(self, from_remote: &'a [&'a [u8]]) -> ProcessIdentifierVectoredRead<'a>
	{
		ProcessIdentifierVectoredRead
		{
			process_identifier: self,
			from_remote,
		}
	}

	/// Only ever returns a `CreationError` of `KernelWouldBeOutOfMemory`, `PermissionDenied` or `ProcessForProcessIdentifierDoesNotExist`.
	///
	/// Even if an error is returned, a partial read can have occurred but there is no way of identifying how much was read.
	///
	/// The `Ok` value returned is the number of bytes read in total.
	/// It is a quantum number; it can never include part of a buffer.
	/// There is no requirement for the length and sizes of `to_local` and `from_remote` to match.
	///
	/// It is not clear what the behaviour is if the process if `self` refers to the current process.
	#[inline(always)]
	pub fn vectored_read(self, to_local: &[&mut [u8]], from_remote: &[&[u8]]) -> Result<usize, CreationError>
	{
		let to_local_length = to_local.len();
		let from_remote_length = from_remote.len();

		debug_assert!(to_local_length < MaximumNumberOfBuffers, "to_local length '{}' equals or exceeds MaximumNumberOfBuffers {}", to_local_length, MaximumNumberOfBuffers);
		debug_assert!(from_remote_length < MaximumNumberOfBuffers, "from_remote length '{}' equals or exceeds MaximumNumberOfBuffers {}", from_remote_length, MaximumNumberOfBuffers);

		const FlagsArgumentIsUnused: u64 = 0;

		// NOTE: Relies on iovec having the same layout as a Rust slice.
		let result = unsafe { process_vm_readv(self.0.get(), to_local.as_ptr() as *const iovec, to_local_length as u64, from_remote.as_ptr() as *const iovec, from_remote_length as u64, FlagsArgumentIsUnused) };
		if likely!(result >= 0)
		{
			Ok(result as usize)
		}
		else if likely!(result == -1)
		{
			use self::CreationError::*;

			match errno().0
			{
				ENOMEM => Err(KernelWouldBeOutOfMemory),
				EPERM => Err(PermissionDenied),
				ESRCH => Err(ProcessForProcessIdentifierDoesNotExist),

				EFAULT => panic!("The memory described by local_iov is outside the caller's accessible address space. Or the memory described by remote_iov is outside the accessible address space of the process pid."),
				EINVAL => panic!("The sum of the iov_len values of either local_iov or remote_iov overflows a ssize_t value. Or flags is not 0. Or liovcnt or riovcnt is too large."),

				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!()
		}
	}

	/// Use this wherever a specific process identifier is written to repeatedly.
	#[inline(always)]
	pub fn to_vectored_write<'a>(self, to_remote: &'a [&'a mut [u8]]) -> ProcessIdentifierVectoredWrite<'a>
	{
		ProcessIdentifierVectoredWrite
		{
			process_identifier: self,
			to_remote,
		}
	}

	/// Only ever returns a `CreationError` of `KernelWouldBeOutOfMemory`, `PermissionDenied` or `ProcessForProcessIdentifierDoesNotExist`.
	///
	/// Even if an error is returned, a partial write can have occurred but there is no way of identifying how much was written.
	///
	/// The `Ok` value returned is the number of bytes written in total.
	/// It is a quantum number; it can never include part of a buffer.
	/// There is no requirement for the length and sizes of `from_local` and `to_remote` to match.
	///
	/// It is not clear what the behaviour is if the process if `self` refers to the current process.
	#[inline(always)]
	pub fn vectored_write(self, from_local: &[&[u8]], to_remote: &[&mut [u8]]) -> Result<usize, CreationError>
	{
		let from_local_length = from_local.len();
		let to_remote_length = to_remote.len();

		debug_assert!(from_local_length < MaximumNumberOfBuffers, "to_local length '{}' equals or exceeds MaximumNumberOfBuffers {}", from_local_length, MaximumNumberOfBuffers);
		debug_assert!(to_remote_length < MaximumNumberOfBuffers, "from_remote length '{}' equals or exceeds MaximumNumberOfBuffers {}", to_remote_length, MaximumNumberOfBuffers);

		const FlagsArgumentIsUnused: u64 = 0;

		// NOTE: Relies on iovec having the same layout as a Rust slice.
		let result = unsafe { process_vm_writev(self.0.get(), from_local.as_ptr() as *const iovec, from_local_length as u64, to_remote.as_ptr() as *const iovec, to_remote_length as u64, FlagsArgumentIsUnused) };
		if likely!(result >= 0)
		{
			Ok(result as usize)
		}
		else if likely!(result == -1)
		{
			use self::CreationError::*;

			match errno().0
			{
				ENOMEM => Err(KernelWouldBeOutOfMemory),
				EPERM => Err(PermissionDenied),
				ESRCH => Err(ProcessForProcessIdentifierDoesNotExist),

				EFAULT => panic!("The memory described by local_iov is outside the caller's accessible address space. Or the memory described by remote_iov is outside the accessible address space of the process pid."),
				EINVAL => panic!("The sum of the iov_len values of either local_iov or remote_iov overflows a ssize_t value. Or flags is not 0. Or liovcnt or riovcnt is too large."),

				_ => unreachable!(),
			}
		}
		else
		{
			unreachable!()
		}
	}
}
