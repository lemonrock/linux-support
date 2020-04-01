// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents the contents of the file `/proc/<N>/stat`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProcStat
{
	/// Process identifier.
	pub process_identifier: ProcessIdentifier,

	/// Executable filename without parentheses.
	pub executable_filename: Box<[u8]>,

	/// State.
	pub state: ProcessState,

	/// Parent process identifier.
	///
	/// `None` if this process is the init process.
	pub parent_process_identifier: Option<ProcessIdentifier>,

	/// Process group identifier.
	pub process_group_identifier: ProcessIdentifier,

	/// Session identifier.
	pub session_identifier: ProcessIdentifier,

	/// Controlling terminal.
	pub controlling_terminal: ControllingTerminal,

	/// Controlling terminal foreground process group.
	pub controlling_terminal_foreground_process_group: ProcessIdentifier,

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
	/// This includes guest time, `guest_time` (time spent running a virtual CPU), so that applications that are not aware of the guest time field do not lose that time from their calculations.
	pub user_mode_time: ClockTicks,

	/// Amount of time that the process has been scheduled in kernel mode.
	pub kernel_mode_time: ClockTicks,

	/// Amount of time that the process's waited-for-children have been scheduled in user mode.
	///
	/// This includes guest time, `cguest_time` (time spent running a virtual CPU), so that applications that are not aware of the guest time field do not lose that time from their calculations.
	pub user_mode_time_for_waited_for_children: ClockTicks,

	/// Amount of time that the process' waited-for-children have been scheduled in kernel mode.
	pub kernel_mode_time_for_waited_for_children: ClockTicks,

	#[allow(missing_docs)]
	pub priority: i8,

	/// Process Niceness.
	pub nice: Nice,

	/// Number of threads.
	pub num_threads: NonZeroUsize,

	/// Relative time the process started after boot.
	pub start_time: ClockTicks,
}

impl ProcStat
{
	/// Status information from `/proc/self/stat`.
	///
	/// Assumes at least Linux 3.5 is in use.
	#[inline(always)]
	pub fn self_stat(proc_path: &ProcPath) -> Result<Self, ProcStatParseError>
	{
		Self::process_stat(proc_path, ProcessIdentifierChoice::Current)
	}

	/// Status information from `/proc/<IDENTIFIER>/status` where `<IDENTIFIER>` is `identifier`.
	///
	/// Assumes at least Linux 3.5 is in use.
	#[inline(always)]
	pub fn process_stat(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> Result<Self, ProcStatParseError>
	{
		const LengthOfOpenParenthesis: usize = 1;
		const AtLeastOneByteOfValue: usize = 1;
		const LengthOfCloseParenthesis: usize = 1;
		const LengthOfSpace: usize = 1;

		use self::ProcStatParseError::*;

		// TODO: Won't work for last field.
		#[inline(always)]
		fn value_bytes<'a>(start_index: &mut usize, bytes: &'a [u8]) -> Result<&'a [u8], ProcStatParseError>
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
			Err(NotEnoughFields)
		}

