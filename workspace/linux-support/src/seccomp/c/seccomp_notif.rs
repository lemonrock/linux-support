// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Size may not actually be `size_of::<Self>()`!
///
/// Size is actually `seccomp_notif_sizes.seccomp_notif as usize`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct seccomp_notif
{
	pub(crate) id: u64,

	/// `None` if the task causing the request is not visible (eg in a different namespace).
	pub process_identifier: Option<ProcessIdentifier>,

	/// Currently zero.
	///
	/// Linux documents a flag, `SECCOMP_NOTIF_FLAG_SIGNALED`, but provides no definition of it.
	flags: u32,

	/// Size may not actually be `size_of::<seccomp_data>()`!
	///
	/// Size is actually `seccomp_notif_sizes.seccomp_data as usize`.
	pub data: seccomp_data,

	variable_size: (),
}

impl seccomp_notif
{
	/// Allocate, zeroed.
	#[inline(always)]
	#[allow(unused)]
	pub(crate) fn allocate_zeroed() -> VariablySized<Self>
	{
		VariablySized::allocate_zeroed(seccomp_notif_sizes::get_listener_notification_sizes().seccomp_notif)
	}
}
