// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[doc(hidden)]
pub trait CgroupConfigurationVariant: Default
{
	#[doc(hidden)]
	fn configure(&self, mount_point: &CgroupMountPoint, cgroup: Rc<impl Cgroup>) -> io::Result<()>;
	
	#[doc(hidden)]
	fn desired_controllers_and_our_depth(&self, mut all_desired_controllers_in_parent: Controllers) -> (Controllers, usize);
	
	#[doc(hidden)]
	fn make_type_threaded_if_needed(mount_point: &CgroupMountPoint, cgroup: &Rc<impl Cgroup>) -> io::Result<()>;
}
