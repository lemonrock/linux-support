// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `pids` controller configuration.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
#[serde(default, deny_unknown_fields)]
pub struct PidsControllerConfiguration
{
	/// Maximum number of process identifiers.
	///
	/// If applied to a threaded cgroup, the maximum number of thread identifiers.
	pub maximum: ProcessIdentifiersMaximum,
}

impl ControllerConfiguration for PidsControllerConfiguration
{
	const Controller: Controller = Controller::pids;
	
	#[inline(always)]
	fn configure<'name>(&self, mount_point: &CgroupMountPoint, cgroup: &Rc<NonRootCgroup<'name>>, _defaults: &DefaultPageSizeAndHugePageSizes) -> io::Result<()>
	{
		cgroup.write_process_identifiers_count_maximum(mount_point, self.maximum)
	}
}
