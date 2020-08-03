// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// General driver and device information.
///
/// Users can use the `ETHTOOL_GSSET_INFO` command to get the number ofstrings in any of the string sets (`SS`):-
///
/// * `ethtool_stringset::ETH_SS_PRIV_FLAGS`.
/// * `ethtool_stringset::ETH_SS_STATS`.
/// * `ethtool_stringset::ETH_SS_TEST`.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub(crate) struct ethtool_drvinfo
{
	/// Always `ETHTOOL_GDRVINFO`.
	pub(crate) cmd: u32,
	
	/// Driver short name.
	///
	/// Must not be an empty string.
	pub(crate) driver: [c_char; 32],
	
	/// Driver version.
	///
	/// Can be an empty string.
	pub(crate) version: [c_char; 32],
	
	/// Firmware version.
	///
	/// Can be an empty string.
	pub(crate) fw_version: [c_char; ETHTOOL_FWVERS_LEN],
	
	/// Device bus address.
	///
	/// Can be an empty string.
	pub(crate) bus_info: [c_char; ETHTOOL_BUSINFO_LEN],
	
	/// Expansion ROM version string.
	///
	/// Can be an empty string.
	pub(crate) erom_version: [c_char; ETHTOOL_EROMVERS_LEN],
	
	reserved2: [c_char; 12],
	
	/// Number of flags valid for `ETHTOOL_GPFLAGS` and `ETHTOOL_SPFLAGS` commands; also the number of strings in the `ethtool_stringset::ETH_SS_PRIV_FLAGS` string set.
	///
	/// If zero then the `ETHTOOL_GPFLAGS` and `ETHTOOL_SPFLAGS` commands are unsupported and the `ethtool_stringset::ETH_SS_PRIV_FLAGS` string set is unsupported.
	///
	/// Use the command `ETHTOOL_GSSET_INFO` instead.
	pub(crate) n_priv_flags: u32,
	
	/// Number of u64 statistics returned by the `ETHTOOL_GSTATS` command; also the number of strings in the `ethtool_stringset::ETH_SS_STATS` string set.
	///
	/// If zero then the `ETHTOOL_GSTATS` command is unsupported and the `ethtool_stringset::ETH_SS_STATS` string set is unsupported.
	///
	/// Use the command `ETHTOOL_GSSET_INFO` instead.
	pub(crate) n_stats: u32,

	/// Number of results returned by the `ETHTOOL_TEST` command; also the number of strings in the `ethtool_stringset::ETH_SS_TEST` string set.
	///
	/// If zero then the `ETHTOOL_TEST` command is unsupported and the `ethtool_stringset::ETH_SS_TEST` string set is unsupported.
	///
	/// Use the command `ETHTOOL_GSSET_INFO` instead.
	#[deprecated]
	pub(crate) testinfo_len: u32,

	/// Size of EEPROM accessible through the `ETHTOOL_GEEPROM` and `ETHTOOL_SEEPROM` commands, in bytes.
	///
	/// If zero then the `ETHTOOL_GEEPROM` and `ETHTOOL_SEEPROM` commands are unsupported.
	pub(crate) eedump_len: u32,

	/// Size of register dump returned by the `ETHTOOL_GREGS` command, in bytes.
	///
	/// If zero then the `ETHTOOL_GREGS` command is unsupported.
	pub(crate) regdump_len: u32,
}

impl EthtoolCommand for ethtool_drvinfo
{
	#[inline(always)]
	fn command(&self) -> u32
	{
		self.cmd
	}
}
