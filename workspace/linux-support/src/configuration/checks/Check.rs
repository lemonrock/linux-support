// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// A check.
pub trait Check: EnumMessage + IntoEnumIterator + Into<&'static str> + PartialEq + Eq + Hash + Debug + Copy + Clone
{
	#[doc(hidden)]
	const Name: &'static str;

	#[doc(hidden)]
	type CheckArguments;

	#[doc(hidden)]
	#[inline(always)]
	fn run_all_checks(suppressed_checks: &HashSet<Self>, check_arguments: &Self::CheckArguments) -> Result<(), FailedChecks<Self>>
	{
		let mut failed_checks = FailedChecks::new();
		for check in Self::iter()
		{
			if unlikely!(suppressed_checks.contains(&check))
			{
				continue
			}

			if unlikely!(!check.check(check_arguments))
			{
				failed_checks.add_failed_check(check)
			}
		}
		Ok(())
	}

	#[doc(hidden)]
	fn check(self, check_arguments: &Self::CheckArguments) -> bool;
}
