// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Valid flags for struct `seccomp_notif_resp`.
///
/// Note, the `SECCOMP_USER_NOTIF_FLAG_CONTINUE` flag must be used with caution!
/// If set by the process supervising the syscalls of another process the syscall will continue.
/// This is problematic because of an inherent TOCTOU.
/// An attacker can exploit the time while the supervised process is waiting on a response from the supervising process to rewrite syscall arguments which are passed as pointers of the intercepted syscall.
/// It should be absolutely clear that this means that the seccomp notifier *cannot* be used to implement a security policy!
/// It should only ever be used in scenarios where a more privileged process supervises the syscalls of a lesser privileged process to get around kernel-enforced security restrictions when the privileged process deems this safe.
/// In other words, in order to continue a syscall the supervising process should be sure that another security mechanism or the kernel itself will sufficiently block syscalls if arguments are rewritten to something unsafe.
///
/// Similar precautions should be applied when stacking `SECCOMP_RET_USER_NOTIF` or `SECCOMP_RET_TRACE`.
/// For `SECCOMP_RET_USER_NOTIF` filters acting on the same syscall, the most recently added filter takes precedence.
/// This means that the new `SECCOMP_RET_USER_NOTIF` filter can override any `SECCOMP_IOCTL_NOTIF_SEND` from earlier filters, essentially allowing all such filtered syscalls to be executed by sending the response `SECCOMP_USER_NOTIF_FLAG_CONTINUE`.
/// Note that `SECCOMP_RET_TRACE` can equally be overriden by `SECCOMP_USER_NOTIF_FLAG_CONTINUE`.
pub(crate) const SECCOMP_USER_NOTIF_FLAG_CONTINUE: u32 = 1 << 0;
