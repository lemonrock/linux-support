// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[derive(Debug)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkDeviceTimestampingInformation
{
	/// `SOF_TIMESTAMPING` flags.
	pub capabilities: HashSet<SOF_TIMESTAMPING>,
	
	/// Precision Time Protocol (PTP) Hardware Clock (PHC) index.
	pub precision_time_protocol_hardware_clock_index: Option<u32>,
	
	/// Transmission types.
	pub hardware_transmit_timestamp_modes: HashSet<hwtstamp_tx_types>,
	
	/// Receive filters.
	pub hardware_receive_filter_modes: HashSet<hwtstamp_rx_filters>,
}
