// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents the contents of the file `/proc/<N>/stat`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Stat
{
	/// Process identifier.
	pub process_identifier: ProcessIdentifier,

	/// Command name (without parentheses).
	pub command_name: CommandName,

	/// State.
	pub state: ProcessState,

	/// Parent process identifier.
	///
	/// `None` if this process is the init process.
	pub parent_process_identifier: Option<ProcessIdentifier>,

	/// Process group identifier.
	pub process_group_identifier: ProcessIdentifier,

	/// Session identifier.
	pub session_identifier: ProcessGroupIdentifier,

	/// Controlling terminal.
	pub controlling_terminal: CharacterDevice,

	/// Controlling terminal foreground process group.
	///
	/// `None` for `init` process.
	pub controlling_terminal_foreground_process_group: Option<ProcessGroupIdentifier>,

	/// The kernel flags word of the process.
	///
	/// For bit meanings, see the `PF_`* defines in the Linux kernel source file `include/linux/sched.h`.
	pub kernel_flags_word: u32,

	/// The number of faults the process has made which have not required loading a memory page from disk.
	pub number_of_minor_page_faults: usize,

	/// The number of faults the process's waited-for-children have made which have not required loading a memory page from disk.
	pub number_of_minor_page_faults_for_waited_for_children: usize,

	/// The number of major faults the process has made which have required loading a memory page from disk.
	pub number_of_major_page_faults: usize,

	/// The number of major faults the process's waited-for-children have made which have required loading a memory page from disk.
	pub number_of_major_page_faults_for_waited_for_children: usize,

	/// Amount of time that the process has been scheduled in user mode.
	///
	/// This includes guest time, `guest_time` (time spent running a virtual CPU), so that applications that are not aware of the `guest_time` field do not lose that time from their calculations.
	pub user_mode_time: ClockTicks,

	/// Amount of time that the process has been scheduled in kernel mode.
	pub kernel_mode_time: ClockTicks,

	/// Guest time of the process (time spent running a virtual CPU for a guest operating system).
	pub guest_time: ClockTicks,

	/// Amount of time that the process's waited-for-children have been scheduled in user mode.
	///
	/// This includes guest time, `guest_time_for_waited_for_children` (time spent running a virtual CPU), so that applications that are not aware of the `guest_time_for_waited_for_children` field do not lose that time from their calculations.
	pub user_mode_time_for_waited_for_children: ClockTicks,

	/// Amount of time that the process' waited-for-children have been scheduled in kernel mode.
	pub kernel_mode_time_for_waited_for_children: ClockTicks,

	/// Amount of time that the process's waited-for-children have been scheduled in user mode. (time spent running a virtual CPU for a guest operating system).
	pub guest_time_for_waited_for_children: ClockTicks,

	/// Aggregated block I/O delays.
	pub aggregated_block_io_delays: ClockTicks,

	#[allow(missing_docs)]
	pub priority: i8,

	/// Process Nice.
	pub nice: Nice,

	/// Number of threads.
	pub num_threads: NonZeroUsize,

	/// Relative time the process started after boot.
	pub start_time: ClockTicks,

	/// Virtual memory size in bytes.
	pub virtual_memory_size_in_bytes: u64,

	/// Resident Set Size (RSS) in pages.
	///
	/// Number of pages the process has in real memory.
	/// This is just the pages which count toward text, data, or stack space.
	/// This does not include pages which have not been demand-loaded in, or which are swapped out.
	pub resident_set_size: NumberOfPages,

	/// Resident Set Size (RSS) soft limit in bytes.
	///
	/// See the description of `RLIMIT_RSS` in `man 2 getrlimit()`.
	pub resident_set_size_soft_limit_in_bytes: u64,

	/// The address above which program text can run.
	///
	/// If ptrace checks prevent accessing this value, then it is `None`.
	pub start_code: Option<VirtualAddress>,

	/// The address below which program text can run.
	///
	/// If ptrace checks prevent accessing this value, then it is `None`.
	pub end_code: Option<VirtualAddress>,

	/// The address of the start (bottom) of the stack.
	///
	/// If ptrace checks prevent accessing this value, then it is `None`.
	pub start_stack: Option<VirtualAddress>,

	/// The current value of the stack pointer (`ESP`) as found in the kernel stack page.
	///
	/// If ptrace checks prevent accessing this value, then it is `None`.
	pub stack_pointer: Option<VirtualAddress>,

	/// The current value of the instruction pointer (`EIP`).
	///
	/// If ptrace checks prevent accessing this value, then it is `None`.
	pub instruction_pointer: Option<VirtualAddress>,

	/// Pending signals.
	///
	/// Obsolete as does not include real-time signals.
	#[deprecated]
	pub pending_non_real_time_signals: BitSet<Signal>,

	/// Blocked signals.
	///
	/// Obsolete as does not include real-time signals.
	#[deprecated]
	pub blocked_non_real_time_signals: BitSet<Signal>,

	/// Ignored signals.
	///
	/// Obsolete as does not include real-time signals.
	#[deprecated]
	pub ignored_non_real_time_signals: BitSet<Signal>,

	/// Caught signals.
	///
	/// Obsolete as does not include real-time signals.
	#[deprecated]
	pub caught_non_real_time_signals: BitSet<Signal>,

	/// Address of a location in the kernel where a proces is sleeping.
	///
	/// Symbolic name is in `/proc/<pid>/wchan`.
	///
	/// If ptrace checks prevent accessing this value, or no process is waiting, then it is `None`.
	pub wait_channel: Option<VirtualAddress>,

	/// Not maintained by Linux kernel.
	#[deprecated]
	pub swap_pages: NumberOfPages,

	/// Not maintained by Linux kernel.
	#[deprecated]
	pub cumulative_swap_pages: NumberOfPages,

	/// As specified when using the `clone()` system call.
	pub signal_sent_to_parent_when_this_child_process_exits: Signal,

	/// Last hyper thread this process executed on.
	pub last_hyper_thread_process_executed_on: HyperThread,

	/// None if this process is not using the real time scheduler.
	pub real_time_priority: Option<RealTimePriority>,

	/// Decode using the `SCHED_*` constants in `linux/sched.h`.
	pub scheduling_policy: u32,

	/// Address above which program initialized and uninitialized (BSS) data are placed.
	///
	/// If ptrace checks prevent accessing this value, then it is `None`.
	pub start_data: Option<VirtualAddress>,

	/// Address below which program initialized and uninitialized (BSS) data are placed.
	///
	/// If ptrace checks prevent accessing this value, then it is `None`.
	pub end_data: Option<VirtualAddress>,

	/// Address above which program heap can be expanded with `brk()`.
	///
	/// If ptrace checks prevent accessing this value, then it is `None`.
	pub start_brk: Option<VirtualAddress>,

	/// Address above which program command-line arguments are placed.
	///
	/// If ptrace checks prevent accessing this value, then it is `None`.
	pub start_command_line_arguments: Option<VirtualAddress>,

	/// Address below which program command-line arguments are placed.
	///
	/// If ptrace checks prevent accessing this value, then it is `None`.
	pub end_command_line_arguments: Option<VirtualAddress>,

	/// Address above which the program's environment variables are placed.
	///
	/// If ptrace checks prevent accessing this value, then it is `None`.
	pub start_environment_variables: Option<VirtualAddress>,

	/// Address below which the program's environment variables are placed.
	///
	/// If ptrace checks prevent accessing this value, then it is `None`.
	pub end_environment_variables: Option<VirtualAddress>,

	/// Value of `status` as might be returned from a thread exit with `waitpid()`.
	///
	/// If ptrace checks prevent accessing this value, then it is `None`.
	pub wait_status: Option<ChildStatus>,
}

