// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Represents a Linux signal number.
///
/// One-based.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[derive(EnumIter)]
#[repr(u8)]
pub enum Signal
{
	#[allow(missing_docs)]
	SIGABRT = Self::SIGABRT_,

	#[allow(missing_docs)]
	SIGALRM = Self::SIGALRM_,

	#[allow(missing_docs)]
	SIGBUS = Self::SIGBUS_,

	#[allow(missing_docs)]
	SIGCHLD = Self::SIGCHLD_,

	#[allow(missing_docs)]
	SIGCONT = Self::SIGCONT_,

	#[allow(missing_docs)]
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))]
	SIGEMT = Self::SIGEMT_,

	#[allow(missing_docs)]
	SIGFPE = Self::SIGFPE_,

	#[allow(missing_docs)]
	SIGHUP = Self::SIGHUP_,

	#[allow(missing_docs)]
	SIGILL = Self::SIGILL_,

	#[allow(missing_docs)]
	SIGINT = Self::SIGINT_,

	#[allow(missing_docs)]
	SIGIO = Self::SIGIO_,

	#[allow(missing_docs)]
	SIGKILL = Self::SIGKILL_,

	#[allow(missing_docs)]
	SIGPIPE = Self::SIGPIPE_,

	#[allow(missing_docs)]
	SIGPROF = Self::SIGPROF_,

	#[allow(missing_docs)]
	SIGPWR = Self::SIGPWR_,

	#[allow(missing_docs)]
	SIGQUIT = Self::SIGQUIT_,

	#[allow(missing_docs)]
	SIGSEGV = Self::SIGSEGV_,

	#[allow(missing_docs)]
	SIGSTKFLT = Self::SIGSTKFLT_,

	#[allow(missing_docs)]
	SIGSTOP = Self::SIGSTOP_,

	#[allow(missing_docs)]
	SIGSYS = Self::SIGSYS_,

	#[allow(missing_docs)]
	SIGTERM = Self::SIGTERM_,

	#[allow(missing_docs)]
	SIGTRAP = Self::SIGTRAP_,

	#[allow(missing_docs)]
	SIGTSTP = Self::SIGTSTP_,

	#[allow(missing_docs)]
	SIGTTIN = Self::SIGTTIN_,

	#[allow(missing_docs)]
	SIGTTOU = Self::SIGTTOU_,

	#[allow(missing_docs)]
	SIGURG = Self::SIGURG_,

	#[allow(missing_docs)]
	SIGUSR1 = Self::SIGUSR1_,

	#[allow(missing_docs)]
	SIGUSR2 = Self::SIGUSR2_,

	#[allow(missing_docs)]
	SIGVTALRM = Self::SIGVTALRM_,

	#[allow(missing_docs)]
	SIGWINCH = Self::SIGWINCH_,

	#[allow(missing_docs)]
	SIGXCPU = Self::SIGXCPU_,

	#[allow(missing_docs)]
	SIGXFSZ = Self::SIGXFSZ_,

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
	#[cfg(target_arch = "mips64")] pub const SIGRTMAX: Self = Self::RealTimeSignal127;

	const InclusiveMinimum: u8 = Self::LinuxThreadsRealTimeSignal0 as u8;

	const InclusiveMaximum: u8 = Self::SIGRTMAX as u8;

	/// C library name.
	#[inline(always)]
	pub fn c_library_name(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(strsignal(self.into()) as *const _) }
	}

	#[inline(always)]
	pub(crate) fn parse_raw_signal_number_non_zero_u7(raw_signal_number_non_zero_u7: NonZeroU8) -> Result<Self, OutOfRangeSignalNumberError>
	{
		use self::OutOfRangeSignalNumberError::*;

		const MaxU7: u8 = 0x7F;

		let raw_signal_number = raw_signal_number_non_zero_u7.get();
		debug_assert!(raw_signal_number <= MaxU7);

		use self::Signal::*;
		match raw_signal_number
		{
			0 => Err(SignalNumberWasZero),

			Self::SIGABRT_ => Ok(SIGABRT),
			Self::SIGALRM_ => Ok(SIGALRM),
			Self::SIGBUS_ => Ok(SIGBUS),
			Self::SIGCHLD_ => Ok(SIGCHLD),
			Self::SIGCONT_ => Ok(SIGCONT),
			#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] Self::SIGEMT_ => Ok(SIGEMT),
			Self::SIGFPE_ => Ok(SIGFPE),
			Self::SIGHUP_ => Ok(SIGHUP),
			Self::SIGILL_ => Ok(SIGILL),
			Self::SIGINT_ => Ok(SIGINT),
			Self::SIGIO_ => Ok(SIGIO),
			Self::SIGKILL_ => Ok(SIGKILL),
			Self::SIGPIPE_ => Ok(SIGPIPE),
			Self::SIGPROF_ => Ok(SIGPROF),
			Self::SIGPWR_ => Ok(SIGPWR),
			Self::SIGQUIT_ => Ok(SIGQUIT),
			Self::SIGSEGV_ => Ok(SIGSEGV),
			Self::SIGSTKFLT_ => Ok(SIGSTKFLT),
			Self::SIGSTOP_ => Ok(SIGSTOP),
			Self::SIGSYS_ => Ok(SIGSYS),
			Self::SIGTERM_ => Ok(SIGTERM),
			Self::SIGTRAP_ => Ok(SIGTRAP),
			Self::SIGTSTP_ => Ok(SIGTSTP),
			Self::SIGTTIN_ => Ok(SIGTTIN),
			Self::SIGTTOU_ => Ok(SIGTTOU),
			Self::SIGURG_ => Ok(SIGURG),
			Self::SIGUSR1_ => Ok(SIGUSR1),
			Self::SIGUSR2_ => Ok(SIGUSR2),
			Self::SIGVTALRM_ => Ok(SIGVTALRM),
			Self::SIGWINCH_ => Ok(SIGWINCH),
			Self::SIGXCPU_ => Ok(SIGXCPU),
			Self::SIGXFSZ_ => Ok(SIGXFSZ),

			Self::InclusiveMinimum ..= Self::InclusiveMaximum => Ok(unsafe { transmute(raw_signal_number) }),

			_ => Err(U7SignalNumberWasOutOfRange { raw_signal_number: raw_signal_number_non_zero_u7 }),
		}
	}

	#[inline(always)]
	pub(crate) fn parse_raw_signal_number_u8(raw_signal_number_u8: u8) -> Result<Self, OutOfRangeSignalNumberError>
	{
		use self::OutOfRangeSignalNumberError::*;

		use self::Signal::*;
		match raw_signal_number_u8
		{
			0 => Err(SignalNumberWasZero),

			Self::SIGABRT_ => Ok(SIGABRT),
			Self::SIGALRM_ => Ok(SIGALRM),
			Self::SIGBUS_ => Ok(SIGBUS),
			Self::SIGCHLD_ => Ok(SIGCHLD),
			Self::SIGCONT_ => Ok(SIGCONT),
			#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] Self::SIGEMT_ => Ok(SIGEMT),
			Self::SIGFPE_ => Ok(SIGFPE),
			Self::SIGHUP_ => Ok(SIGHUP),
			Self::SIGILL_ => Ok(SIGILL),
			Self::SIGINT_ => Ok(SIGINT),
			Self::SIGIO_ => Ok(SIGIO),
			Self::SIGKILL_ => Ok(SIGKILL),
			Self::SIGPIPE_ => Ok(SIGPIPE),
			Self::SIGPROF_ => Ok(SIGPROF),
			Self::SIGPWR_ => Ok(SIGPWR),
			Self::SIGQUIT_ => Ok(SIGQUIT),
			Self::SIGSEGV_ => Ok(SIGSEGV),
			Self::SIGSTKFLT_ => Ok(SIGSTKFLT),
			Self::SIGSTOP_ => Ok(SIGSTOP),
			Self::SIGSYS_ => Ok(SIGSYS),
			Self::SIGTERM_ => Ok(SIGTERM),
			Self::SIGTRAP_ => Ok(SIGTRAP),
			Self::SIGTSTP_ => Ok(SIGTSTP),
			Self::SIGTTIN_ => Ok(SIGTTIN),
			Self::SIGTTOU_ => Ok(SIGTTOU),
			Self::SIGURG_ => Ok(SIGURG),
			Self::SIGUSR1_ => Ok(SIGUSR1),
			Self::SIGUSR2_ => Ok(SIGUSR2),
			Self::SIGVTALRM_ => Ok(SIGVTALRM),
			Self::SIGWINCH_ => Ok(SIGWINCH),
			Self::SIGXCPU_ => Ok(SIGXCPU),
			Self::SIGXFSZ_ => Ok(SIGXFSZ),

			Self::InclusiveMinimum ..= Self::InclusiveMaximum => Ok(unsafe { transmute(raw_signal_number_u8) }),

			_ => Err(U8SignalNumberWasOutOfRange { raw_signal_number: unsafe { NonZeroU8::new_unchecked(raw_signal_number_u8) } }),
		}
	}

	#[inline(always)]
	pub(crate) fn parse_raw_signal_number_u32(raw_signal_number_u32: u32) -> Result<Self, OutOfRangeSignalNumberError>
	{
		use self::OutOfRangeSignalNumberError::*;

		if likely!(raw_signal_number_u32 < u8::MAX as u32)
		{
			use self::Signal::*;

			let raw_signal_number_u8 = raw_signal_number_u32 as u8;
			match raw_signal_number_u8
			{
				0 => Err(SignalNumberWasZero),

				Self::SIGABRT_ => Ok(SIGABRT),
				Self::SIGALRM_ => Ok(SIGALRM),
				Self::SIGBUS_ => Ok(SIGBUS),
				Self::SIGCHLD_ => Ok(SIGCHLD),
				Self::SIGCONT_ => Ok(SIGCONT),
				#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] Self::SIGEMT_ => Ok(SIGEMT),
				Self::SIGFPE_ => Ok(SIGFPE),
				Self::SIGHUP_ => Ok(SIGHUP),
				Self::SIGILL_ => Ok(SIGILL),
				Self::SIGINT_ => Ok(SIGINT),
				Self::SIGIO_ => Ok(SIGIO),
				Self::SIGKILL_ => Ok(SIGKILL),
				Self::SIGPIPE_ => Ok(SIGPIPE),
				Self::SIGPROF_ => Ok(SIGPROF),
				Self::SIGPWR_ => Ok(SIGPWR),
				Self::SIGQUIT_ => Ok(SIGQUIT),
				Self::SIGSEGV_ => Ok(SIGSEGV),
				Self::SIGSTKFLT_ => Ok(SIGSTKFLT),
				Self::SIGSTOP_ => Ok(SIGSTOP),
				Self::SIGSYS_ => Ok(SIGSYS),
				Self::SIGTERM_ => Ok(SIGTERM),
				Self::SIGTRAP_ => Ok(SIGTRAP),
				Self::SIGTSTP_ => Ok(SIGTSTP),
				Self::SIGTTIN_ => Ok(SIGTTIN),
				Self::SIGTTOU_ => Ok(SIGTTOU),
				Self::SIGURG_ => Ok(SIGURG),
				Self::SIGUSR1_ => Ok(SIGUSR1),
				Self::SIGUSR2_ => Ok(SIGUSR2),
				Self::SIGVTALRM_ => Ok(SIGVTALRM),
				Self::SIGWINCH_ => Ok(SIGWINCH),
				Self::SIGXCPU_ => Ok(SIGXCPU),
				Self::SIGXFSZ_ => Ok(SIGXFSZ),

				Self::InclusiveMinimum ..= Self::InclusiveMaximum => Ok(unsafe { transmute(raw_signal_number_u8) }),

				_ => Err(U32SignalNumberWasOutOfRange { raw_signal_number: unsafe { NonZeroU32::new_unchecked(raw_signal_number_u32) } }),
			}
		}
		else
		{
			Err(U32SignalNumberWasOutOfRange { raw_signal_number: unsafe { NonZeroU32::new_unchecked(raw_signal_number_u32) } })
		}
	}

	const SIGABRT_: u8 = 6;

	const SIGALRM_: u8 = 14;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] const SIGBUS_: u8 = 7;
	#[cfg(target_arch = "mips64")] const SIGBUS_: u8 = 10;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] const SIGCHLD_: u8 = 17;
	#[cfg(target_arch = "mips64")] const SIGCHLD_: u8 = 18;
	#[cfg(target_arch = "sparc64")] const SIGCHLD_: u8 = 20;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] const SIGCONT_: u8 = 18;
	#[cfg(target_arch = "mips64")] const SIGCONT_: u8 = 25;
	#[cfg(target_arch = "sparc64")] const SIGCONT_: u8 = 19;

	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] const SIGEMT_: u8 = 7;

	const SIGFPE_: u8 = 8;

	const SIGHUP_: u8 = 1;

	const SIGILL_: u8 = 4;

	const SIGINT_: u8 = 2;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] const SIGIO_: u8 = 29;
	#[cfg(target_arch = "mips64")] const SIGIO_: u8 = 22;
	#[cfg(target_arch = "sparc64")] const SIGIO_: u8 = 23;

	const SIGKILL_: u8 = 9;

	const SIGPIPE_: u8 = 13;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] const SIGPROF_: u8 = 27;
	#[cfg(target_arch = "mips64")] const SIGPROF_: u8 = 29;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] const SIGPWR_: u8 = 30;
	#[cfg(target_arch = "mips64")] const SIGPWR_: u8 = 19;
	#[cfg(target_arch = "sparc64")] const SIGPWR_: u8 = 29;

	const SIGQUIT_: u8 = 3;

	const SIGSEGV_: u8 = 11;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] const SIGSTKFLT_: u8 = 16;
	#[cfg(target_arch = "mips64")] const SIGSTKFLT_: u8 = 7;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] const SIGSTOP_: u8 = 19;
	#[cfg(target_arch = "mips64")] const SIGSTOP_: u8 = 23;
	#[cfg(target_arch = "sparc64")] const SIGSTOP_: u8 = 17;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] const SIGSYS_: u8 = 31;
	#[cfg(any(target_arch = "mips64", target_arch = "sparc64"))] const SIGSYS_: u8 = 12;

	const SIGTERM_: u8 = 15;

	const SIGTRAP_: u8 = 5;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] const SIGTSTP_: u8 = 20;
	#[cfg(target_arch = "mips64")] const SIGTSTP_: u8 = 24;
	#[cfg(target_arch = "sparc64")] const SIGTSTP_: u8 = 18;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] const SIGTTIN_: u8 = 21;
	#[cfg(target_arch = "mips64")] const SIGTTIN_: u8 = 26;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] const SIGTTOU_: u8 = 22;
	#[cfg(target_arch = "mips64")] const SIGTTOU_: u8 = 27;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] const SIGURG_: u8 = 23;
	#[cfg(target_arch = "mips64")] const SIGURG_: u8 = 21;
	#[cfg(target_arch = "sparc64")] const SIGURG_: u8 = 16;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] const SIGUSR1_: u8 = 10;
	#[cfg(target_arch = "mips64")] const SIGUSR1_: u8 = 16;
	#[cfg(target_arch = "sparc64")] const SIGUSR1_: u8 = 30;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "x86_64"))] const SIGUSR2_: u8 = 12;
	#[cfg(target_arch = "mips64")] const SIGUSR2_: u8 = 17;
	#[cfg(target_arch = "sparc64")] const SIGUSR2_: u8 = 31;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] const SIGVTALRM_: u8 = 26;
	#[cfg(target_arch = "mips64")] const SIGVTALRM_: u8 = 28;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] const SIGWINCH_: u8 = 28;
	#[cfg(target_arch = "mips64")] const SIGWINCH_: u8 = 20;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] const SIGXCPU_: u8 = 24;
	#[cfg(target_arch = "mips64")] const SIGXCPU_: u8 = 30;

	#[cfg(any(target_arch = "aarch64", target_arch = "powerpc64", target_arch = "riscv64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))] const SIGXFSZ_: u8 = 25;
	#[cfg(target_arch = "mips64")] const SIGXFSZ_: u8 = 31;
}
