// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Size may not actually be `size_of::<Self>()`!
///
/// Size is actually `seccomp_notif_sizes.seccomp_notif_resp as usize`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub(crate) struct seccomp_notif_resp
{
	pub(crate) id: u64,

	pub(crate) val: i64,

	pub(crate) error: i32,

	pub(crate) flags: UserNotificationFlags,

	variable_size: (),
}

impl seccomp_notif_resp
{
	/// Allocate, zeroed.
	#[inline(always)]
	#[allow(unused)]
	pub(crate) fn allocate_zeroed() -> VariablySized<Self>
	{
		VariablySized::allocate_zeroed(seccomp_notif_sizes::get_listener_notification_sizes().seccomp_notif_resp)
	}
}
