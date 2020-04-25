// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// An User Identifier or a Group Identifier.
///
/// Linux uid and gid values can never be negative (unlike, say HP-UX).
pub trait UserOrGroupIdentifier: Sized + Copy + Into<u32> + Default + PartialEq + Eq + PartialOrd + Ord + Hash
{
	/// Zero.
	const Zero: Self;

	/// File name.
	const FileName: &'static str;

	/// Root.
	const Root: Self;

	/// Current real value (also default).
	fn current_real() -> Self;

	/// Current effective value.
	fn current_effective() -> Self;

	/// Current real, effective and saved-set values.
	fn current_real_effective_and_saved_set() -> (Self, Self, Self);

	/// Is root user (or group).
	#[inline(always)]
	fn is_root(self) -> bool
	{
		self == Self::Root
	}
}
