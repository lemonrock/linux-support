// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_drvinfo
{
	pub cmd: u32,
	pub driver: [c_char; 32],
	pub version: [c_char; 32],
	pub fw_version: [c_char; ETHTOOL_FWVERS_LEN],
	pub bus_info: [c_char; ETHTOOL_BUSINFO_LEN],
	pub erom_version: [c_char; ETHTOOL_EROMVERS_LEN],
	pub reserved2: [c_char; 12],
	pub n_priv_flags: u32,
	pub n_stats: u32,
	pub testinfo_len: u32,
	pub eedump_len: u32,
	pub regdump_len: u32,
}

impl Default for ethtool_drvinfo
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
