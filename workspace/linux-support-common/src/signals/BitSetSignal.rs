// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


impl BitSet<Signal>
{
	/// Block all signals on the current thread.
	#[inline(always)]
	pub fn block_all_signals_on_current_thread()
	{
		Self::default().block_all_signals_on_current_thread_bar()
	}

	/// Block all signals except those in `self` specified on the current thread.
	#[inline(always)]
	pub fn block_all_signals_on_current_thread_bar(self)
	{
		let result = unsafe
		{
			let set = self.to_complement_sigset_t();
			pthread_sigmask(SIG_SETMASK, &set, null_mut())
		};

		match result
		{
			0 => (),
			-1 => panic!("pthread_sigmask returned an error"),
			_ => panic!("pthread_sigmask returned an invalid result '{}'", result)
		}
	}

	/// Converts a BitSet of signals to a libc `sigset_t` which does not contain them.
	#[inline(always)]
	pub fn to_complement_sigset_t(self) -> sigset_t
	{
		unsafe
		{
			#[allow(deprecated)] let mut set = uninitialized();
			sigfillset(&mut set);
			for exclude_signal in self.iterate()
			{
				sigdelset(&mut set, exclude_signal.into());
			}
			set
		}
	}

	/// Converts a BitSet of signals to a libc `sigset_t`.
	#[inline(always)]
	pub fn to_sigset_t(self) -> sigset_t
	{
		unsafe
		{
			#[allow(deprecated)] let mut set = uninitialized();
			sigemptyset(&mut set);
			for signal in self.iterate()
			{
				sigaddset(&mut set, signal.into());
			}
			set
		}
	}
}
