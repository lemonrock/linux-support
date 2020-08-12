// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


// These per-process flags are from `include/linux/sched.h`.

/// I am an IDLE thread.
pub(crate) const PF_IDLE: u32 = 0x00000002;

/// Getting shut down.
pub(crate) const PF_EXITING: u32 = 0x00000004;

/// I'm a virtual CPU.
pub(crate) const PF_VCPU: u32 = 0x00000010;

/// I'm a workqueue worker.
pub(crate) const PF_WQ_WORKER: u32 = 0x00000020;

/// Forked but didn't exec.
pub(crate) const PF_FORKNOEXEC: u32 = 0x00000040;

/// Process policy on mce errors.
pub(crate) const PF_MCE_PROCESS: u32 = 0x00000080;

/// Used super-user privileges.
pub(crate) const PF_SUPERPRIV: u32 = 0x00000100;

/// Dumped core.
pub(crate) const PF_DUMPCORE: u32 = 0x00000200;

/// Killed by a signal.
pub(crate) const PF_SIGNALED: u32 = 0x00000400;

/// Allocating memory.
pub(crate) const PF_MEMALLOC: u32 = 0x00000800;

/// `set_user()` noticed that `RLIMIT_NPROC` was exceeded.
pub(crate) const PF_NPROC_EXCEEDED: u32 = 0x00001000;

/// If unset the FPU must be initialized before use.
pub(crate) const PF_USED_MATH: u32 = 0x00002000;

/// Used `async_schedule*()`, used by module init.
pub(crate) const PF_USED_ASYNC: u32 = 0x00004000;

/// This thread should not be frozen.
pub(crate) const PF_NOFREEZE: u32 = 0x00008000;

/// Frozen for system suspend.
pub(crate) const PF_FROZEN: u32 = 0x00010000;

/// I am `kswapd`.
pub(crate) const PF_KSWAPD: u32 = 0x00020000;

/// All allocation requests will inherit `GFP_NOFS`.
pub(crate) const PF_MEMALLOC_NOFS: u32 = 0x00040000;

/// All allocation requests will inherit `GFP_NOIO`.
pub(crate) const PF_MEMALLOC_NOIO: u32 = 0x00080000;

/// Throttle me less: I clean memory.
pub(crate) const PF_LESS_THROTTLE: u32 = 0x00100000;

/// I am a kernel thread.
pub(crate) const PF_KTHREAD: u32 = 0x00200000;

/// Randomize virtual address space.
pub(crate) const PF_RANDOMIZE: u32 = 0x00400000;

/// Allowed to write to swap.
pub(crate) const PF_SWAPWRITE: u32 = 0x00800000;

/// I'm an Usermodehelper process.
pub(crate) const PF_UMH: u32 = 0x02000000;

/// Userland is not allowed to meddle with cpus_mask.
pub(crate) const PF_NO_SETAFFINITY: u32 = 0x04000000;

/// Early kill for mce process policy.
pub(crate) const PF_MCE_EARLY: u32 = 0x08000000;

/// All allocation request will have `_GFP_MOVABLE` cleared.
pub(crate) const PF_MEMALLOC_NOCMA: u32 = 0x10000000;

/// Task is an IO worker.
pub(crate) const PF_IO_WORKER: u32 = 0x20000000;

/// Freezer should not count it as freezable.
pub(crate) const PF_FREEZER_SKIP: u32 = 0x40000000;

/// This thread called `freeze_processes()` and should not be frozen.
pub(crate) const PF_SUSPEND_TASK: u32 = 0x80000000;
