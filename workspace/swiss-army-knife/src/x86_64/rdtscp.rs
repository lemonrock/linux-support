// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Returns processor timestamp and the value of the `IA32_TSC_AUX` MSR.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub fn rdtscp() -> (ProcessorTimestampCounter, IA32_TSC_AUX)
{
	#[cfg(target_arch = "x86_64")]
	
	let mut IA32_TSC_AUX_MSR = MaybeUninit::uninit();
	let processor_timestamp_counter = unsafe { __rdtscp(IA32_TSC_AUX_MSR.as_mut_ptr()) };
	(processor_timestamp_counter, IA32_TSC_AUX(unsafe { IA32_TSC_AUX_MSR.assume_init() }))
}
