// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Transmit queue traffic class.
///
/// Only exists for multiqueue network devices.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct TransmitQueueTrafficClass
{
	/// If `subordinate_device.is_some()` this is the traffic class of the subordinate device.
	///
	/// A value of `0` is commonly displayed as `TC0`.
	pub traffic_class: u32,

	/// Subordinate device.
	pub subordinate_device: Option<u32>,
}
