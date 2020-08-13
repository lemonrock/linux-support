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
	pub process_group_identifier: Option<ProcessGroupIdentifier>,

	/// Session identifier.
	pub session_identifier: Option<ProcessGroupIdentifier>,

	/// Controlling terminal.
	pub controlling_terminal: Option<CharacterDevice>,

	/// Controlling terminal foreground process group.
	///
	/// `None` for `init` process.
	pub controlling_terminal_foreground_process_group: Option<ProcessGroupIdentifier>,

	/// The kernel flags word of the process.
	///
	/// For bit meanings, see the `PF_`* defines in the Linux kernel source file `include/linux/sched.h`.
	///
	/// Useful for identifying `kthread`s and `kswapd` processes.
	pub kernel_flags: StatProcessFlags,

	/// The number of faults the process has made which have not required loading a memory page from disk.
	///
	/// A zero value usually means the statistic is temporarily unvailable.
	pub number_of_minor_page_faults: usize,

	/// The number of faults the process's waited-for-children have made which have not required loading a memory page from disk.
	///
	/// A zero value usually means the statistic is temporarily unvailable.
	pub number_of_minor_page_faults_for_waited_for_children: usize,

	/// The number of major faults the process has made which have required loading a memory page from disk.
	///
	/// A zero value usually means the statistic is temporarily unvailable.
	pub number_of_major_page_faults: usize,

	/// The number of major faults the process's waited-for-children have made which have required loading a memory page from disk.
	///
	/// A zero value usually means the statistic is temporarily unvailable.
	pub number_of_major_page_faults_for_waited_for_children: usize,

	/// Amount of time that the process has been scheduled in user mode.
	///
	/// This includes guest time, `guest_time` (time spent running a virtual CPU), so that applications that are not aware of the `guest_time` field do not lose that time from their calculations.
	///
	/// A zero value usually means the statistic is temporarily unvailable.
	pub user_mode_time: ClockTicks,

	/// Amount of time that the process has been scheduled in kernel mode.
	///
	/// A zero value usually means the statistic is temporarily unvailable.
	pub kernel_mode_time: ClockTicks,

	/// Guest time of the process (time spent running a virtual CPU for a guest operating system).
	///
	/// A zero value usually means the statistic is temporarily unvailable.
	pub guest_time: ClockTicks,

	/// Amount of time that the process's waited-for-children have been scheduled in user mode.
	///
	/// This includes guest time, `guest_time_for_waited_for_children` (time spent running a virtual CPU), so that applications that are not aware of the `guest_time_for_waited_for_children` field do not lose that time from their calculations.
	///
	/// A zero value usually means the statistic is temporarily unvailable.
	pub user_mode_time_for_waited_for_children: ClockTicks,

	/// Amount of time that the process' waited-for-children have been scheduled in kernel mode.
	///
	/// A zero value usually means the statistic is temporarily unvailable.
	pub kernel_mode_time_for_waited_for_children: ClockTicks,

	/// Amount of time that the process's waited-for-children have been scheduled in user mode. (time spent running a virtual CPU for a guest operating system).
	///
	/// A zero value usually means the statistic is temporarily unvailable.
	pub guest_time_for_waited_for_children: ClockTicks,

	/// Aggregated block I/O delays.
	///
	/// A zero value usually means the statistic is temporarily unvailable.
	pub aggregated_block_io_delays: ClockTicks,

	/// Normal tasks are centered around 0, value goes from -16 to +15.
	///
	/// Real-time (RT) tasks are offset by `-200`.
	pub priority: i32,

	/// Process Nice.
	pub nice: Nice,

	/// Number of threads.
	///
	/// A `None` value usually means the statistic is temporarily unvailable.
	pub num_threads: Option<NonZeroUsize>,

	/// Relative time the process started after boot.
	pub start_time: ClockTicks,

	/// Virtual memory size in bytes.
	///
	/// A zero value means the statistic is unvailable.
	pub virtual_memory_size_in_bytes: u64,

	/// Resident Set Size (RSS) in pages.
	///
	/// Number of pages the process has in real memory.
	/// This is just the pages which count toward text, data, or stack space.
	/// This does not include pages which have not been demand-loaded in, or which are swapped out.
	///
	/// A zero value means the statistic is unvailable.
	pub resident_set_size: NumberOfPages,

	/// Resident Set Size (RSS) soft limit in bytes.
	///
	/// See the description of `RLIMIT_RSS` in `man 2 getrlimit()`.
	///
	/// A zero value usually means the statistic is temporarily unvailable.
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
	pub pending_non_real_time_signals: Signals,

	/// Blocked signals.
	///
	/// Obsolete as does not include real-time signals.
	#[deprecated]
	pub blocked_non_real_time_signals: Signals,

	/// Ignored signals.
	///
	/// Obsolete as does not include real-time signals.
	///
	/// A zero value usually means the statistic is temporarily unvailable.
	#[deprecated]
	pub ignored_non_real_time_signals: Signals,

	/// Caught signals.
	///
	/// Obsolete as does not include real-time signals.
	///
	/// A zero value usually means the statistic is temporarily unvailable.
	#[deprecated]
	pub caught_non_real_time_signals: Signals,

	/// Indicates whether an absolute address is in `/proc/<pid>/wchan`.
	pub wait_channel: bool,

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
	
	/// Status information from `/proc/<IDENTIFIER>/stat` where `<IDENTIFIER>` is `identifier`.
	///
	/// Assumes at least Linux 3.5 is in use.
	#[inline(always)]
	pub fn process_stat(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> Result<Self, StatParseError>
	{
		use self::StatParseError::*;
		
		#[inline(always)]
		fn clock_ticks_unsigned(value: u64) -> Result<ClockTicks, StatParseError>
		{
			Ok(ClockTicks::from(value))
		}
		
		#[inline(always)]
		fn clock_ticks_signed(value: i64) -> Result<ClockTicks, StatParseError>
		{
			if likely!(value >= 0)
			{
				clock_ticks_unsigned(value as u64)
			}
			else
			{
				Err(NegativeValue)
			}
		}
		
		#[inline(always)]
		fn optional_signed_process_identifier(value: i64) -> Result<Option<ProcessIdentifier>, StatParseError>
		{
			if unlikely!(value < 0)
			{
				Err(NegativeValue)
			}
			else if unlikely!(value > (i32::MAX as i64))
			{
				Err(LargeValue)
			}
			else if unlikely!(value == 0)
			{
				Ok(None)
			}
			else
			{
				Ok(Some(ProcessIdentifier::from(unsafe { NonZeroI32::new_unchecked(value as i32) })))
			}
		}
		
		#[inline(always)]
		fn unsigned_process_identifier(value: u64) -> Result<ProcessIdentifier, StatParseError>
		{
			if unlikely!(value == 0)
			{
				Err(ZeroValue)
			}
			else if unlikely!(value > (i32::MAX as u64))
			{
				Err(LargeValue)
			}
			else
			{
				Ok(ProcessIdentifier::from(unsafe { NonZeroI32::new_unchecked(value as i32) }))
			}
		}
		
		#[inline(always)]
		fn optional_process_group_identifier(value: i64) -> Result<Option<ProcessGroupIdentifier>, StatParseError>
		{
			if value == -1
			{
				return Ok(None)
			}
			process_group_identifier(value).map(|process_group_identifier| Some(process_group_identifier))
		}
		
		#[inline(always)]
		fn process_group_identifier(value: i64) -> Result<ProcessGroupIdentifier, StatParseError>
		{
			if unlikely!(value < 1)
			{
				Err(NegativeValue)
			}
			else if unlikely!(value > (i32::MAX as i64))
			{
				Err(LargeValue)
			}
			else
			{
				Ok(ProcessGroupIdentifier::from(unsafe { NonZeroI32::new_unchecked(value as i32) }))
			}
		}
		
		#[inline(always)]
		fn real_time_priority(value: u64) -> Result<Option<RealTimePriority>, StatParseError>
		{
			if value == 0
			{
				Ok(None)
			}
			else if value <= 99
			{
				Ok(Some(unsafe { transmute(value as u8) }))
			}
			else
			{
				Err(LargeValue)
			}
		}
		
		#[inline(always)]
		fn boolean(value: u64) -> Result<bool, StatParseError>
		{
			match value
			{
				0 => Ok(false),
				1 => Ok(true),
				_ => Err(LargeValue)
			}
		}
		
		#[inline(always)]
		fn virtual_address(value: u64) -> Result<Option<VirtualAddress>, StatParseError>
		{
			if value == 0
			{
				Ok(None)
			}
			else
			{
				Ok(Some(VirtualAddress::from(to_usize(value)?)))
			}
		}
		
		#[inline(always)]
		fn to_usize(value: u64) -> Result<usize, StatParseError>
		{
			if likely!(value <= (usize::MAX as u64))
			{
				Ok(value as usize)
			}
			else
			{
				Err(LargeValue)
			}
		}
		
		#[inline(always)]
		fn num_threads(value: i64) -> Result<Option<NonZeroUsize>, StatParseError>
		{
			if unlikely!(value < 0)
			{
				Err(NegativeValue)
			}
			else if likely!((value as u64) <= (usize::MAX as u64))
			{
				Ok(NonZeroUsize::new(value as usize))
			}
			else
			{
				Err(LargeValue)
			}
		}
		
		#[inline(always)]
		fn to_i32(value: i64) -> Result<i32, StatParseError>
		{
			if likely!(value >= (i32::MIN as i64) && value <= (i32::MAX as i64))
			{
				Ok(value as i32)
			}
			else
			{
				Err(LargeValue)
			}
		}
		
		#[inline(always)]
		fn to_u32(value: u64) -> Result<u32, StatParseError>
		{
			if likely!(value <= (u32::MAX as u64))
			{
				Ok(value as u32)
			}
			else
			{
				Err(LargeValue)
			}
		}
		
		#[inline(always)]
		fn nice(value: i64) -> Result<Nice, StatParseError>
		{
			if likely!(value >= -20 && value <= 19)
			{
				Ok(unsafe { transmute(value as i32) })
			}
			else
			{
				Err(LargeValue)
			}
		}
		
		#[inline(always)]
		fn child_status(value: i64) -> Result<Option<ChildStatus>, StatParseError>
		{
			if value == 0
			{
				Ok(None)
			}
			else if likely!(value >= (i32::MIN as i64) && value <= (i32::MAX as i64))
			{
				Ok(Some(ChildStatus::parse(value as i32)?))
			}
			else
			{
				Err(LargeValue)
			}
		}
		
		#[inline(always)]
		fn signal(value: i64) -> Result<Signal, StatParseError>
		{
			 if likely!(value >= 0 && value <= (u8::MAX as i64))
			{
				Ok(Signal::parse_raw_signal_number_u8(value as u8)?)
			}
			else
			{
				Err(LargeValue)
			}
		}
		
		#[inline(always)]
		fn optional_character_device(value: i64) -> Result<Option<CharacterDevice>, StatParseError>
		{
			if value == 0
			{
				Ok(None)
			}
			else
			{
				Ok(Some(CharacterDevice::from(value as dev_t)))
			}
		}
		
		#[inline(always)]
		fn process_state(value: i8) -> Result<ProcessState, StatParseError>
		{
			Ok(ProcessState::try_from(value)?)
		}
		
		#[inline(always)]
		fn kernel_flags(value: u64) -> Result<StatProcessFlags, StatParseError>
		{
			if unlikely!(value > (u32::MAX as u64))
			{
				Err(LargeValue)
			}
			else
			{
				Ok(StatProcessFlags::from_bits_truncate(value as u32))
			}
		}
		
		#[inline(always)]
		fn signals(value: u64) -> Result<Signals, StatParseError>
		{
			Ok(Signals(BitSet::new_from_u64(value)))
		}
		
		#[inline(always)]
		fn hyper_thread(value: i64) -> Result<HyperThread, StatParseError>
		{
			if value > (i16::MAX as i64)
			{
				Err(LargeValue)
			}
			else if value < 0
			{
				Err(NegativeValue)
			}
			else
			{
				Ok(HyperThread::try_from(value as u16)?)
			}
		}
		
		let bytes = proc_path.process_file_path(process_identifier, "stat").read_raw_without_line_feed()?;
		let bytes = &bytes[..];
		let mut fields = StatFieldIterator::new(bytes);
		
		#[allow(deprecated)]
		let this = Self
		{
			process_identifier: fields.decimal_unsigned_long_long_to("process_identifier", unsigned_process_identifier)?,
			command_name: CommandName::from_bytes(fields.next_field("command_name")?)?,
			state: fields.character_to("state", process_state)?,
			parent_process_identifier: fields.decimal_signed_long_long_to("ppid", optional_signed_process_identifier)?,
			process_group_identifier: fields.decimal_signed_long_long_to("pgrp", optional_process_group_identifier)?,
			session_identifier: fields.decimal_signed_long_long_to("session", optional_process_group_identifier)?,
			controlling_terminal: fields.decimal_signed_long_long_to("tty_nr", optional_character_device)?,
			controlling_terminal_foreground_process_group: fields.decimal_signed_long_long_to("tpgid", optional_process_group_identifier)?,
			kernel_flags: fields.decimal_unsigned_long_long_to("flags", kernel_flags)?,
			number_of_minor_page_faults: fields.decimal_unsigned_long_long_to("minflt", to_usize)?,
			number_of_minor_page_faults_for_waited_for_children: fields.decimal_unsigned_long_long_to("cminflt", to_usize)?,
			number_of_major_page_faults: fields.decimal_unsigned_long_long_to("majflt", to_usize)?,
			number_of_major_page_faults_for_waited_for_children: fields.decimal_unsigned_long_long_to("cmajflt", to_usize)?,
			user_mode_time: fields.decimal_unsigned_long_long_to("utime", clock_ticks_unsigned)?,
			kernel_mode_time: fields.decimal_unsigned_long_long_to("stime", clock_ticks_unsigned)?,
			user_mode_time_for_waited_for_children: fields.decimal_signed_long_long_to("cutime", clock_ticks_signed)?,
			kernel_mode_time_for_waited_for_children: fields.decimal_signed_long_long_to("cstime", clock_ticks_signed)?,
			priority: fields.decimal_signed_long_long_to("priority", to_i32)?,
			nice: fields.decimal_signed_long_long_to("nice", nice)?,
			num_threads: fields.decimal_signed_long_long_to("num_threads", num_threads)?,
			start_time:
			{
				fields.zero_decimal_unsigned_long_long("itrealvalue")?;
				fields.decimal_unsigned_long_long_to("starttime", clock_ticks_unsigned)?
			},
			virtual_memory_size_in_bytes: fields.decimal_unsigned_long_long("vsize")?,
			resident_set_size: fields.decimal_unsigned_long_long("rss")?,
			resident_set_size_soft_limit_in_bytes: fields.decimal_unsigned_long_long("rsslim")?,
			start_code: fields.decimal_unsigned_long_long_to("startcode", virtual_address)?,
			end_code: fields.decimal_unsigned_long_long_to("endcode", virtual_address)?,
			start_stack: fields.decimal_unsigned_long_long_to("startstack", virtual_address)?,
			stack_pointer: fields.decimal_unsigned_long_long_to("kstkesp", virtual_address)?,
			instruction_pointer: fields.decimal_unsigned_long_long_to("kstkeip", virtual_address)?,
			pending_non_real_time_signals: fields.decimal_unsigned_long_long_to("signal", signals)?,
			blocked_non_real_time_signals: fields.decimal_unsigned_long_long_to("blocked", signals)?,
			ignored_non_real_time_signals: fields.decimal_unsigned_long_long_to("sigignore", signals)?,
			caught_non_real_time_signals: fields.decimal_unsigned_long_long_to("sigcatch", signals)?,
			wait_channel: fields.decimal_unsigned_long_long_to("wchan", boolean)?,
			signal_sent_to_parent_when_this_child_process_exits:
			{
				fields.zero_decimal_unsigned_long_long("nswap")?;
				fields.zero_decimal_unsigned_long_long("cnswap")?;
				fields.decimal_signed_long_long_to("exit_signal", signal)?
			},
			last_hyper_thread_process_executed_on: fields.decimal_signed_long_long_to("processor", hyper_thread)?,
			real_time_priority: fields.decimal_unsigned_long_long_to("rt_priority", real_time_priority)?,
			scheduling_policy: fields.decimal_unsigned_long_long_to("policy", to_u32)?,
			aggregated_block_io_delays: fields.decimal_unsigned_long_long_to("delayacct_blkio_ticks", clock_ticks_unsigned)?,
			guest_time: fields.decimal_unsigned_long_long_to("guest_time", clock_ticks_unsigned)?,
			guest_time_for_waited_for_children: fields.decimal_signed_long_long_to("cguest_time", clock_ticks_signed)?,
			start_data: fields.decimal_unsigned_long_long_to("start_data", virtual_address)?,
			end_data: fields.decimal_unsigned_long_long_to("end_data", virtual_address)?,
			start_brk: fields.decimal_unsigned_long_long_to("start_brk", virtual_address)?,
			start_command_line_arguments: fields.decimal_unsigned_long_long_to("arg_start", virtual_address)?,
			end_command_line_arguments: fields.decimal_unsigned_long_long_to("arg_end", virtual_address)?,
			start_environment_variables: fields.decimal_unsigned_long_long_to("env_start", virtual_address)?,
			end_environment_variables: fields.decimal_unsigned_long_long_to("env_end", virtual_address)?,
			wait_status: fields.decimal_signed_long_long_to("exit_code", child_status)?,
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
			
			match (this.controlling_terminal, this.controlling_terminal_foreground_process_group)
			{
				(None, None) => (),
				(Some(_), Some(_)) => (),
				_ => panic!("controlling_terminal and controlling_terminal_foreground_process_group must either both be None or both be Some")
			}
		}
		
		Ok(this)
	}
}
