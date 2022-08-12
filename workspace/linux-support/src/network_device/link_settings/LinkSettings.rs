// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Link settings.
#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LinkSettings
{
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
	pub speed: SPEED,
	
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
	pub duplex: DUPLEX,
	
	/// Physical connector type.
	pub(crate) port_connector: Option<PortConnector>,
	
	/// `MDIO` address of PHY (transceiver); 0 or 255 if not applicable.
	///
	/// For clause 45 PHYs this is the `PRTAD`.
	pub(crate) phy_address: u8,
	
	/// Enable/disable auto-negotiation and auto-detection.
	pub(crate) auto_negotiation: AUTONEG,
	
	/// Bitmask of `ETH_MDIO_SUPPORTS` flags for the `MDIO` protocols supported by the interface.
	///
	/// The value `ETH_TP_MDI_AUTO` is not valid for this field.
	///
	/// Read-only.
	pub(crate) mdio_support: ETH_MDIO_SUPPORTS,

	pub(crate) we_support: SpeedsPortConnectorsPausesAndForwardErrorConnectionsSettings,

	pub(crate) we_advertise: SpeedsPortConnectorsPausesAndForwardErrorConnectionsSettings,

	pub(crate) our_link_partner_advertises: SpeedsPortConnectorsPausesAndForwardErrorConnectionsSettings,
}
