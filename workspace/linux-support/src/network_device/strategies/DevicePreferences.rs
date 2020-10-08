// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct DevicePreferences
{
	/// Used by the `pfifo_fast` transmit queuing discipline (QDisc).
	pub transmission_queue_length: Option<u32>,
	
	/// Change queue depths.
	pub queue_depths: SetToSpecificValueOrMaximize<(QueueDepth, QueueDepth)>,
	
	pub generic_receive_offload_flush_timeout_in_nanoseconds: u32,
	
	pub counter_to_decrement_before_processing_hard_interrupt_requests: Option<NonZeroU32>,
	
	pub receive_coalescing_preference: ReceiveCoalescingPreference,
	
	pub transmit_coalescing_preference: TransmitCoalescingPreference,

	/// Zero is possible but very silly.
	///
	/// A value such as 2048 is more sensible.
	///
	/// Effectively matches the number of active (as opposed to open) network sockets.
	pub receive_flow_steering_table_count_per_queue: usize,
}
