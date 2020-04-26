// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Failed checks.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct FailedChecks<C: Check>(HashSet<C>);

impl<C: Check> Display for FailedChecks<C>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		write!(f, "{}", self.to_message())
	}
}

impl<C: Check> error::Error for FailedChecks<C>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		None
	}
}

impl<C: Check> FailedChecks<C>
{
	#[inline(always)]
	pub(crate) fn new() -> Self
	{
		Self(HashSet::new())
	}

	#[inline(always)]
	pub(crate) fn add_failed_check(&mut self, check: C)
	{
		let previously_inserted = self.0.insert(check);
		debug_assert_eq!(previously_inserted, false)
	}

	#[inline(always)]
	pub(crate) fn to_message(&self) -> String
	{
		let mut message = format!("The following {} checks failed:", C::Name);
		for check in self.0.iter()
		{
			message.push_str(&format!(" {}({})", check.get_message().unwrap(), check.get_detailed_message().unwrap()))
		}
		message
	}
}
