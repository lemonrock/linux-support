// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[inline(always)]
fn configure_controller<'name, CC: ControllerConfiguration>(controller_configuration: &Option<CC>, mount_point: &CgroupMountPoint, cgroup: &Rc<NonRootCgroup<'name>>, available_controllers: &Controllers) -> io::Result<()>
{
	if let Some(ref controller_configuration) = controller_configuration
	{
		if available_controllers.contains(&CC::Controller)
		{
			controller_configuration.configure(mount_point, cgroup)?;
		}
	}
	
	Ok(())
}
