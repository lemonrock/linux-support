// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


macro_rules! wait_for_signals
{
	($self: ident, $signals_to_wait_for: ident, $running_interactively: ident) =>
	{
		{
			use self::TimedSignalWait::*;

			match one_millisecond_timed_wait_for_signals(&$signals_to_wait_for)
			{
				TimedOut => (),

				OtherSignalInterrupted =>
				{
					$self.should_function_terminate.exit_signalled(None);

					return None
				}

				Signalled(signal_number) =>
				{
					$self.should_function_terminate.exit_signalled(Some(signal_number));

					return if $running_interactively
					{
						match signal_number
						{
							SIGTERM => None,
							SIGHUP => None,
							SIGINT => Some(SIGINT),
							SIGQUIT => Some(SIGQUIT),

							_ => panic!("Blocked signal '{:?}' received", signal_number),
						}
					}
					else
					{
						match signal_number
						{
							SIGTERM => None,

							_ => panic!("Blocked signal '{:?}' received", signal_number),
						}
					}
				}
			}
		}
	}
}

/// Master loop.
pub struct MasterLoop
{
	should_function_terminate: Arc<ShouldFunctionTerminate>,
}

impl MasterLoop
{
	/// Main loop.
	pub fn main_loop(&self, running_interactively: bool) -> Option<Signal>
	{
		let success_or_failure = catch_unwind(AssertUnwindSafe(||
		{
			self.progress_busy_loop_with_signal_handling(running_interactively)
		}));

		match success_or_failure
		{
			Err(panic_payload) =>
			{
				self.should_function_terminate.we_panicked(&panic_payload);

				resume_unwind(panic_payload)
			}

			Ok(reraise_signal) =>
			{
				reraise_signal
			}
		}
	}

	fn progress_busy_loop_with_signal_handling(&self, running_interactively: bool) -> Option<Signal>
	{
		use self::Signal::*;

		let signals_to_accept: Signals = if running_interactively
		{
			bit_set!
			{
				SIGTERM,
				SIGHUP,
				SIGINT,
				SIGQUIT
			}
		}
		else
		{
			bit_set!
			{
				SIGTERM
				// NOTE: `SIGHUP` has been used conventionally to force a daemon to re-read its configuration; we're probably better off using `SIGUSR1` or `SIGUSR2`.
				// `SIGUSR1` / `SIGUSR2` can also be used, with `sigqueue`, to send a 32-bit value to a process using `SI_QUEUE` `si_code`.
			}
		};

		signals_to_accept.block_all_signals_on_current_thread_bar();
		let signals_to_wait_for = signals_to_accept.to_sigset_t();

		while self.should_function_terminate.should_continue()
		{
			wait_for_signals!(self, signals_to_wait_for, running_interactively)
		}

		None
	}
}
