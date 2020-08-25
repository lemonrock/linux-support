// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Very few drivers support firmware dumping:-
///.
/// * QLogic `qlcnic`.
/// * Broadcom `bnxt`.
/// * Broadcom `bnx2x`.
/// * Chelsio `cxgb4`.
/// * Emulex `benet`.
/// * Fujitsu Extended Socket `fjes`.
/// * Netronome `nfp`.
/// * QLogic `qede`.
/// * QLogic `netxen`
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct NetworkDeviceFirmware
{
	/// Device driver version.
	///
	/// If `Some` will not be empty.
	pub device_version: Option<ObjectName32>,
	
	/// Driver specific flags for firmware dump.
	///
	/// If `ETH_FW_DUMP_DISABLE` (`0`) then firmware dumping is disabled.
	pub driver_specific_flags: u32,
	
	/// Firmware type (structure or layout) version.
	pub version: u32,
	
	/// Firmware binary data.
	pub binary_data: Option<Box<[u8]>>,
}
