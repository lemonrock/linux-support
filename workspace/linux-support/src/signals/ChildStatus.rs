// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Child status, as might be set by `waitpid()` or returned in a `siginfo_t`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChildStatus
{
	/// The child process was resumed by delivery of `SIGCONT`.
	///
	/// Since Linux 2.6.10.
	Continued,

	/// Exited.
	Exited
	{
		/// Exit code.
		///
		/// `0` is success conventionally.
		exit_code: u8,
	},

	/// This is only possible if the call was done using `WUNTRACED` or when the child is being traced with `ptrace`.
	Stopped
	{
		/// Signal.
		signal: Signal,

		/// One of:-
		///
		/// * `PTRACE_EVENT_FORK`.
		/// * `PTRACE_EVENT_VFORK`.
		/// * `PTRACE_EVENT_CLONE`.
		/// * `PTRACE_EVENT_EXEC`.
		/// * `PTRACE_EVENT_VFORK_DONE`.
		/// * `PTRACE_EVENT_EXIT`.
		/// * `PTRACE_EVENT_SECCOMP`.
		/// * `PTRACE_EVENT_STOP`.
		ptrace_event_if_sigtrap: Option<u8>,
	},

	/// Terminated by a signal.
	Signalled
	{
		/// Signal.
		signal: Signal,

		/// Did a core dump occur?
		core_dump: bool,
	},
}

impl ParseNumber for ChildStatus
{
	#[inline(always)]
	fn parse_number(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Self, ParseNumberError>
	{
		let value = i32::parse_number(bytes, radix, parse_byte)?;
		Self::parse(value).map_err(|_| ParseNumberError::OutOfRange)
	}
}

impl ParseNumberOption for ChildStatus
{
	#[inline(always)]
	fn parse_number_option(bytes: &[u8], radix: Radix, parse_byte: impl Fn(Radix, u8) -> Result<u8, ParseNumberError>) -> Result<Option<Self>, ParseNumberError>
	{
		let value = i32::parse_number(bytes, radix, parse_byte)?;
		if unlikely!(value == 0)
		{
			Ok(None)
		}
		else
		{
			Ok(Some(ChildStatus::parse(value).map_err(|_| ParseNumberError::OutOfRange)?))
		}
	}
}

impl ChildStatus
{
	// Constants are named for values in libc's `sys/wait.h`.
	pub(crate) fn parse(status: i32) -> Result<Self, OutOfRangeSignalNumberError>
	{
		use self::ChildStatus::*;

		let WIFCONTINUED = status == 0xFFFF;
		if WIFCONTINUED
		{
			return Ok(Continued)
		}

		// true: 0x7F (0x00 ..= 0xFF).
		let WIFSTOPPED = (status & 0xFF) == 0x7F;
		if WIFSTOPPED
		{
			let WSTOPSIG = (status >> 8) & 0xFF;

			let raw_signal_number = Signal::parse_raw_signal_number_u8(WSTOPSIG as u8)?;
			let ptrace_event_if_sigtrap = if raw_signal_number == Signal::SIGTRAP
			{
				Some(((((status >> 8) as u32) & !(Signal::SIGTRAP_ as u32)) >> 8) as u8)
			}
			else
			{
				None
			};

			return Ok
			(
				Stopped
				{
					signal: raw_signal_number,
					ptrace_event_if_sigtrap,
				}
			)
		}

		// true: 0x00 (0x00 ..= 0xFF).
		let WIFEXITED = (status & 0x7F) == 0;
		if WIFEXITED
		{
			let WEXITSTATUS = (status >> 8) & 0xFF;

			return Ok
			(
				Exited
				{
					exit_code: WEXITSTATUS as u8
				}
			)
		}

		// false: 0x00 (0x00 ..= 0xFF).
		// false: 0x7F (0x00 ..= 0xFF).
		// false: 0x80 (0x00 ..= 0xFF).
		let WIFSIGNALED = ((status & 0x7F) + 1) as i8 >= 2;
		if WIFSIGNALED
		{
			let WTERMSIG = status & 0x7F;
			let WCOREDUMP = (status & 0x80) != 0;

			let raw_signal_number_u7 = WTERMSIG as u8;
			debug_assert!(raw_signal_number_u7 != 0, "Already tested for zero with `WIFEXITED`");

			return
			Ok
			(
				Signalled
				{
					signal: Signal::parse_raw_signal_number_non_zero_u7(unsafe { NonZeroU8::new_unchecked(raw_signal_number_u7) })?,
					core_dump: WCOREDUMP,
				}
			)
		}

		unreachable!("Should not be a possible state")
	}
}
