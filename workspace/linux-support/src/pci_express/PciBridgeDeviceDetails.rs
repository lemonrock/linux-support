// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// PCI device details if a bridge.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PciBridgeDeviceDetails
{
	/// The highest bus number of all of the busses that can be reached downstream of the bridge.
	#[allow(missing_docs)]
	pub subordinate_bus_number: BusNumber,
	
	/// The bus number immediately downstream of the PCI-PCI Bridge,
	#[allow(missing_docs)]
	pub secondary_bus_number: BusNumber,
}
