// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Reads the hyper thread and NUMA node of the currently executing CPU from the `IA32_TSC_AUX` model state register, which Linux populates.
///
/// Currently uses the `RDTSCP` instruction, but, once Ice Lake is widely available, could be changed to use the `RDPID` instruction.
#[inline(always)]
pub fn current_numa_node_and_hyper_thread() -> (NumaNode, HyperThread)
{
	// The value of the timestamp register is stored into the `RDX` and `RAX` registers.
	// The value of the hyper thread and NUMA node is stored into the `RCX` register.
	// The top 32-bits of `RDX`, `RAX` and `RCX` are cleared (zero).
	#[inline(always)]
	unsafe fn rdtscp() -> u64
	{
		let _rax: u64;
		let _rdx: u64;
		let rcx: u64;

		asm!
		(
			"rdtscp"
			:
				"={rax}"(_rax), "={rdx}"(_rdx), "={rcx}"(rcx)
			:
			:
			:
				"volatile"
		);

		rcx
	}
	let rcx = unsafe { rdtscp() };

	let numa_node = (rcx & 0x00000000_0FFFF000) >> 12;
	debug_assert!(numa_node <= u16::MAX as u64);
	debug_assert!(numa_node < NumaNode::LinuxMaximum as u64);

	let hyper_thread = rcx & 0x00000000_00000FFF;
	debug_assert!(hyper_thread <= u16::MAX as u64);
	debug_assert!(hyper_thread < HyperThread::LinuxMaximum as u64);

	(unsafe { transmute(numa_node as u16) }, unsafe { transmute(hyper_thread as u16) })
}
