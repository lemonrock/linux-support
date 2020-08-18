// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


pub(super) trait ControllersConfiguration
{
	fn configure<'name>(&self, mount_point: &CgroupMountPoint, cgroup: &Rc<NonRootCgroup<'name>>, available_controllers: &Controllers, defaults: &DefaultPageSizeAndHugePageSizes) -> io::Result<()>;
	
	fn to_desired_controllers(&self) -> Controllers;
}
