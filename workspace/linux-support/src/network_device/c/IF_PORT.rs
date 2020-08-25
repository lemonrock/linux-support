// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// ?Legacy? media selection option.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[repr(u32)]
pub enum IF_PORT
{
	/// Unknown.
	///
	/// Default.
	#[serde(rename = "Unknown")] IF_PORT_UNKNOWN = 0,
	
	/// `10BASE-2`.
	#[serde(rename = "10BASE-2")] IF_PORT_10BASE2 = 1,
	
	/// `10BASE-T`.
	#[serde(rename = "10BASE-T")] IF_PORT_10BASET = 2,
	
	/// `AUI`.
	#[serde(rename = "AUI")] IF_PORT_AUI = 3,
	
	/// `100BASE-T`.
	#[serde(rename = "100BASE-T")] IF_PORT_100BASET = 4,
	
	/// `100BASE-TX`.
	#[serde(rename = "100BASE-TX")] IF_PORT_100BASETX = 5,
	
	/// `100BASE-FX`.
	#[serde(rename = "100BASE-FX")] IF_PORT_100BASEFX = 6,
}

impl Default for IF_PORT
{
	#[inline(always)]
	fn default() -> Self
	{
		IF_PORT::IF_PORT_UNKNOWN
	}
}
