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
	#[allow(missing_docs)] SIGABRT = 6,

	#[allow(missing_docs)] SIGALRM = 14,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] SIGBUS = 7,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGBUS = 10,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] SIGCHLD = 17,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGCHLD = 18,
	#[allow(missing_docs)] #[cfg(target_arch = "sparc64")] SIGCHLD = 20,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] SIGCONT = 18,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGCONT = 25,
	#[allow(missing_docs)] #[cfg(target_arch = "sparc64")] SIGCONT = 19,

	#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] SIGEMT = 7,

	#[allow(missing_docs)] SIGFPE = 8,

	#[allow(missing_docs)] SIGHUP = 1,

	#[allow(missing_docs)] SIGILL = 4,

	#[allow(missing_docs)] SIGINT = 2,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] SIGIO = 29,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGIO = 22,
	#[allow(missing_docs)] #[cfg(target_arch = "sparc64")] SIGIO = 23,

	#[allow(missing_docs)] SIGKILL = 9,

	#[allow(missing_docs)] SIGPIPE = 13,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] SIGPROF = 27,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGPROF = 29,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] SIGPWR = 30,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGPWR = 19,
	#[allow(missing_docs)] #[cfg(target_arch = "sparc64")] SIGPWR = 29,

	#[allow(missing_docs)] SIGQUIT = 3,

	#[allow(missing_docs)] SIGSEGV = 11,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] SIGSTKFLT = 16,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGSTKFLT = 7,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] SIGSTOP = 19,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGSTOP = 23,
	#[allow(missing_docs)] #[cfg(target_arch = "sparc64")] SIGSTOP = 17,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] SIGSYS = 31,
	#[allow(missing_docs)] #[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] SIGSYS = 12,

	#[allow(missing_docs)] SIGTERM = 15,

	#[allow(missing_docs)] SIGTRAP = 5,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] SIGTSTP = 20,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGTSTP = 24,
	#[allow(missing_docs)] #[cfg(target_arch = "sparc64")] SIGTSTP = 18,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] SIGTTIN = 21,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGTTIN = 26,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] SIGTTOU = 22,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGTTOU = 27,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] SIGURG = 23,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGURG = 21,
	#[allow(missing_docs)] #[cfg(target_arch = "sparc64")] SIGURG = 16,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] SIGUSR1 = 10,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGUSR1 = 16,
	#[allow(missing_docs)] #[cfg(target_arch = "sparc64")] SIGUSR1 = 30,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] SIGUSR2 = 12,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGUSR2 = 17,
	#[allow(missing_docs)] #[cfg(target_arch = "sparc64")] SIGUSR2 = 31,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] SIGVTALRM = 26,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGVTALRM = 28,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] SIGWINCH = 28,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGWINCH = 20,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] SIGXCPU = 24,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGXCPU = 30,

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] SIGXFSZ = 25,
	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] SIGXFSZ = 31,

	#[allow(missing_docs)] LinuxThreadsRealTimeSignal0 = 32,
	#[allow(missing_docs)] LinuxThreadsRealTimeSignal1 = 33,
	#[allow(missing_docs)] LinuxThreadsRealTimeSignal2 = 34,

	#[allow(missing_docs)] RealTimeSignal35 = 35,
	#[allow(missing_docs)] RealTimeSignal36 = 36,
	#[allow(missing_docs)] RealTimeSignal37 = 37,
	#[allow(missing_docs)] RealTimeSignal38 = 38,
	#[allow(missing_docs)] RealTimeSignal39 = 39,
	#[allow(missing_docs)] RealTimeSignal40 = 40,
	#[allow(missing_docs)] RealTimeSignal41 = 41,
	#[allow(missing_docs)] RealTimeSignal42 = 42,
	#[allow(missing_docs)] RealTimeSignal43 = 43,
	#[allow(missing_docs)] RealTimeSignal44 = 44,
	#[allow(missing_docs)] RealTimeSignal45 = 45,
	#[allow(missing_docs)] RealTimeSignal46 = 46,
	#[allow(missing_docs)] RealTimeSignal47 = 47,
	#[allow(missing_docs)] RealTimeSignal48 = 48,
	#[allow(missing_docs)] RealTimeSignal49 = 49,
	#[allow(missing_docs)] RealTimeSignal50 = 50,
	#[allow(missing_docs)] RealTimeSignal51 = 51,
	#[allow(missing_docs)] RealTimeSignal52 = 52,
	#[allow(missing_docs)] RealTimeSignal53 = 53,
	#[allow(missing_docs)] RealTimeSignal54 = 54,
	#[allow(missing_docs)] RealTimeSignal55 = 55,
	#[allow(missing_docs)] RealTimeSignal56 = 56,
	#[allow(missing_docs)] RealTimeSignal57 = 57,
	#[allow(missing_docs)] RealTimeSignal58 = 58,
	#[allow(missing_docs)] RealTimeSignal59 = 59,
	#[allow(missing_docs)] RealTimeSignal60 = 60,
	#[allow(missing_docs)] RealTimeSignal61 = 61,
	#[allow(missing_docs)] RealTimeSignal62 = 62,
	#[allow(missing_docs)] RealTimeSignal63 = 63,
	#[allow(missing_docs)] RealTimeSignal64 = 64,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal65 = 65,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal66 = 66,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal67 = 67,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal68 = 68,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal69 = 69,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal70 = 70,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal71 = 71,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal72 = 72,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal73 = 73,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal74 = 74,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal75 = 75,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal76 = 76,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal77 = 77,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal78 = 78,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal79 = 79,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal80 = 80,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal81 = 81,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal82 = 82,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal83 = 83,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal84 = 84,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal85 = 85,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal86 = 86,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal87 = 87,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal88 = 88,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal89 = 89,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal90 = 90,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal91 = 91,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal92 = 92,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal93 = 93,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal94 = 94,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal95 = 95,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal96 = 96,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal97 = 97,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal98 = 98,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal99 = 99,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal100 = 100,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal101 = 101,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal102 = 102,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal103 = 103,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal104 = 104,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal105 = 105,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal106 = 106,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal107 = 107,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal108 = 108,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal109 = 109,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal110 = 110,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal111 = 111,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal112 = 112,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal113 = 113,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal114 = 114,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal115 = 115,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal116 = 116,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal117 = 117,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal118 = 118,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal119 = 119,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal120 = 120,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal121 = 121,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal122 = 122,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal123 = 123,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal124 = 124,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal125 = 125,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal126 = 126,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal127 = 127,
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] #[allow(missing_docs)] RealTimeSignal128 = 128,
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
	const LinuxMaximum: u16 = Self::SIGRTMAX as u8 as u16;

	const InclusiveMinimum: Self = unsafe { transmute(0u8) };

	const InclusiveMaximum: Self = Self::SIGRTMAX;

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
	#[allow(missing_docs)] pub const SIGIOT: Self = Self::SIGABRT;

	#[allow(missing_docs)] #[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] pub const SIGLOST: Self = Self::SIGIO;

	#[allow(missing_docs)] pub const SIGPOLL: Self = Self::SIGIO;

	#[allow(missing_docs)] pub const SIGUNUSED: Self = Self::SIGSYS;

	#[allow(missing_docs)] #[cfg(target_arch = "mips64")] pub const SIGCLD: Self = Self::SIGCHLD;

	/// This is an inclusive minimum.
	pub const SIGRTMIN: Self = Self::RealTimeSignal35;

	/// This is an inclusive maximum.
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] pub const SIGRTMAX: Self = Self::RealTimeSignal64;
	#[cfg(target_arch = "mips64")] pub const SIGRTMAX: Self = Self::RealTimeSignal128;

	/// C library name.
	#[inline(always)]
	pub fn c_library_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(strsignal(self.into()) as *const _) }
	}
}
