// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Personality flags.
    pub struct PersonalityFlags: i32
    {
        /// Have `uname()` report a 2.6.40+ version number rather than a 3.x version number.
        /// Added as a stopgap measure to support broken applications that could not handle the kernel version numbering switch from 2.6.x to 3.x.
        const UNAME26 = 0x0020000;

        /// With this flag set disable address-space-layout randomization.
        const ADDR_NO_RANDOMIZE = 0x0040000;

        /// User-space function pointers to signal handlers point (on certain architectures) to descriptors.
        const FDPIC_FUNCPTRS = 0x0080000;

        /// Map page 0 as read-only (to support binaries that depend on this SVr4 behavior).
        const MMAP_PAGE_ZERO = 0x0100000;

        /// With this flag set, provide legacy virtual address space layout.
        const ADDR_COMPAT_LAYOUT = 0x0200000;

        /// With this flag set `PROT_READ` implies `PROT_EXEC` for `mmap()`.
        const READ_IMPLIES_EXEC = 0x0400000;

        /// Limit the address space to 32 bits.
        const ADDR_LIMIT_32BIT = 0x0800000;

        /// ?No effects?
        const SHORT_INODE = 0x1000000;

        /// ?No effects?
        const WHOLE_SECONDS = 0x2000000;

        /// With this flag set, `select()`, `pselect()` and `ppoll()` do not modify the returned timeout argument when interrupted by a signal handler.
        const STICKY_TIMEOUTS = 0x4000000;

        /// With this flag set use `0xC0000000` as the offset at which to search a virtual memory chunk on `mmap()`.
        /// With this falg unset, use `0xFFFFE000` instead.
        const ADDR_LIMIT_3GB = 0x8000000;

        /// Linux personality.
        const PER_LINUX = 0x0000;

        /// Implies `ADDR_LIMIT_32BIT`.
        const PER_LINUX_32BIT = PersonalityFlags::PER_LINUX.bits | PersonalityFlags::ADDR_LIMIT_32BIT.bits;

        /// Implies `FDPIC_FUNCPTRS`.
        const PER_LINUX_FDPIC = PersonalityFlags::PER_LINUX.bits | PersonalityFlags::FDPIC_FUNCPTRS.bits;

        /// Implies `STICKY_TIMEOUTS` and `MMAP_PAGE_ZERO`; otherwise no effects.
        const PER_SVR4 = 0x0001 | PersonalityFlags::STICKY_TIMEOUTS.bits | PersonalityFlags::MMAP_PAGE_ZERO.bits;

        /// Implies `STICKY_TIMEOUTS` and `SHORT_INODE`; otherwise no effects.
        const PER_SVR3 = 0x0002 | PersonalityFlags::STICKY_TIMEOUTS.bits | PersonalityFlags::SHORT_INODE.bits;

        /// Implies `STICKY_TIMEOUTS`, `WHOLE_SECONDS`, and `SHORT_INODE`; otherwise no effects.
        const PER_SCOSVR3 = 0x0003 | PersonalityFlags::STICKY_TIMEOUTS.bits | PersonalityFlags::WHOLE_SECONDS.bits | PersonalityFlags::SHORT_INODE.bits;

        /// Implies `STICKY_TIMEOUTS` and `WHOLE_SECONDS`; otherwise no effects.
        const PER_OSR5 = 0x0003 | PersonalityFlags::STICKY_TIMEOUTS.bits | PersonalityFlags::WHOLE_SECONDS.bits;

        /// Implies `STICKY_TIMEOUTS` and `SHORT_INODE`; otherwise no effects.
        const PER_WYSEV386 = 0x0004 | PersonalityFlags::STICKY_TIMEOUTS.bits | PersonalityFlags::SHORT_INODE.bits;

        /// Implies STICKY_TIMEOUTS; otherwise no effects.
        const PER_ISCR4 = 0x0005 | PersonalityFlags::STICKY_TIMEOUTS.bits;

        /// BSD.
        ///
        /// No effects.
        const PER_BSD = 0x0006;

        /// Implies `STICKY_TIMEOUTS`.
        ///
        /// Divert library and dynamic linker searches to `/usr/gnemul`.
        /// Buggy, largely unmaintained, and almost entirely unused; support was removed in Linux 2.6.26.
        const PER_SUNOS = PersonalityFlags::PER_BSD.bits | PersonalityFlags::STICKY_TIMEOUTS.bits;

        /// Implies `STICKY_TIMEOUTS` and `SHORT_INODE`; otherwise no effects.
        const PER_XENIX = 0x0007 | PersonalityFlags::STICKY_TIMEOUTS.bits | PersonalityFlags::SHORT_INODE.bits;

        /// ?
        const PER_LINUX32 = 0x0008;

        /// Implies `ADDR_LIMIT_3GB`.
        const PER_LINUX32_3GB = PersonalityFlags::PER_LINUX32.bits | PersonalityFlags::ADDR_LIMIT_3GB.bits;

        /// IRIX 5 32-bit.
        ///
        /// Never fully functional; support dropped in Linux 2.6.27.
        /// Implies `STICKY_TIMEOUTS`.
        const PER_IRIX32 = 0x0009 | PersonalityFlags::STICKY_TIMEOUTS.bits;

        /// IRIX 6 new 32-bit.
        ///
        /// Implies `STICKY_TIMEOUTS`; otherwise no effects.
        const PER_IRIXN32 = 0x000A | PersonalityFlags::STICKY_TIMEOUTS.bits;

        /// IRIX 6 64-bit.
        ///
        /// Implies `STICKY_TIMEOUTS`; otherwise no effects.
        const PER_IRIX64 = 0x000B | PersonalityFlags::STICKY_TIMEOUTS.bits;

        /// ?
        const PER_RISCOS = 0x000C;

        /// Implies `STICKY_TIMEOUTS`; otherwise no effects.
        const PER_SOLARIS = 0x000D | PersonalityFlags::STICKY_TIMEOUTS.bits;

        /// Implies `STICKY_TIMEOUTS` and `MMAP_PAGE_ZERO`; otherwise no effects.
        const PER_UW7 = 0x000E | PersonalityFlags::STICKY_TIMEOUTS.bits | PersonalityFlags::MMAP_PAGE_ZERO.bits;

        /// OSF/1 v4.
        ///
        /// On the Alpha implementation of Linux, clears the top 32 bits of `iov_len` in the user's buffer for compatibility with old versions of OSF/1 where `iov_len` was defined as `int`.
        const PER_OSF4 = 0x000F;

        /// Support for 32-bit HP/UX.
        ///
        /// This support was never complete, and was dropped so that since Linux 4.0.
        /// This value has no effect.
        const PER_HPUX = 0x0010;

        /// ?
        const PER_MASK = 0x00FF;
    }
}

