// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Get settings.
///
/// Please use `ETHTOOL_GLINKSETTINGS`.
#[deprecated]
#[allow(dead_code)]
pub(crate) const ETHTOOL_GSET: u32 = 0x00000001;

/// Set settings.
///
/// Please use `ETHTOOL_SLINKSETTINGS`.
#[deprecated]
#[allow(dead_code)]
pub(crate) const ETHTOOL_SSET: u32 = 0x00000002;

/// Get driver information.
pub(crate) const ETHTOOL_GDRVINFO: u32 = 0x00000003;

/// Get NIC registers.
pub(crate) const ETHTOOL_GREGS: u32 = 0x00000004;

/// Get wake-on-lan options.
pub(crate) const ETHTOOL_GWOL: u32 = 0x00000005;

/// Set wake-on-lan options.
pub(crate) const ETHTOOL_SWOL: u32 = 0x00000006;

/// Get driver message level
pub(crate) const ETHTOOL_GMSGLVL: u32 = 0x00000007;

/// Set driver message level.
pub(crate) const ETHTOOL_SMSGLVL: u32 = 0x00000008;

/// Restart autonegotiation.
#[allow(dead_code)]
pub(crate) const ETHTOOL_NWAY_RST: u32 = 0x00000009;

/// Get link status for host, ie whether the interface *and* the physical port (if there is one) are up (is an `ethtool_value`).
pub(crate) const ETHTOOL_GLINK: u32 = 0x0000000A;

/// Get EEPROM data.
pub(crate) const ETHTOOL_GEEPROM: u32 = 0x0000000B;

/// Set EEPROM data.
#[allow(dead_code)]
pub(crate) const ETHTOOL_SEEPROM: u32 = 0x0000000C;

/// Get coalesce configuration.
pub(crate) const ETHTOOL_GCOALESCE: u32 = 0x0000000E;

/// Set coalesce configuration.
pub(crate) const ETHTOOL_SCOALESCE: u32 = 0x0000000F;

/// Get ring parameters.
pub(crate) const ETHTOOL_GRINGPARAM: u32 = 0x00000010;

/// Set ring parameters.
pub(crate) const ETHTOOL_SRINGPARAM: u32 = 0x00000011;

/// Get pause parameters.
pub(crate) const ETHTOOL_GPAUSEPARAM: u32 = 0x00000012;

/// Set pause parameters.
pub(crate) const ETHTOOL_SPAUSEPARAM: u32 = 0x00000013;

/// Get receive hardware checksum enable (is an `ethtool_value`).
#[allow(dead_code)]
pub(crate) const ETHTOOL_GRXCSUM: u32 = 0x00000014;

/// Set receive hardware checksum enable (is an `ethtool_value`).
#[allow(dead_code)]
pub(crate) const ETHTOOL_SRXCSUM: u32 = 0x00000015;

/// Get transmit hardware checksum enable (is an `ethtool_value`).
#[allow(dead_code)]
pub(crate) const ETHTOOL_GTXCSUM: u32 = 0x00000016;

/// Set transmit hardware checksum enable (is an `ethtool_value`).
#[allow(dead_code)]
pub(crate) const ETHTOOL_STXCSUM: u32 = 0x00000017;

/// Get scatter-gather enable (is an `ethtool_value`).
#[allow(dead_code)]
pub(crate) const ETHTOOL_GSG: u32 = 0x00000018;

/// Set scatter-gather enable (is an `ethtool_value`).
#[allow(dead_code)]
pub(crate) const ETHTOOL_SSG: u32 = 0x00000019;

/// Execute NIC self-test.
#[allow(dead_code)]
pub(crate) const ETHTOOL_TEST: u32 = 0x0000001A;

/// Get specified string set.
pub(crate) const ETHTOOL_GSTRINGS: u32 = 0x0000001B;

/// Identify the NIC.
#[allow(dead_code)]
pub(crate) const ETHTOOL_PHYS_ID: u32 = 0x0000001C;

