// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A version 2 root cgroup.
///
/// See <https://www.kernel.org/doc/Documentation/cgroup-v2.txt>.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RootCgroup;

impl Cgroup for RootCgroup
{
	#[inline(always)]
	fn to_path<'b>(&self, mount_point: &'b CgroupMountPoint) -> Cow<'b, Path>
	{
		Cow::Borrowed(mount_point.to_path())
	}
	
	/// Does not check if the child exists.
	#[inline(always)]
	fn child(self: Rc<Self>, name: CgroupName) -> Rc<NonRootCgroup>
	{
		Rc::new(NonRootCgroup::ChildOfRoot { name })
	}
}
