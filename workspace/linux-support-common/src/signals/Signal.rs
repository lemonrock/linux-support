// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a Linux signal number.
///
/// One-based.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[derive(EnumIter)]
#[repr(u8)]
pub enum Signal
{
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGHUP = 1,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGINT = 2,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGQUIT = 3,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGILL = 4,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGTRAP = 5,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGABRT = 6,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGBUS = 7,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGFPE = 8,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGKILL = 9,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGUSR1 = 10,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGSEGV = 11,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGUSR2 = 12,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGPIPE = 13,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGALRM = 14,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGTERM = 15,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGSTKFLT = 16,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGCHLD = 17,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGCONT = 18,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGSTOP = 19,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGTSTP = 20,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGTTIN = 21,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGTTOU = 22,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGURG = 23,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGXCPU = 24,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGXFSZ = 25,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGVTALRM = 26,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGPROF = 27,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGWINCH = 28,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGPOLL = 29,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGPWR = 30,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] SIGSYS = 31,

	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] LinuxThreadsRealTimeSignal0 = 32,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] LinuxThreadsRealTimeSignal1 = 33,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] LinuxThreadsRealTimeSignal2 = 34,

	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal35 = 35,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal36 = 36,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal37 = 37,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal38 = 38,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal39 = 39,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal40 = 40,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal41 = 41,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal42 = 42,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal43 = 43,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal44 = 44,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal45 = 45,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal46 = 46,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal47 = 47,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal48 = 48,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal49 = 49,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal50 = 50,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal51 = 51,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal52 = 52,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal53 = 53,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal54 = 54,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal55 = 55,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal56 = 56,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal57 = 57,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal58 = 58,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal59 = 59,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal60 = 60,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal61 = 61,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal62 = 62,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal63 = 63,
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] RealTimeSignal64 = 64,
}

bit_set_aware!(Signal);

impl Into<u16> for Signal
{
	#[inline(always)]
	fn into(self) -> u16
	{
		self as u8 as u16
	}
}

impl BitSetAware for Signal
{
	const LinuxMaximum: u16 = Signal::RealTimeSignal64 as u8 as u16;

	const InclusiveMinimum: Self = unsafe { transmute(0u8) };

	const InclusiveMaximum: Self = unsafe { transmute(Self::LinuxMaximum as u8) };

	const OneBasedCorrection: u16 = 1;

	#[inline(always)]
	fn from_validated_u16(value: u16) -> Self
	{
		debug_assert!(value != 0);
		debug_assert!(value < Self::LinuxMaximum);

		unsafe { transmute(value as u8) }
	}
}

impl Signal
{
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] pub const SIGIOT: Self = Self::SIGABRT;

	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] pub const SIGIO: Self = Self::SIGPOLL;

	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] #[allow(missing_docs)] pub const SIGUNUSED: Self = Self::SIGSYS;

	/// This is an inclusive minimum.
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] pub const SIGRTMIN: Self = Self::RealTimeSignal35;

	/// This is an inclusive maximum.
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "x86_64"))] pub const SIGRTMAX: Self = Self::RealTimeSignal64;

	/// C library name.
	#[inline(always)]
	pub fn c_library_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(strsignal(self.into()) as *const _) }
	}
}
