// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Link control and status.
///
/// Equivalent to Linux kernel's private `ethtool_link_usettings` (note the `u` in front of `settings`).
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub(crate) struct ethtool_link_settings
{
	/// Always either `ETHTOOL_GLINKSETTINGS` or `ETHTOOL_SLINKSETTINGS`.
	///
	/// Can be set to zero by the kernel after a call to `ETHTOOL_GLINKSETTINGS`; see documentation of `link_mode_masks_nwords` below.
	pub(crate) cmd: u32,

	/// Link speed (Mbps).
	///
	/// If the link is down, may be:-
	///
	/// * `0`;
	/// * `SPEED_UNKNOWN`;
	/// * Highest-enabled;
	///
	/// Read-only if `autoneg` is `AUTONEG_ENABLE`.
	/// Writable if `autoneg` is `AUTONEG_DISABLE` and the driver supports multiple speeds (link modes).
	///
	/// Also available at `/sys/class/net/<network_interface_name>/speed`, as raw `i32` value of `SPEED` (eg `-1` for `SPEED_UNKNOWN`).
	/// Read-only is sysfs.
	/// Unknown for virtual interfaces.
	pub(crate) speed: SPEED,

	/// Duplex mode; one of enum `DUPLEX`.
	///
	/// If the link is down, may be:-
	///
	/// * `DUPLEX_UNKNOWN`;
	/// * Highest-enabled;
	///
	/// Read-only if `autoneg` is `AUTONEG_ENABLE`.
	/// Writable if `autoneg` is `AUTONEG_DISABLE` and the driver supports multiple duplexes (link modes).
	///
	/// Also available at `/sys/class/net/<network_interface_name>/duplex`, but turned into a string.
	/// Read-only is sysfs.
	/// Unknown for virtual interfaces.
	///
	/// String representations in sysfs are:-
	///
	/// * `half`.
	/// * `full`.
	/// * `unknown`.
	pub(crate) duplex: DUPLEX,

	/// Physical connector type; one of the enum `PORT`.
	///
	/// May be writable if multiple PHYs or physical connectors are fitted or the driver does detect if multiple PHYs or physical connectors are fitted, especially if `autoneg` is `AUTONEG_DISABLE`.
	pub(crate) port: PORT,

	/// `MDIO` address of PHY (transceiver); 0 or 255 if not applicable.
	///
	/// For clause 45 PHYs this is the `PRTAD`.
	///
	/// May be writable if multiple PHYs or physical connectors are fitted or the driver does detect if multiple PHYs or physical connectors are fitted, especially if `autoneg` is `AUTONEG_DISABLE`.
	pub(crate) phy_address: u8,

	/// Enable/disable autonegotiation and auto-detection.
	///
	/// Either `AUTONEG_DISABLE` or `AUTONEG_ENABLE`.
	pub(crate) autoneg: AUTONEG,

	/// Bitmask of `ETH_MDIO_SUPPORTS` flags for the `MDIO` protocols supported by the interface.
	///
	/// The value `ETH_TP_MDI_AUTO` is not valid for this field.
	///
	/// Read-only.
	pub(crate) mdio_support: ETH_MDIO_SUPPORTS,

	/// Ethernet twisted-pair MDI(-X) status; one of enum `ETH_TP_MDI`.
	///
	/// If the status is unknown or not applicable, the value will be `ETH_TP_MDI::ETH_TP_MDI_INVALID`.
	///
	/// Read-only.
	pub(crate) eth_tp_mdix: ETH_TP_MDI,

	/// Ethernet twisted pair MDI(-X) control; one of enum `ETH_TP_MDI`.
	///
	/// If MDI(-X) control is not implemented, reads yield `ETH_TP_MDI::ETH_TP_MDI_INVALID` and writes may be ignored or rejected.
	/// When written successfully, the link should be renegotiated if necessary.
	pub(crate) eth_tp_mdix_ctrl: ETH_TP_MDI,

	/// Number of 32-bit words for each of the `supported`, `advertising` and `lp_advertising` link mode bitmaps.
	///
	/// Usually `LinkModeBitSet::__ETHTOOL_LINK_MODE_MASK_NU32 as i8`.
	///
	/// For the `ETHTOOL_GLINKSETTINGS` command: on entry, number of words passed by user (>= 0); on return, if handshake in progress, negative if request size unsupported by kernel: absolute value indicates kernel expected size and all the other fields but `cmd` are 0; otherwise (handshake completed), strictly positive to indicate size used by kernel and `cmd` field stays `ETHTOOL_GLINKSETTINGS`, all other fields populated by driver.
	///
	/// For the `ETHTOOL_SLINKSETTINGS` command: must be valid on entry, ie a positive value returned previously by `ETHTOOL_GLINKSETTINGS`, otherwise refused.
	pub(crate) link_mode_masks_nwords: i8,

	/// Used to distinguish different possible PHY types, reported consistently by PHYLIB.
	///
	/// Read-only.
	#[deprecated]
	pub(crate) transceiver: XCVR,

	reserved1: [u8; 3],

	reserved: [u32; 7],
	
	/// Read-only.
	///
	/// In theory, variably sized; in practice, fixed but not necessarily fully-populated.
	link_mode_masks: [u32; 3 * LinkModeBitSet::__ETHTOOL_LINK_MODE_MASK_NU32],
}

