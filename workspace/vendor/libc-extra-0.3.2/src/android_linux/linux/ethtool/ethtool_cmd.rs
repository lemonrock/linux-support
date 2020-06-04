// This file is part of libc-extra. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT. No part of libc-extra, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016-2018 The developers of libc-extra. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/libc-extra/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ethtool_cmd
{
	pub cmd: u32,
	pub supported: u32,
	pub advertising: u32,
	pub speed: u16,
	pub duplex: u8,
	pub port: u8,
	pub phy_address: u8,
	pub transceiver: u8,
	pub autoneg: u8,
	pub mdio_support: u8,
	pub maxtxpkt: u32,
	pub maxrxpkt: u32,
	pub speed_hi: u16,
	pub eth_tp_mdix: u8,
	pub eth_tp_mdix_ctrl: u8,
	pub lp_advertising: u32,
	pub reserved: [u32; 2],
}

impl Default for ethtool_cmd
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
