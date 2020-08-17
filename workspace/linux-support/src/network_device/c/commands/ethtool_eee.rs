// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Energy Efficient Ethernet (EEE) information.
#[repr(C)]
pub(crate) struct ethtool_eee
{
	/// Either `ETHTOOL_GEEE` or `ETHTOOL_SEEE`.
	pub(crate) cmd: u32,
	
	/// Mask of `SUPPORTED_*` flags for the speed/duplex combinations for which there is EEE support.
	///
	/// Can not accommodate newer flags.
	///
	/// Read-only.
	pub(crate) supported: BitSetWord,
	
	/// Mask of `ADVERTISED_*` flags for the speed/duplex combinations advertised as eee capable.
	///
	/// Can not accommodate newer flags.
	///
	/// Read-write.
	pub(crate) advertised: BitSetWord,
	
	/// Mask of `ADVERTISED_*` flags for the speed/duplex combinations advertised by the link partner as eee capable.
	///
	/// Can not accommodate newer flags.
	///
	/// Read-only.
	pub(crate) lp_advertised: BitSetWord,
	
	/// Result of the eee auto negotiation.
	///
	/// A boolean.
	///
	/// Read-only.
	pub(crate) eee_active: u32,
	
	/// EEE configured mode (enabled/disabled).
	///
	/// A boolean.
	///
	/// Read-write.
	pub(crate) eee_enabled: u32,
	
	///  Whether the interface should assert its transmit low power idle (LPI), given that eee was negotiated.
	///
	/// A boolean.
	///
	/// Read-write.
	pub(crate) tx_lpi_enabled: u32,
	
	/// Time in microseconds the interface delays prior to asserting its transmit low power idle (LPI) (after reaching 'idle' state).
	///
	/// Effective only when EEE was negotiated and `tx_lpi_enabled` was set.
	///
	/// Read-write.
	pub(crate) tx_lpi_timer: u32,
	
	pub(crate) reserved: [u32; 2],
}

impl EthtoolCommand for ethtool_eee
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}

impl ethtool_eee
{
	#[inline(always)]
	pub(crate) fn supports(&self, speed: ethtool_link_mode_bit_indices_speed) -> bool
	{
		Self::is_set(self.supported, speed)
	}
	
	#[inline(always)]
	pub(crate) fn set_we_advertise(&mut self, speed: ethtool_link_mode_bit_indices_speed)
	{
		self.advertised |= 1 << (speed as u32)
	}
	
	#[inline(always)]
	pub(crate) fn we_advertise(&self, speed: ethtool_link_mode_bit_indices_speed) -> bool
	{
		Self::is_set(self.advertised, speed)
	}
	
	#[inline(always)]
	pub(crate) fn link_partner_advertises(&self, speed: ethtool_link_mode_bit_indices_speed) -> bool
	{
		Self::is_set(self.lp_advertised, speed)
	}
	
	#[inline(always)]
	pub(crate) fn is_active(&self) -> bool
	{
		self.eee_active != 0
	}
	
	#[inline(always)]
	pub(crate) fn is_enabled(&self) -> bool
	{
		self.eee_enabled != 0
	}
	
	#[inline(always)]
	pub(crate) fn transmit_low_power_idle_microseconds(&self) -> Option<u32>
	{
		if self.tx_lpi_enabled == 0
		{
			None
		}
		else
		{
			Some(self.tx_lpi_timer)
		}
	}
	
	#[inline(always)]
	fn is_set(bit_set_word: BitSetWord, speed: ethtool_link_mode_bit_indices_speed) -> bool
	{
		let speed = speed as u32;
		if speed > 31
		{
			return false
		}
		
		(bit_set_word & (1 << speed)) != 0
	}
}
