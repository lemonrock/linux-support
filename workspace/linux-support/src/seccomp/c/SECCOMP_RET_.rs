// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// All BPF programs must return a 32-bit value.
//
// The bottom 16-bits are for optional return data.
// The upper 16-bits are ordered from least permissive values to most, as a signed value (so `0x8000000` is negative).
//
// The ordering ensures that a `min_t()` over composed return values always selects the least permissive choice.

/// kill the process.
pub(crate) const SECCOMP_RET_KILL_PROCESS: u32 = 0x80000000;

/// kill the thread.
pub(crate) const SECCOMP_RET_KILL_THREAD: u32 = 0x00000000;

#[allow(dead_code)]
pub(crate) const SECCOMP_RET_KILL: u32 = SECCOMP_RET_KILL_THREAD;

/// disallow and force a `SIGSYS`.
pub(crate) const SECCOMP_RET_TRAP: u32 = 0x00030000;

/// returns an errno.
pub(crate) const SECCOMP_RET_ERRNO: u32 = 0x00050000;

/// notifies userspace.
pub(crate) const SECCOMP_RET_USER_NOTIF: u32 = 0x7FC00000;

/// pass to a tracer or disallow.
pub(crate) const SECCOMP_RET_TRACE: u32 = 0x7FF00000;

/// allow after logging.
pub(crate) const SECCOMP_RET_LOG: u32 = 0x7FFC0000;

/// allow.
pub(crate) const SECCOMP_RET_ALLOW: u32 = 0x7FFF0000;

/// Mask for the return value sections.
#[allow(dead_code)]
pub(crate) const SECCOMP_RET_ACTION_FULL: u32 = 0xFFFF0000;

/// Mask for the return value sections.
#[allow(dead_code)]
pub(crate) const SECCOMP_RET_ACTION: u32 = 0x7FFF0000;

/// Mask for the return value sections.
#[allow(dead_code)]
pub(crate) const SECCOMP_RET_DATA: u32 = 0x0000FFFF;
