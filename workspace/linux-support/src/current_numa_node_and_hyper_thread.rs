// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


use swiss_army_knife::x86_64::{rdpid, rdtscp};

/// Reads the hyper thread and NUMA node of the currently executing CPU from the `IA32_TSC_AUX` model state register, which Linux populates.
///
/// Currently uses the `RDTSCP` instruction, but, once Ice Lake is widely available, could be changed to use the `RDPID` instruction.
///
/// `RDPID` is more modern.
#[inline(always)]
pub fn current_numa_node_and_hyper_thread() -> (NumaNode, HyperThread)
{
	if cfg!(feature = "rdpid")
	{
		rdpid()
	}
	else
	{
		rdtscp().1
	};
	/*
		RDPID support is for
			AMD
				Zen 2
			Intel
				Ice Lake	(Sunny Cove)
				Comet Lake
				Tiger Lake
		
		is_x86_feature_detected
		
		target feature is tsc but not rdpid
		
	 */
	
	
	let IA32_TSC_AUX_MSR =
	{
		let mut IA32_TSC_AUX_MSR = MaybeUninit::uninit();
		let _processor_timestamp_counter = unsafe { __rdtscp(IA32_TSC_AUX_MSR.as_mut_ptr()) };
		unsafe { IA32_TSC_AUX_MSR.assume_init() }
	};
	
	const Twelve: u32 = 12;
	const TwelveBitMask: u32 = (1 << Twelve) - 1;
	
	// Bits 21 to 12 inclusive.
	let numa_node = (IA32_TSC_AUX_MSR >> 12) & TwelveBitMask;
	debug_assert!(numa_node < NumaNode::LinuxMaximum);
	
	// Bits 11 to 0 inclusive.
	let hyper_thread = IA32_TSC_AUX_MSR & TwelveBitMask;
	debug_assert!(hyper_thread < HyperThread::LinuxMaximum);

	(unsafe { transmute(numa_node as u16) }, unsafe { transmute(hyper_thread as u16) })
}