/// Get NIC-specific statistics.
pub(crate) const ETHTOOL_GSTATS: u32 = 0x0000001D;

/// Get TSO enable (is an `ethtool_value`).
#[allow(dead_code)]
pub(crate) const ETHTOOL_GTSO: u32 = 0x0000001E;

/// Set TSO enable (is an `ethtool_value`).
#[allow(dead_code)]
pub(crate) const ETHTOOL_STSO: u32 = 0x0000001F;

/// Get permanent hardware address.
#[allow(dead_code)]
pub(crate) const ETHTOOL_GPERMADDR: u32 = 0x00000020;

/// Get UFO enable (is an `ethtool_value`).
#[allow(dead_code)]
pub(crate) const ETHTOOL_GUFO: u32 = 0x00000021;

/// Set UFO enable (is an `ethtool_value`).
#[allow(dead_code)]
pub(crate) const ETHTOOL_SUFO: u32 = 0x00000022;

/// Get GSO enable (is an `ethtool_value`).
#[allow(dead_code)]
pub(crate) const ETHTOOL_GGSO: u32 = 0x00000023;

/// Set GSO enable (is an `ethtool_value`).
#[allow(dead_code)]
pub(crate) const ETHTOOL_SGSO: u32 = 0x00000024;

/// Get flags bitmap (is an `ethtool_value`).
#[allow(dead_code)]
pub(crate) const ETHTOOL_GFLAGS: u32 = 0x00000025;

/// Set flags bitmap (is an `ethtool_value`).
#[allow(dead_code)]
pub(crate) const ETHTOOL_SFLAGS: u32 = 0x00000026;

/// Get driver-private flags bitmap.
pub(crate) const ETHTOOL_GPFLAGS: u32 = 0x00000027;

/// Set driver-private flags bitmap.
pub(crate) const ETHTOOL_SPFLAGS: u32 = 0x00000028;

/// Get receive flow hash configuration.
pub(crate) const ETHTOOL_GRXFH: u32 = 0x00000029;

/// Set receive flow hash configuration.
pub(crate) const ETHTOOL_SRXFH: u32 = 0x0000002A;

/// Get GRO enable (is an `ethtool_value`).
#[allow(dead_code)]
pub(crate) const ETHTOOL_GGRO: u32 = 0x0000002B;

/// Set GRO enable (is an `ethtool_value`).
#[allow(dead_code)]
pub(crate) const ETHTOOL_SGRO: u32 = 0x0000002C;

/// Get receive rings available for load-balancing (LB).
pub(crate) const ETHTOOL_GRXRINGS: u32 = 0x0000002D;

/// Get receive class rule count.
///
/// Used by Network Flow Classifier.
#[allow(dead_code)]
pub(crate) const ETHTOOL_GRXCLSRLCNT: u32 = 0x0000002E;

/// Get receive classification rule.
///
/// Used by Network Flow Classifier.
#[allow(dead_code)]
pub(crate) const ETHTOOL_GRXCLSRULE: u32 = 0x0000002F;

/// Get all receive classification rule.
///
/// Used by Network Flow Classifier.
#[allow(dead_code)]
pub(crate) const ETHTOOL_GRXCLSRLALL: u32 = 0x00000030;

/// Delete receive classification rule.
///
/// Used by Network Flow Classifier.
#[allow(dead_code)]
pub(crate) const ETHTOOL_SRXCLSRLDEL: u32 = 0x00000031;

/// Insert receive classification rule.
///
/// Used by Network Flow Classifier.
#[allow(dead_code)]
pub(crate) const ETHTOOL_SRXCLSRLINS: u32 = 0x00000032;

/// Flash firmware to device.
#[allow(dead_code)]
pub(crate) const ETHTOOL_FLASHDEV: u32 = 0x00000033;

/// Reset hardware.
#[allow(dead_code)]
pub(crate) const ETHTOOL_RESET: u32 = 0x00000034;

