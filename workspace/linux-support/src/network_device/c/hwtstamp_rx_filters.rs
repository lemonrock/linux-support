// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Strings are in the `ethtool_stringset::ETH_SS_TS_RX_FILTERS` string set.
///
/// Used in `hwtstamp_config.rx_filter`.
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum hwtstamp_rx_filters
{
	/// Timestamp no incoming packet at all.
	///
	/// String set value is `none`.
	HWTSTAMP_FILTER_NONE = 0,
	
	/// Timestamp any incoming packet.
	///
	/// String set value is `all`.
	HWTSTAMP_FILTER_ALL = 1,
	
	/// Return value: timestamp all packets requested plus some others.
	///
	/// String set value is `some`.
	HWTSTAMP_FILTER_SOME = 2,
	
	/// Precision Time Protocol (PTP) v1, UDP, any kind of event packet.
	///
	/// String set value is `ptpv1-l4-event`.
	HWTSTAMP_FILTER_PTP_V1_L4_EVENT = 3,
	
	/// Precision Time Protocol (PTP) v1, UDP, Sync packet.
	///
	/// String set value is `ptpv1-l4-sync`.
	HWTSTAMP_FILTER_PTP_V1_L4_SYNC = 4,
	
	/// Precision Time Protocol (PTP) v1, UDP, Delay_req packet.
	///
	/// String set value is `ptpv1-l4-delay-req`.
	HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ = 5,
	
	/// Precision Time Protocol (PTP) v2, UDP, any kind of event packet.
	///
	/// String set value is `ptpv2-l4-event`.
	HWTSTAMP_FILTER_PTP_V2_L4_EVENT = 6,
	
	/// Precision Time Protocol (PTP) v2, UDP, Sync packet.
	///
	/// String set value is `ptpv2-l4-sync`.
	HWTSTAMP_FILTER_PTP_V2_L4_SYNC = 7,
	
	/// Precision Time Protocol (PTP) v2, UDP, Delay_req packet.
	///
	/// String set value is `ptpv2-l4-delay-req`.
	HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ = 8,
	
	/// 802.AS1, Ethernet, any kind of event packet.
	///
	/// String set value is `ptpv2-l2-event`.
	HWTSTAMP_FILTER_PTP_V2_L2_EVENT = 9,
	
	/// 802.AS1, Ethernet, Sync packet.
	///
	/// String set value is `ptpv2-l2-sync`.
	HWTSTAMP_FILTER_PTP_V2_L2_SYNC = 10,
	
	/// 802.AS1, Ethernet, Delay_req packet.
	///
	/// String set value is `ptpv2-l2-delay-req`.
	HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ = 11,
	
	/// Precision Time Protocol (PTP) v2 or 802.AS1, any layer, any kind of event packet.
	///
	/// String set value is `ptpv2-event`.
	HWTSTAMP_FILTER_PTP_V2_EVENT = 12,
	
	/// Precision Time Protocol (PTP) v2 or 802.AS1, any layer, Sync packet.
	///
	/// String set value is `ptpv2-sync`.
	HWTSTAMP_FILTER_PTP_V2_SYNC = 13,
	
	/// Precision Time Protocol (PTP) v2 or 802.AS1, any layer, Delay_req packet.
	///
	/// String set value is `ptpv2-delay-req`.
	HWTSTAMP_FILTER_PTP_V2_DELAY_REQ = 14,
	
	/// NTP, UDP, all versions and packet modes.
	///
	/// String set value is `ntp-all`.
	HWTSTAMP_FILTER_NTP_ALL = 15,
}

impl hwtstamp_rx_filters
{
	const __HWTSTAMP_FILTER_CNT: usize = 16;
}
