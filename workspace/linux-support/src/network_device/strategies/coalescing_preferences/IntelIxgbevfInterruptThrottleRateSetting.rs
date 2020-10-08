// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Also known as `ITR`.
///
/// See `ixgbevf_update_itr`; used in rx_itr_setting and tx_itr_setting
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub enum IntelIxgbevfInterruptThrottleRateSetting
{
	/// Turns off interrupt moderation.
	///
	/// May improve small packet latency.
	/// However, this is generally not suitable for bulk throughput traffic due to the increased CPU utilization of the higher interrupt rate.
	///
	/// Setting this for 82599, X540 and X550 adapters causes "HW RSC" to be disabled.
	Off,

	/// This attempts to moderate interrupts per vector while maintaining very low latency.
	/// This can sometimes cause extra CPU utilization.
	/// Intel says: "If planning on deploying ixgbevf in a latency sensitive environment, this parameter should be considered".
	Dynamic,

	/// This will program the adapter to send at most that many interrupts per second, even if more packets have come in.
	///
	/// This reduces interrupt load on the system and can lower CPU utilization under heavy load, but will increase latency as packets are not processed as quickly.
	Explicit(IntelIxgbevfInterruptThrottleRate)
}

impl Into<u32> for IntelIxgbevfInterruptThrottleRateSetting
{
	#[inline(always)]
	fn into(self) -> u32
	{
		use self::IntelIxgbevfInterruptThrottleRateSetting::*;
		
		match self
		{
			Off => Self::OffValue,
			
			Dynamic => Self::DynamicValue,
			
			Explicit(explicit) => explicit.into(),
		}
	}
}

impl Default for IntelIxgbevfInterruptThrottleRateSetting
{
	#[inline(always)]
	fn default() -> Self
	{
		IntelIxgbevfInterruptThrottleRateSetting::Dynamic
	}
}

impl TryFrom<u32> for IntelIxgbevfInterruptThrottleRateSetting
{
	type Error = ParseNumberError;
	
	#[inline(always)]
	fn try_from(value: u32) -> Result<Self, Self::Error>
	{
		use self::IntelIxgbevfInterruptThrottleRateSetting::*;
		
		match value
		{
			Self::OffValue => Ok(Off),
			
			Self::DynamicValue => Ok(Dynamic),
			
			_ => IntelIxgbevfInterruptThrottleRate::try_from(value).map(Explicit)
		}
	}
}

impl IntelIxgbevfInterruptThrottleRateSetting
{
	const OffValue: u32 = 0;
	
	const DynamicValue: u32 = 1;
}
