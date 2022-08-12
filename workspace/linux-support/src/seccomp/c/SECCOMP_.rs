// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Operation for `seccomp()` syscall.
///
/// The only system calls that the calling thread is permitted to make are `read()`, `write()`, `_exit()` (but not `exit_group()`), and `sigreturn()`.
///
/// Other system calls result in the delivery of a `SIGKILL` signal.
/// Strict secure computing mode is useful for number-crunching applications that may need to execute untrusted byte code, perhaps obtained by reading from a pipe or socket.
///
/// Note that although the calling thread can no longer call `sigprocmask()`, it can use `sigreturn()` to block all signals apart from `SIGKILL` and `SIGSTOP`.
/// This means that `alarm()` (for example) is not sufficient for restricting the process's execution time.
/// Instead, to reliably terminate the process, `SIGKILL` must be used.
/// This can be done by using `timer_create()` with `SIGEV_SIGNAL` and `sigev_signo` set to `SIGKILL`, or by using `setrlimit()` to set the hard limit for `RLIMIT_CPU`.
///
/// The value of `flags` must be `0`, and `args` must be `NULL`.
///
/// This operation is functionally identical to the call `prctl(PR_SET_SECCOMP, SECCOMP_MODE_STRICT)`.
pub(crate) const SECCOMP_SET_MODE_STRICT: u32 = 0;

/// Operation for `seccomp()` syscall.
///
/// The system calls allowed are defined by a pointer to a Berkeley Packet Filter (BPF) passed via `args`.
/// This argument is a pointer to a `sock_fprog`; it can be designed to filter arbitrary system calls and system call arguments.
/// If the filter is invalid, `seccomp()` fails, returning `EINVAL` in `errno`.
///
/// Filters are inherited across `clone()`, `fork()` and `execve()`.
///
/// In order to use the `SECCOMP_SET_MODE_FILTER` operation, either the calling thread must have the `CAP_SYS_ADMIN` capability in its user namespace, or the thread must already have the `no_new_privs` bit set.
/// Otherwise, the `SECCOMP_SET_MODE_FILTER` operation fails and returns `EACCES` in `errno`.
pub(crate) const SECCOMP_SET_MODE_FILTER: u32 = 1;

/// Operation for `seccomp()` syscall.
///
/// Test to see if an action is supported by the kernel.
///
/// This operation is helpful to confirm that the kernel knows of a  more recently added filter return action since the kernel treats all unknown actions as `SECCOMP_RET_KILL_PROCESS`.
///
/// The value of `flags` must be `0`, and `args` must be a pointer to an unsigned 32-bit filter return action.
pub(crate) const SECCOMP_GET_ACTION_AVAIL: u32 = 2;

/// Notification sizes.
pub(crate) const SECCOMP_GET_NOTIF_SIZES: u32 = 3;
