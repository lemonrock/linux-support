// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Derived from `enum ParsedSignal` in the Linux sources.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParsedSignal
{
	/// Linux calls this `SIL_KILL`.
	///
	/// This is *ALWAYS* kernel-generated, so the data is trustworthy, but be aware that it is Linux's default when no other cases match.
	Kill
	{
		/// Signal.
		signal: Signal,

		/// Process identifier.
		process_identifier: ProcessIdentifier,

		/// User identifier.
		user_identifier: UserIdentifier,
	},

	/// Linux calls this `SIL_FAULT`.
	///
	/// Covers the signals `SIGILL`, `SIGFPE`, `SIGSEGV`, `SIGTRAP`, `SIGEMT` (on mips64 and sparc64) and a specific variant of the signal `SIGBUS`.
	///
	/// This is *ALWAYS* kernel-generated, so the data is trustworthy.
	Fault
	{
		/// The address of the fault.
		address: VirtualAddress,

		/// The trap number of the fault, if supported:-
		///
		/// * Set u32 for mips64 and sparc64.
		/// * Set i32 for aarch64 and powerpc64 when `ptrace()` creates a `SIGTRAP` for `aarch64` and `powerpc64` (see `force_sig_ptrace_errno_trap()`).
		#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] trap_number: u32,
		#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64"))] trap_number: i32,

		/// Signal-specific fault code.
		fault_code: FaultCode
	},

	/// Linux calls this `SIL_FAULT_MCEERR`.
	///
	/// Covers a specific variant of the signal `SIGBUS`.
	/// 
	/// This is *ALWAYS* kernel-generated, so the data is trustworthy.
	FaultMachineCheck
	{
		/// The address of the fault.
		address: VirtualAddress,

		/// The trap number of the fault, if supported:-
		///
		/// * Set u32 for mips64 and sparc64.
		/// * Set i32 for aarch64 and powerpc64 when `ptrace()` creates a `SIGTRAP` for `aarch64` and `powerpc64` (see `force_sig_ptrace_errno_trap()`).
		#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] trap_number: u32,
		#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64"))] trap_number: i32,

		/// SIGBUS-specific fault code.
		hardware_error_machine_check_bus_code: HardwareErrorMachineCheckBusCode,

		/// The least significant bit of the reported address; indicates the extent of the corruption.
		///
		/// For example, if a full page was corrupted, this field would be `log2(sysconf(_SC_PAGESIZE))`.
		///
		/// Only populated since Linux 2.6.37, although the codes were added in Linux 2.6.32.
		address_least_significant_bit: u16,
	},

	/// Linux calls this `SIL_CHLD`.
	///
	/// Covers the signal `SIGCHLD` (aka `SIGCLD` on mips64).
	///
	/// This is *ALWAYS* kernel-generated, so the data is trustworthy.
	Child
	{
		/// Process identifier.
		process_identifier: ProcessIdentifier,

		/// User identifier.
		user_identifier: UserIdentifier,

		/// The child's exit status.
		status: ChildStatus,

		/// User CPU time consumed in 'clock ticks'.
		user_cpu_time_consumed_in_clock_ticks: ClockTicks,

		/// System CPU time consumed in 'clock ticks'.
		system_cpu_time_consumed_in_clock_ticks: ClockTicks,

		/// Child code.
		child_code: ChildCode,
	},

	/// Linux calls this `SIL_SYS`.
	///
	/// Covers the signal `SIGSYS` (aka `SIGUNUSED`).
	///
	/// This is *ALWAYS* kernel-generated, so the data is trustworthy.
	SeccompSystemCall
	{
		/// Contains `SECCOMP_RET_DATA` as specified in a Seccomp filter; see the `SECCOMP_RET_TRAP` filter return value.
		seccomp_ret_data: i32,

		/// The system call number.
		system_call_number: UnconstrainedSystemCallNumber,

		/// The address of the fault.
		address: VirtualAddress,

		/// The system call architecture.
		///
		/// None if the underlying was zero or not parsable.
		audit_architecture: Option<AuditArchitecture>,

		/// Not yet very useful; may never be.
		system_call_code: SystemCallCode,
	},

	/// Linux calls this `SIL_POLL`.
	///
	/// Treat `band_event` and `file_descriptor` carefully if `user_generated` is `Some`.
	///
	/// This is one of two default cases if other signalled events do not match; interpret the data carefully.
	///
	/// This *may be* user generated.
	Poll
	{
		/// I am not convinced Linux always validates signal numbers correctly from user space.
		signal: Signal,

		/// Band event.
		band_event: u32,

		/// File descriptor.
		file_descriptor: RawFd,

		/// Poll code.
		poll_code: PollCode,
	},

	/// Linux calls this `SIL_POLL`.
	///
	/// Treat `band_event` and `file_descriptor` carefully if `user_generated` is `Some`.
	///
	/// This is one of two default cases if other signalled events do not match; interpret the data carefully.
	///
	/// This *may be* user generated.
	UserspacePoll
	{
		/// I am not convinced Linux always validates signal numbers correctly from user space.
		signal: Result<Signal, OutOfRangeSignalNumberError>,

		/// Almost always zero.
		error_number: i32,

		/// Band event.
		band_event: u32,

		/// File descriptor.
		file_descriptor: RawFd,
	},

	/// Linux calls this `SIL_TIMER`.
	///
	/// This is *ALWAYS* user-generated, so treat `timer_identifier`, `overrun_count`, `pointer` and `int` carefully.
	UserspaceTimer
	{
		/// I am not convinced Linux always validates signal numbers correctly from user space.
		signal: Result<Signal, OutOfRangeSignalNumberError>,

		/// Almost always zero.
		error_number: i32,

		/// Kernel timer identifier.
		timer_identifier: u32,

		/// POSIX timer overrun count.
		overrun_count: u32,

		/// Arbitrary pointer value.
		pointer: VirtualAddress,

		/// Arbitrary int value.
		int: i32,
	},

	/// Linux calls this `SIL_RT`.
	///
	/// This is *ALWAYS* user-generated, so treat `pointer` and `int` carefully.
	UserspaceRealTime
	{
		/// I am not convinced Linux always validates signal numbers correctly from user space.
		signal: Result<Signal, OutOfRangeSignalNumberError>,
		
		/// Almost always zero and only ever non-zero when a signal has been initiated by userspace I believe.
		error_number: i32,

		/// Process identifier (can be controlled by userspace).
		process_identifier: Result<ProcessIdentifier, ParseNumberError>,

		/// User identifier.
		user_identifier: UserIdentifier,

		/// Arbitrary pointer value.
		pointer: VirtualAddress,

		/// Arbitrary int value.
		int: i32,

		/// An attempt to translate commonly-encountered 'userspace' si_code.
		userspace_signal_code: UserspaceSignalCode,
	},
}