impl Stat
{
	/// Status information from `/proc/self/stat`.
	///
	/// Assumes at least Linux 3.5 is in use.
	#[inline(always)]
	pub fn self_stat(proc_path: &ProcPath) -> Result<Self, StatParseError>
	{
		Self::process_stat(proc_path, ProcessIdentifierChoice::Current)
	}

	/// Status information from `/proc/<IDENTIFIER>/status` where `<IDENTIFIER>` is `identifier`.
	///
	/// Assumes at least Linux 3.5 is in use.
	#[inline(always)]
	pub fn process_stat(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> Result<Self, StatParseError>
	{
		const LengthOfOpenParenthesis: usize = 1;
		const AtLeastOneByteOfValue: usize = 1;
		const LengthOfCloseParenthesis: usize = 1;
		const LengthOfSpace: usize = 1;

		use self::StatParseError::*;

		// NOTE: Won't work for last field or command name (`comm`) field.
		#[inline(always)]
		fn value_bytes<'a>(start_index: &mut usize, bytes: &'a [u8], index_u8: u8, name: &'static str) -> Result<&'a [u8], StatParseError>
		{
			let value_starts_at_index = *start_index;
			let mut index = value_starts_at_index + AtLeastOneByteOfValue;
			while index < bytes.len()
			{
				if bytes[index] == b' '
				{
					let value_ends_at_index = index;
					*start_index = index + LengthOfSpace;
					return Ok(&bytes[value_starts_at_index .. value_ends_at_index])
				}
				index += 1;
			}
			Err(NotEnoughFields { index: unsafe { NonZeroU8::new_unchecked(index_u8) }, name })
		}

		#[inline(always)]
		fn last_field_value_bytes<'a>(start_index: usize, bytes: &'a [u8], index: u8, name: &'static str) -> Result<&'a [u8], StatParseError>
		{
			let slice = &bytes[start_index .. ];
			if unlikely!(slice.is_empty())
			{
				Err(NotEnoughFields { index: unsafe { NonZeroU8::new_unchecked(index) }, name })
			}
			else
			{
				Ok(slice)
			}
		}

		#[inline(always)]
		fn command_name_bytes<'a>(start_index: &mut usize, bytes: &'a [u8]) -> Result<&'a [u8], StatParseError>
		{
			let name_starts_at_index = *start_index;
			let mut index = name_starts_at_index + AtLeastOneByteOfValue;
			const TerminatingSequenceLength: usize = LengthOfCloseParenthesis + LengthOfSpace;

			while index < (bytes.len() - TerminatingSequenceLength)
			{
				if &bytes[index .. index + TerminatingSequenceLength] == b") "
				{
					let name_ends_at_index = index;
					*start_index = name_ends_at_index + TerminatingSequenceLength;
					return Ok(&bytes[name_starts_at_index .. name_ends_at_index])
				}
				index += 1;
			}
			Err(ExecutableNameIsUnterminated)
		}

		#[inline(always)]
		fn from_bytes<F: FromBytes<Error=ParseNumberError>>(start_index: &mut usize, bytes: &[u8], index: u8, name: &'static str) -> Result<F, StatParseError>
		{
			let next_bytes = value_bytes(start_index, bytes, index, name)?;
			F::from_bytes(next_bytes).map_err(|cause| ParseNumber { index: unsafe { NonZeroU8::new_unchecked(index)}, name, cause})
		}

		#[inline(always)]
		fn last_field_from_bytes<F: FromBytes<Error=ParseNumberError>>(start_index: usize, bytes: &[u8], index: u8, name: &'static str) -> Result<F, StatParseError>
		{
			let next_bytes = last_field_value_bytes(start_index, bytes, index, name)?;
			F::from_bytes(next_bytes).map_err(|cause| ParseNumber { index: unsafe { NonZeroU8::new_unchecked(index)}, name, cause})
		}

		#[inline(always)]
		fn controlling_terminal_from_bytes(start_index: &mut usize, bytes: &[u8], index: u8, name: &'static str) -> Result<CharacterDevice, StatParseError>
		{
			let next_bytes = value_bytes(start_index, bytes, index, name)?;
			let dev = u32::from_bytes(next_bytes).map_err(|cause| ParseNumber { index: unsafe { NonZeroU8::new_unchecked(index)}, name, cause})?;
			Ok(CharacterDevice::from(dev))
		}

		#[inline(always)]
		fn obsolete_signal_bit_set_from_bytes(start_index: &mut usize, bytes: &[u8], index: u8, name: &'static str) -> Result<BitSet<Signal>, StatParseError>
		{
			let next_bytes = value_bytes(start_index, bytes, index, name)?;
			let bits = u64::from_bytes(next_bytes).map_err(|cause| ParseNumber { index: unsafe { NonZeroU8::new_unchecked(index)}, name, cause})?;
			Ok(BitSet::new_from_u64(bits))
		}

		let bytes = proc_path.process_file_path(process_identifier, "stat").read_raw_without_line_feed()?;
		let bytes = &bytes[..];

		let mut start_index = 0;

		#[allow(deprecated)]
		let this = Self
		{
			process_identifier: from_bytes(&mut start_index, bytes, 1, "pid")?,
			command_name:
			{
				if unlikely!(bytes.len() < (LengthOfOpenParenthesis + AtLeastOneByteOfValue + LengthOfCloseParenthesis + LengthOfSpace))
				{
					return Err(NotEnoughBytesForExecutabeName)
				}

				if unlikely!(bytes[start_index] != b'(')
				{
					return Err(ExecutableNameDoesNotStartWithOpenParenthesis)
				}
				start_index += 1;
				if unlikely!(bytes[start_index] == b')')
				{
					return Err(ExecutableNameIsEmpty)
				}

				CommandName::from_bytes(command_name_bytes(&mut start_index, bytes)?)?
			},
			state: from_bytes(&mut start_index, bytes, 3, "state")?,
			parent_process_identifier: from_bytes(&mut start_index, bytes, 4, "ppid")?,
			process_group_identifier: from_bytes(&mut start_index, bytes, 5, "pgrp")?,
			session_identifier: from_bytes(&mut start_index, bytes, 6, "session")?,
			controlling_terminal: controlling_terminal_from_bytes(&mut start_index, bytes, 7, "tty_nr")?,
			controlling_terminal_foreground_process_group: from_bytes(&mut start_index, bytes, 8, "tpgid")?,
			kernel_flags_word: from_bytes(&mut start_index, bytes, 9, "flags")?,
			number_of_minor_page_faults: from_bytes(&mut start_index, bytes, 10, "minflt")?,
			number_of_minor_page_faults_for_waited_for_children: from_bytes(&mut start_index, bytes, 11, "cminflt")?,
			number_of_major_page_faults: from_bytes(&mut start_index, bytes, 12, "majflt")?,
			number_of_major_page_faults_for_waited_for_children: from_bytes(&mut start_index, bytes, 13, "cmajflt")?,
			user_mode_time: from_bytes(&mut start_index, bytes, 14, "utime")?,
			kernel_mode_time: from_bytes(&mut start_index, bytes, 15, "stime")?,
			user_mode_time_for_waited_for_children: from_bytes(&mut start_index, bytes, 16, "cutime")?,
			kernel_mode_time_for_waited_for_children: from_bytes(&mut start_index, bytes, 17, "cstime")?,
			priority: from_bytes(&mut start_index, bytes, 18, "prority")?,
			nice: from_bytes(&mut start_index, bytes, 19, "nice")?,
			num_threads: from_bytes(&mut start_index, bytes, 20, "num_threads")?,
			start_time:
			{
				let value = from_bytes(&mut start_index, bytes, 21, "itrealvalue")?;
				if unlikely!(value != 0)
				{
					return Err(IntervalTimerRealValueWasNotZero { value: unsafe { NonZeroU64::new_unchecked(value) } })
				}

				from_bytes(&mut start_index, bytes, 22, "starttime")?
			},
			virtual_memory_size_in_bytes: from_bytes(&mut start_index, bytes, 23, "vsize")?,
			resident_set_size: from_bytes(&mut start_index, bytes, 24, "rss")?,
			resident_set_size_soft_limit_in_bytes: from_bytes(&mut start_index, bytes, 25, "rsslim")?,
			start_code: from_bytes(&mut start_index, bytes, 26, "startcode")?,
			end_code: from_bytes(&mut start_index, bytes, 27, "endcode")?,
			start_stack: from_bytes(&mut start_index, bytes, 28, "startstack")?,
			stack_pointer: from_bytes(&mut start_index, bytes, 29, "kstkesp")?,
			instruction_pointer: from_bytes(&mut start_index, bytes, 30, "kstkeip")?,
			pending_non_real_time_signals: obsolete_signal_bit_set_from_bytes(&mut start_index, bytes, 31, "signal")?,
			blocked_non_real_time_signals: obsolete_signal_bit_set_from_bytes(&mut start_index, bytes, 32, "blocked")?,
			ignored_non_real_time_signals: obsolete_signal_bit_set_from_bytes(&mut start_index, bytes, 33, "sigignore")?,
			caught_non_real_time_signals: obsolete_signal_bit_set_from_bytes(&mut start_index, bytes, 34, "sigcatch")?,
			wait_channel: from_bytes(&mut start_index, bytes, 35, "wchan")?,
			swap_pages: from_bytes(&mut start_index, bytes, 36, "nswap")?,
			cumulative_swap_pages: from_bytes(&mut start_index, bytes, 37, "cnswap")?,
			signal_sent_to_parent_when_this_child_process_exits: from_bytes(&mut start_index, bytes, 38, "exit_signal")?,
			last_hyper_thread_process_executed_on: from_bytes(&mut start_index, bytes, 39, "processor")?,
			real_time_priority: from_bytes(&mut start_index, bytes, 40, "rt_priority")?,
			scheduling_policy: from_bytes(&mut start_index, bytes, 41, "policy")?,
			aggregated_block_io_delays: from_bytes(&mut start_index, bytes, 42, "delayacct_blkio_ticks")?,
			guest_time: from_bytes(&mut start_index, bytes, 43, "guest_time")?,
			guest_time_for_waited_for_children: from_bytes(&mut start_index, bytes, 44, "cguest_time")?,
			start_data: from_bytes(&mut start_index, bytes, 45, "start_data")?,
			end_data: from_bytes(&mut start_index, bytes, 46, "end_data")?,
			start_brk: from_bytes(&mut start_index, bytes, 47, "start_brk")?,
			start_command_line_arguments: from_bytes(&mut start_index, bytes, 48, "arg_start")?,
			end_command_line_arguments: from_bytes(&mut start_index, bytes, 49, "arg_end")?,
			start_environment_variables: from_bytes(&mut start_index, bytes, 50, "env_start")?,
			end_environment_variables: from_bytes(&mut start_index, bytes, 51, "env_end")?,
			wait_status: last_field_from_bytes(start_index, bytes, 52, "exit_code")?,
		};

		if cfg!(debug_assertions)
		{
			if this.parent_process_identifier.is_none()
			{
				debug_assert!(!this.process_identifier.should_have_parent());
				debug_assert!(this.controlling_terminal_foreground_process_group.is_none())
			}
			else
			{
				debug_assert!(this.process_identifier.should_have_parent());
				debug_assert!(this.controlling_terminal_foreground_process_group.is_some())
			}
		}

		Ok(this)
	}
}
