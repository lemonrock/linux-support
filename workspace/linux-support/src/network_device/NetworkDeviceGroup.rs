// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// 'net device group'.
///
/// All network devices belong to group 0 (`INIT_NETDEV_GROUP`) by default.
///
/// Also available at `/sys/class/net/<network_interface_name>/netdev_group`.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NetworkDeviceGroup(pub u32);

impl NetworkDeviceGroup
{
	/// All network devices belong to group 0 (`INIT_NETDEV_GROUP`) by default.
	pub const Initial: Self = Self(0);
}
