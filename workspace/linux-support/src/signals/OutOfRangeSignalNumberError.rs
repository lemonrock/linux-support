// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A signal number from Linux was out-of-range.
///
/// It is extremely hard to validate the Linux kernel is doing the right thing as it uses a panoply of different signal number types (known of which are anything other than variously-sized integers) and paths through the kernel.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OutOfRangeSignalNumberError
{
	/// A signal number was zero.
	SignalNumberWasZero,

	/// A signal number was out of range.
	///
	/// Does not occur for MIPS64.
	#[cfg(any(target_arch = "aarch64", target_arch = "riscv64", target_arch = "powerpc64", target_arch = "s390x", target_arch = "sparc64", target_arch = "x86_64"))]
	U7SignalNumberWasOutOfRange
	{
		/// Out-of-range value.
		raw_signal_number: NonZeroU8,
	},

	/// A signal number was out of range.
	U8SignalNumberWasOutOfRange
	{
		/// Out-of-range value.
		raw_signal_number: NonZeroU8,
	},

	/// A signal number was out of range.
	U32SignalNumberWasOutOfRange
	{
		/// Out-of-range value.
		raw_signal_number: NonZeroU32,
	},
}

impl Display for OutOfRangeSignalNumberError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		<OutOfRangeSignalNumberError as Debug>::fmt(self, f)
	}
}

impl error::Error for OutOfRangeSignalNumberError
{
	#[inline(always)]
	fn source(&self) ->  Option<&(dyn error::Error + 'static)>
	{
		None
	}
}

impl OutOfRangeSignalNumberError
{
}