/// Add an n-tuple filter to device.
///
/// No longer used by any drivers in Linux 5.8.
#[allow(dead_code)]
#[deprecated]
pub(crate) const ETHTOOL_SRXNTUPLE: u32 = 0x00000035;

/// No longer used by any drivers in Linux 5.8.
#[allow(dead_code)]
#[deprecated]
pub(crate) const ETHTOOL_GRXNTUPLE: u32 = 0x00000036;

/// Get string set info.
pub(crate) const ETHTOOL_GSSET_INFO: u32 = 0x00000037;

/// Get receive flow hash indirection table.
pub(crate) const ETHTOOL_GRXFHINDIR: u32 = 0x00000038;

/// Set receive flow hash indirection table.
pub(crate) const ETHTOOL_SRXFHINDIR: u32 = 0x00000039;

/// Get device offload settings.
pub(crate) const ETHTOOL_GFEATURES: u32 = 0x0000003A;

/// Change device offload settings.
pub(crate) const ETHTOOL_SFEATURES: u32 = 0x0000003B;

/// Get number of channels.
pub(crate) const ETHTOOL_GCHANNELS: u32 = 0x0000003C;

/// Set number of channels.
pub(crate) const ETHTOOL_SCHANNELS: u32 = 0x0000003D;

/// Set dump settings.
#[allow(dead_code)]
pub(crate) const ETHTOOL_SET_DUMP: u32 = 0x0000003E;

/// Get dump settings.
pub(crate) const ETHTOOL_GET_DUMP_FLAG: u32 = 0x0000003F;

/// Get dump data.
pub(crate) const ETHTOOL_GET_DUMP_DATA: u32 = 0x00000040;

/// Get time stamping and PHC information.
pub(crate) const ETHTOOL_GET_TS_INFO: u32 = 0x00000041;

/// Get plug-in module information.
pub(crate) const ETHTOOL_GMODULEINFO: u32 = 0x00000042;

/// Get plug-in module eeprom.
pub(crate) const ETHTOOL_GMODULEEEPROM: u32 = 0x00000043;

/// Get Energy Efficient Ethernet (EEE) settings.
pub(crate) const ETHTOOL_GEEE: u32 = 0x00000044;

/// Set Energy Efficient Ethernet (EEE) settings.
pub(crate) const ETHTOOL_SEEE: u32 = 0x00000045;

/// Get receive flow hash configuration.
pub(crate) const ETHTOOL_GRSSH: u32 = 0x00000046;

/// Set receive flow hash configuration.
pub(crate) const ETHTOOL_SRSSH: u32 = 0x00000047;

/// Get tunable configuration.
pub(crate) const ETHTOOL_GTUNABLE: u32 = 0x00000048;

/// Set tunable configuration.
pub(crate) const ETHTOOL_STUNABLE: u32 = 0x00000049;

/// get PHY-specific statistics.
pub(crate) const ETHTOOL_GPHYSTATS: u32 = 0x0000004A;

/// Set per queue options.
pub(crate) const ETHTOOL_PERQUEUE: u32 = 0x0000004B;

/// Get ethtool_link_settings.
#[allow(dead_code)]
pub(crate) const ETHTOOL_GLINKSETTINGS: u32 = 0x0000004C;

/// Set ethtool_link_settings.
#[allow(dead_code)]
pub(crate) const ETHTOOL_SLINKSETTINGS: u32 = 0x0000004D;

/// Get PHY tunable configuration.
pub(crate) const ETHTOOL_PHY_GTUNABLE: u32 = 0x0000004E;

/// Set PHY tunable configuration.
pub(crate) const ETHTOOL_PHY_STUNABLE: u32 = 0x0000004F;

/// Get Forward Error Correction (FEC) settings.
pub(crate) const ETHTOOL_GFECPARAM: u32 = 0x00000050;

/// Set Forward Error Correction (FEC) settings.
pub(crate) const ETHTOOL_SFECPARAM: u32 = 0x00000051;
