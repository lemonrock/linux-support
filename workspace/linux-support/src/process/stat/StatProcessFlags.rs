// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


bitflags!
{
	/// Stat process flags.
	pub struct StatProcessFlags: u32
	{
		/// I am an IDLE thread.
		const IAmAnIdleThread = PF_IDLE;
		
		/// Getting shut down.
		const Existing = PF_EXITING;
		
		/// I'm a virtual CPU.
		const VirtualCpu = PF_VCPU;
		
		/// I'm a workqueue worker.
		const WorkQueueWorker = PF_WQ_WORKER;
		
		/// Forked but didn't exec.
		const ForkedButDidNotExec = PF_FORKNOEXEC;
		
		/// Process policy on mce errors.
		const MachineCheckExceptionPolicy = PF_MCE_PROCESS;
		
		/// Used super-user privileges.
		const UsedSuperUserPrivileges = PF_SUPERPRIV;
		
		/// Dumped core.
		const DumpedCore = PF_DUMPCORE;
		
		/// Killed by a signal.
		const KilledByASignal = PF_SIGNALED;
		
		/// Allocating memory.
		const AllocatingMemory = PF_MEMALLOC;
		
		/// `set_user()` noticed that `RLIMIT_NPROC` was exceeded.
		const RLimitNProcExceeded = PF_NPROC_EXCEEDED;
		
		/// If unset the FPU must be initialized before use.
		const UsedFpu = PF_USED_MATH;
		
		/// Used `async_schedule*()`, used by module init.
		const UsedAsyncSchedule = PF_USED_ASYNC;
		
		/// This thread should not be frozen.
		const DoNotFreeze = PF_NOFREEZE;
		
		/// Frozen for system suspend.
		const Frozen = PF_FROZEN;
		
		/// I am `kswapd`.
		const IAmKswapd = PF_KSWAPD;
		
		/// All allocation requests will inherit `GFP_NOFS`.
		const MemoryAllocationInherit_GPF_NOFS = PF_MEMALLOC_NOFS;
		
		/// All allocation requests will inherit `GFP_NOIO`.
		const MemoryAllocationInherit_GPF_NOIO = PF_MEMALLOC_NOIO;
		
		/// Throttle me less: I clean memory.
		const ThrottleMeLess = PF_LESS_THROTTLE;
		
		/// I am a kernel thread.
		const IAmAKthread = PF_KTHREAD;
		
		/// Randomize virtual address space.
		const RandomizeVirtualAddressSpace = PF_RANDOMIZE;
		
		/// Allowed to write to swap.
		const AllowedToSwapOnWrite = PF_SWAPWRITE;
		
		/// I'm an Usermodehelper process.
		const UsermodeHelperProcess = PF_UMH;
		
		/// Userland is not allowed to meddle with cpus_mask.
		const UserLandCanNotChangeSetAffinity = PF_NO_SETAFFINITY;
		
		/// Early kill for mce process policy.
		const MachineCheckExceptionEarlyKill = PF_MCE_EARLY;
		
		/// All allocation request will have `_GFP_MOVABLE` cleared.
		const MemoryAllocationHasCleared_GFP_MOVABLE = PF_MEMALLOC_NOCMA;
		
		/// Task is an IO worker.
		const InputOutputWorker = PF_IO_WORKER;
		
		/// Freezer should not count it as freezable.
		const FreezerSkip = PF_FREEZER_SKIP;
		
		/// This thread called `freeze_processes()` and should not be frozen.
		const SuspendTask = PF_SUSPEND_TASK;
	}
}

impl FromBytes for StatProcessFlags
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error>
	{
		let bits = u32::from_bytes(bytes)?;
		Ok(StatProcessFlags::from_bits_truncate(bits))
	}
}
