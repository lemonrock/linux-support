// This file is part of file-descriptors. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT. No part of file-descriptors, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2018-2019 The developers of file-descriptors. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/file-descriptors/master/COPYRIGHT.


/// Supposedly generated from within the kernel without libc intervention.
///
/// Kernel `include/uapi/asm-generic/siginfo.h`: "sent by the kernel from somewhere".
///
/// This is an exclusive maximum for the range of Kernel-valid 'SI' codes.
pub const SI_KERNEL: c_int = 0x80;

/// Caused by the syscall the libc function `kill()` calls.
///
/// Kernel `include/uapi/asm-generic/siginfo.h`: "sent by kill, sigsend, raise".
///
/// This is an exclusive minimum for the range of Kernel-valid 'SI' codes.
pub const SI_USER: c_int = 0;

/// Supposedly caused by the libc function `sigqueue()`, but the syscall this calls can take any *negative* `SI_*` value (ie not `SI_KERNEL` or `SI_USER`) apart from `SI_TKILL`.
///
/// Kernel `include/uapi/asm-generic/siginfo.h`: "sent by sigqueue".
pub const SI_QUEUE: c_int = -1;

/// Caused by libc & kernel implementations of POSIX timers.
///
/// Kernel `include/uapi/asm-generic/siginfo.h`: "sent by timer expiration".
///
/// Constant value which is defined differently for the MIPS architecture for no good reason.
#[cfg(not(any(target_arch = "mips64")))] pub const SI_TIMER: c_int = -2;
#[cfg(any(target_arch = "mips64"))] pub const SI_TIMER: c_int = -3;

/// Caused by libc implementations of POSIX message queues.
///
/// Kernel `include/uapi/asm-generic/siginfo.h`: "sent by real time mesq state change".
///
/// Constant value which is defined differently for the MIPS architecture for no good reason.
#[cfg(not(any(target_arch = "mips64")))] pub const SI_MESGQ: c_int = -3;
#[cfg(any(target_arch = "mips64"))] pub const SI_MESGQ: c_int = -4;

/// Caused by libc implementations of POSIX AIO.
///
/// Kernel `include/uapi/asm-generic/siginfo.h`: "sent by AIO completion".
///
/// Constant value which is defined differently for the MIPS architecture for no good reason.
#[cfg(not(any(target_arch = "mips64")))] pub const SI_ASYNCIO: c_int = -4;
#[cfg(any(target_arch = "mips64"))] pub const SI_ASYNCIO: c_int = -2;

/// Equivalent to `SIGPOLL`.
///
/// Kernel `include/uapi/asm-generic/siginfo.h`: "sent by queued SIGIO".
pub const SI_SIGIO: c_int = -5;

/// Caused by the syscalls the libc functions `raise()`, ``tkill()` and `tgkill()` call.
///
/// Kernel `include/uapi/asm-generic/siginfo.h`: "sent by tkill system call".
pub const SI_TKILL: c_int = -6;

/// Not obvious if this can escape the kernel.
///
/// Kernel `include/uapi/asm-generic/siginfo.h`: "sent by execve() killing subsidiary threads".
pub const SI_DETHREAD: c_int = -7;

/// Kernel `include/uapi/asm-generic/siginfo.h`: "sent by glibc async name lookup completion".
pub const SI_ASYNCNL: c_int = -60;

/// Only exists on Sparc and possibly only used for Solaris compatibility as part of Linux personality.
#[cfg(target_arch = "sparc64")] pub const SI_NOINFO: c_int = 32767;
