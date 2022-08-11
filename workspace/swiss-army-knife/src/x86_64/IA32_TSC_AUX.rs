// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Value of the x86_64 `IA32_TSC_AUX` model-specific register (MSR).
#[repr(transparent)]
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct IA32_TSC_AUX(u32);

impl const From<IA32_TSC_AUX> for u32
{
	#[inline(always)]
	fn from(value: IA32_TSC_AUX) -> Self
	{
		value.0
	}
}

#[cfg(target_os = "linux")]
impl IA32_TSC_AUX
{
	const Twelve: u32 = 12;
	
	const TwelveBitMask: u32 = (1 << Self::Twelve) - 1;
	
	/// Hyper thread index (zero-based).
	///
	/// Actually a 12-bit value (`u12`); maximum value is `2^12 - 1`, ie `4095`.
	#[inline(always)]
	pub const fn hyper_thread_index(self) -> u16
	{
		// Bits 11 to 0 inclusive.
		(self.0 & Self::TwelveBitMask) as u16
	}
	
	/// NUMA node index (zero-based).
	///
	/// Actually a 12-bit value (`u12`); maximum value is `2^12 - 1`, ie `4095`.
	#[inline(always)]
	pub const fn numa_node_index(self) -> u16
	{
		// Bits 21 to 12 inclusive.
		((self.0 >> 12) & Self::TwelveBitMask) as u16
	}
}
