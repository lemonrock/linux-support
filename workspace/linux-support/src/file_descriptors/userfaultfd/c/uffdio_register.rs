// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2021 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub(super) struct uffdio_register
{
	pub(super) range: uffdio_range,
	
	pub(super) mode: PageFaultEventNotificationSetting,
	
	/// `ioctls` is written by the ioctl.
	pub(super) ioctls: SupportedInputOutputControlRequests,
}