impl FromBytes for PersonalityFlags
{
	type Error = ParseNumberError;

	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		Ok(Self::from_bits(i32::parse_hexadecimal_number_lower_case(bytes)?).expect("Invalid bits in previous personality flags"))
	}
}

impl PersonalityFlags
{
	/// Checks if the personality is standard linux.
	#[inline(always)]
	pub fn is_standard_linux(self) -> bool
	{
		self == PersonalityFlags::PER_LINUX
	}

	/// Gets the current process domain exection model.
	#[inline(always)]
	pub fn current() -> Result<Self, ()>
	{
		const SpecialFlagPattern: c_ulong = 0xFFFF_FFFF;
		Self::personality(SpecialFlagPattern)
	}

	/// Gets a process' current process domain exection model.
	#[inline(always)]
	pub fn for_process(proc_path: &ProcPath, process_identifier: ProcessIdentifierChoice) -> io::Result<Self>
	{
		proc_path.process_file_path(process_identifier, "personality").read_value()
	}

	/// Tries to set the process domain execution model to the current set of flags.
	///
	/// On success, the previous personality flags are returned.
	#[inline(always)]
	pub fn set_process_domain_execution_model(self) -> Result<Self, ()>
	{
		Self::personality(self.bits() as c_ulong)
	}

	#[inline(always)]
	fn personality(bits: c_ulong) -> Result<Self, ()>
	{
		let result = unsafe { personality(bits) };

		if likely!(result != -1)
		{
			return Ok(Self::from_bits(result).expect("Invalid bits in previous personality flags"))
		}

		match errno().0
		{
			EINVAL => Err(()),

			unexpected @ _ => panic!(format!("Unexpected error number from 'personality' of '{:?}'", unexpected)),
		}
	}
}
