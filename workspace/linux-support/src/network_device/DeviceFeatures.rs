// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Device features.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeviceFeatures
{
	/// Available.
	pub available: HashSet<NETIF_F>,
	
	/// Requested.
	pub requested: HashSet<NETIF_F>,
	
	/// Active.
	pub active: HashSet<NETIF_F>,
	
	/// Never change.
	pub never_changed: HashSet<NETIF_F>,
}
