// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// The format the BPF program executes over.
///
/// * `nr`: the system call number
/// * `arch`: indicates system call convention as an `AUDIT_ARCH_*` value as defined in `linux/audit.h`.
/// * `instruction_pointer`: at the time of the system call.
/// * `args`: up to 6 system call arguments always stored as 64-bit values regardless of the architecture.
///
/// Size may not actually be `size_of::<Self>()`!
///
/// Size is actually `seccomp_notif_sizes.seccomp_data as usize`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct seccomp_data
{
	/// System call number; usually 0 or greater but can be -1 for some ptrace scenarios.
	///
	/// Often a value in the `SYS` enumeration but not necessarily.
	pub nr: i32,

	/// System call architecture.
	pub arch: AuditArchitecture,

	/// Instruction pointer.
	pub instruction_pointer: u64,

	/// Up to 6 system call arguments; if the system call does not use all 6 arguments, then unused arguments can be set to garbage *under the control of userspace*!
	pub args: [u64; 6],

	variable_size: (),
}

impl seccomp_data
{
	/// Allocate, zeroed.
	#[inline(always)]
	#[allow(unused)]
	pub(crate) fn allocate_zeroed() -> VariablySized<Self>
	{
		VariablySized::allocate_zeroed(seccomp_notif_sizes::get_listener_notification_sizes().seccomp_data as usize)
	}
}
