// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// `rdma` controller configuration.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields, default)]
pub struct RdmaControllerConfiguration
{
	#[serde(flatten)] pub maximum: RdmaFile,
}

impl ControllerConfiguration for RdmaControllerConfiguration
{
	const Controller: Controller = Controller::rdma;
	
	#[inline(always)]
	fn configure(&self, mount_point: &CgroupMountPoint, cgroup: &Rc<NonRootCgroup>, _defaults: &DefaultPageSizeAndHugePageSizes) -> io::Result<()>
	{
		cgroup.write_rdma_maximum(mount_point, &self.maximum)
	}
}
