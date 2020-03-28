// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A signal bit set (mask).
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalBitSet(pub(crate) u64);

impl SignalBitSet
{
	/// Contains a standard signal?
	#[inline(always)]
	pub const fn contains_standard_signal(self, standard_signal: SignalNumber) -> bool
	{
		self.0 & 1 << ((standard_signal as u32) as u64) != 0
	}

	/// Contains a realtime signal?
	#[inline(always)]
	pub fn contains_real_time_signal(self, real_time_signal: SignalNumber) -> bool
	{
		#[cfg(all(debug_assertions, target_env = "musl"))]
		{
			extern "C"
			{
				fn __libc_current_sigrtmin() -> c_int;
				fn __libc_current_sigrtmax() -> c_int;
			}
			debug_assert!(real_time_signal >= unsafe { __libc_current_sigrtmin() }, "real_time_signal {} is less than SIGRTMIN {}", real_time_signal, unsafe { __libc_current_sigrtmin() });
			debug_assert!(real_time_signal <= unsafe { __libc_current_sigrtmax() }, "real_time_signal {} is more than SIGRTMAX {}", real_time_signal, unsafe { __libc_current_sigrtmax() });
		}
		self.0 & 1 << (((real_time_signal as u32) as u64) + size_of::<u32>() as u64) != 0
	}

	// macros SIGRTMIN and SIGRTMAX
}
