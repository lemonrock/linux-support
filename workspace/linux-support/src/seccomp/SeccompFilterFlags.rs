// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Flags.
	pub struct SeccompFilterFlags: u32
	{
		/// When adding a new filter, synchronize all other threads of the calling process to the same seccomp filter tree.
		///
		/// A "filter tree" is the ordered list of filters attached to a thread.
		/// (Attaching identical filters in separate `seccomp()` calls results in different filters from this perspective).
		///
		/// If any thread cannot synchronize to the same filter tree, the call will not attach the new seccomp filter, and will fail, returning the first thread ID found that cannot synchronize.
		/// Synchronization will fail if another thread in the same process is in `SECCOMP_MODE_STRICT` or if it has attached new seccomp filters to itself, diverging from the calling thread's filter tree.
		const SynchronizeAllThreads = SECCOMP_FILTER_FLAG_TSYNC;

		/// All filter return actions except `SECCOMP_RET_ALLOW` should be logged.
		///
		/// An administrator may override this filter flag by preventing specific actions from being logged via the `/proc/sys/kernel/seccomp/actions_logged` file.
		///
		/// Since Linux 4.14.
		const Log = SECCOMP_FILTER_FLAG_LOG;

		/// Disable Speculative Store Bypass mitigation.
		///
		/// Since Linux 4.17.
		const DisableSpeculativeStoreBypassMitigation = SECCOMP_FILTER_FLAG_SPEC_ALLOW;

		/// Returns a new file descriptor which can receive events when a BPF program executes return instructions with `SyscallOutcome::NotifyUserspace`.
		const NewListener = SECCOMP_FILTER_FLAG_NEW_LISTENER;

		/// ?
		const SynchronizeAllThreadsESRCH = SECCOMP_FILTER_FLAG_TSYNC_ESRCH;
	}
}