impl EthtoolCommand for ethtool_link_settings
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}

impl ethtool_link_settings
{
	#[inline(always)]
	pub(crate) fn to_get_link_mode_masks_nwords() -> Self
	{
		let mut this: Self = unsafe_zeroed();
		this.cmd = ETHTOOL_GLINKSETTINGS;
		this
	}
	
	#[inline(always)]
	pub(crate) fn to_get(negative_link_mode_masks_nwords_from_to_get_link_mode_masks_nwords: i8) -> Self
	{
		let mut this: Self = unsafe_zeroed();
		this.cmd = ETHTOOL_GLINKSETTINGS;
		this.link_mode_masks_nwords = -negative_link_mode_masks_nwords_from_to_get_link_mode_masks_nwords;
		this
	}
	
	/// `supported` is a bit set with each bit meaning given by `ethtool_link_mode_bit_indices` for the link modes, physical connectors and other link features for which the interface supports autonegotiation or auto-detection.
	///
	/// Read-only.
	pub(crate) fn supported(&self) -> LinkModeBitSet
	{
		self.link_mode_bit_set(0)
	}
	
	/// `advertising` is a bit set with each bit meaning given by `ethtool_link_mode_bit_indices` for the link modes, physical connectors and other link features that are advertised through autonegotiation or enabled for auto-detection.
	///
	/// Read-write.
	pub(crate) fn advertising(&self) -> LinkModeBitSet
	{
		self.link_mode_bit_set(1)
	}
	
	/// `lp_advertising` is a bit set with each bit meaning given by `ethtool_link_mode_bit_indices` for the link modes, physical connectors and other link features that the link partner advertised through autonegotiation.
	///
	/// Read-only.
	pub(crate) fn lp_advertising(&self) -> LinkModeBitSet
	{
		self.link_mode_bit_set(2)
	}
	
	#[inline(always)]
	fn link_mode_bit_set(&self, field_index: usize) -> LinkModeBitSet
	{
		debug_assert!(self.link_mode_masks_nwords >= 0);
		
		let length = self.link_mode_masks_nwords as usize;
		debug_assert!(length <= LinkModeBitSet::__ETHTOOL_LINK_MODE_MASK_NU32);
		
		let offset = field_index * length;
		
		let mut link_mode_bit_set_array: [BitSetWord; LinkModeBitSet::__ETHTOOL_LINK_MODE_MASK_NU32] = unsafe_uninitialized();
		unsafe
		{
			link_mode_bit_set_array.as_mut_ptr().copy_from_nonoverlapping(self.link_mode_masks.as_ptr().add(offset), length);
			link_mode_bit_set_array.as_mut_ptr().add(length).write_bytes(0x00, LinkModeBitSet::__ETHTOOL_LINK_MODE_MASK_NU32 - length);
		}
		LinkModeBitSet(link_mode_bit_set_array)
	}
}
