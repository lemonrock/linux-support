// This file is part of linux-support. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT. No part of linux-support, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2020 The developers of linux-support. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/linux-support/master/COPYRIGHT.


/// Failed checks.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct FailedChecks(HashMap<&'static str, String>);

impl FailedChecks
{
	#[inline(always)]
	pub(crate) fn to_message(self, checks_name: &str) -> Result<(), String>
	{
		if self.0.is_empty()
		{
			return Ok(())
		}

		let mut message = format!("The following '{}' checks failed:", checks_name);
		for (name, message) in self.0
		{
			message.push_str(&format!(" The check '{}' ('{}').", name, message))
		}
		Err(message)
	}

	#[inline(always)]
	pub(crate) fn check(&mut self, suppress: &HashSet<&'static str>, check_name: &'static str, message: Cow<'_, str>, true_if_should_not_warn: impl FnOnce() -> bool)
	{
		if true_if_should_not_warn()
		{
			return
		}

		if suppress.contains(check_name)
		{
			return
		}

		let previously_inserted = self.0.insert(check_name, message.into_owned());
		debug_assert!(previously_inserted.is_none())
	}
}
