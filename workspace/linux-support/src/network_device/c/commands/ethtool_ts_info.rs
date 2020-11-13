// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
pub(crate) struct ethtool_ts_info
{
	/// Always `ETHTOOL_GET_TS_INFO`.
	cmd: u32,

	/// Bit mask of the sum of the supported `SO_TIMESTAMPING` flags.
	pub(crate) so_timestamping: BitSetWord,
	
	/// Index of the associated Precision Time Protocol (PTP) Hardware Clock (PHC) index -1 if there isn't one.
	pub(crate) phc_index: i32,

	/// Bit mask of the supported `hwtstamp_tx_types` enumeration values.
	pub(crate) tx_types: BitSetWord,
	
	tx_reserved: [u32; 3],
	
	/// Bit mask of the supported `hwtstamp_rx_filters` enumeration values.
	pub(crate) rx_filters: BitSetWord,
	
	rx_reserved: [u32; 3],
}

impl EthtoolCommand for ethtool_ts_info
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}

impl ethtool_ts_info
{
	#[inline(always)]
	pub(crate) fn get() -> Self
	{
		Self
		{
			cmd: ETHTOOL_GET_TS_INFO,
			so_timestamping: 0,
			phc_index: 0,
			tx_types: 0,
			tx_reserved: unsafe_zeroed(),
			rx_filters: 0,
			rx_reserved: unsafe_zeroed(),
		}
	}
	
	pub(crate) fn capabilities(&self) -> HashSet<SOF_TIMESTAMPING>
	{
		Self::to_set(self.so_timestamping)
	}
	
	#[inline(always)]
	pub(crate) fn precision_time_protocol_hardware_clock_index(&self) -> Option<u32>
	{
		match self.phc_index
		{
			-1 => None,
			index @ _ => Some(index as u32),
		}
	}
	
	pub(crate) fn hardware_transmit_timestamp_modes(&self) -> HashSet<hwtstamp_tx_types>
	{
		Self::to_set(self.tx_types)
	}
	
	pub(crate) fn hardware_receive_filter_modes(&self) -> HashSet<hwtstamp_rx_filters>
	{
		Self::to_set(self.rx_filters)
	}
	
	#[inline(always)]
	pub(crate) fn to_set<E: Into<u32> + IntoEnumIterator + EnumCount + Eq + Hash + Copy>(bit_set_word: BitSetWord) -> HashSet<E>
	{
		let mut set = HashSet::with_capacity(E::COUNT);
		for e in E::iter()
		{
			if bit_set_word & (1 << e.into()) != 0
			{
				set.insert(e);
			}
		}
		set.shrink_to_fit();
		set
	}
}