		#[inline(always)]
		fn executable_name_bytes<'a>(start_index: &mut usize, bytes: &'a [u8]) -> Result<&'a [u8], ProcStatParseError>
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
		fn from_bytes<F: FromBytes>(start_index: &mut usize, bytes: &[u8]) -> Result<F, ProcStatParseError>
		where F::Error: Into<ProcStatParseError>
		{
			let next_bytes = value_bytes(start_index, bytes)?;
			F::from_bytes(next_bytes).map_err(|error| error.into())
		}

		let bytes = proc_path.process_file_path(process_identifier, "stat").read_raw_without_line_feed()?;
		let bytes = &bytes[..];

		let mut start_index = 0;
		let this = Self
		{
			process_identifier: from_bytes(&mut start_index, bytes)?,
			executable_filename:
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

				let name_bytes = executable_name_bytes(&mut start_index, bytes)?;
				name_bytes.to_vec().into_boxed_slice()
			},
			state: from_bytes(&mut start_index, bytes)?,
			parent_process_identifier: from_bytes(&mut start_index, bytes)?,
			process_group_identifier: from_bytes(&mut start_index, bytes)?,
			session_identifier: from_bytes(&mut start_index, bytes)?,
			controlling_terminal: from_bytes(&mut start_index, bytes)?,
			controlling_terminal_foreground_process_group: from_bytes(&mut start_index, bytes)?,
			kernel_flags_word: from_bytes(&mut start_index, bytes)?,
			number_of_minor_page_faults: from_bytes(&mut start_index, bytes)?,
			number_of_minor_page_faults_for_waited_for_children: from_bytes(&mut start_index, bytes)?,
			number_of_major_page_faults: from_bytes(&mut start_index, bytes)?,
			number_of_major_page_faults_for_waited_for_children: from_bytes(&mut start_index, bytes)?,
			user_mode_time: from_bytes(&mut start_index, bytes)?,
			kernel_mode_time: from_bytes(&mut start_index, bytes)?,
			user_mode_time_for_waited_for_children: from_bytes(&mut start_index, bytes)?,
			kernel_mode_time_for_waited_for_children: from_bytes(&mut start_index, bytes)?,
			priority: from_bytes(&mut start_index, bytes)?,
			nice: from_bytes(&mut start_index, bytes)?,
			num_threads: from_bytes(&mut start_index, bytes)?,
			start_time:
			{
				let itrealvalue: isize = from_bytes(&mut start_index, bytes)?;
				debug_assert_eq!(itrealvalue, 0, "itrealvalue has been zero since Linux 2.6.17");

				from_bytes(&mut start_index, bytes)?
			},
		};

		/*(14)

              (18) priority  %ld
                        (Explanation for Linux 2.6) For processes running a
                        real-time scheduling policy (policy below; see
                        sched_setscheduler(2)), this is the negated schedul‐
                        ing priority, minus one; that is, a number in the
                        range -2 to -100, corresponding to real-time priori‐
                        ties 1 to 99.  For processes running under a non-
                        real-time scheduling policy, this is the raw nice
                        value (setpriority(2)) as represented in the kernel.
                        The kernel stores nice values as numbers in the
                        range 0 (high) to 39 (low), corresponding to the
                        user-visible nice range of -20 to 19.

              (19) nice  %ld
                        The nice value (see setpriority(2)), a value in the
                        range 19 (low priority) to -20 (high priority).

              (20) num_threads  %ld
                        Number of threads in this process (since Linux 2.6).

              (21) itrealvalue  %ld
                        The time in jiffies before the next SIGALRM is sent
                        to the process due to an interval timer.  Since ker‐
                        nel 2.6.17, this field is no longer maintained, and
                        is hard coded as 0.

              (22) starttime  %llu
                        The time the process started after system boot.  In
                        kernels before Linux 2.6, this value was expressed
                        in jiffies.  Since Linux 2.6, the value is expressed
                        in clock ticks (divide by sysconf(_SC_CLK_TCK)).

                        The format for this field was %lu before Linux 2.6.

              (23) vsize  %lu
                        Virtual memory size in bytes.

              (24) rss  %ld
                        Resident Set Size: number of pages the process has
                        in real memory.  This is just the pages which count
                        toward text, data, or stack space.  This does not
                        include pages which have not been demand-loaded in,
                        or which are swapped out.

              (25) rsslim  %lu
                        Current soft limit in bytes on the rss of the
                        process; see the description of RLIMIT_RSS in
                        getrlimit(2).

              (26) startcode  %lu  [PT]
                        The address above which program text can run.

              (27) endcode  %lu  [PT]
                        The address below which program text can run.

              (28) startstack  %lu  [PT]
                        The address of the start (i.e., bottom) of the
                        stack.

              (29) kstkesp  %lu  [PT]
                        The current value of ESP (stack pointer), as found
                        in the kernel stack page for the process.

              (30) kstkeip  %lu  [PT]
                        The current EIP (instruction pointer).

              (31) signal  %lu
                        The bitmap of pending signals, displayed as a deci‐
                        mal number.  Obsolete, because it does not provide
                        information on real-time signals; use
                        /proc/[pid]/status instead.

              (32) blocked  %lu
                        The bitmap of blocked signals, displayed as a deci‐
                        mal number.  Obsolete, because it does not provide
                        information on real-time signals; use
                        /proc/[pid]/status instead.

              (33) sigignore  %lu
                        The bitmap of ignored signals, displayed as a deci‐
                        mal number.  Obsolete, because it does not provide
                        information on real-time signals; use
                        /proc/[pid]/status instead.

              (34) sigcatch  %lu
                        The bitmap of caught signals, displayed as a decimal
                        number.  Obsolete, because it does not provide
                        information on real-time signals; use
                        /proc/[pid]/status instead.

              (35) wchan  %lu  [PT]
                        This is the "channel" in which the process is wait‐
                        ing.  It is the address of a location in the kernel
                        where the process is sleeping.  The corresponding
                        symbolic name can be found in /proc/[pid]/wchan.

              (36) nswap  %lu
                        Number of pages swapped (not maintained).

              (37) cnswap  %lu
                        Cumulative nswap for child processes (not main‐
                        tained).

              (38) exit_signal  %d  (since Linux 2.1.22)
                        Signal to be sent to parent when we die.

              (39) processor  %d  (since Linux 2.2.8)
                        CPU number last executed on.

              (40) rt_priority  %u  (since Linux 2.5.19)
                        Real-time scheduling priority, a number in the range
                        1 to 99 for processes scheduled under a real-time
                        policy, or 0, for non-real-time processes (see
                        sched_setscheduler(2)).

              (41) policy  %u  (since Linux 2.5.19)
                        Scheduling policy (see sched_setscheduler(2)).
                        Decode using the SCHED_* constants in linux/sched.h.

                        The format for this field was %lu before Linux
                        2.6.22.

              (42) delayacct_blkio_ticks  %llu  (since Linux 2.6.18)
                        Aggregated block I/O delays, measured in clock ticks
                        (centiseconds).

              (43) guest_time  %lu  (since Linux 2.6.24)
                        Guest time of the process (time spent running a vir‐
                        tual CPU for a guest operating system), measured in
                        clock ticks (divide by sysconf(_SC_CLK_TCK)).

              (44) cguest_time  %ld  (since Linux 2.6.24)
                        Guest time of the process's children, measured in
                        clock ticks (divide by sysconf(_SC_CLK_TCK)).

              (45) start_data  %lu  (since Linux 3.3)  [PT]
                        Address above which program initialized and unini‐
                        tialized (BSS) data are placed.

              (46) end_data  %lu  (since Linux 3.3)  [PT]
                        Address below which program initialized and unini‐
                        tialized (BSS) data are placed.

              (47) start_brk  %lu  (since Linux 3.3)  [PT]
                        Address above which program heap can be expanded
                        with brk(2).

              (48) arg_start  %lu  (since Linux 3.5)  [PT]
                        Address above which program command-line arguments
                        (argv) are placed.

              (49) arg_end  %lu  (since Linux 3.5)  [PT]
                        Address below program command-line arguments (argv)
                        are placed.

              (50) env_start  %lu  (since Linux 3.5)  [PT]
                        Address above which program environment is placed.

              (51) env_end  %lu  (since Linux 3.5)  [PT]
                        Address below which program environment is placed.

              (52) exit_code  %d  (since Linux 3.5)  [PT]
                        The thread's exit status in the form reported by
                        waitpid(2).
		*/

		if cfg!(debug_assertions)
		{
			if this.parent_process_identifier.is_none()
			{
				debug_assert!(!this.process_identifier.should_have_parent())
			}
			else
			{
				debug_assert!(this.process_identifier.should_have_parent())
			}
		}

		Ok(this)
	}
}
