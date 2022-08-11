// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Reads the hyper thread and NUMA node of the currently executing CPU from the `IA32_TSC_AUX` model state register, which Linux populates.
///
/// Currently uses the `RDTSCP` instruction, but, once Ice Lake is widely available, could be changed to use the `RDPID` instruction.
///
/// `RDPID` is more modern.
#[inline(always)]
pub fn current_numa_node_and_hyper_thread() -> (NumaNode, HyperThread)
{
	let ia32_tsc_aux = rdpid();
	
	(
		{
			let numa_node_index = ia32_tsc_aux.numa_node_index();
			debug_assert!(numa_node_index < NumaNode::LinuxExclusiveMaximum);
			unsafe { transmute(numa_node_index) }
		},
		{
			let hyper_thread_index = ia32_tsc_aux.hyper_thread_index();
			debug_assert!(hyper_thread_index < HyperThread::LinuxExclusiveMaximum);
			unsafe { transmute(hyper_thread_index) }
		},
	)
}
