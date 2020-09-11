// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Strings are in the `ethtool_stringset::ETH_SS_SOF_TIMESTAMPING` string set.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[derive(EnumIter, EnumCount)]
#[repr(u32)]
pub enum SOF_TIMESTAMPING
{
	/// String set value is `hardware-transmit`.
	#[serde(rename = "hardware-transmit")] SOF_TIMESTAMPING_TX_HARDWARE = 1 << 0,
	
	/// String set value is `software-transmit`.
	#[serde(rename = "software-transmit")] SOF_TIMESTAMPING_TX_SOFTWARE = 1 << 1,
	
	/// String set value is `hardware-receive`.
	#[serde(rename = "hardware-receive")] SOF_TIMESTAMPING_RX_HARDWARE = 1 << 2,
	
	/// String set value is `software-receive`.
	#[serde(rename = "software-receive")] SOF_TIMESTAMPING_RX_SOFTWARE = 1 << 3,
	
	/// String set value is `software-system-clock`.
	#[serde(rename = "software-system-clock")] SOF_TIMESTAMPING_SOFTWARE = 1 << 4,
	
	/// String set value is `hardware-legacy-clock`.
	#[serde(rename = "hardware-legacy-clock")] SOF_TIMESTAMPING_SYS_HARDWARE = 1 << 5,
	
	/// String set value is `hardware-raw-clock`.
	#[serde(rename = "hardware-raw-clock")] SOF_TIMESTAMPING_RAW_HARDWARE = 1 << 6,
	
	/// String set value is `option-id`.
	#[serde(rename = "option-id")] SOF_TIMESTAMPING_OPT_ID = 1 << 7,
	
	/// String set value is `sched-transmit`.
	#[serde(rename = "sched-transmit")] SOF_TIMESTAMPING_TX_SCHED = 1 << 8,
	
	/// String set value is `ack-transmit`.
	#[serde(rename = "ack-transmit")] SOF_TIMESTAMPING_TX_ACK = 1 << 9,
	
	/// String set value is `option-cmsg`.
	#[serde(rename = "option-cmsg")] SOF_TIMESTAMPING_OPT_CMSG = 1 << 10,
	
	/// String set value is `option-tsonly`.
	#[serde(rename = "option-tsonly")] SOF_TIMESTAMPING_OPT_TSONLY = 1 << 11,
	
	/// String set value is `option-stats`.
	#[serde(rename = "option-stats")] SOF_TIMESTAMPING_OPT_STATS = 1 << 12,
	
	/// String set value is `option-pktinfo`.
	#[serde(rename = "option-pktinfo")] SOF_TIMESTAMPING_OPT_PKTINFO = 1 << 13,
	
	/// String set value is `option-tx-swhw`.
	#[serde(rename = "option-tx-swhw")] SOF_TIMESTAMPING_OPT_TX_SWHW = 1 << 14,
}

impl Into<u32> for SOF_TIMESTAMPING
{
	#[inline(always)]
	fn into(self) -> u32
	{
		self as u32
	}
}

impl SOF_TIMESTAMPING
{
	#[allow(dead_code)]
	const SOF_TIMESTAMPING_LAST: u32 = SOF_TIMESTAMPING::SOF_TIMESTAMPING_OPT_TX_SWHW as u32;
	
	#[allow(dead_code)]
	const SOF_TIMESTAMPING_MASK: u32 = (Self::SOF_TIMESTAMPING_LAST - 1) | Self::SOF_TIMESTAMPING_LAST;
	
	/// `SO_TIMESTAMPING` (sic) flags are either for recording a packet timestamp or for reporting the timestamp to user space.
	/// Recording flags can be set both via socket options and control messages.
	#[allow(dead_code)]
	const SOF_TIMESTAMPING_TX_RECORD_MASK: u32 = SOF_TIMESTAMPING::SOF_TIMESTAMPING_TX_HARDWARE as u32 | SOF_TIMESTAMPING::SOF_TIMESTAMPING_TX_SOFTWARE as u32 | SOF_TIMESTAMPING::SOF_TIMESTAMPING_TX_SCHED as u32  | SOF_TIMESTAMPING::SOF_TIMESTAMPING_TX_ACK as u32;
}
