// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Transmit coalescing preference.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct TransmitCoalescingPreference
{
	/// Preferred microseconds interval.
	///
	/// Used by Amazon ENA.
	/// Can be any possible value.
	pub preferred_micoseconds_interval: u32,
	
	/// Interrupt Throttle Rate (ITR) setting.
	///
	/// Use by Intel ixgbevf but only if not using paired receive-transmit queues.
	pub interrupt_throttle_rate_setting: IntelIxgbevfInterruptThrottleRateSetting,
	
	/// NAPI poll weighting.
	///
	/// Used by virtio net.
	pub enable_napi_weight: bool,
}
