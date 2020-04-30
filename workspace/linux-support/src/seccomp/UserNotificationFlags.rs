// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// User notifications flags.
	pub struct UserNotificationFlags: u32
	{
		/// This flag must be used with caution!
		/// If set by the process supervising the syscalls of another process the syscall will continue.
		/// This is problematic because of an inherent TOCTOU.
		/// An attacker can exploit the time while the supervised process is waiting on a response from the supervising process to rewrite syscall arguments which are passed as pointers of the intercepted syscall.
		/// It should be absolutely clear that this means that the seccomp notifier *cannot* be used to implement a security policy!
		/// It should only ever be used in scenarios where a more privileged process supervises the syscalls of a lesser privileged process to get around kernel-enforced security restrictions when the privileged process deems this safe.
		/// In other words, in order to continue a syscall the supervising process should be sure that another security mechanism or the kernel itself will sufficiently block syscalls if arguments are rewritten to something unsafe.
		const Continue = SECCOMP_USER_NOTIF_FLAG_CONTINUE;
	}
}
