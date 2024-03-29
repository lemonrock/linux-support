// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `BitSet` of `Signal`.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[repr(transparent)]
pub struct Signals(pub BitSet<Signal>);

impl Deref for Signals
{
	type Target = BitSet<Signal>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		&self.0
	}
}

impl DerefMut for Signals
{
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target
	{
		&mut self.0
	}
}

impl Signals
{
	/// Block all signals on the current thread.
	#[inline(always)]
	pub fn block_all_signals_on_current_thread()
	{
		Self::default().block_all_signals_on_current_thread_bar()
	}

	/// Block all signals except those in `self` specified on the current thread.
	#[inline(always)]
	pub fn block_all_signals_on_current_thread_bar(&self)
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
	pub fn to_complement_sigset_t(&self) -> sigset_t
	{
		let mut set = unsafe_uninitialized();
		unsafe
		{
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
	pub fn to_sigset_t(&self) -> sigset_t
	{
		let mut set = unsafe_uninitialized();
		unsafe
		{
			sigemptyset(&mut set);
			for signal in self.iterate()
			{
				sigaddset(&mut set, signal.into());
			}
			set
		}
	}

	/// Block signals.
	#[inline(always)]
	pub fn block_signals(signal_mask: &sigset_t)
	{
		// TODO: The parsing of the result is possible wrong - the man page is poor; examination of musl libc code seems to suggest that `__syscall_ret()` is not called to set the `errno`.
		match unsafe { pthread_sigmask(SIG_BLOCK, signal_mask, null_mut()) }
		{
			0 => (),
			
			SystemCallErrorNumber::NegativeEFAULT => (),
			SystemCallErrorNumber::NegativeEINVAL => (),
			
			unexpected_error @ SystemCallErrorNumber::NegativeInclusiveMinimumI32 ..= SystemCallErrorNumber::NegativeInclusiveMaximumI32 => unexpected_error!(pthread_sigmask, SystemCallErrorNumber::from_negative_i32_unchecked(unexpected_error)),
			
			result @ _ => unexpected_result!(pthread_sigmask, result)
		}
	}

	#[inline(always)]
	pub(crate) fn filled_signal_mask() -> sigset_t
	{
		let mut signal_mask = unsafe_uninitialized();
		let result = unsafe { sigfillset(&mut signal_mask) };
		if likely!(result == 0)
		{
			signal_mask
		}
		else if likely!(result == -1)
		{
			match SystemCallErrorNumber::from_errno_panic()
			{
				EINVAL => panic!("Invalid arguments"),
				unexpected_error @ _ => unexpected_error!(sigfillset, unexpected_error),
			}
		}
		else
		{
			unexpected_result!(sigfillset, result)
		}
	}
}
