// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Block all signals specified the current thread.
#[inline(always)]
pub fn block_all_signals_on_current_thread_bar(signals: &HashSet<SignalNumber>)
{
	let result = unsafe
	{
		#[allow(deprecated)] let mut set = uninitialized();
		sigfillset(&mut set);
		for signal in signals.iter()
		{
			sigdelset(&mut set, *signal);
		}
		pthread_sigmask(SIG_SETMASK, &set, null_mut())
	};

	match result
	{
		0 => (),
		-1 => panic!("pthread_sigmask returned an error"),
		_ => panic!("pthread_sigmask returned an invalid result '{}'", result)
	}
}
